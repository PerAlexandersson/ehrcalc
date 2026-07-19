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

The repository is a standalone Cargo workspace.  Its internal
`ehrcalc-kostka-engine` and `ehrcalc-foundations` crates own the migrated
algorithms needed by the public CLI and MCP server.  No sibling repository is
needed to build or test Ehrcalc.

## Scope

Supported families are:

- Gelfand-Tsetlin, Kostka, skew Kostka, and flagged Kostka data;
- key-polynomial Ehrhart families;
- order polytopes;
- acyclic flow polytopes; and
- exact transform-only workflows from supplied Ehrhart or h* data.

Littlewood-Richardson coefficient calculation is available as a direct
counting command.  Ehrhart data for LR objects will require an explicitly
specified family, such as a hive-polytope model, rather than treating every
LR coefficient as an Ehrhart value by default.

## CLI

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

Ehrcalc is an orchestrator and exact transformation layer.  It owns the
migrated GT/Kostka/LR/flow and key/order engines in its internal workspace
crates.  The legacy `kostka` repository is a frozen compatibility and research
reference, not a runtime dependency.  `polytool` remains the source of truth
for real-rootedness and interlacing checks.

## Legacy Transition

Ehrcalc is the maintained public CLI, library, and MCP surface for the
algorithms formerly exposed by `kostka`.  New supported calculations and fixes
belong here.  We retain `kostka` unchanged as a historical reference until the
Ehrcalc command-level compatibility audit is complete; it should not receive
new features.

See:

- [EHRHART_TOOL_SPEC.md](EHRHART_TOOL_SPEC.md) for requirements and the
  phased migration plan.
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for package boundaries.
- [docs/cli.md](docs/cli.md) for generated CLI documentation.
- [docs/OUTPUT_AND_MCP.md](docs/OUTPUT_AND_MCP.md) for output and MCP design.
- [docs/TESTING.md](docs/TESTING.md) for the required test strategy.

## Development

From this repository root, use bounded, low-priority commands:

```bash
timeout 60s nice -n 10 cargo test --workspace
timeout 60s nice -n 10 cargo run -- --help
```

All mathematical conclusions must use exact arithmetic.  A passing
interpolation check alone is not sufficient: each family must also verify its
dimension and independent sample points or reciprocity where applicable.
