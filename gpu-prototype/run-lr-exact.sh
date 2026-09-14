#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 3 || $# -gt 4 ]]; then
    echo "usage: $0 LAMBDA MU NU [MAX_TRANSITIONS]" >&2
    exit 2
fi

lambda=$1
mu=$2
nu=$3
maximum_transitions=${4:-150000000}
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cargo_target=${EHRGPU_CARGO_TARGET_DIR:-/mnt/2TB-Babel/ai-storage/cargo-target}
planner="$cargo_target/release/lr_kostka_plan"
temporary_directory=$(mktemp -d)
trap 'rm -rf -- "$temporary_directory"' EXIT
plan_file="$temporary_directory/plan.tsv"
counts_file="$temporary_directory/counts.tsv"

CARGO_TARGET_DIR="$cargo_target" cargo build \
    --manifest-path "$repo_root/Cargo.toml" --release \
    -p ehrcalc-kostka-engine --bin lr_kostka_plan
"$planner" plan "$lambda" "$mu" "$nu" >"$plan_file"

if [[ $(cut -f1 <"$plan_file" | head -n1) == result ]]; then
    cut -f2 <"$plan_file"
    exit 0
fi

job_count=$(wc -l <"$plan_file")
job_index=0
while IFS=$'\t' read -r kind id upper_bound outer inner weight; do
    if [[ $kind != job ]]; then
        echo "invalid planner record $kind" >&2
        exit 1
    fi
    job_index=$((job_index + 1))
    echo "LR Kostka job $job_index/$job_count: $id" >&2
    count=$("$repo_root/gpu-prototype/run-exact.sh" \
        "$upper_bound" 1 "$outer" "$inner" "$weight" - - \
        "$maximum_transitions")
    printf '%s\t%s\n' "$id" "$count" >>"$counts_file"
done <"$plan_file"

exec "$planner" reconstruct "$lambda" "$mu" "$nu" "$counts_file"
