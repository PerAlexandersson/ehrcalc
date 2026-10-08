# Boundary-defect repairs, 2026-10-08

This repairs BCA-1 through BCA-4 from `BROAD_CORNER_AUDIT.md`. All seven
previously ignored regression tests are enabled, preserving or strengthening
their correct expected answers.

## Arithmetic

- Netflow validation sums in `i128`, including partial sums.
- Flow DP netflow/inflow states use `i128`, with checked accumulation. A
  unique flow containing an edge of size `2^63` is counted exactly. Input
  dilation still explicitly rejects scaled netflow entries outside `i64`.
- `Partition::size_wide()` sums in `u128`. Legacy weak, flagged and strict
  Kostka, direct LR, and LR matrix inversion use wide size comparisons.
  The old `size() -> u32` API rejects overflow explicitly in both profiles.
  SYT totals also use the wide API. The matrix-inversion enumerator explicitly
  rejects a skew size beyond its `u32` partition-enumeration domain.
- GT-pattern weights subtract wide row sums, even when individual weight
  coordinates fit `u32`. Strict bound increments/decrements are checked.

## Key interiors

For a single Kogan face, the interlacing inequalities define a marked order
polytope. The implementation builds its directed comparison graph, adding
reverse edges for imposed equalities and equal top-row marks. Endpoints in
the same strongly connected component must agree everywhere. Only the
remaining inequalities become strict. Components not containing a top-row
mark give the affine dimension. This uses the special order-constraint
structure, not a new generic LP implementation or an upward crate dependency.

The key region can be a union of faces. Summing the interiors of its maximal
faces alone misses shared strata. The reciprocal counter now combines
intersections by inclusion-exclusion and applies reciprocity to each face,
with its own dimension sign. For `(2,1,0)` and permutation `[3,1,2]`,
the counting polynomial is `(t+1)(3t+2)/2`: the reciprocal count at two is
two, whereas the union of separate face interiors contains just one point.

`strict_gt_patterns_with_equalities_n` enumerates a single face's relative
interior. `key_strict_lattice_point_count` computes the reciprocal count of
the union for positive dilations; at zero it returns the collapsed point.
The documentation no longer claims that the main key-Ehrhart interpolator
uses strict counts: both interpolation variants use nonnegative samples.
Empty-rank interpolation is supported. The exhaustive Kogan subset engine
now explicitly rejects 64 or more equality positions instead of wrapping
its `u64` enumeration bound; this is a supported-size limit, not a new
large-rank algorithm. Face-intersection enumeration can also be exponential.

## Tests and the previous gaps

`tests/key_interior_geometry.rs` compares 272 individual faces with the
existing exact rational affine-hull solver, and checks 192 three-/four-row
key families against independent positive-sample interpolation. Additional
fixtures cover a shared edge, extreme repeated marks, empty rank and the
subset-width guard. The original arithmetic tests now cover adjacent legacy
flagged/strict paths, matrix inversion, oversized content, merging inflows
and relative interiors as well as the original witnesses.

The old tests missed these paths for specific reasons:

- Modern Kostka already had wide-total tests; its legacy and LR callers did
  not share that implementation or those tests.
- Small flow examples never crossed an intermediate machine-word boundary.
  A checked input product does not protect a later sum of inflows.
- Key polynomial tests checked positive interpolation, which did not call
  the strict counter. Their names/documentation suggested more reciprocity
  coverage than the executed code provided.
- Backend agreement cannot detect a mathematical assumption shared by both
  backends. New tests include exact geometry and explicit known counts.

The CI workflow runs the workspace tests in both debug and release, so
release wrapping cannot hide behind successful debug-only checks. Historical
database rows are not automatically certified by these repairs.
