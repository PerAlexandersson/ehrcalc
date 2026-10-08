# Broader corner-case review, 2026-10-08

Reviewed baseline: c23e7ec. This checkpoint adds tests, not algorithm fixes.
Four unresolved issues were reproduced in both debug and release builds.
The seven witness tests are explicitly ignored pending repairs; their
assertions retain the mathematically correct answers. A passing default
suite does **not** mean these issues are resolved.

## BCA-1: netflow validation overflows

`FlowPolytope::new` in `crates/ehrcalc-kostka-engine/src/flow.rs` sums its
netflow vector in `i64`.

- Two vertices, no edges, netflow `[i64::MIN, i64::MIN]`: the true sum is
  `-2^64`, so validation must reject. Debug panics; release accepts.
- Three vertices, edges `0->1, 1->2`, netflow `[i64::MAX, 1, i64::MIN]`:
  the true sum is zero. Debug panics in validation; release accepts and
  correctly reports dimension zero. A checked supported-range error would
  also be acceptable to this regression.

Use a wider/exact accumulator, or explicitly reject unsupported inputs.

## BCA-2: intermediate flow arithmetic overflows

Three vertices, edges `0->2, 2->1`, netflow `[i64::MAX, i64::MIN, 1]`.
Constructor partial sums fit. There is exactly one feasible flow: edge
values `2^63-1` and `2^63`. Dimension correctly returns zero, but
`count_lattice_points(1, None)` panics in debug and returns zero in release.

`CountContext::count_from` adds inflow and netflow in `i64`;
`distribute_outflow` also accumulates incoming flow in `i64`. Both need a
complete checked/wider arithmetic review. Returning an explicit error is
acceptable; returning `Ok(0)` is not. This example has no large enumeration.

## BCA-3: translated skew shapes overflow total sizes

Let `M = u32::MAX / 3 = 1431655765`.

- Outer `(M+1,M,M)`, inner `(M,M,M)`, content `(1)`: a single box, LR
  coefficient one. Both `lr::try_lr_dp` and public `families::lr_count`
  panic in debug and return zero in release.
- Outer `(M+3,M+2,M+1)`, inner `(M,M,M)`, content `(1,1,3,1)`: the
  translated six-box Kostka fiber has two tableaux. `skew_kostka_legacy`
  panics in debug and returns zero in release. The modern full counter,
  dimension routine, and modern interior counter pass this same input
  (respectively 2, 1, and 0).

`Partition::size()` sums in `u32`; subsequent `saturating_sub` cannot repair
an already-wrapped sum. LR and maintained legacy callers still use this
path. Fix all affected size/content comparisons consistently. These are
small fibers with large coordinates, not expensive enumeration examples.

## BCA-4: key strict counter misses implicit equalities

For `lambda=(1,1)`, `sigma=[2,1]`, every positive dilation of the full GT
polytope is a single point. Its relative interior has one lattice point.
`key_strict_lattice_point_count(..., 1)` returns zero in debug and release.

`enumerate_gt_entries` in
`crates/ehrcalc-foundations/src/key_polynomial.rs` makes both interlacing
inequalities strict at every position not explicitly forced by the Kogan
face. Repeated top entries can force further equalities. Thus the function
does not satisfy its documented relative-interior/reciprocity contract.

The current `key_ehrhart` polynomial routine uses nonnegative-dilation
counts and passes this example (dimension zero, polynomial one). This
witness does not show that its output is wrong. Its reciprocity-related
documentation nevertheless describes a strict-count route it does not
currently execute. A repair should address the precise relative-interior
semantics, not merely special-case this partition.

## New passing coverage

The engine's seven enabled tests cover row counts through 65, 124/128/132-bit
packing boundaries, zero-weight insertion with forbidden unused labels,
large skew translation, parallel-edge simplex counts, flow relabeling,
redundant rational constraints and implicit equalities, and bounded CRT
with non-prime coprime moduli.

The five enabled adapter tests cover 125 exact transform/calculus examples,
negative evaluation points, extreme interpolation abscissae, repeated key
top rows, chain/cube order-polytope formulas (including counts beyond
machine-word size), graph relabeling and redundancy, and invalid posets.
These passed in debug and release before ignoring any known failures.

## Reproduce

From this repository, with an external `CARGO_TARGET_DIR`:

```sh
cargo test --locked -p ehrcalc-kostka-engine --test broad_corner_cases
cargo test --locked --test broad_adapter_corners

# Unresolved witnesses: expected to fail until the implementations are fixed.
cargo test --locked -p ehrcalc-kostka-engine --test broad_corner_cases -- --ignored
cargo test --locked --test broad_adapter_corners -- --ignored
```

Repeat with `--release` before `--`. In debug the ignored engine group has
five failures; in release it has four failures and one pass (the balanced
constructor case). Both adapter witnesses fail in both modes. Remove the
relevant ignore attributes only after a fix passes in both modes.

No historical database results were revalidated by this review. Larger
state spaces, every public API, and the C ABI were not exhaustively checked.
