#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 7 || $# -gt 9 ]]; then
    echo "usage: $0 UPPER_BOUND DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS [MAX_TRANSITIONS [FORBIDDEN_MASKS]]" >&2
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

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cargo_target=${EHRGPU_CARGO_TARGET_DIR:-/mnt/2TB-Babel/ai-storage/cargo-target}
derive_masks="$cargo_target/release/derive_masked_interior"

mkdir -p "$cargo_target"
CARGO_TARGET_DIR="$cargo_target" cargo build \
    --manifest-path "$repo_root/Cargo.toml" --release \
    -p ehrcalc-kostka-engine --bin derive_masked_interior

mask_json=$(
    "$derive_masks" "$outer" "$inner" "$weight" "$upper_flags" \
        "$lower_flags" "$forbidden_masks"
)
readarray -t derived < <(
    python3 -c '
import json
import sys

data = json.loads(sys.argv[1])
if data["empty"]:
    print("empty")
else:
    print("nonempty")
    print(data["dimension"])
    print(",".join(map(str, data["strict_lower_masks"])))
    print(",".join(map(str, data["strict_diagonal_masks"])))
    lower = [value for level in data["lower_bounds"] for value in level]
    upper = [value for level in data["upper_bounds"] for value in level]
    rows = len(sys.argv[2].split(","))
    outer = list(map(int, sys.argv[2].split(",")))
    lower.extend(outer)
    upper.extend(outer)
    expected = len(data["strict_lower_masks"]) * rows
    if len(lower) != expected or len(upper) != expected:
        raise SystemExit("derived level-bound dimensions are inconsistent")
    print(",".join(map(str, lower)))
    print(",".join(map(str, upper)))
' "$mask_json" "$outer"
)

if [[ ${derived[0]} == empty ]]; then
    derive_binary_sha256=$(sha256sum "$derive_masks" | cut -d' ' -f1)
    echo "EHRGPU_EMPTY_FACE {\"kind\":\"empty_face\",\"reconstructed\":\"0\",\"derive_binary_sha256\":\"$derive_binary_sha256\"}" >&2
    echo 0
    exit 0
fi

dimension=${derived[1]}
strict_lower_masks=${derived[2]}
strict_diagonal_masks=${derived[3]}
level_lower_bounds=${derived[4]}
level_upper_bounds=${derived[5]}
derive_binary_sha256=$(sha256sum "$derive_masks" | cut -d' ' -f1)
constraint_payload_sha256=$(printf '%s' "$mask_json" | sha256sum | cut -d' ' -f1)
echo "derived relative-interior masks for dimension $dimension" >&2

EHRGPU_DERIVE_BINARY_SHA256=$derive_binary_sha256 \
EHRGPU_CONSTRAINT_PAYLOAD_SHA256=$constraint_payload_sha256 \
exec "$repo_root/gpu-prototype/run-exact.sh" \
    "$upper_bound" "$dilation" "$outer" "$inner" "$weight" \
    "$upper_flags" "$lower_flags" "$maximum_transitions" \
    "$forbidden_masks" "$strict_lower_masks" "$strict_diagonal_masks" \
    "$level_lower_bounds" "$level_upper_bounds"
