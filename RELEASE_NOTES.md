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
