# Initial RX 9070 XT packed modular sort/reduce comparison

Date: 2026-09-13

Commit base: `f0a52ee` plus the uncommitted GPU prototype.

Hardware: AMD Radeon RX 9070 XT (`gfx1201`, 16 GiB) on Euler.

Software: AMD ROCm 7.2.4 development container, rocPRIM 4.2.0, `hipcc -O3
--offload-arch=gfx1201`. The host graphics driver was not replaced.

Each case generated deterministic 128-bit keys and 31-bit modular values. The
GPU performed a rocPRIM radix sort and modular reduce-by-key. Every returned
key and value was compared with an independent CPU `std::sort` plus serial
modular reduction. All cases matched exactly.

GPU sort and reduction times are medians of five runs after one warm-up. GPU
pipeline time includes the measured host-to-device transfer, median sort,
median reduction, and device-to-host transfer. It excludes allocation, process
startup, and deterministic input generation. CPU time excludes input generation
and is a single `std::sort`/reduce run. Therefore this is an aggregation-kernel
comparison, not yet an end-to-end Ehrcalc speedup.

| Records | Requested groups | Actual groups | GPU pipeline ms | CPU reference ms | CPU/GPU |
|---:|---:|---:|---:|---:|---:|
| 100,000 | 100,000 | 63,167 | 5.506 | 6.687 | 1.21x |
| 100,000 | 25,000 | 24,551 | 4.761 | 6.093 | 1.28x |
| 100,000 | 1,562 | 1,562 | 4.764 | 4.606 | 0.97x |
| 1,000,000 | 1,000,000 | 632,061 | 7.135 | 74.991 | 10.51x |
| 1,000,000 | 250,000 | 245,417 | 6.770 | 69.235 | 10.23x |
| 1,000,000 | 15,625 | 15,625 | 6.997 | 55.269 | 7.90x |
| 5,000,000 | 5,000,000 | 3,159,944 | 18.771 | 403.870 | 21.52x |
| 5,000,000 | 1,250,000 | 1,227,070 | 16.969 | 380.795 | 22.44x |
| 5,000,000 | 78,125 | 78,125 | 16.162 | 307.706 | 19.04x |
| 10,000,000 | 10,000,000 | 6,320,779 | 31.417 | 850.772 | 27.08x |
| 10,000,000 | 2,500,000 | 2,454,346 | 25.910 | 783.551 | 30.24x |
| 10,000,000 | 156,250 | 156,250 | 27.616 | 639.072 | 23.14x |

At one million records with approximately four emitted records per requested
group, the median device work split was:

```text
host to device   4.852 ms
radix sort       1.557 ms
reduce by key    0.057 ms
device to host   0.304 ms
total            6.770 ms
CPU reference   69.235 ms
```

At ten million records in the corresponding collision regime:

```text
host to device  10.706 ms
radix sort      12.771 ms
reduce by key    0.549 ms
device to host   1.885 ms
total           25.910 ms
CPU reference  783.551 ms
```

The crossover is between 100,000 and one million emitted records on this first
implementation. Host-to-device startup/transfer dominates small cases. Radix
sort dominates large GPU cases, while reduce-by-key is inexpensive for the
tested distributions.

## Limitations and next comparison

- Synthetic keys do not reproduce the exact key locality or fanout of a Kostka
  DP layer.
- One residue lane was tested; the intended exact engine uses several lanes.
- The CPU comparator is comparison sort, not the planned CPU radix-sort backend.
- Recursive horizontal-strip transition generation is not included.
- Allocation and process startup are excluded.
- No 100-million-record or chunked run has been attempted.

The next gate is exporting real transition records from selected Ehrcalc
levels, replaying them through this kernel, and comparing both results and the
aggregation phase timing with the packed modular CPU hash backend.
