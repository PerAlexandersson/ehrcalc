# Ehrcalc Handoff

## Active ownership — 2026-09-08

- Owner: Codex, exact KTT counterexample-search session.
- Scope: the `ktt-search` lane only: audited scanner execution, independently
  revalidated database rows, ignored reports/logs, and KTT handoff documentation.
- Search requirements: exact maintained Ehrcalc Kostka/Ehrhart code; bounded,
  resumable scans; no historical ignored report accepted as certified evidence.
- Repository state at adoption: clean tracked worktree at `1d53fc3`; no live worker
  reported by the user and no prior handoff ownership remained active.
- Generated KTT JSON reports and logs remain ignored and untracked. No push or
  publication is authorized.
- Fresh bounded genetic checkpoint (2026-09-08): the canonical two-hole negative
  was rerun with 2 generations, 8 elites, 16 persisted immigrants, 8 random
  immigrants, mutation strength 2, and effective-cell bias 0.15. The 45-second
  wall bound completed with 863 cache rows independently revalidated (545
  negative, 20 bad-edge-pruned, 3 deferred), zero new exact rows, and no changed
  frontier. Best remained mask `0x200000000010000001000`, dimension 84,
  equality count 3, and holes `(4,6),(6,10)`.
- A second fresh-seed batch (3 generations, mutation strength 3, minimum side 2,
  effective-cell bias 0.05, 24 persisted plus 24 random immigrants) completed
  under the 60-second guard. It evaluated 2,040 proposals (271 unique),
  independently validated 857 cached rows and 1 new exact row, and found no
  frontier improvement; the same dimension-84 two-hole certificate remains best.
- Complementary straight-shape/weight run (32 random seeds, exact cap 32,
  50-second bound) produced no negative and no completed new exact row; 11
  candidates were limited by state/time. Its best positive candidate has zero
  nonflag inequalities and polynomial degree 21, so it does not alter the KTT
  negative frontier.
- Larger-radius KTT escape batch (2 generations, mutation strength 4, minimum
  side 3, flag-only bias, 32 persisted plus 32 random immigrants) completed
  under 60 seconds. It evaluated 843 exact rows (3 new, 840 cached) and 58
  bad-edge prunes; all 534 negative rows remained on the existing frontier.
  No candidate beat two holes or dimension 84.
- A controlled three-hole lineage expansion (3-generation target, 1--2-cell
  mutations, 48 persisted and 16 random immigrants) was cut at the 60-second
  bound after one complete generation. It produced 1,964 proposals/134 unique
  candidates, revalidated 633 cached rows, and found no lower-degree or fewer-hole
  negative; the best remained the same two-hole degree-84 certificate.
- The persisted two-equality/two-hole seed was independently mutated with
  minimum side 2, mutation strength 3, and 48 immigrants. The bounded run
  revalidated 928 cached rows (557 negative), with 35 bad-edge and 3 size
  prunes and no new exact row. Cross-lineage ranking still selects the canonical
  degree-84, three-equality certificate as best; no fewer-hole result appeared.
- Shrink-biased mutation batch (2-generation target, 2-cell mutations, effective
  bias 0.45, 64 persisted immigrants) completed one generation at the 60-second
  bound: 2,344 proposals/134 unique, 646 cached exact rows, and no new exact
  evaluation. No size or degree improvement was found.
- Direct one-hole falsification batch (3-generation target, max bad edges 1,
  flag-only bias, 96 immigrants) completed one generation at the bound: 2,376
  proposals/166 unique, 649 cached exact rows, and 29 bad-edge prunes. No
  one-hole negative was found; the canonical two-hole certificate remained the
  best retained negative.
- Broader exploratory batch (max four holes, mutation strength 3, 48 persisted
  and 48 random immigrants) completed one generation at the bound: 1,616
  proposals/166 unique, 649 exact rows including 3 newly computed, and 29
  bad-edge prunes. No lower-degree negative or improved hole count appeared.
- High-radius shrink/escape batch (minimum side 4, mutation strength 5, 32
  persisted plus 32 random immigrants) completed two generations within the
  bound. It produced 863 exact statuses (858 cached, 5 new), 83 bad-edge
  prunes, one deferred and one timeout; no candidate improved the two-hole,
  degree-84 frontier.
- Larger-side one-hole batch (minimum side 4, max bad edges 1, mutation strength
  4, 48 immigrants) completed two generations: 931 cached exact rows and 66
  bad-edge prunes, with no new exact row and no one-hole negative.
- Fixed and exercised the hole-swap frontier adapter so current audited
  certificates derive selected cells from `mask_hex` when legacy `pairs` are
  absent. Its first bounded run evaluated 18 proposals (10 exact, 8 limited)
  and found 4 negative polynomials, including a new 3-hole degree-87 negative
  from replacing `(5,14)` with `(7,12)`. This does not beat the two-hole
  degree-84 frontier, but supplies a persisted negative lineage for repair.
- Durable result: straight unflagged KTT cases are freshly complete through size 11
  and, at size 12, through GT dimension 37 (2,614 of 2,618 eligible cases). No
  negative coefficient or error was found. Four size-12 holes remain at dimensions
  38,38,38,39; the first dimension-38 case exceeded a monitored 30-minute bound.
  See `ktt-search/HANDOFF.md` for exact bounds, pruning, timing, and the blocked pair.

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
