# Ehrcalc GPU prototype

This directory contains isolated HIP microbenchmarks for the packed modular
Kostka backend. It does not change Ehrcalc's default exact implementation.

Build the userspace-only development image:

```text
docker build -t ehrcalc-rocm:7.2.4 gpu-prototype
```

Compile and run the exact sort/reduce check with access to the host AMD GPU:

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
