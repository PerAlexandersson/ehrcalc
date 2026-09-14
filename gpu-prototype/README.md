# Ehrcalc GPU prototype

This directory contains isolated HIP implementations and microbenchmarks for
packed modular counting. It does not change Ehrcalc's default exact CPU
implementation. The currently supported GPU routes are flagged/skew Kostka,
generic order-poset maps, order-polytope Ehrhart interpolation, and an exact
Kostka-inversion route for Littlewood--Richardson coefficients.

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
  [MAX_TRANSITIONS] [MODULI] \
  [FORBIDDEN_MASKS STRICT_LOWER_MASKS STRICT_DIAGONAL_MASKS]
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

The final three lists are optional per-label row bitmasks. Bit `r` in
`FORBIDDEN_MASKS[i]` forces label `i+1` to occur zero times in zero-indexed row
`r`; this represents arbitrary forbidden `(row,label)` pairs in a
complement-row lift of an individual Kogan face. `STRICT_LOWER_MASKS` requires
positive row increments, while `STRICT_DIAGONAL_MASKS` requires strict
horizontal-strip diagonal inequalities. Flags, face masks, and both strictness
masks are enforced together in transition counting and emission. The masked
interface supports at most 32 shape rows.

Compile and run the device-free host regressions:

```text
gpu-prototype/run-host-tests.sh
```

These checks cover 64-bit transition-total accounting, an empty frontier,
strict numeric parsing, a non-interval Kogan-face hole, lower and diagonal
strictness, a flag-forced zero row, and agreement between transition counting
and emission. The regular HIP source is also safe against an exclusive scan
exceeding 32 bits: transition counts and offsets use 64-bit storage, while the
configured per-layer cap remains below `2^32-1`.

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

For a relative-interior count, including an individual Kogan face with
non-interval holes, use the strict wrapper:

```text
gpu-prototype/run-exact-strict.sh \
  UPPER_BOUND DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS \
  [MAX_TRANSITIONS [FORBIDDEN_MASKS]]
```

The wrapper calls the exact Rust affine-structure analyzer on the undilated
input, derives the lower and diagonal strictness masks, and passes them to the
same GPU/CRT driver. The analyzer propagates flags and forbidden equalities,
closes directed cycles in the GT order graph, propagates exact rational
component bounds through the level-weight equations, and computes their exact
rank. Small masked faces are exhaustively checked against
Ehrhart--Macdonald reciprocity in the engine tests. The standalone helper
`derive_masked_interior` prints the dimension and masks as JSON for other KTT
drivers.

Counts beyond the legacy wrapper's `u64` bound use the separate wide mode:

```text
gpu-prototype/run-exact-strict-wide.sh \
  BOUND_OR_BITS DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS \
  [MAX_TRANSITIONS [FORBIDDEN_MASKS]]
```

`BOUND_OR_BITS` is either a nonnegative decimal integer or `bits:N`, which
means the certified bound `2^N-1`. The wide driver collects sequential batches
from 24 configured 31-bit primes and requires their product to exceed twice
the bound before reconstructing. It supports bounds through 700 bits. Its
machine-readable `EHRGPU_CRT` provenance record contains the normalized bound,
threshold, moduli, residues, reconstruction, HIP source hash, and hashes of the
GPU binaries used for each batch. The original strict command and its `u64`
host-bridge contract remain unchanged.

These APIs count one face. A union of reduced Kogan faces or a full key
polynomial still needs overlap deduplication or inclusion--exclusion outside
this kernel; summing face counts directly is not valid in general.

## Order polytopes

`order_polytope_gpu_resident.hip.cpp` implements the existing order-poset
frontier DP on the GPU. At each vertex it emits the possible new frontier
states, then uses rocPRIM radix sort and modular reduce-by-key. Input labels do
not need to be a natural labeling: the host validates acyclicity and computes
a deterministic topological relabeling before it builds the frontier plans.

Count weak or strict order-preserving maps exactly with:

```text
gpu-prototype/run-order-exact.sh \
  VERTICES COVERS COLORS weak|strict [MAX_TRANSITIONS]
```

`COVERS` uses zero-based `lower<upper` pairs separated by commas; use `-` for
an antichain. The wrapper uses the certified bound `COLORS^VERTICES`, selects
enough CRT primes, and refuses an answer if the eight available primes do not
exceed that bound. The packed device state requires
`maximum_frontier * ceil(log2(COLORS+1)) <= 128`.

Compute the complete order-polytope Ehrhart polynomial from `n+1` exact
positive samples with:

```text
gpu-prototype/run-order-ehrhart.sh VERTICES COVERS [MAX_TRANSITIONS]
```

The final JSON is produced by Ehrcalc's normal exact interpolation layer. Both
weak and strict kernels are exercised by the host and GPU smoke suites.

## Littlewood--Richardson coefficients

The existing flagged-Kostka kernel does not directly encode the extra
Yamanouchi state in the primary LR dynamic program. The first exact GPU route
therefore uses the unitriangular Kostka identity already implemented in
`ehrcalc-kostka-engine`:

```text
gpu-prototype/run-lr-exact.sh LAMBDA MU NU [MAX_TRANSITIONS]
```

`lr_kostka_plan` generates only the skew and ordinary Kostka jobs required up
to `NU`, omits dominance-forced zeros, and gives every job the multinomial
bound obtained by forgetting tableau inequalities. The wrapper evaluates the
jobs with the GPU/CRT Kostka driver and reconstructs the LR coefficient with
exact `BigInt` back-substitution. This route is intentionally a practical
baseline rather than an optimal LR kernel: the number of jobs grows with the
partitions preceding `NU` in dominance order.

`gpu-prototype/run-lr-smoke.sh` checks one zero and one nonzero coefficient
against the independent Yamanouchi-DP fixtures.

The host-test, smoke, and exact drivers share a nonblocking build-directory
lock. Exact-run logs include a digest of the arguments and HIP source so two
different invocations cannot silently reuse the same log filename.

Do not replace the normal `hipMalloc`/`hipFree` buffers with ROCm asynchronous
memory-pool allocation on this machine. A measured `hipMallocAsync` experiment
caused an AMDGPU memory-aperture fault and wedged the display until a
suspend/resume cycle. That implementation was removed and its binary trashed.

The measured real-layer and full-DP results are in `gpu-prototype/results/`.
