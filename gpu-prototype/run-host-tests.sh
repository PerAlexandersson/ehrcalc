#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
image_name=${EHRGPU_IMAGE:-ehrcalc-rocm:7.2.4}
build_dir=${EHRGPU_BUILD_DIR:-/mnt/2TB-Babel/ai-storage/gpu-build}

if ! docker image inspect "$image_name" >/dev/null 2>&1; then
    echo "missing Docker image $image_name; build it from gpu-prototype/Dockerfile" >&2
    exit 1
fi
mkdir -p "$build_dir"
exec 9>"$build_dir/ehrgpu.lock"
if ! flock -n 9; then
    echo "another Ehrcalc GPU build or run holds $build_dir/ehrgpu.lock" >&2
    exit 1
fi
docker run --rm \
    -v "$repo_root/gpu-prototype:/source:ro" \
    -v "$build_dir:/build" \
    "$image_name" bash -lc \
    'hipcc --offload-arch=gfx1201 -O3 -std=c++17 \
       -Wall -Wextra -Werror -Wno-unused-function \
       -Wno-unused-const-variable -DEHRGPU_HOST_TEST \
       /source/packed_flagged_kostka_gpu_resident.hip.cpp \
       -o /build/packed-flagged-kostka-host-tests \
     && /build/packed-flagged-kostka-host-tests \
     && hipcc --offload-arch=gfx1201 -O3 -std=c++17 \
       -Wall -Wextra -Werror -Wno-unused-function \
       -Wno-unused-const-variable -DEHRGPU_HOST_TEST \
       /source/order_polytope_gpu_resident.hip.cpp \
       -o /build/order-polytope-host-tests \
     && /build/order-polytope-host-tests'
