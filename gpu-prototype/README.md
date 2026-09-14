# Ehrcalc GPU prototype

This directory contains isolated HIP microbenchmarks for the packed modular
Kostka backend. It does not change Ehrcalc's default exact implementation.

Build the userspace-only development image:

```text
docker build -t ehrcalc-rocm:7.2.4 gpu-prototype
```

Compile and run the synthetic sort/reduce check with access to the host AMD GPU:

```text
docker run --rm \
  --device=/dev/kfd --device=/dev/dri \
  --group-add video --security-opt seccomp=unconfined \
  -v "$PWD/gpu-prototype:/source:ro" \
  -v /mnt/2TB-Babel/ai-storage/gpu-build:/build \
  ehrcalc-rocm:7.2.4 bash -lc \
  'hipcc --offload-arch=gfx1201 -O3 -std=c++17 \
     /source/packed_modular_reduce.hip.cpp -o /build/packed-modular-reduce \
   && /build/packed-modular-reduce 1000000 250000'
```

The program generates deterministic 128-bit destination keys and modular
contributions, sorts them with rocPRIM, reduces equal keys modulo a 31-bit
prime, and compares every output record with an independent CPU sort/reduce.
It exits nonzero on any mismatch.

This tests the aggregation half of a future DP level. Transition generation is
still on the CPU and is not represented in this microbenchmark.

The directory now also contains two end-to-end modular prototypes:

- `packed_flagged_kostka_gpu.hip.cpp` enumerates transitions on the CPU and
  sends every layer to the GPU. It is retained as a measured negative result.
- `packed_flagged_kostka_gpu_resident.hip.cpp` keeps states on the GPU and
  counts and emits horizontal-strip transitions there before rocPRIM
  sort/reduce. This is the useful implementation.

Compile the GPU-resident variant:

```text
docker run --rm \
  --device=/dev/kfd --device=/dev/dri \
  --group-add video --security-opt seccomp=unconfined \
  -v "$PWD/gpu-prototype:/source:ro" \
  -v /mnt/2TB-Babel/ai-storage/gpu-build:/build \
  ehrcalc-rocm:7.2.4 bash -lc \
  'hipcc --offload-arch=gfx1201 -O3 -std=c++17 \
     /source/packed_flagged_kostka_gpu_resident.hip.cpp \
     -o /build/packed-flagged-kostka-gpu-resident'
```

Its command line is:

```text
packed-flagged-kostka-gpu-resident \
  DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS \
  [MAX_TRANSITIONS] [MODULI]
```

Lists are comma-separated; use `-` for an empty partition or absent flags.
The default transition limit is 150 million and the default modulus is
2,147,483,647. The modulus must be in `2..2^31`; exact integer answers require
enough pairwise-coprime residue runs to exceed a separately certified bound.
Numeric arguments are parsed strictly: suffixes, empty list entries, overflow,
and non-coprime modulus lists are rejected. A mathematically valid empty DP
frontier returns zero rather than being treated as a runtime failure.
Compile with `-DEHRGPU_RESIDUE_LANES=N` and pass `N` comma-separated moduli to
carry as many as eight residue lanes together. The exact driver batches at most
three lanes because that is the measured safe memory point on the 16 GiB card.

Compile and run the device-free host regressions:

```text
gpu-prototype/run-host-tests.sh
```

These checks cover 64-bit transition-total accounting, an empty frontier, and
strict numeric parsing. The regular HIP source is also safe against an exclusive
scan exceeding 32 bits: transition counts and offsets use 64-bit storage, while
the configured per-layer cap remains below `2^32-1`.

Run the small two-modulus CPU/GPU smoke comparison from the repository root:

```text
gpu-prototype/run-smoke.sh
```

This builds outside the repository, compares skew, flagged, and zero-weight
cases with the Rust packed-modular reference, and exits on the first mismatch.
For an exact integer computation, supply a mathematically certified upper bound:

```text
gpu-prototype/run-exact.sh \
  UPPER_BOUND DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS
```

`run-exact.sh` first determines how many primes are required for the bound,
compiles a one-, two-, or three-lane kernel, and evaluates batches of up to
three pairwise-coprime residues. It prints an integer only after the combined
modulus exceeds the supplied bound and the reconstruction lies below it;
otherwise it fails rather than treating an ambiguous residue as exact.
The host-test, smoke, and exact drivers share a nonblocking build-directory
lock. Exact-run logs include a digest of the arguments and HIP source so two
different invocations cannot silently reuse the same log filename.

Do not replace the normal `hipMalloc`/`hipFree` buffers with ROCm asynchronous
memory-pool allocation on this machine. A measured `hipMallocAsync` experiment
caused an AMDGPU memory-aperture fault and wedged the display until a
suspend/resume cycle. That implementation was removed and its binary trashed.

The measured real-layer and full-DP results are in `gpu-prototype/results/`.
