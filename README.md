# Ehrcalc

Ehrcalc is an exact Rust library, command-line tool, and MCP server for
Ehrhart polynomials, h*-vectors, affine dimensions, and closely related
combinatorial counting problems.

Every family follows the same pipeline: obtain exact lattice-point counts
under dilation, determine the affine dimension, interpolate the Ehrhart
polynomial over the rationals, and derive h* and binomial-basis data.  All
arithmetic uses arbitrary-size integers and rationals.

## Features

- Gelfand-Tsetlin polytopes, Kostka, skew Kostka, and flagged Kostka numbers
  (with automatic factorization and forced-prefix reductions), plus
  Littlewood-Richardson coefficients.
- Key-polynomial Ehrhart families and resumable `key-scan` scans of key
  h*-polynomials over symmetric groups.
- Order polytopes, with h* from Stanley's linear-extension theorem or a
  frontier dynamic program for wide posets.
- Rational constraint polytopes given by bounds, differences, and subset sums.
- Acyclic flow polytopes.
- Transform-only workflows: interpolation, power / h* / binomial-basis
  conversion, finite differences, sums, evaluation, and independent checks.
- Exact h* and binomial-basis output; trailing zero h* coefficients are kept.
- An MCP stdio server that calls the same library API as the CLI.

## Installation

```bash
cargo install --git https://github.com/PerAlexandersson/ehrcalc.git ehrcalc
cargo install --git https://github.com/PerAlexandersson/ehrcalc.git ehrcalc-mcp  # optional MCP server
```

Or build from a checkout:

```bash
git clone https://github.com/PerAlexandersson/ehrcalc.git
cd ehrcalc
cargo build --release   # binaries: target/release/ehrcalc, target/release/ehrcalc-mcp
```

## Quick start

Order polytope of the poset given by `--shape 4,4,4`:

```console
$ ehrcalc order --shape 4,4,4
dimension: 12
Ehrhart(n): 1/1036800 n^12 + 7/172800 n^11 + ... + 259/60 n + 1
hstar(t): [1, 22, 113, 190, 113, 22, 1, 0, 0, 0, 0, 0, 0]
binomial_basis: [1, 34, 421, 2750, 11033, 29378, 54097, 70168, 64120, 40488, 16842, 4158, 462]
```

Gelfand-Tsetlin polytope and a Kostka number:

```console
$ ehrcalc gt --lambda 4,2 --weight 2,2,2
dimension: 1
Ehrhart(n): 2 n + 1
hstar(t): [1, 1]
binomial_basis: [1, 2]
$ ehrcalc kostka --lambda 3,2,1 --weight 2,2,2
kostka: 2
```

Acyclic flow polytope (vertices and edges are zero-based):

```console
$ ehrcalc flow --vertices 4 --edges '0->1,0->2,1->3,2->3,1->2' --netflow 2,0,0,-2
dimension: 2
Ehrhart(n): 2 n^2 + 3 n + 1
hstar(t): [1, 3, 0]
binomial_basis: [1, 5, 4]
```

Transform-only (Ehrhart coefficients in ascending degree to h*) and a
constraint polytope from a JSON description:

```console
$ ehrcalc hstar --dimension 2 --coefficients 1,3/2,1/2
dimension: 2
Ehrhart(n): 1/2 n^2 + 3/2 n + 1
hstar(t): [1, 0, 0]
binomial_basis: [1, 2, 1]
$ ehrcalc polytope --input examples/constraints/hypersimplex.json
dimension: 3
Ehrhart(n): 2/3 n^3 + 2 n^2 + 7/3 n + 1
hstar(t): [1, 2, 1, 0]
...
```

Other families can feed `polytope` with `--emit-constraints`; see
[docs/CONSTRAINT_POLYTOPES.md](docs/CONSTRAINT_POLYTOPES.md).

Run `ehrcalc --help` or `ehrcalc <command> --help` for all options.

## Output formats

Every command accepts `--format text|json|latex`.  JSON is the stable
machine-readable contract (large integers and rationals are encoded as
strings); plain text and LaTeX are presentation layers over the same result.

## MCP server

`ehrcalc-mcp` serves the library over MCP stdio transport; clients start it
with no arguments.  Tools: `ehrhart_transform`, `compute_family`, and
`constraint_polytope_ehrhart`.

## Documentation

- [docs/cli.md](docs/cli.md): generated CLI reference.
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md): package layers, counting
  engines, and exact interpolation.
- [docs/OUTPUT_AND_MCP.md](docs/OUTPUT_AND_MCP.md): output schema and MCP
  request contract.
- [docs/CONSTRAINT_POLYTOPES.md](docs/CONSTRAINT_POLYTOPES.md): constraint
  polytope input syntax, GT adapters, and ordering controls.
- [docs/KEY_HSTAR.md](docs/KEY_HSTAR.md): key h* conventions, `key-scan` modes,
  checkpoints, dilation shortcuts, and block-constant weights.
- [docs/FORCED_PREFIX.md](docs/FORCED_PREFIX.md): forced first-row reduction.
- [docs/BROAD_CORNER_FIXES.md](docs/BROAD_CORNER_FIXES.md): boundary semantics.
- [docs/KTT_SEARCH.md](docs/KTT_SEARCH.md): the separate KTT research scanner.
- [docs/TESTING.md](docs/TESTING.md): test strategy.
- [EHRHART_TOOL_SPEC.md](EHRHART_TOOL_SPEC.md): requirements and design goals.

## Testing

```bash
cargo test --workspace
```

CI runs the suite in debug and release; see [docs/TESTING.md](docs/TESTING.md).

## License

MIT; see [LICENSE](LICENSE).
