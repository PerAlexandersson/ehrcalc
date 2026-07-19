# Output And MCP

## Result Model

Public commands and MCP tools return a shared result model.  A calculation
record includes:

- family name and normalized input;
- affine dimension;
- exact Ehrhart polynomial in ascending power-basis order;
- h* vector with its declared dimension;
- binomial-basis data when requested;
- sampled evaluations and verification outcomes; and
- family-specific diagnostics, such as GT flags or Bruhat cover packets.

JSON is the canonical external representation.  Integer and rational values
are strings in JSON when they may exceed safe JSON numeric precision.

## Formats

Each public calculation supports:

- `text`: concise human-readable mathematical output;
- `json`: stable structured output for programs, fixtures, and MCP clients;
- `latex`: display-ready formulas using documented variable conventions.

Formatters must consume the same result object.  No formatter may recompute a
mathematical result.

## CLI

The CLI uses explicit family subcommands, such as `gt`, `key`, `order`, and
`flow`, plus transform-only commands such as `interpolate`, `hstar`, `delta`,
and `sum`.  Expensive scans must have bounded ranges and resumable JSONL
checkpoints.  Diagnostic options must identify the first failure
deterministically.

The `clap` command model is the source of truth for flags, command summaries,
and examples that belong in terminal help.  `ehrcalc --help` and
`ehrcalc <command> --help` are the primary command documentation.
`ehrcalc docs cli` renders `docs/cli.md` from that model.  A test compares the
rendered result with the checked-in file, so command help and the Markdown
reference cannot drift.  The README and architecture documents remain
handwritten because they explain workflows and mathematical design rather than
enumerating flags.

## MCP

The `ehrcalc-mcp` binary exposes structured library operations over stdio.
Its first tools are `ehrhart_transform` for exact conversion and transform
operations, and `compute_family` for GT, Kostka, LR, key, order, and flow
requests.  Each tool calls the same Ehrcalc library adapter as the CLI.

The MCP input schema uses strings for arbitrary-size integers and rationals
where a JSON number would be unsafe.  It rejects unknown fields and ambiguous
transform inputs.  The initial tool surface covers:

- exact transforms and parsing;
- one-instance calculations for each mature family;
- bounded family scans; and
- verification of supplied Ehrhart data.

MCP request schemas must reject ambiguous input forms.  The response data must
match the CLI JSON schema closely enough that fixtures can exercise both
surfaces.  Expensive tool calls need explicit limits and must report partial
results or resumable checkpoint locations rather than relying on terminal
progress output.
