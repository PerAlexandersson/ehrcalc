#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 7 || $# -gt 8 ]]; then
    echo "usage: $0 UPPER_BOUND DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS [MAX_TRANSITIONS]" >&2
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

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
image_name=${EHRGPU_IMAGE:-ehrcalc-rocm:7.2.4}
build_dir=${EHRGPU_BUILD_DIR:-/mnt/2TB-Babel/ai-storage/gpu-build}
cargo_target=${EHRGPU_CARGO_TARGET_DIR:-/mnt/2TB-Babel/ai-storage/cargo-target}
reconstruct="$cargo_target/release/reconstruct_modular"
modulus_candidates=(
    2147483647 2147483629 2147483587 2147483579
    2147483563 2147483549 2147483543 2147483497
)

if ! docker image inspect "$image_name" >/dev/null 2>&1; then
    echo "missing Docker image $image_name; build it from gpu-prototype/Dockerfile" >&2
    exit 1
fi
mkdir -p "$build_dir" "$cargo_target"
CARGO_TARGET_DIR="$cargo_target" cargo build \
    --manifest-path "$repo_root/Cargo.toml" --release \
    -p ehrcalc-kostka-engine --bin reconstruct_modular
docker run --rm \
    --device=/dev/kfd --device=/dev/dri \
    --group-add video --security-opt seccomp=unconfined \
    -v "$repo_root/gpu-prototype:/source:ro" \
    -v "$build_dir:/build" \
    "$image_name" bash -lc \
    'hipcc --offload-arch=gfx1201 -O3 -std=c++17 \
       /source/packed_flagged_kostka_gpu_resident.hip.cpp \
       -o /build/packed-flagged-kostka-gpu-resident'

residues=()
moduli=()
for modulus in "${modulus_candidates[@]}"; do
    log_path="$build_dir/exact-d${dilation}-p${modulus}.log"
    gpu_output=$(
        docker run --rm \
            --device=/dev/kfd --device=/dev/dri \
            --group-add video --security-opt seccomp=unconfined \
            -v "$build_dir:/build" \
            "$image_name" /build/packed-flagged-kostka-gpu-resident \
            "$dilation" "$outer" "$inner" "$weight" "$upper_flags" \
            "$lower_flags" "$maximum_transitions" "$modulus" 2>"$log_path"
    )
    residue=$(sed -n 's/.*"residue":\([0-9][0-9]*\).*/\1/p' <<<"$gpu_output")
    if [[ -z $residue ]]; then
        echo "failed to parse GPU result; see $log_path" >&2
        exit 1
    fi
    residues+=("$residue")
    moduli+=("$modulus")
    residues_csv=$(IFS=,; echo "${residues[*]}")
    moduli_csv=$(IFS=,; echo "${moduli[*]}")
    echo "prime ${#moduli[@]}: modulus=$modulus residue=$residue" >&2
    if exact=$(
        "$reconstruct" "$upper_bound" "$residues_csv" "$moduli_csv" 2>/dev/null
    ); then
        echo "$exact"
        exit 0
    fi
done

echo "eight moduli did not certify an answer below the supplied bound" >&2
exit 1
