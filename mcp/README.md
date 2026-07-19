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

All arbitrary-size integer and rational inputs use strings in JSON.
