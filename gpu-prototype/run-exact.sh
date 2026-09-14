#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 7 || ($# -gt 8 && $# -ne 11) ]]; then
    echo "usage: $0 UPPER_BOUND DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS [MAX_TRANSITIONS [FORBIDDEN_MASKS STRICT_LOWER_MASKS STRICT_DIAGONAL_MASKS]]" >&2
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
forbidden_masks=${9:--}
strict_lower_masks=${10:--}
strict_diagonal_masks=${11:--}

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
exec 9>"$build_dir/ehrgpu.lock"
if ! flock -n 9; then
    echo "another Ehrcalc GPU build or run holds $build_dir/ehrgpu.lock" >&2
    exit 1
fi
run_id=$(
    {
        printf '%s\0' "$@"
        sha256sum "$repo_root/gpu-prototype/packed_flagged_kostka_gpu_resident.hip.cpp"
    } | sha256sum | cut -c1-16
)
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
    if "$reconstruct" "$upper_bound" "$trial_residues_csv" \
        "$trial_moduli_csv" >/dev/null 2>&1; then
        needed_moduli=${#trial_moduli[@]}
        break
    fi
done
if ((needed_moduli == 0)); then
    echo "eight moduli do not exceed the supplied certified bound" >&2
    exit 1
fi

residues=()
moduli=()
batch_start=0
while ((batch_start < needed_moduli)); do
    remaining=$((needed_moduli - batch_start))
    lane_count=$((remaining < 3 ? remaining : 3))
    batch_moduli=("${modulus_candidates[@]:batch_start:lane_count}")
    batch_moduli_csv=$(IFS=,; echo "${batch_moduli[*]}")
    binary_name="packed-flagged-kostka-gpu-resident-lanes${lane_count}"
    docker run --rm \
        --device=/dev/kfd --device=/dev/dri \
        --group-add video --security-opt seccomp=unconfined \
        -v "$repo_root/gpu-prototype:/source:ro" \
        -v "$build_dir:/build" \
        "$image_name" bash -lc \
        "hipcc --offload-arch=gfx1201 -O3 -std=c++17 \
          -DEHRGPU_RESIDUE_LANES=$lane_count \
          /source/packed_flagged_kostka_gpu_resident.hip.cpp \
          -o /build/$binary_name"
    log_path="$build_dir/exact-${run_id}-batch${batch_start}.log"
    gpu_output=$(
        docker run --rm \
            --device=/dev/kfd --device=/dev/dri \
            --group-add video --security-opt seccomp=unconfined \
            -v "$build_dir:/build" \
            "$image_name" "/build/$binary_name" \
            "$dilation" "$outer" "$inner" "$weight" "$upper_flags" \
            "$lower_flags" "$maximum_transitions" "$batch_moduli_csv" \
            "$forbidden_masks" "$strict_lower_masks" "$strict_diagonal_masks" \
            2>"$log_path"
    )
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
    if exact=$(
        "$reconstruct" "$upper_bound" "$residues_csv" "$moduli_csv" 2>/dev/null
    ); then
        echo "$exact"
        exit 0
    fi
    batch_start=$((batch_start + lane_count))
done

echo "GPU residues did not reconstruct an answer below the supplied bound" >&2
exit 1
