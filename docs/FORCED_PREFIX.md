# Forced first-row prefixes

The generic Kostka and relative-interior counters remove an isolated forced
prefix before counting. The GT Ehrhart path uses the same reduction after
checking that the exact affine dimension is preserved. The implementation is
`kostka_normalization::forced_first_row_reduction` in the internal engine.

We consider a skew tableau polytope of shape `lambda/mu` and content `alpha`,
with label 1 permitted in the first row. Put

```text
T = max(mu_1, lambda_2)
r = max(0, alpha_1 - T).
```

For `r > 0` and `alpha_1 <= lambda_1`, we subtract `r` from both `lambda_1`
and `alpha_1`. All other shape parts, content parts, and row flags remain
unchanged. For example, `(7,3,1)/(1)` with content `(5,3,2)` becomes
`(5,3,1)/(1)` with content `(3,3,2)`.

To prove the reduction, a 1 below the first row must occur in one of the
first `mu_1` columns: a later column has a first-row cell above it. Each
column contains at most one 1. At least `alpha_1 - mu_1` ones therefore
occur in the first row, whose cells through column `alpha_1` are all 1.
Columns `T+1,...,alpha_1` each contain only a first-row cell because
`T >= lambda_2`. We delete these `r` cells. The new first part is
`lambda_1 - alpha_1 + T >= T`, so it remains at least both `lambda_2` and
`mu_1`.

Conversely, a reduced tableau has `T` ones, at most `mu_1` of them below the
first row. Its first-row cells through column `T` are therefore 1. We insert
`r` ones after that column. This preserves row weak increase, every body
column, and the first-row flag. Insertion and deletion are inverse.

At dilation `N` the GT map subtracts `Nr` from the first coordinate at every
positive alphabet level and fixes the other coordinates. The inverse adds
the same integers. Rational dilations and continuity give inverse affine
lattice maps on the real polytopes. They preserve affine dimension, relative
interiors, ordinary counts, and interior counts. Hence the entire Ehrhart
polynomial and its h* vector are unchanged.

The helper checks containment, total size, flag lengths, the first-row flag,
and the bound `alpha_1 <= lambda_1`. It leaves trivial one-row problems to
the existing counter, so row indices and flag vectors never need remapping.
Its work is linear in the input vectors; it does not iterate over boxes or
construct conjugate partitions. Size comparisons use wide arithmetic.

The presolve is used by `try_skew_kostka_stats`,
`try_flagged_skew_kostka`, `try_strict_flagged_skew_kostka`, and the maintained
GT Ehrhart implementation. The legacy ordinary counter and the low-level
masked and packed interfaces retain their original presentations. Arbitrary
mask constraints require their own translation and are not changed here.

Regression tests compare small dilations with independent legacy ordinary
counts and the unreduced repaired masked-interior counter, check exact
dimensions and complete polynomial equality, and exercise flags, empty
inputs, boundary cases, and parts near `u32::MAX`.
