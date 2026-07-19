# Ehrcalc

Ehrcalc is an exact Rust library, command-line tool, and MCP server for
Ehrhart polynomials, h*-vectors, dimensions, and closely related
combinatorial counting problems.

The project unifies a common workflow across several families: obtain exact
lattice-point or combinatorial counts under dilation, determine the relevant
dimension, interpolate the Ehrhart polynomial exactly, and derive h* and
binomial-basis data.  It also exposes supporting calculations such as Kostka
and Littlewood-Richardson coefficients when those are useful inputs to an
Ehrhart computation.

## Status

Version 0.1 implements the exact interpolation and transform core, the
Gelfand-Tsetlin, Kostka, flagged Kostka, Littlewood-Richardson, key, order,
and acyclic-flow command families, JSON/LaTeX/plain-text rendering, and an
MCP stdio companion.  The Key-HStar-Bruhat scan and packet workflow remains a
benchmark-driven extension; it is specified but intentionally not represented
as a completed command.

For now, the source is workspace-first: `Cargo.toml` uses the sibling
`../kostka` and `../combinatoric-core` crates.  A standalone public release
requires publishing the currently local engine revisions or extracting stable
engine crates first.  The implementation and test suite run from the parent
Rust workspace as documented below.

## Scope

Supported families are:

- Gelfand-Tsetlin, Kostka, skew Kostka, and flagged Kostka data;
- key-polynomial Ehrhart families;
- order polytopes;
- acyclic flow polytopes; and
- exact transform-only workflows from supplied Ehrhart or h* data.

Littlewood-Richardson coefficient calculation will be available as a direct
counting command.  Ehrhart data for LR objects will require an explicitly
specified family, such as a hive-polytope model, rather than treating every
LR coefficient as an Ehrhart value by default.

## Intended CLI

```text
ehrcalc gt ...
ehrcalc kostka ...
ehrcalc lr ...
ehrcalc key ...
ehrcalc order ...
ehrcalc flow ...
ehrcalc interpolate ...
ehrcalc hstar ...
ehrcalc verify ...
```

Every command supports plain text and JSON.  Commands producing mathematical
expressions also support LaTeX.  JSON is the stable
machine-readable contract, with large integers and rationals encoded as
strings where JSON numeric precision would be unsafe.

The command model is the source of truth for the CLI reference.  Run
`ehrcalc --help` or `ehrcalc <command> --help` for terminal documentation.
The checked-in [CLI reference](docs/cli.md) is generated from that same model
and is tested for drift.  This README deliberately stays a short overview and
does not duplicate every option.

The `ehrcalc-mcp` companion binary exposes the same exact library operations
over MCP stdio transport.  See [docs/OUTPUT_AND_MCP.md](docs/OUTPUT_AND_MCP.md)
for its request and output contract.

## Design

Ehrcalc is an orchestrator and exact transformation layer.  It reuses existing
domain implementations from `kostka` and `combinatoric-core`; it leaves
`polytool` as the
source of truth for real-rootedness and interlacing checks.

See:

- [EHRHART_TOOL_SPEC.md](EHRHART_TOOL_SPEC.md) for requirements and the
  phased migration plan.
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for package boundaries.
- [docs/cli.md](docs/cli.md) for generated CLI documentation.
- [docs/OUTPUT_AND_MCP.md](docs/OUTPUT_AND_MCP.md) for output and MCP design.
- [docs/TESTING.md](docs/TESTING.md) for the required test strategy.

## Development

From the parent Rust workspace, use bounded, low-priority commands:

```bash
timeout 60s nice -n 10 cargo test -p ehrcalc
timeout 60s nice -n 10 cargo run -p ehrcalc -- --help
```

All mathematical conclusions must use exact arithmetic.  A passing
interpolation check alone is not sufficient: each family must also verify its
dimension and independent sample points or reciprocity where applicable.
