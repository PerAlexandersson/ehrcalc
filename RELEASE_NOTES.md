# Unreleased

- Constraint audit fixes warn on fractional fixed marks/implicit equations and
  invalid fitted degree or leading coefficient even with zero extra samples.
  CLI/MCP requests cap variables at 256 and extra samples at 64, reject explicit
  order/heuristic conflicts, and offer `fail-on-mismatch`. Conflicting text
  bounds now represent an empty system. LR levels stream into capped maps,
  including the initial strip. Single-component samples/plans are reused,
  split estimates share one order, positive-only skips interior work, and auto
  subset DP stops at 16 variables (explicit 22). Strict pilots can add up to
  two bounded probes at later informative scales.

- Constraint-polytope auto ordering adds reverse minimum-degree/minimum-fill
  elimination, peripheral Cuthill–McKee variants, GT level/row sweeps and
  adjacent-swap improvement. Bounded dilation-one/two pilots select among
  candidates using actual frontier sizes; JSON records estimates, measurements,
  censoring and predictions. Explicit orders/heuristics bypass pilots and
  plan-only remains estimate-only. Auto subset minimax is limited to 16 free
  variables; explicit `exact-subset` retains its 22-variable limit. Counts and
  CRT certificates are unchanged. A portable JSONL benchmark adapter and
  Python 3.12 runner support bounded, resumable comparison batches.
- `ehrcalc order` computes h* by a dynamic program over order ideals
  (Stanley's descent generating function) when the poset has a moderate
  number of order ideals, and otherwise falls back to the previous frontier
  method.  The `6 x 6` rectangle drops from about a minute to a few milliseconds, and
  the `8 x 8` rectangle takes about 0.1 s.
  New options: `--method auto|ideals|frontier` and `--max-states`; matching
  optional MCP fields `method` and `max_states` on the order requests.
  JSON results gain the additive field `method`.  Results are identical for
  both methods.
- `HStarPolynomial::to_ehrhart` accumulates integer numerators and divides by
  `d!` once per coefficient, which speeds up large h* conversions.

# Ehrcalc 0.1.0-rc.1

First release candidate for the exact Ehrhart library, CLI and MCP server.
All workspace packages use version `0.1.0-rc.1`.

## Included

- Exact Ehrhart, h-star and binomial-basis transforms; GT/Kostka, flagged
  Kostka, LR counting, key, order and acyclic-flow families.
- Certified affine dimensions and relative-interior constraints for GT
  families, including degenerate and rational-only feasible directions.
- Wide arithmetic for flow validation/counting and LR/legacy Kostka size
  comparisons, with checked rejection at remaining fixed-width API limits.
- Correct key-face interiors and reciprocal counts including shared face
  intersections. The main key polynomial interpolation uses positive counts.
- Deterministic boundary regressions, independent exact-geometry checks,
  and debug/release CI.

## Build and check

```sh
cargo build --release --locked -p ehrcalc -p ehrcalc-mcp
cargo test --workspace --locked
cargo test --workspace --release --locked
```

This GitHub prerelease supplies source archives; no prebuilt binaries or
crates.io packages are published with it. Build with a current stable Rust
toolchain. See README.md for CLI/MCP usage and docs/BROAD_CORNER_FIXES.md for
the repaired cases and supported-size limits. Exhaustive Kogan enumeration
remains size-limited. Previously cached research results are not revalidated
merely by upgrading the implementation.
