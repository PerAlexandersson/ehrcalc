# Ehrcalc Project Guide

## Scope

Ehrcalc is a public Rust library, CLI, and MCP server for exact Ehrhart and
related combinatorial computations.  Read `EHRHART_TOOL_SPEC.md` before adding
a family or changing a public command.

## Design Rules

- Keep exact arithmetic as the default.  Do not make floating-point output the
  basis for mathematical conclusions.
- Keep family-specific counting code behind small adapters.  Core
  interpolation and transforms must not depend on Gelfand-Tsetlin, key, poset,
  or flow types.
- Record affine dimension separately from the trimmed polynomial degree and
  the h* degree.  Trailing zero h* coefficients are meaningful.
- Treat the internal `ehrcalc-kostka-engine` and `ehrcalc-foundations` crates
  as the maintained implementations for their migrated algorithms.  Preserve
  their provenance notes and use legacy `kostka` and `combinatoric-core` only
  as comparison/reference sources.  Continue to use `polytool` for
  real-rootedness and interlacing checks.
- JSON is the stable machine contract.  Plain text and LaTeX are presentation
  layers derived from the same exact result data.
- The `clap` command model is the source of truth for flags, subcommands, and
  CLI reference documentation.  Keep `docs/cli.md` generated from it.
- The MCP server must call the same library API as the CLI; it must not
  reimplement calculations or output formatting.

## Tests And Documentation

- Every transform needs exact unit and round-trip tests.
- Every family needs small known examples, independent cross-checks, and
  deterministic regression fixtures for prior failures.
- Add CLI and MCP integration tests when adding a public command or tool.
- Update `README.md` and the relevant file under `docs/` with every public API
  or command change.
- Update the generated CLI reference and its drift test whenever command help
  changes.

## Commands

Run Rust commands from this directory or the parent workspace with:

```bash
timeout 60s nice -n 10 cargo ...
```

Use a longer bound only when a documented benchmark requires it.  Keep
long-running scans resumable and do not commit generated checkpoint data.
