# Provenance

This crate was extracted for the Ehrcalc 0.1 migration from the local
`rust/combinatoric-core` source tree, itself part of
`PerAlexandersson/polynomial-tools`, at source revision
`a84c5c05d76f8b8201aeecef501bc5ef2372acff`.

It retains only the modules needed by Ehrcalc:

- `partition.rs`;
- `permutation.rs`;
- `key_polynomial.rs`; and
- `poset.rs`, without its unrelated comparability-graph API.

The source is MIT licensed.  Subsequent maintenance for these extracted
algorithms belongs in the Ehrcalc repository; the broader `combinatoric-core`
crate remains a separate general-purpose project.
