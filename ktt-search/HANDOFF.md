# KTT Search Handoff

## Genetic search and exact remote tails — 2026-09-11 (active)

- `a2be13af...` has a complete locally exact degree-25 reconstruction from
  ordinary `L(0)..L(14)` and structural strict counts 1--11.  It is positive;
  normalized minimum ~`5.79997e-5` improves the proper near-miss but is not a
  counterexample.
- Strict dilation 11 is `2721676964378370070744` and matched independently on
  Abacus (1984.865/2820.208 s local/remote). Direct out-of-sample `L(15)` job
  `20260910T205734-3fbdace166c2` also matched the prediction exactly at
  `729349049394323624299655020`; `a2be13af...` is certified positive.
- Exact genetic runs found no improved negative: the one-hole run covered
  8,467 proposal slots, while the two-hole/lower-degree run covered 13,064.
  The remote one-hole batch reproduced the negative parent then proved seven
  degree-71--78 one-hole descendants nonnegative.
- A fresh paired generation around `a2be13af...` classified 10,254 proposals
  into 4,668 nonempty exact short signatures, 3,085 dimension prunes, and
  2,328 empty outcomes. Proper degree-19 leaders `286549f5...` and
  `02909c47...` are exact locally through dilation 13; Abacus jobs
  `20260911T061817-5cdaaf727fef` and `20260911T062008-54ea5ea7f1dd` are
  finishing their exact tails serially. Do not duplicate these jobs.
- Closest proven negative remains degree-84 `612334d7...`, skew hook
  `(11,1^9)/(1)`, weight `1^19`, with exactly two internal nonflag holes
  `(4,6)` and `(6,10)`; `(5,14)` is the third equality and is flag-absorbed.

## Full corrected tail audit — 2026-09-10 (superseded checkpoint)

- Size-50 audit complete: 395 classes = 260 structurally valid ranked + 121
  durable deficient-span exclusions + 14 size prunes; zero timeouts.
- Degree-24 zero-hole `161a292e...` leads at tail depth 7, but becomes positive
  at tail depth 10.  Partial tail ordering is heuristic only.
- Structural degree-25 `a2be13af...` has locally/remotely matched exact strict
  counts through dilation 9.  Dilation 10 is active locally and on Abacus job
  `20260910T200302-bc3142e84596`; the over-cap predecessor is fully recorded.
- Abacus job `20260910T171434-ce38e10af4fa` independently matched proper
  `42d92b82...` at exact `L(17)=392796087462135314187172131`.
- Best actual negative remains degree 84 with two bad edges (`612334d7...`).
  Best fully certified proper near-miss remains positive degree 23
  (`42d92b82...`, normalized minimum ~`8.64986e-5`).

## Corrected structural-hull checkpoint — 2026-09-10 (superseded checkpoint)

- Owner remains Codex, sole KTT/private-companion worker and local MariaDB
  writer.  Generated reports/logs remain ignored; no push or publication.
- An apparent degree-25 negative was false: its scale-one lattice-point span
  was 24 while the structural polytope dimension is 25.  All 121 affected
  candidates were audited; 862 strict samples, 120 derived ordinary samples,
  13 polynomials, and their attempts were quarantined or invalidated.
- The maintained engine now counts flagged relative interiors from propagated
  structural bounds.  The crate has 38 passing tests and strict Clippy passes.
  Correct `a2be13af...` strict counts through dilation 7 are
  `0,0,0,56655893,136866338728,48574493345190,5544119621688912`; its
  partial signed tail is strongly positive.
- Best certified negative remains `612334d7...`, degree 84 and two bad edges.
  Best proper zero-edge near-miss remains positive `42d92b82...`, degree 23,
  normalized smallest coefficient about `8.64986e-5`.
- Abacus direct validation job `20260910T171434-ce38e10af4fa` is running.
  Structural control `20260910T192616-487d1344de57` is queued with ledger
  `f5b86942...`; wait for its exact local-count match before useful submission.

## Active fixed-content zero-hole search — 2026-09-10 (superseded checkpoint)

- Owner remains Codex, sole local MariaDB writer; generated reports/logs are
  ignored and no push/publication is authorized.
- The search is correctly distinguishing ambient flagged-Schur positivity
  from fixed-content flagged Kostka fibers.  A complete 8,159-proposal paired
  size-50 generation is stored in MariaDB: 4,125 nonempty exact short
  signatures, 2,544 exact-empty fibers, 1,490 dimension prunes, and no pending
  attempt.  Its eligible 381 signature classes have certified reciprocal
  tails; no completed candidate is negative.
- Best certified negative remains the degree-84 reduced Kogan face with two
  nonflag holes.  Best certified proper zero-hole near-miss remains positive,
  degree 21, normalized margin about `1.26228e-4` (`2af751a0...`).  New
  size-50/degree-21 `62ce9739...` is positive at about `1.3136e-4`.
- One zero-hole degree-23 class `42d92b82...` remains unresolved after exact
  local samples through `L(16)`; the last value is
  `105750769630621524527579594` and took 2270.763 s.  Local `L(17)` is running
  under 7,200 seconds.  The first Abacus `L(17)` attempt hit its recorded
  3,300-second cap; Abacus `L(16)` is active and wider `L(17)` job
  `20260910T171434-ce38e10af4fa` is queued with 6,900/7,200-second bounds.
  Abacus/local packed-counter matches have passed on a known control and four
  useful degree-21 counts; no partial result is being interpreted as a sign.

## Active ownership — 2026-09-08

- Owner: Codex, exact KTT counterexample-search session.
- Tracked scope: this handoff and any narrowly necessary audited scanner fixes.
- Generated scope: bounded/resumable exact scans and fresh JSON/log reports, all
  ignored and untracked.
- Every database hit must be parsed and cross-validated by the current scanner;
  historical ignored reports are planning hints only, not certified results.
- Starting point: clean tracked worktree at `1d53fc3`, with no live worker reported
  by the user and no prior KTT ownership remaining active.

## Exact search checkpoint — 2026-09-08

- Focused tests passed: 5/5. The release scanner was built with
  `CARGO_TARGET_DIR=/cargo-target/ai-projects`.
- Fresh straight, unflagged revalidation completed for sizes 2--11: 6,717 pairs,
  3,573 dominance prunes, 3,118 cached rows validated, 26 missing rows recomputed,
  zero unverified rows/errors/witnesses (`ktt-certified-straight-n2-11-20260908.json`).
- Size 12 contains 5,929 partition pairs: 3,311 fail dominance and 2,618 are
  eligible. Exact rows now exist for 2,614 cases, covering every dimension 0--37.
  Final per-dimension passes through 37 all ended `completed`, with zero cached-row
  defects, computation/database errors, negative coefficients, or Hibi--Stanley
  failures. Aggregate recorded compute time for size 12 is 6,009,538 ms.
- Four size-12 holes remain: three at dimension 38 and one at dimension 39. The
  first is `lambda=(3,3,2,1,1,1,1)`, `weight=(1^12)`, with empty inner shape and
  no flags. It stayed CPU-bound with controlled memory but failed to finish first
  under 60 s and then under a monitored 1,800 s bound. The engine has no
  within-case checkpoint, so this is the current genuine computational blocker.
- All fresh JSON/log filenames encode size, dimension, batch, and date. They remain
  ignored and untracked. Historical reports were used only to plan ranges.

## Completed ownership record — 2026-09-06

- Owner: Codex, current Ehrcalc audit-fix session.
- The user explicitly authorized adopting this directory despite its older ownership
  record, and confirmed that no other worker is editing the repository.
- Tracked scope: `Cargo.toml`, `src/main.rs`, this handoff, and project documentation.
- Generated scan JSON, JSONL, logs, local lockfiles, and local target output are ignored
  and must remain untracked.
- Status: implementation and verification are complete; no KTT file remains actively
  owned after the final checkpoint.

## Current implementation

`ktt-search` is a workspace binary for bounded exact KTT coefficient-positivity and
Hibi–Stanley scans backed by the `gt_ehrhart` MariaDB/MySQL table. It calls the
maintained `ehrcalc-kostka-engine` implementation and stores exact polynomials and h*
vectors.

The 2026-09-06 audit fixed these correctness defects:

- upper and lower flag requests are rejected independently when malformed;
- reaching `--max-cases` produces `status: "capped"` and `capped: true`;
- a database hit reloads its dimension, Kostka value, exact polynomial, and h* vector;
  the scanner parses and cross-validates them, reruns coefficient and Hibi–Stanley
  checks, and counts the row as `cached_verified` only if every check succeeds;
- malformed or inconsistent cached rows become `cached_unverified`, record
  `first_unverified`, and force an `incomplete` rather than no-failure conclusion;
- the Hibi ranges exclude invalid endpoints, including the dimension-two square
  regression h* = `(1,6,1)`;
- `--continue-after-first` retains the first witness rather than overwriting it.

Five focused unit tests cover flag independence, terminal status, exact stored
polynomial parsing, cached-row coefficient validation, and both endpoints of the Hibi
range. Strict Clippy passes for the entire workspace with `--all-targets -D warnings`.

## Historical report warning

The ignored JSON reports predate these corrections. In particular, a report labeled
`completed` may actually have stopped at a cap, and database skips in old reports were
not revalidated. Those reports are retained only as local work data and must not be
used as certified no-failure evidence. Rerun any range whose conclusion matters.
