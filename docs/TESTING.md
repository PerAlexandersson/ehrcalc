# Testing Strategy

Ehrcalc is intended to support mathematical research.  Exactness, regression
coverage, and independently checked examples take priority over superficial
command coverage.

## Test Layers

### Exact Algebra Unit Tests

Every operation on exact polynomial data needs focused tests:

- Lagrange and Newton interpolation at arbitrary distinct rational points;
- power, h*, and binomial-basis conversions in both directions;
- finite difference, discrete summation, dilation, shifting, products, and
  scalar operations;
- preservation of declared dimension and trailing h* zeros; and
- invalid input, including inconsistent dimension and non-integral h* data.

Use examples with hand-computable answers, round trips, and identities checked
at several exact rational values.

### Family Tests

Each family must include:

- small examples with published or hand-verified values;
- an independent count or formula whenever practical;
- interpolation validation at unsampled dilations;
- reciprocity validation when an interior-count model is available; and
- dimension tests, including degenerate and empty inputs where meaningful.

GT and flagged examples must match the frozen `kostka` reference fixtures.
Order-polytope examples must match direct order-preserving-map counts.  Flow
examples must match small enumerated flows.  Key examples must match checked
fixtures before a new evaluator becomes the default.  The fast key scanner is
cross-checked exhaustively against the maintained Kogan-face route in small ranks,
including dominant weights with repeated parts.  Resume tests must reject malformed
permutations, inconsistent derived vectors, and conflicting duplicate rows.

### Regression Fixtures

Keep small, reviewed JSON fixtures under version control.  They include known
GT and order examples, key staircase rows, the Key-HStar-Bruhat `S_3` and
`S_4` tables, the `S_5` summary counts, and the documented non-staircase first
failure.  Fixtures must record the command input, the exact expected data, and
the source or derivation.

### CLI And MCP Integration Tests

Every public command and MCP tool needs success, invalid-input, JSON-schema,
and deterministic-first-failure tests.  The text and LaTeX renderers are
tested against small golden outputs, but mathematical assertions are made on
the shared exact result model rather than formatted strings.

## Required Checks

Run the full standalone workspace tests during development:

```bash
timeout 60s nice -n 10 cargo test --workspace
```

The database-backed KTT research binary has focused tests that do not require a
database:

```bash
timeout 60s nice -n 10 cargo test -p ktt-search --all-targets
timeout 60s nice -n 10 cargo clippy -p ktt-search --all-targets --no-deps -- -D warnings
```

They cover independent upper/lower flag validation, capped and incomplete terminal
states, exact cached-polynomial revalidation, and the dimension-two square h* vector
`(1,6,1)`.

Before a feature is merged, also run the relevant cross-project comparison
tests and record any long benchmark separately.  Tests that require more than
60 seconds must be explicit, bounded, and not become the default test suite.

For a quick command-level key h-star check, run:

```bash
timeout 60s nice -n 10 cargo run --quiet -- \
  key-scan --n 3 --staircase --packets d-route
timeout 60s nice -n 10 cargo run --quiet -- \
  key-scan --n 3 --staircase --sigma 3,1,2 \
  --packets none --rows --format json
```

The first command must report `6/6` real-rooted h-star rows and `8/8`
strong-cover interlacings.  The second must return the exact h-star vector
`["1", "2", "0"]`; see [`KEY_HSTAR.md`](KEY_HSTAR.md).
