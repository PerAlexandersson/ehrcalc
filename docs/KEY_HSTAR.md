# Key h-star computations

Ehrcalc computes exact principal specializations of key polynomials, their
Ehrhart polynomials, and their h-star numerators.  The fast `key-scan` command
supports a single permutation, a bounded subset of a symmetric group, or a
complete rank.

This guide explains the mathematical convention, the two key commands, exact
output fields, and safe ways to resume expensive computations.  For the full
option list, see [`cli.md`](cli.md) or run `ehrcalc key-scan --help`.

## Mathematical convention

Fix a weakly decreasing partition

```text
lambda = (lambda_1, ..., lambda_n)
```

and a permutation `sigma` in `S_n`, written in one-line, one-based notation.
Ehrcalc computes the dilation polynomial

```text
P_{sigma,lambda}(k) = kappa_{k lambda,sigma}(1,...,1).
```

If its actual degree is `d`, the h-star numerator is defined by

```text
sum_{k >= 0} P_{sigma,lambda}(k) t^k
  = H_{sigma,lambda}(t) / (1-t)^(d+1).
```

Coefficient vectors are always in ascending degree order.  For example,
`hstar = ["1", "2", "0"]` means `H(t)=1+2t`, with the trailing zero retained
to record dimension two.

There are three equivalent input forms for the dominant weight:

- `--staircase` uses `rho_n=(n-1,n-2,...,1,0)`;
- `--lambda 4,2,1` supplies a weakly decreasing partition and pads it with
  trailing zeroes to length `n`;
- `--gaps 2,1,1` supplies the adjacent differences and reconstructs
  `lambda=(4,2,1,0)`.

Exactly one of these forms is required.  A `--gaps` list for `S_n` must have
`n-1` entries.

## `key` versus `key-scan`

The commands deliberately expose two evaluators.

- `ehrcalc key` is the original, general Kogan-face computation.  It accepts
  one `lambda,sigma` pair and an optional proven degree bound.
- `ehrcalc key-scan` is the fast principal-specialization evaluator used for
  h-star experiments.  It reuses one Demazure evaluation plan across all
  dilations of a row and supports rank scans, exact property checks, and
  resumable checkpoints.

They are independent computational routes for small cases.  The fast scanner
does not call the Kogan-face evaluator.

For weights with repeated parts, the fast evaluator derives the full Demazure
operator word from `sigma`, independently of the stabilizer of `lambda`.  Sorting
`lambda` after permutation would incorrectly discard operators.  The regression
`lambda=(1,1,0), sigma=231` and exhaustive comparisons through `S_3` check the fast
route against the maintained Kogan-face route.

## Quick start

When running from a source checkout, prefix the examples with
`cargo run --release --`.  For example:

```bash
cargo run --release -- key-scan --n 3 --staircase --rows
```

With an installed binary, use:

```bash
ehrcalc key-scan --n 3 --staircase --rows
```

To compute just one row as stable JSON:

```bash
ehrcalc key-scan \
  --n 3 \
  --staircase \
  --sigma 3,1,2 \
  --packets none \
  --rows \
  --format json
```

The row for `sigma=312` contains

```json
{
  "sigma": "312",
  "length": 2,
  "dimension": 2,
  "ehrhart_power": ["1", "5/2", "3/2"],
  "hstar": ["1", "2", "0"],
  "B": ["1", "4", "3"],
  "D": ["1", "3"]
}
```

Thus

```text
P_312(k) = 1 + (5/2)k + (3/2)k^2,
H_312(t) = 1 + 2t.
```

The complete JSON object also includes normalized inputs, run metadata, and
the lower strong-Bruhat covers.

## Exact row fields

For each computed permutation, `--rows` exposes:

- `sigma`: the compact one-line permutation;
- `index`: its zero-based position in Ehrcalc's lexicographic enumeration;
- `length`: its inversion number;
- `dimension`: the actual degree of the Ehrhart polynomial;
- `ehrhart_power`: exact rational coefficients of `P(k)` in ascending order;
- `hstar`: `dimension+1` exact integer coefficients, including trailing zeros;
- `B`: the exact binomial-basis coefficients in
  `P(k)=sum_j B_j binom(k,j)`;
- `D`: the staircase transform `B(u)/(1+u)` for nonidentity rows, with
  `D_id=1`; it is `null` outside the staircase specialization; and
- `lower_covers`: lower strong-Bruhat covers, their position labels `(i,j)`,
  and weights `lambda_i-lambda_j`.

JSON encodes arbitrary-size integers and rationals as strings.  It is the
stable format for scripts and fixtures.  Text output is intended for quick
inspection.

For example, extract the h-star row with `jq`:

```bash
ehrcalc key-scan --n 3 --staircase --sigma 3,1,2 --rows --format json \
  | jq '.ranks[0].rows[0] | {sigma, dimension, hstar}'
```

## Scan and packet modes

The `--packets` option controls work beyond computing row data.

- `none` computes rows only.  Use this for a single h-star polynomial or for
  expensive data collection.
- `h-checks` checks h-star coefficient nonnegativity, exact real-rootedness,
  and `H_tau << H_sigma` on lower strong-Bruhat covers.
- `d-route` additionally computes the staircase-only `D/G/W/L/bar K` packet
  diagnostics and exact interlacing checks.
- `auto` selects `d-route` for staircases and `h-checks` otherwise.

The real-rootedness and interlacing decisions come from the exact `polytool`
algorithms.  They do not use floating approximations.

Global summaries are produced only when every row of a rank is available.
Selecting one `--sigma`, a range, or block representatives produces exact row
data but leaves rank-wide summaries as `null` or unavailable.

A complete small-rank diagnostic is:

```bash
ehrcalc key-scan --n 3 --staircase --packets d-route
```

It should report six real-rooted rows and eight passing strong-cover
interlacings.  These finite checks are evidence about the selected rank, not
proofs of a general conjecture.

## Bounded and resumable rank scans

The number of rows is `n!`, and individual high-length rows can dominate the
runtime.  Start new experiments with a bounded range:

```bash
timeout 60s nice -n 10 cargo run --release -- \
  key-scan --n 6 --staircase --packets none \
  --start-index 0 --limit 100 \
  --checkpoint rows-s6.jsonl
```

Each completed row is appended and flushed as one JSONL record.  Continue with
the same file:

```bash
timeout 60s nice -n 10 cargo run --release -- \
  key-scan --n 6 --staircase --packets none \
  --start-index 100 --limit 100 \
  --resume rows-s6.jsonl --checkpoint rows-s6.jsonl
```

Once the checkpoint contains all rows, load it without a range to compute the
requested complete summary:

```bash
cargo run --release -- \
  key-scan --n 6 --staircase --packets d-route \
  --resume rows-s6.jsonl
```

Checkpoint records include the normalized rank, weight, and specialization.
Ehrcalc validates each resumed permutation and lexicographic index, inversion
length, dimension and exact coefficient-vector lengths, polynomial-derived h* and
binomial data, staircase `D`, and lower strong-Bruhat covers.  Conflicting duplicate
rows are rejected.  Generated checkpoint files are working data and should not be
committed by default.

## Resuming one difficult row

`--sample-checkpoint` works below the row level.  It writes and flushes each
completed dilation value before interpolation:

```bash
timeout 60s nice -n 10 cargo run --release -- \
  key-scan --n 6 --staircase --sigma 5,6,3,4,2,1 \
  --packets none --rows --format json \
  --sample-checkpoint samples-563421.jsonl
```

If the command times out, run the identical command again.  Previously saved
dilations are verified and skipped.  The file is tied to the exact
`n`, `sigma`, `lambda`, and degree bound; incompatible reuse is rejected.
Later dilations are often slower, so one bounded run may save several new
samples, only one, or none.  Exit status 124 from `timeout` is expected in
that workflow; inspect the JSONL file before deciding whether to continue.

`--sample-checkpoint` cannot be combined with the row-level `--checkpoint` or
`--resume` options.  It is also unavailable when the weight has a common
factor, because those rows use the dilation shortcut described below.

## Exact evaluation strategy

For a row, Ehrcalc constructs the composition determined by `lambda` and
`sigma`, obtains a sorting word, and applies the corresponding Demazure
operators to sparse exponent states.  It retains only variables needed by
later operators.  Packed integer states and checked `i64` coefficients are
used when possible; arithmetic automatically falls back to arbitrary-size
integers if necessary.

The evaluator samples `P(k)` at enough exact integral dilations to interpolate
within the Bruhat-length degree bound, then trims the exact power polynomial to
its actual dimension before deriving h-star and binomial-basis data.  The
longest staircase row uses its closed principal-specialization formula.

If all nonzero parts of `lambda` have a common factor `c`, Ehrcalc first
computes the primitive row and applies the exact identity

```text
P_{sigma,c mu}(k) = P_{sigma,mu}(c k).
```

This shortcut covers scaled staircases and rectangles without resampling the
larger weight.

## Block-constant weights

For repeated parts, `--block-summary` groups available rows by the contingency
matrix between value blocks and position blocks.  A complete scan also reports
whether one matrix class produced more than one h-star row.

Use `--block-representatives` to compute only the first lexicographic
permutation in each matrix class:

```bash
ehrcalc key-scan \
  --n 6 --lambda 4,3,3,2,0,0 \
  --block-representatives --block-summary --rows --format json
```

Representative-only output is intentionally incomplete and does not claim a
rank-wide property summary.

## Library and MCP use

Library callers use `key_scan::run_key_scan` with a `KeyScanInput`.  The CLI
and MCP server call this same function; neither maintains a second evaluator.
The MCP `compute_family` tool exposes the corresponding `KeyScan` request,
including row, packet, block, and checkpoint controls.  See
[`OUTPUT_AND_MCP.md`](OUTPUT_AND_MCP.md) for the shared output contract.

## Limits and interpretation

- Compact row identifiers currently restrict the command to `n <= 9`.
  Factorial growth and sparse-state growth usually impose a lower practical
  limit.
- Packet summaries use lower **strong** Bruhat covers.  They are not weak-order
  scans unless a separate caller explicitly filters the covers.
- `d-route` and the `D` field are currently staircase-only.
- Exact arithmetic prevents numerical root-classification errors, but an
  exhaustive finite scan is still not a mathematical proof in all ranks.
- Preserve checkpoints for hard rows if the process may be terminated; final
  JSON is printed only after the requested rows finish.

## Verification for contributors

Run the generated-document drift test and the full workspace suite:

```bash
timeout 60s nice -n 10 cargo run -- docs cli --write docs/cli.md
timeout 60s nice -n 10 cargo test --workspace
```

The key scanner tests include staircase route counts, a general-weight first
failure, single-row selection, both checkpoint layers, malformed resume rows,
common-factor dilations, repeated-part Kogan cross-checks, and block-class summaries.
