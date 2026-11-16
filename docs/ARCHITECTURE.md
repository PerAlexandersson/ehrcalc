# Architecture

## Goal

Ehrcalc provides one exact workflow for several families of counting problems:

1. determine a mathematically justified affine dimension;
2. obtain exact dilation counts;
3. interpolate the Ehrhart polynomial;
4. derive h*, binomial-basis, and finite-difference data; and
5. render one result as plain text, JSON, LaTeX, CLI output, or MCP output.

The library is the sole computational implementation.  The CLI and MCP server
are thin clients of that library.

## Layers

```text
family adapters       GT | key | order | flow | future families
        |
Ehrhart service       dimension, count scheduling, interpolation, verification
        |
exact algebra         rational polynomials, binomial basis, h*, Delta, sums
        |
presentation          text, JSON, LaTeX, CLI, MCP
```

Family adapters provide an independently justified dimension and exact counts.
When available, they also provide relative-interior counts for
Ehrhart-Macdonald reciprocity.  The generic service owns the decision to use
positive samples, reciprocal samples, or both.

GT dimensions, emptiness, and relative-interior masks come from the exact
affine hull of the rational polytope (`ehrcalc-kostka-engine::affine_hull`).
Interval propagation is used only as a sound presolve and as cheap pruning
bounds for counting recursions; it is incomplete, because a level-weight
equation can force several interlacing inequalities to equality jointly.
The affine hull is computed by one homogenized linear program (Freund,
Roundy and Todd, 1985) whose answer is checked by an exact relative-interior
point and an exact dual certificate.  It must not be inferred from lattice
points at any fixed dilation, which need not span the rational polytope.

`kostka_normalization::forced_first_row_reduction` deletes an isolated
forced prefix when label 1 is allowed in row one. With
`T=max(mu_1,lambda_2)` and `r=alpha_1-T>0`, it subtracts `r` from `lambda_1`
and `alpha_1`, retaining all other parts and flags. This is a
dilation-compatible affine lattice bijection, including relative interiors.
Generic ordinary and strict counters share the helper; the Ehrhart path
checks exact dimension preservation before recursing. Legacy and low-level
masked/packed paths remain independent. The proof and guards are in
[FORCED_PREFIX.md](FORCED_PREFIX.md).

`kostka_factorization::column_capacity_split` cuts at a label `t` when its
prefix content attains the column-capacity bound `sum_j min(t,h_j)`, where
`h_j` is the height of skew column `j`. Every column must then have its
top `min(t,h_j)` entries at most `t`. The intermediate partition is forced:
`beta'_j = min(lambda'_j, mu'_j+t)`. The two factors have shapes `beta/mu`
and `lambda/beta`, and their respective portions of the original content.
Only the second alphabet is relabeled; row indices and row flags stay fixed.
This applies to skew shapes and flagged inputs without sorting their content.
Computing `beta_i=lambda_i` for `i<=t` and
`beta_i=min(lambda_i,mu_(i-t))` afterwards avoids iterating over boxes.

Both factors form a dilation-compatible lattice product. Ordinary and
relative-interior counts share this split; the Ehrhart path multiplies
factor polynomials after checking dimension additivity. For straight shapes
it recovers the primitive Kostka factorization recalled in Section 6 of
[Garber--Pak--Slonim, On stretched Kostka polynomials (2026)](https://www.math.ucla.edu/~pak/papers/Kostka_Polynomials_9.pdf),
which cites McAllister (2008), Section 1. Geometrically disconnected skew
components alone do not justify a product: their content allocations may
vary. Legacy unsplit counters are retained for regression comparisons.

Flow-polytope support is the set of edges that are positive for some
feasible flow.  It is computed from one maximum flow and the strongly
connected components of its residual graph, not from reachability alone.

Order polytopes have two interchangeable exact routes in
`ehrcalc-foundations`: a dynamic program over order ideals that yields h*
directly (Stanley's descent generating function for a naturally labeled
poset), and a frontier dynamic program for order-preserving map counts with
reciprocity and interpolation.  The `order` adapter selects between them
(`OrderMethod`, `Auto` by default, with a state budget for the ideal program)
and feeds both through the same `EhrhartData` constructors, so declared
dimension and trailing h* zeros do not depend on the method.

The public workspace owns two focused internal engines:

- `ehrcalc-kostka-engine` for GT, Kostka, flagged Kostka, LR, flow, Ehrhart,
  h*, and standard-tableau algorithms migrated from legacy `kostka`; and
- `ehrcalc-foundations` for the partition, permutation, Kogan-key, and
  order-poset algorithms migrated from `combinatoric-core`.

Their provenance files name the source revisions and deliberately bounded
scope.  This makes Ehrcalc independently buildable while leaving the broader
`combinatoric-core` project separate.  The legacy `kostka` repository is a
frozen compatibility/reference project and is never a runtime dependency.
`polytool` remains the authority for real-rootedness and interlacing checks and
is consumed from a pinned public Git revision.

The dependency direction must never make `polytool` or the legacy projects
depend on an Ehrcalc family adapter.

## Exact Representation

The primary result must retain all three notions below:

```text
affine dimension       dimension of the intended Ehrhart object
polynomial degree      degree after zero coefficients are trimmed
h* degree              largest nonzero h* coefficient index
```

An h* vector is always stored with `affine_dimension + 1` entries.  Trailing
zeros must not be discarded merely because the numerator degree is smaller.

Power-basis coefficients use arbitrary-size rationals.  h* and binomial-basis
coefficients use arbitrary-size integers when the mathematical input warrants
them.  Overflow-prone presentation formats serialize them as strings.

## Interpolation

The public documentation may describe the method as exact Lagrange
interpolation.  At consecutive integral sample points, the implementation
should use the equivalent Newton forward-difference form.  This naturally
produces integer-valued binomial-basis coordinates and is better suited to
finite differences and discrete summation than a generic matrix solve.

Each computed polynomial must be checked at at least one unsampled point.
When a family supplies interior counts, reciprocity should provide an
additional independent check.
