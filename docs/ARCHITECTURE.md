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

The public workspace owns two focused internal engines:

- `ehrcalc-kostka-engine` for GT, Kostka, flagged Kostka, LR, flow, Ehrhart,
  h*, and standard-tableau algorithms migrated from legacy `kostka`; and
- `ehrcalc-foundations` for the partition, permutation, Kogan-key, and
  order-poset algorithms migrated from `combinatoric-core`.

Their provenance files name the source revisions and deliberately bounded
scope.  This makes Ehrcalc independently buildable while leaving the broader
`combinatoric-core` project separate.  The legacy `kostka` repository is a
frozen compatibility/reference project and is never a runtime dependency.
`polytool` remains the authority for real-rootedness and interlacing checks.

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
