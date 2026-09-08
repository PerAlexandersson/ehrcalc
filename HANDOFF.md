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
- Iterating that explicit-pairs 3-hole lineage with move distance 3 evaluated
  154 proposals (26 exact, 23 limited) and found 11 distinct negative
  polynomials. The best stayed degree 87 with three holes; a new representative
  swaps `(4,6)` to `(5,6)` while retaining `(6,10),(7,12)`. No two-hole or
  lower-degree improvement was found.
- A radius-6 iteration evaluated 943 proposals (21 exact, 23 limited) and found
  9 distinct negative polynomials from 11 negative parents. The best remained
  degree 87 with 3 holes, represented by `(4,6),(5,9),(7,13)`; no degree or
  hole-count improvement was found.
- Radius-8 continuation evaluated 959 proposals (30 exact, 23 limited) from 9
  negative parents and found 14 distinct negative polynomials (15 negative
  results). The best remains degree 87 with 3 holes, represented by
  `(4,6),(6,9),(7,13)`; no two-hole or lower-degree result appeared.
- Radius-10 continuation evaluated 1,886 proposals (39 exact, 23 limited) from
  15 negative parents and found 12 distinct negative polynomials (14 negative
  results). The best remains degree 87 with 3 holes, `(4,6),(5,9),(7,13)`;
  no two-hole or lower-degree result appeared.
- Parameterized hole-swap proposals by target hole count and ran a direct
  two-hole repair pass from the 3-hole frontier. It evaluated 254 proposals (57
  exact, 22 limited) and found 2 negative results; the best is the existing
  canonical degree-84 two-hole certificate, reached by replacing `(8,14)` with
  `(5,14)` at move distance 3. No new lower-degree or one-hole result appeared.
- Chained the two-hole repair frontier into a one-hole target pass (radius 10,
  320 proposal cap). Four proposals were exact and all were positive; no
  one-hole negative was found.
- Fresh broad two-hole genetic batch (2 generations, mutation strength 4,
  48 persisted plus 48 random immigrants) evaluated 3,066 proposals/449
  unique candidates. It produced 1,029 exact rows (8 newly computed, 1,021
  cached), 96 bad-edge prunes, and 2 deferred rows; no frontier improvement.
- Radius-15 direct repair over the 3-hole frontier evaluated 261 proposals (53
  exact, 22 limited) from 14 negative parents and found 4 negative results. It
  recovered only the existing degree-84 two-hole certificate (best move
  distance 2); no lower-degree two-hole or one-hole result appeared.
- Fresh larger-instance genetic batch (minimum side 5, mutation strength 5, 32
  persisted plus 64 random immigrants) evaluated 2,388 proposals/353 unique
  candidates with 893 exact
  statuses (890 cached, 3 new) and 138 bad-edge prunes. It found no candidate
  beyond the degree-84, two-hole frontier.
- High-radius-20 two-hole repair sweep evaluated 279 proposals (53 exact, 22
  limited) from 14 negative parents and found 4 negative results. It recovered
  only the canonical degree-84 certificate; no lower-degree or one-hole result
  appeared.
- Larger-side-6 fresh-immigrant batch evaluated 841 cached exact rows and 160
  bad-edge prunes across two generations; no new exact candidate or frontier
  improvement was found.
- Fresh nonskew straight-shape/weight batch (64 random seeds, 48 exact cap,
  50-second bound) produced 1 exact and 12 limited candidates, with zero
  negative coefficients. The best remained the previously known degree-21,
  zero-nonflag candidate.
- Expanded nonskew batch (128 random seeds, 64 exact cap, 50-second bound)
  produced 13 limited candidates and no completed exact or negative result;
  the known degree-21 zero-nonflag candidate remains best.
- Added a provenance-preserving multi-swap operator (`--swap-count`) to the
  hole-swap frontier. A two-swap, one-hole pass evaluated 126 proposals (108
  exact, 18 limited) from 14 negative parents; all were nonnegative, so no
  one-hole counterexample was found.
- The first two-swap two-hole run found a new exact negative lineage at
  dimension 85, with nonflag holes `(5,8),(6,10)` and mask
  `0x100000010001000000`. It is weaker than the dimension-84 certificate but
  confirms multi-swap reachability of distinct two-hole negatives and is now
  persisted for further mutation.
- Mutating that degree-85 lineage with two-swap moves produced 5 additional
  negative records, all still dimension 85 (e.g. holes `(5,7),(5,9)`). No
  degree-84-or-lower or one-hole descendant appeared.
- A three-swap one-hole pass examined 759 proposals, capped after 120 exact
  evaluations. All 120 were nonnegative; no one-hole negative was found.
- Fixed multi-swap proposal generation to enforce `--max-proposals` during
  combination enumeration (rather than after exhaustive construction). A
  diagnostic three-swap two-hole run now bounded correctly at 30 proposals,
  with 18 limited evaluations and no negative result.
- The first full bounded three-swap/two-hole run generated 300 proposals; 24
  reached evaluation and all were limited by the exact state/time cap. No
  negative result was obtained.
- A long-budget follow-up evaluated 10 three-swap/two-hole proposals exactly
  (8-second per-candidate cap, 2-million-state ceiling); all 10 were positive.
- The disjoint offset-10 batch (using total cap 20) evaluated the next 10
  proposals exactly under the same bounds; all were positive. The attempted
  cap-10/offset-10 invocation correctly produced an empty slice and was not
  counted as a scan.
- The subsequent offset-20 batch (total cap 30) evaluated proposals 20–29
  exactly; all 10 were positive.
- The offset-30 batch (total cap 40) evaluated proposals 30–39 exactly; all 10
  were positive.
- The offset-40 batch (total cap 50) evaluated proposals 40–49 exactly; all 10
  were positive.
- The offset-50 batch (total cap 60) evaluated proposals 50–59 exactly; all 10
  were positive.
- The offset-60 batch (total cap 70) evaluated proposals 60–69 exactly; all 10
  were positive.
- The offset-70 batch (total cap 80) evaluated proposals 70–79 exactly; all 10
  were positive.
- The offset-80 batch (total cap 90) evaluated proposals 80–89 exactly; all 10
  were positive.
- The offset-90 batch (total cap 100) evaluated proposals 90–99 exactly; all 10
  were positive.
- The offset-100 batch (total cap 110) evaluated proposals 100–109 exactly; all
  10 were positive.
- The offset-110 batch (total cap 120) evaluated proposals 110–119 exactly; all
  10 were positive.
- The offset-120 batch (total cap 130) evaluated proposals 120–129 exactly; all
  10 were positive.
- The offset-130 batch (total cap 140) evaluated proposals 130–139 exactly; all
  10 were positive.
- The offset-140 batch (total cap 150) evaluated proposals 140–149 exactly; all
  10 were positive.
- The offset-150 batch (total cap 160) evaluated proposals 150–159 exactly; all
  10 were positive.
- The offset-160 batch (total cap 170) evaluated proposals 160–169 exactly; all
  10 were positive.
- The offset-170 batch (total cap 180) evaluated proposals 170–179 exactly; all
  10 were positive.
- The offset-180 batch (total cap 190) evaluated proposals 180–189 exactly; all
  10 were positive.
- The offset-190 batch (total cap 200) evaluated proposals 190–199 exactly; all
  10 were positive.
- The offset-200 batch (total cap 210) evaluated proposals 200–209 exactly; all
  10 were positive.
- The offset-210 batch (total cap 220) evaluated proposals 210–219 exactly; all
  10 were positive.
- The offset-220 batch (total cap 230) evaluated proposals 220–229 exactly; all
  10 were positive.
- The offset-230 batch (total cap 240) evaluated proposals 230–239 exactly; all
  10 were positive.
- The offset-240 batch (total cap 250) evaluated proposals 240–249 exactly; all
  10 were positive.
- The offset-250 batch (total cap 260) evaluated proposals 250–259 exactly; all
  10 were positive.
- The offset-260 batch (total cap 270) evaluated proposals 260–269 exactly; all
  10 were positive.
- The offset-270 batch (total cap 280) evaluated proposals 270–279 exactly; all
  10 were positive.
- The offset-280 batch (total cap 290) evaluated proposals 280–289 exactly; all
  10 were positive.
- The offset-290 batch (total cap 300) evaluated proposals 290–299 exactly; all
  10 were positive.
- The offset-300 batch (total cap 310) evaluated proposals 300–309 exactly; all
  10 were positive.
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
