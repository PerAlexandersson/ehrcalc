# KTT search utility

The `ktt-search` workspace binary performs bounded exact searches for negative
coefficients of stretched Kostka Ehrhart polynomials and for Hibi–Stanley h*
inequality failures. It is a research utility, not part of the stable `ehrcalc` CLI
contract, and requires a MariaDB/MySQL database URL.

Run it from the repository root with an external Cargo target directory:

```bash
CARGO_TARGET_DIR=/cargo-target/ai-projects \
  timeout 60s nice -n 10 cargo run -p ktt-search --release -- \
  --db-url "$KOSTKA_DB_URL" --min-size 2 --max-size 10 \
  --max-cases 100 --report-path ktt-search/scan-report.json
```

The scanner uses exact rational polynomial coefficients and exact integer h* vectors.
Upper and lower flags are normalized and validated independently. A supplied empty
flag list is invalid for a nonempty weight; omitting a flag option means that side is
unflagged.

## Report status and database rows

The JSON report distinguishes these terminal states:

- `completed`: the requested range finished with no counterexample and no unverified
  or failed cases;
- `counterexample_found`: a negative coefficient or Hibi–Stanley witness was found;
- `capped`: `--max-cases` stopped the requested range early;
- `incomplete`: a computation, database operation, or cached-row validation failed;
- `dry_run`: enumeration finished without performing the exact computations.

A database hit is not trusted merely because its key exists. Before it contributes to
a no-failure result, the scanner reloads and validates the stored dimension, exact
polynomial, Kostka value, and h* vector, checks integral sample values, recomputes h*,
and reruns coefficient and Hibi–Stanley checks. Invalid rows increment
`cached_unverified`, populate `first_unverified`, and make the report incomplete.

The 2026-10-08 correctness repair replaced GT interval-propagation dimensions by
exact affine hulls.  Before it, some GT dimensions were overestimated and some
empty fibers were treated as nonempty, which corrupted reciprocity-interpolated
polynomials.  Cached rows whose stored dimension differs from the repaired
dimension now fail validation as `cached_unverified`, and fibers now proved
empty are skipped before any cache lookup.

Agreement with the repaired dimension is only a necessary check.  It does not
validate an old reciprocity-derived polynomial: the scanner's cached-row checks
compare stored data with itself (degree, `L(0)`, integrality, stored Kostka
value, recomputed h*) and do not count fresh lattice points.  Every historical
row therefore remains unaudited until its polynomial is checked against fresh
direct positive counts beyond its interpolation samples.  The repair did not
read, modify, or delete database rows.

Generated JSON, JSONL, and log files under `ktt-search/` are ignored working data. Do
not commit them. Reports produced before the 2026-09-06 validation fixes are not
reliable no-failure certificates and should be rerun when their conclusions matter.

## Focused checks

```bash
CARGO_TARGET_DIR=/cargo-target/ai-projects \
  timeout 60s nice -n 10 cargo test -p ktt-search --all-targets
CARGO_TARGET_DIR=/cargo-target/ai-projects \
  timeout 60s nice -n 10 cargo clippy -p ktt-search --all-targets --no-deps -- -D warnings
```
