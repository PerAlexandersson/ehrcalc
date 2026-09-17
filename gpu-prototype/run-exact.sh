#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 7 || ($# -gt 8 && $# -ne 11 && $# -ne 13) ]]; then
    echo "usage: $0 UPPER_BOUND DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS [MAX_TRANSITIONS [FORBIDDEN_MASKS STRICT_LOWER_MASKS STRICT_DIAGONAL_MASKS [LEVEL_LOWER_BOUNDS LEVEL_UPPER_BOUNDS]]]" >&2
    exit 2
fi

upper_bound=$1
dilation=$2
outer=$3
inner=$4
weight=$5
upper_flags=$6
lower_flags=$7
maximum_transitions=${8:-150000000}
chunk_transitions=${EHRGPU_CHUNK_TRANSITIONS:-}
forbidden_masks=${9:--}
strict_lower_masks=${10:--}
strict_diagonal_masks=${11:--}
level_lower_bounds=${12:--}
level_upper_bounds=${13:--}
crt_margin=${EHRGPU_CRT_MARGIN:-1}

read -r maximum_transitions chunk_transitions < <(
    python3 - "$maximum_transitions" "$chunk_transitions" <<'PY'
import re
import sys

total_text, chunk_text = sys.argv[1:]
if not re.fullmatch(r"[0-9]+", total_text):
    raise SystemExit("total transition limit must be an unsigned decimal integer")
total = int(total_text)
if not 1 <= total < 2**32 - 1:
    raise SystemExit("total transition limit must lie in 1..2^32-2")
if chunk_text:
    if not re.fullmatch(r"[0-9]+", chunk_text):
        raise SystemExit("chunk transition limit must be an unsigned decimal integer")
    chunk = int(chunk_text)
else:
    chunk = min(total, 150_000_000)
if not 1 <= chunk <= total:
    raise SystemExit("chunk transition limit must lie in 1..total")
print(total, chunk)
PY
)

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
image_name=${EHRGPU_IMAGE:-ehrcalc-rocm:7.2.4}
build_dir=${EHRGPU_BUILD_DIR:-/mnt/2TB-Babel/ai-storage/gpu-build}
cargo_target=${EHRGPU_CARGO_TARGET_DIR:-/mnt/2TB-Babel/ai-storage/cargo-target}
reconstruct="$cargo_target/release/reconstruct_modular"
run_token=${EHRGPU_RUN_TOKEN:-standalone-$$-${RANDOM}}
if [[ ! $run_token =~ ^[A-Za-z0-9_.-]{1,64}$ ]]; then
    echo "EHRGPU_RUN_TOKEN must use 1..64 safe identifier characters" >&2
    exit 2
fi
container_names=()
cleanup_containers() {
    local name
    for name in "${container_names[@]}"; do
        docker rm -f "$name" >/dev/null 2>&1 || true
    done
}
trap cleanup_containers EXIT HUP INT TERM
modulus_candidates=(
    2147483647 2147483629 2147483587 2147483579
    2147483563 2147483549 2147483543 2147483497
    2147483489 2147483477 2147483423 2147483399
    2147483353 2147483323 2147483269 2147483249
    2147483237 2147483179 2147483171 2147483137
    2147483123 2147483077 2147483069 2147483059
)

maximum_strip_size=$(python3 - "$dilation" "$weight" <<'PY'
import re
import sys

dilation_text, weight_text = sys.argv[1:]
if not re.fullmatch(r"[0-9]+", dilation_text):
    raise SystemExit("dilation must be an unsigned decimal integer")
if not weight_text or any(not re.fullmatch(r"[0-9]+", part) for part in weight_text.split(",")):
    raise SystemExit("weight must be a nonempty comma-separated unsigned integer list")
dilation = int(dilation_text)
maximum = dilation * max(map(int, weight_text.split(",")))
if maximum > 512:
    raise SystemExit("scaled weight part exceeds GPU strip-size limit 512")
print(max(1, maximum))
PY
)

if [[ $crt_margin != 1 && $crt_margin != 2 ]]; then
    echo "EHRGPU_CRT_MARGIN must be 1 or 2" >&2
    exit 2
fi
modulus_threshold=$(python3 - "$upper_bound" "$crt_margin" <<'PY'
import sys

bound = int(sys.argv[1])
margin = int(sys.argv[2])
if bound < 0:
    raise SystemExit("upper bound must be nonnegative")
print(bound * margin)
PY
)

if ! docker image inspect "$image_name" >/dev/null 2>&1; then
    echo "missing Docker image $image_name; build it from gpu-prototype/Dockerfile" >&2
    exit 1
fi
mkdir -p "$build_dir" "$cargo_target"
exec 9>"$build_dir/ehrgpu.lock"
if ! flock -n 9; then
    echo "another Ehrcalc GPU build or run holds $build_dir/ehrgpu.lock" >&2
    exit 1
fi
run_id=$run_token
CARGO_TARGET_DIR="$cargo_target" cargo build \
    --manifest-path "$repo_root/Cargo.toml" --release \
    -p ehrcalc-kostka-engine --bin reconstruct_modular
needed_moduli=0
trial_moduli=()
trial_residues=()
for modulus in "${modulus_candidates[@]}"; do
    trial_moduli+=("$modulus")
    trial_residues+=(0)
    trial_moduli_csv=$(IFS=,; echo "${trial_moduli[*]}")
    trial_residues_csv=$(IFS=,; echo "${trial_residues[*]}")
    if "$reconstruct" "$modulus_threshold" "$trial_residues_csv" \
        "$trial_moduli_csv" >/dev/null 2>&1; then
        needed_moduli=${#trial_moduli[@]}
        break
    fi
done
if ((needed_moduli == 0)); then
    echo "configured moduli do not exceed the required CRT threshold" >&2
    exit 1
fi

residues=()
moduli=()
binary_sha256s=()
binary_names=()
declare -A compiled_binary_sha256=()
source_path="$repo_root/gpu-prototype/packed_flagged_kostka_gpu_resident.hip.cpp"
source_hash=$(sha256sum "$source_path" | cut -d' ' -f1)
batch_start=0
while ((batch_start < needed_moduli)); do
    remaining=$((needed_moduli - batch_start))
    lane_count=$((remaining < 3 ? remaining : 3))
    batch_moduli=("${modulus_candidates[@]:batch_start:lane_count}")
    batch_moduli_csv=$(IFS=,; echo "${batch_moduli[*]}")
    binary_name="packed-flagged-kostka-gpu-resident-lanes${lane_count}-strip${maximum_strip_size}"
    binary_path="$build_dir/$binary_name"
    binary_sha256=${compiled_binary_sha256[$lane_count]:-}
    if [[ -z $binary_sha256 ]]; then
        compile_container="ehrgpu-${run_token}-compile-${batch_start}"
        container_names+=("$compile_container")
        set +e
        docker run --rm --name "$compile_container" \
            --label "ai.ehrcalc.euler-gpu-run=$run_token" \
            --label "ai.ehrcalc.euler-gpu-role=compile" \
            --device=/dev/kfd --device=/dev/dri \
            --group-add video --security-opt seccomp=unconfined \
            -v "$repo_root/gpu-prototype:/source:ro" \
            -v "$build_dir:/build" \
            "$image_name" bash -lc \
            "hipcc --offload-arch=gfx1201 -O3 -std=c++17 \
              -DEHRGPU_RESIDUE_LANES=$lane_count \
              -DEHRGPU_MAX_STRIP_SIZE=$maximum_strip_size \
              /source/packed_flagged_kostka_gpu_resident.hip.cpp \
              -o /build/$binary_name"
        compile_status=$?
        set -e
        if ((compile_status != 0)); then
            echo "EHRGPU_FAILURE {\"kind\":\"compile\",\"batch\":$batch_start,\"container_exit_code\":$compile_status}" >&2
            exit "$compile_status"
        fi
        binary_sha256=$(sha256sum "$binary_path" | cut -d' ' -f1)
        compiled_binary_sha256[$lane_count]=$binary_sha256
    fi
    binary_sha256s+=("$binary_sha256")
    binary_names+=("$binary_name")
    log_path="$build_dir/exact-${run_id}-batch${batch_start}.log"
    execute_container="ehrgpu-${run_token}-execute-${batch_start}"
    container_names+=("$execute_container")
    set +e
    gpu_output=$(
        docker run --rm --name "$execute_container" \
            --label "ai.ehrcalc.euler-gpu-run=$run_token" \
            --label "ai.ehrcalc.euler-gpu-role=execute" \
            --device=/dev/kfd --device=/dev/dri \
            --group-add video --security-opt seccomp=unconfined \
            -e EHRGPU_CHUNK_TRANSITIONS="$chunk_transitions" \
            -v "$build_dir:/build" \
            "$image_name" "/build/$binary_name" \
            "$dilation" "$outer" "$inner" "$weight" "$upper_flags" \
            "$lower_flags" "$maximum_transitions" "$batch_moduli_csv" \
            "$forbidden_masks" "$strict_lower_masks" "$strict_diagonal_masks" \
            "$level_lower_bounds" "$level_upper_bounds" \
            2>"$log_path"
    )
    gpu_status=$?
    set -e
    if ((gpu_status != 0)); then
        echo "GPU batch failed; diagnostic tail from $log_path:" >&2
        tail -n 80 "$log_path" >&2 || true
        log_name=$(basename "$log_path")
        echo "EHRGPU_FAILURE {\"kind\":\"gpu_batch\",\"batch\":$batch_start,\"container_exit_code\":$gpu_status,\"log_name\":\"$log_name\"}" >&2
        exit "$gpu_status"
    fi
    batch_residues_csv=$(
        sed -n 's/.*"residues":\[\([^]]*\)\].*/\1/p' <<<"$gpu_output"
    )
    if [[ -z $batch_residues_csv ]]; then
        echo "failed to parse GPU result; see $log_path" >&2
        exit 1
    fi
    IFS=, read -r -a batch_residues <<<"$batch_residues_csv"
    residues+=("${batch_residues[@]}")
    moduli+=("${batch_moduli[@]}")
    residues_csv=$(IFS=,; echo "${residues[*]}")
    moduli_csv=$(IFS=,; echo "${moduli[*]}")
    echo "GPU batch: moduli=$batch_moduli_csv residues=$batch_residues_csv" >&2
    if ((${#moduli[@]} < needed_moduli)); then
        batch_start=$((batch_start + lane_count))
        continue
    fi
    if exact=$(
        "$reconstruct" "$upper_bound" "$residues_csv" "$moduli_csv" 2>/dev/null
    ); then
        binary_sha256s_json=$(printf '\"%s\",' "${binary_sha256s[@]}")
        binary_sha256s_json=${binary_sha256s_json%,}
        binary_names_json=$(printf '\"%s\",' "${binary_names[@]}")
        binary_names_json=${binary_names_json%,}
        provenance_json=""
        if [[ ${EHRGPU_DERIVE_BINARY_SHA256:-} =~ ^[0-9a-f]{64}$ && ${EHRGPU_CONSTRAINT_PAYLOAD_SHA256:-} =~ ^[0-9a-f]{64}$ ]]; then
            provenance_json=",\"derive_binary_sha256\":\"$EHRGPU_DERIVE_BINARY_SHA256\",\"constraint_payload_sha256\":\"$EHRGPU_CONSTRAINT_PAYLOAD_SHA256\""
        fi
        echo "EHRGPU_CRT {\"upper_bound\":\"$upper_bound\",\"modulus_threshold\":\"$modulus_threshold\",\"maximum_strip_size\":$maximum_strip_size,\"maximum_transitions\":$maximum_transitions,\"chunk_transitions\":$chunk_transitions,\"moduli\":[$moduli_csv],\"residues\":[$residues_csv],\"reconstructed\":\"$exact\",\"source_sha256\":\"$source_hash\",\"binary_names\":[$binary_names_json],\"binary_sha256s\":[$binary_sha256s_json]$provenance_json}" >&2
        echo "$exact"
        exit 0
    fi
    batch_start=$((batch_start + lane_count))
done

echo "GPU residues did not reconstruct an answer below the supplied bound" >&2
exit 1
