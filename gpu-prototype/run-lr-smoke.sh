#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

run_case() {
    local expected=$1
    local lambda=$2
    local mu=$3
    local nu=$4
    local actual

    actual=$("$repo_root/gpu-prototype/run-lr-exact.sh" \
        "$lambda" "$mu" "$nu")
    if [[ $actual != "$expected" ]]; then
        echo "LR ($lambda)/($mu),($nu): expected=$expected GPU=$actual" >&2
        exit 1
    fi
    echo "LR ($lambda)/($mu),($nu): $actual (match)"
}

run_case 0 3,2 1 2,1,1
run_case 1 3,2,1 1,1 2,1,1
echo "GPU LR inversion smoke suite passed"
