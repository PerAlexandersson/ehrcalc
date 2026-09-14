#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 7 || $# -gt 9 ]]; then
    echo "usage: $0 BOUND_OR_BITS DILATION OUTER INNER WEIGHT UPPER_FLAGS LOWER_FLAGS [MAX_TRANSITIONS [FORBIDDEN_MASKS]]" >&2
    exit 2
fi

bound_spec=$1
upper_bound=$(python3 - "$bound_spec" <<'PY'
import re
import sys

spec = sys.argv[1]
match = re.fullmatch(r"bits:([0-9]+)", spec)
if match:
    bits = int(match.group(1))
    if not 1 <= bits <= 700:
        raise SystemExit("bit bound must be in 1..700")
    print((1 << bits) - 1)
elif re.fullmatch(r"[0-9]+", spec):
    bound = int(spec)
    if bound.bit_length() > 700:
        raise SystemExit("decimal bound exceeds 700 bits")
    print(bound)
else:
    raise SystemExit("bound must be an unsigned decimal integer or bits:N")
PY
)

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
export EHRGPU_CRT_MARGIN=2
exec "$script_dir/run-exact-strict.sh" "$upper_bound" "${@:2}"
