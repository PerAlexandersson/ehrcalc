# Ehrcalc Handoff

## Active ownership — 2026-09-06

- Owner: Codex, current verification/fix session.
- Authorized scope: adopt and preserve the pre-existing formatting-only changes in
  `crates/ehrcalc-foundations/src/poset.rs`, `mcp/src/main.rs`, `src/exact.rs`,
  `src/families.rs`, and `src/render.rs`; adopt all source and handoff material under
  `ktt-search/`; fix the audit findings in `src/key_scan.rs`, KTT scanning, manifests,
  tests, and documentation.
- Generated KTT JSON reports and logs remain untracked and must not be committed.
- Host supervisor confirmation from the user: no other active worker is editing this
  repository.

## Current task

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
- Focused key-scan tests pass: 14 tests, including the repeated-part witness
  `lambda=(1,1,0), sigma=231`, exhaustive maintained-Kogan comparisons through
  `S_3`, and malformed/conflicting resume-row regressions.
- Focused KTT tests pass: 5 tests covering independent flags, terminal status,
  cached-row validation, exact polynomial parsing, and the square h* regression.
- `cargo clippy -p ktt-search --all-targets --no-deps -- -D warnings` passes.
- Workspace-wide strict Clippy currently exposes pre-existing lints in the two
  maintained internal crates; these remain to be resolved before final handoff.
