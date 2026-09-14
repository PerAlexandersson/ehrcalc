#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
image_name=${EHRGPU_IMAGE:-ehrcalc-rocm:7.2.4}
build_dir=${EHRGPU_BUILD_DIR:-/mnt/2TB-Babel/ai-storage/gpu-build}
cargo_target=${EHRGPU_CARGO_TARGET_DIR:-/mnt/2TB-Babel/ai-storage/cargo-target}
reference="$cargo_target/release/export_modular_layer"

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

CARGO_TARGET_DIR="$cargo_target" cargo build \
    --manifest-path "$repo_root/Cargo.toml" --release \
    -p ehrcalc-kostka-engine --bin export_modular_layer

docker run --rm \
    --device=/dev/kfd --device=/dev/dri \
    --group-add video --security-opt seccomp=unconfined \
    -v "$repo_root/gpu-prototype:/source:ro" \
    -v "$build_dir:/build" \
    "$image_name" bash -lc \
    'hipcc --offload-arch=gfx1201 -O3 -std=c++17 \
       /source/packed_flagged_kostka_gpu_resident.hip.cpp \
       -o /build/packed-flagged-kostka-gpu-resident \
     && hipcc --offload-arch=gfx1201 -O3 -std=c++17 \
       /source/order_polytope_gpu_resident.hip.cpp \
       -o /build/order-polytope-gpu-resident'

run_case() {
    local name=$1
    local dilation=$2
    local outer=$3
    local inner=$4
    local weight=$5
    local upper=$6
    local lower=$7
    local modulus=$8
    local forbidden=${9:--}
    local strict_lower=${10:--}
    local strict_diagonal=${11:--}
    local cpu_output
    local gpu_output
    local cpu_residue
    local gpu_residue

    cpu_output=$(
        "$reference" - 0 "$dilation" "$outer" "$inner" "$weight" \
            "$upper" "$lower" "$modulus" "$forbidden" "$strict_lower" \
            "$strict_diagonal" 2>&1
    )
    cpu_residue=$(sed -n 's/.*residues=\[\([0-9][0-9]*\)\].*/\1/p' <<<"$cpu_output")
    gpu_output=$(
        docker run --rm \
            --device=/dev/kfd --device=/dev/dri \
            --group-add video --security-opt seccomp=unconfined \
            -v "$build_dir:/build" \
            "$image_name" /build/packed-flagged-kostka-gpu-resident \
            "$dilation" "$outer" "$inner" \
            "$weight" "$upper" "$lower" 150000000 "$modulus" \
            "$forbidden" "$strict_lower" "$strict_diagonal" \
            2>"$build_dir/smoke-$name-$modulus.log"
    )
    gpu_residue=$(sed -n 's/.*"residue":\([0-9][0-9]*\).*/\1/p' <<<"$gpu_output")
    if [[ -z $cpu_residue || $cpu_residue != "$gpu_residue" ]]; then
        echo "$name modulus $modulus: CPU=$cpu_residue GPU=$gpu_residue" >&2
        exit 1
    fi
    echo "$name modulus $modulus: residue $gpu_residue (match)"
}

run_order_case() {
    local name=$1
    local vertices=$2
    local covers=$3
    local colors=$4
    local mode=$5
    local expected=$6
    local modulus=$7
    local gpu_output
    local gpu_residue

    gpu_output=$(
        docker run --rm \
            --device=/dev/kfd --device=/dev/dri \
            --group-add video --security-opt seccomp=unconfined \
            -v "$build_dir:/build" \
            "$image_name" /build/order-polytope-gpu-resident \
            "$vertices" "$covers" "$colors" "$mode" 150000000 "$modulus" \
            2>"$build_dir/smoke-order-$name-$modulus.log"
    )
    gpu_residue=$(sed -n 's/.*"residue":\([0-9][0-9]*\).*/\1/p' <<<"$gpu_output")
    if [[ -z $gpu_residue || $gpu_residue != "$expected" ]]; then
        echo "$name modulus $modulus: expected=$expected GPU=$gpu_residue" >&2
        exit 1
    fi
    echo "$name modulus $modulus: residue $gpu_residue (match)"
}

for modulus in 2147483647 2147483629; do
    run_case skew 1 5,4,2 1 3,4,3 - - "$modulus"
    run_case flagged 1 4,3,1 1 2,3,2 2,3,3 1,1,2 "$modulus"
    run_case zero-strip 1 3,2 1 2,0,2 - - "$modulus"
    run_case masked-face 1 2,1 - 1,1,1 - - "$modulus" 0,1,0 0,0,1 0,0,0
    run_case strict-forced-diagonal 1 2,1 - 1,1,1 2,2,1 - "$modulus" \
        0,0,0 0,0,0 0,0,2
    run_order_case order-chain 3 '0<1,1<2' 4 weak 20 "$modulus"
    run_order_case order-antichain 3 - 4 weak 64 "$modulus"
    run_order_case order-v-strict 3 '0<2,1<2' 4 strict 14 "$modulus"
done

echo "GPU-resident smoke suite passed"
