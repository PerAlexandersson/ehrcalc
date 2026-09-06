# Ehrcalc Handoff

## Completed ownership record — 2026-09-06

- Owner: Codex, current verification/fix session.
- Authorized scope: adopt and preserve the pre-existing formatting-only changes in
  `crates/ehrcalc-foundations/src/poset.rs`, `mcp/src/main.rs`, `src/exact.rs`,
  `src/families.rs`, and `src/render.rs`; adopt all source and handoff material under
  `ktt-search/`; fix the audit findings in `src/key_scan.rs`, KTT scanning, manifests,
  tests, and documentation.
- Generated KTT JSON reports and logs remain untracked and must not be committed.
- Host supervisor confirmation from the user: no other active worker is editing this
  repository.
- Status: implementation and verification are complete; no file remains actively owned
  after the final checkpoint.

## Completed task

1. Make repeated-part key scans agree with the maintained Kogan-face implementation,
   with exhaustive small cross-checks.
2. Make KTT flag handling, capped status, cached-row verification, and Hibi–Stanley
   validation sound and tested.
3. Validate resumed key-scan rows and reject conflicts or inconsistent derived data.
4. Integrate the KTT scanner into the workspace, update documentation, run full tests
   and strict Clippy with an external Cargo target directory, and make focused local
   checkpoint commits without pushing.

## Verification status

- Baseline `cargo test --workspace --all-targets`: 193 tests passed.
- Baseline `ktt-search` had zero tests and failed strict Clippy.
- Baseline formatting-only dirty diffs were inspected and contain no semantic changes.
- Checkpoint `a41abd3` preserves the five inherited formatting-only changes and this
  ownership record.
- Checkpoint `27de446` fixes repeated-part key scans and rejects invalid resume data.
- Checkpoint `85d5e72` integrates and hardens `ktt-search`, with generated reports
  ignored and historical report limitations documented.
- Checkpoint `f163a8e` makes strict Clippy clean across every workspace target.
- Focused key-scan tests pass: 14 tests, including the repeated-part witness
  `lambda=(1,1,0), sigma=231`, exhaustive maintained-Kogan comparisons through
  `S_3`, and malformed/conflicting resume-row regressions.
- Focused KTT tests pass: 5 tests covering independent flags, terminal status,
  cached-row validation, exact polynomial parsing, and the square h* regression.
- `cargo clippy -p ktt-search --all-targets --no-deps -- -D warnings` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes after narrow,
  behavior-preserving lint cleanup in the maintained crates.
- `cargo test --workspace --all-targets` passes all 202 tests.
- Exact command-level comparison for `lambda=(1,1,0), sigma=231` agrees between
  `key` and `key-scan`: power coefficients `(1,3/2,1/2)`, h* `(1,0,0)`, and
  binomial coefficients `(1,2,1)`.
