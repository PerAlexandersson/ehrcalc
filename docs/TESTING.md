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

The two order-polytope h* methods (`ideals` and `frontier`) are cross-checked
for exact equality of the full `EhrhartData` on chains, antichains, fences,
shapes, products of chains, covers given in non-natural labelings and
deterministic random posets, against known Eulerian and Narayana values, and
against the order-ideal count `L(1)`.  Budget behavior is tested at the exact
boundary: error for `ideals`, fallback for `auto`, rejection for `frontier`.

GT and flagged examples must match the frozen `kostka` reference fixtures.
The two-label fast path is checked against exhaustive integer box points,
legacy counts for small skew shapes and flags, and direct intermediate-chain
enumeration for every combination of two-row forbidden and strict masks.
Known hypersimplex ordinary/interior counts, a flag-forced point, wide translated
totals, large capacities, state-budget zero, zero-weight labels, and the twelve
coordinate fallback guard are covered by `two_label::tests`.
Order-polytope examples must match direct order-preserving-map counts.  Flow
examples must match small enumerated flows.  Key examples must match checked
fixtures before a new evaluator becomes the default.  The fast key scanner is
cross-checked exhaustively against the maintained Kogan-face route in small ranks,
including dominant weights with repeated parts.  Resume tests must reject malformed
permutations, inconsistent derived vectors, and conflicting duplicate rows.

### Regression Fixtures

Constraint-polytope comparisons live in `tests/constraint_gt.rs` and
`tests/constraint_fixtures.rs`. The GT gate uses 1,024 deterministic flagged
and skew inputs: both adapters match ordinary and relative-interior Kostka
counts at dilations 1, 2, 3 on exact and CRT paths, and match `gt_ehrhart`
dimensions and full data with and without reciprocity. It includes empty
fibres, zero-weight labels, and both 2026-10-08 flag-relaxation regressions.
The geometric fixtures use the hypersimplex inclusion-exclusion formula,
Beck–Pixton's full Birkhoff B3/B4 polynomials, original-box enumeration for
transportation polytopes, and brute-force sums of independent-set/basis
incidence vectors for uniform and small graphic matroids. Loops, parallel
edges, bridges, degenerate polytopes and inconsistent margins are included.

`tests/constraint_cli.rs` checks source parsing, converter round trips,
plan-only results, ordering controls, exact counts, budgets, all formats and
visible extra-sample warnings. `mcp/tests/constraint_stdio.rs` starts the
actual MCP binary and checks tool registration, schema, shared-service
equality, warnings and deterministic invalid requests.

For the serial timing table (excluded from default tests), run the guarded
Cargo command below with `test -p ehrcalc --test constraint_gt
timing_full_reduced_and_existing_gt_engine -- --ignored --nocapture
--test-threads=1`. It includes adapter construction and all service work.
The existing engine is observed with `kostka_dp::profile_kostka_state_peak`,
which measures actual maintained partition-DP layers on the calling thread.
This benchmark uses adaptive reciprocity, whose sampling is sequential;
the observer does not capture Rayon worker threads. Closed formulas that
create no DP map contribute zero. Peaks are largest single layers, rather
than the sum of concurrently allocated maps or the affine-hull workspace.

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

In the constraint-polytope lane, prefix **every** Cargo command (including
formatting, docs generation and metadata) with this serialized memory guard:

```sh
/cargo-target/rr-lanes/tools/heavy.sh START_MB=8000 -- \
  env CARGO_TARGET_DIR=/cargo-target/polytool-lanes/ehrcalc-cp \
  CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0 timeout 1200 nice -n 10 cargo ...
```

Required final commands are `fmt --all -- --check`,
`clippy --workspace --all-targets -- -D warnings`, and `test --workspace`.
The guard waits for the shared lock; other workers' builds remain untouched.

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
