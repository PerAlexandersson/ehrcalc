# Provenance

This crate contains the algorithms that Ehrcalc migrated from the legacy
[`PerAlexandersson/kostka`](https://github.com/PerAlexandersson/kostka)
repository at source revision
`90fb3921867b31a26d0fe2eedcfcce2bda131896`.

The migration includes the exact Gelfand--Tsetlin, Kostka, flagged Kostka,
Littlewood--Richardson, flow-polytope, Ehrhart, h*, partition, and standard
Young-tableau modules.  It intentionally excludes the retired `kostka` CLI,
database population features, and exploratory binaries.  Ehrcalc is now the
maintained command-line and MCP interface for these algorithms.

The source is MIT licensed.  Fixes and new supported behavior belong here;
the legacy repository remains a frozen compatibility and research reference.
