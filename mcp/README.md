# Ehrcalc MCP Server

`ehrcalc-mcp` exposes the exact Ehrcalc library through MCP stdio transport.
It is intentionally a thin server: it parses structured requests, calls the
same functions used by the `ehrcalc` CLI, and returns the shared JSON result
model.

Run `ehrcalc-mcp --help` for local server usage.  MCP clients start it with no
arguments and communicate over standard input and output.

Initial tools:

- `ehrhart_transform`: exact interpolation, h* conversion, finite difference,
  discrete summation, and binomial conversion.
- `compute_family`: GT, Kostka, LR, key, order, and acyclic flow requests.
- `constraint_polytope_ehrhart`: rational bounds, differences, and subset sums,
  with exact affine dimension, ordering plans, counts, CRT evidence, and
  polynomiality warnings. Its `input` is an
  `ehrcalc.constraint-polytope.v1` object; see
  [the constraint guide](../docs/CONSTRAINT_POLYTOPES.md) for options and output.

The order requests of `compute_family` take optional `method` (`auto`,
`ideals`, `frontier`; default `auto`) and `max_states` fields and report the
method used in the response field `method`.

All arbitrary-size integer and rational inputs use strings in JSON.
