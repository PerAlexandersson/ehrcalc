#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 || $# -gt 3 ]]; then
    echo "usage: $0 VERTICES COVERS [MAX_TRANSITIONS]" >&2
    exit 2
fi

vertices=$1
covers=$2
maximum_transitions=${3:-150000000}
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cargo_target=${EHRGPU_CARGO_TARGET_DIR:-/mnt/2TB-Babel/ai-storage/cargo-target}
ehrcalc="$cargo_target/release/ehrcalc"
points=()

for ((dilation = 0; dilation <= vertices; ++dilation)); do
    colors=$((dilation + 1))
    count=$("$repo_root/gpu-prototype/run-order-exact.sh" \
        "$vertices" "$covers" "$colors" weak "$maximum_transitions")
    points+=("$dilation:$count")
done

CARGO_TARGET_DIR="$cargo_target" cargo build \
    --manifest-path "$repo_root/Cargo.toml" --release --bin ehrcalc
points_csv=$(IFS=,; echo "${points[*]}")
exec "$ehrcalc" interpolate --dimension "$vertices" \
    --points "$points_csv" --format json
