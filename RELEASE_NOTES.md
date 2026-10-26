# Unreleased

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
