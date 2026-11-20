#!/usr/bin/env python3
"""Python 3.12 stdlib runner for one portable Linux x86-64 benchmark binary.

Each case/policy gets its own process, 8 GiB address-space cap, CPU and wall
limits. JSONL is append-only and resumable by (id, policy, mode, dilation,
strict, max_states). The whole job stops within six hours, including its last
child. Use heavy.sh JOB_MB=9000 -- python3.12 ... on the local coordinator.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import signal
import subprocess
import tempfile
import time


def arguments():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--inputs", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--policies", nargs="+", choices=["legacy-auto", "auto", "gt-level"], default=["legacy-auto", "auto"])
    parser.add_argument("--timeout", type=float, default=45)
    parser.add_argument("--job-seconds", type=float, default=21_600)
    parser.add_argument("--max-states", type=int, default=2_000_000)
    parser.add_argument("--limit", type=int, default=100)
    parser.add_argument("--start", type=int, default=0)
    parser.add_argument("--mode", choices=["fit", "count"], default="fit")
    parser.add_argument("--dilation", type=int, default=2)
    parser.add_argument("--strict", action="store_true")
    args = parser.parse_args()
    if not (0 < args.timeout <= 21_600 and 0 < args.job_seconds <= 21_600):
        parser.error("timeouts must be positive and at most six hours")
    if args.max_states < 1 or not 1 <= args.limit <= 100 or args.start < 0:
        parser.error("require max-states > 0, 1 <= limit <= 100, start >= 0")
    args.binary = args.binary.resolve(strict=True)
    return args


def key(case, policy, args):
    return (case["id"], policy, args.mode, args.dilation, args.strict, args.max_states)


def run_child(binary, request, wall_seconds):
    def limits():
        resource.setrlimit(resource.RLIMIT_AS, (8 * 1024**3, 8 * 1024**3))
        resource.setrlimit(resource.RLIMIT_CPU, (max(1, int(wall_seconds) + 1), max(2, int(wall_seconds) + 2)))
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))

    started = time.monotonic()
    environment = {"PATH": os.defpath, "LANG": "C.UTF-8", "RAYON_NUM_THREADS": "1", "OMP_NUM_THREADS": "1"}
    with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
        child = subprocess.Popen([str(binary)], stdin=subprocess.PIPE, stdout=stdout, stderr=stderr,
                                 process_group=0, preexec_fn=limits, env=environment)
        session_inherited = os.getsid(child.pid) == os.getsid(0)
        private_process_group = os.getpgid(child.pid) == child.pid
        child.stdin.write(json.dumps(request).encode())
        child.stdin.close()
        timed_out = False
        while True:
            pid, status, usage = os.wait4(child.pid, os.WNOHANG)
            if pid:
                child.returncode = os.waitstatus_to_exitcode(status)
                break
            if time.monotonic() - started >= wall_seconds:
                timed_out = True
                # Only this runner's new process group is terminated.
                try:
                    os.killpg(child.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                _, status, usage = os.wait4(child.pid, 0)
                child.returncode = os.waitstatus_to_exitcode(status)
                break
            time.sleep(min(0.05, max(0.001, wall_seconds - (time.monotonic() - started))))
        stdout.seek(0)
        events = []
        for line in stdout:
            try:
                events.append(json.loads(line))
            except (ValueError, UnicodeDecodeError):
                events.append({"event": "invalid-output", "text": line.decode(errors="replace")[:1000]})
        stderr.seek(0)
        result = next((e for e in reversed(events) if e.get("event") == "result"), {})
        return {
            "status": "timeout" if timed_out else result.get("status", "process-error"),
            "returncode": child.returncode, "elapsed_seconds": time.monotonic() - started,
            "session_inherited": session_inherited, "private_process_group": private_process_group,
            "peak_rss_kib": usage.ru_maxrss, "cpu_seconds": usage.ru_utime + usage.ru_stime,
            "peak_states": result.get("peak_states", max(
                (e["peak_states"] for e in events if e.get("event") == "sample"), default=None)),
            "peak_is_lower_bound": result.get("peak_is_lower_bound", True),
            "events": events, "stderr": stderr.read(8192).decode(errors="replace"),
        }


def main():
    args = arguments()
    started = time.monotonic()
    binary_hash = hashlib.file_digest(args.binary.open("rb"), "sha256").hexdigest()
    input_hash = hashlib.file_digest(args.inputs.open("rb"), "sha256").hexdigest()
    cases = [json.loads(line) for line in args.inputs.read_text().splitlines() if line.strip()]
    done = set()
    if args.output.exists():
        for line in args.output.read_text().splitlines():
            record = json.loads(line)  # Refuse a torn line; preserve it for review.
            if record.get("binary_sha256") != binary_hash or record.get("input_sha256") != input_hash:
                raise SystemExit("refusing to mix different binary/input provenance in one output")
            done.add(tuple(record["key"]))
    with args.output.open("a") as output:
        for case in cases[args.start:args.start + args.limit]:
            for policy in args.policies:
                run_key = key(case, policy, args)
                if run_key in done:
                    continue
                remaining = args.job_seconds - (time.monotonic() - started) - 1
                if remaining <= 0:
                    return
                request = {"system": case["system"], "policy": policy, "max_states": args.max_states,
                           "mode": args.mode, "dilation": args.dilation, "strict": args.strict}
                result = run_child(args.binary, request, min(args.timeout, remaining))
                record = {"id": case["id"], "dimension": case["dimension"], "phase": case["phase"],
                          "policy": policy, "key": run_key, "binary_sha256": binary_hash,
                          "input_sha256": input_hash, "request": {k: v for k, v in request.items() if k != "system"},
                          "limits": {"address_space_bytes": 8 * 1024**3, "wall_seconds": min(args.timeout, remaining),
                                     "job_seconds": args.job_seconds}, **result}
                output.write(json.dumps(record, separators=(",", ":")) + "\n")
                output.flush()
                os.fsync(output.fileno())
                print(f"{case['id']} d={case['dimension']} {policy}: {result['status']} "
                      f"peak={result['peak_states']} {result['elapsed_seconds']:.3f}s "
                      f"RSS={result['peak_rss_kib']}KiB", flush=True)


if __name__ == "__main__":
    main()
