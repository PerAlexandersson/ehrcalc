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
MCP stdio companion.  It also includes the `key-scan` command for
Key-HStar-Bruhat full-rank scans with resumable JSONL checkpoints.  The fast
scanner supports staircase shapes, explicit dominant partitions, and adjacent
gap vectors; staircase scans can also run the `D/G/W/bar K/L` packet checks.
Scans whose parts have a common factor use the exact dilation shortcut
`P_{sigma,k mu}(m)=P_{sigma,mu}(km)`, including scaled staircases and
rectangles `(k^r,0^{n-r})`.  Single-row scans with `--sigma` can also use
`--sample-checkpoint` to preserve each exact dilation sample before final
interpolation.  For block-constant shapes, `--block-summary`
groups rows by the block contingency matrix, and `--block-representatives`
computes one lexicographic representative for each matrix class.

The repository is a standalone Cargo workspace.  Its internal
`ehrcalc-kostka-engine` and `ehrcalc-foundations` crates own the migrated
algorithms needed by the public CLI and MCP server.  No sibling repository is
needed to build it: `polytool`, used for exact real-rootedness and interlacing,
is pinned to a public Git revision.

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

Build a fresh checkout with the stable Rust toolchain:

```bash
git clone https://github.com/PerAlexandersson/ehrcalc.git
cd ehrcalc
cargo build --release
./target/release/ehrcalc --help
```

The exact `polytool` dependency is fetched from its pinned public Git revision;
no sibling checkout is required.

```text
ehrcalc gt ...
ehrcalc kostka ...
ehrcalc lr ...
ehrcalc key ...
ehrcalc key-scan --max-n 5 --staircase
ehrcalc key-scan --n 4 --lambda 4,2,1 --packets h-checks
ehrcalc key-scan --n 4 --gaps 2,1,1 --rows --format json
ehrcalc key-scan --n 5 --lambda 2,1,1 --sigma 4,2,5,1,3 --rows
ehrcalc key-scan --n 6 --lambda 7,5,5,2,0,0 --sigma 5,4,6,2,1,3 \
  --sample-checkpoint samples.jsonl --rows --format json
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

See [Key h-star computations](docs/KEY_HSTAR.md) for the mathematical
convention, exact row schema, packet modes, and resumable workflows for
`key-scan`.

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
- [docs/KEY_HSTAR.md](docs/KEY_HSTAR.md) for key h-star computations.
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
