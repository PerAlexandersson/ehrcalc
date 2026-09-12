# KTT Search Handoff

## Continuation — 2026-09-12 (active)

- Codex remains sole KTT/private worker and MariaDB writer; host uses one
  low-priority CPU and Abacus is idle after v58.
- Abacus v58 `20260912T101538-f425bb2d86d2` independently exactly validated
  one-hole negative `ee163...` against the archived local replay (3,181,134
  states; control `468b1c...` 2,938,320); input `062039f2...`, runner
  `4f6842b...`, helper `a4aae249...`, archive `bf1cfa42...`, reuse ledger
  `3cca053d...`. v74/v75 added six distinct one-hole 6M-state limits and four
  bad-edge prunes; no exact zero-hole negative. Zero-hole closures remain
  positive. Generated artifacts stay untracked; no push/publication.

- Latest local expansion v85/v87 produced exact one-hole negatives `cc6d...`
  (d114) and `be158...` (d111), whose six fresh zero-hole flag closures are
  exact-positive (noncached d112/d109 checks 95.518/95.180 s); distinct d125/
  d128 moves reached the 6M cap. Codex remains sole writer.

- Latest verification (2026-09-12): host v89--v99 found fresh exact one-hole
  negatives `596092...` (d115), `4805...` (d117), `a9e07...` (d114),
  `1dda...` (d109), and `31f1...` (d113). Their fresh zero-hole closures are
  all exact-positive: d114/d116 (86.622/87.192 s), d118 (87.896 s),
  d110/d115 (86.047/87.963 s), d105 (81.115 s), and `31f1...` d112/d114
  (125.609 s/cached). The exact zero-hole-negative count remains zero. v100
  is the sole active low-priority host process: a distinct bounded
  negative-reservoir growth/augmentation/closure pass (4 exact cases, 6M
  states/360 s each, 850 s outer), no remote job. Inspect recorded IDs and
  Abacus state before a new remote submission; fetch, local-replay, and ingest
  every new remote result before any claim. Generated artifacts are untracked.

- Closure audit update (2026-09-12): v100 completed 179 bounded proposals
  (324 cache hits), yielding exact one-hole `317502...` (d111), `64559a...`
  (d113), `f16494...` (d116), and three d109 variants. v101--v106 exhausted
  their direct zero-hole closures: fresh d110 `545932...` (86.819 s), d115
  `1a848e...` (98.129 s), and d117 `7b71dd...` (100.018 s) are
  exact-positive, as are cached d112 `326811...`, d114 `edfae...`, and three
  d105 faces. Exact zero-hole-negative count remains zero. Fresh Abacus status
  has no nonterminal job and v58 remains the latest done KTT ID; no duplicate
  remote submission occurred.

- Latest closure audit (2026-09-12): v107 broadened the reservoir sample
  (230 proposals/330 cache hits), yielding exact one-hole negatives
  `d685db...` (d109), `0f13b3...` (d111), and `cd65c1...` (d115); d104 was
  exact-positive. v108--v110 exhausted their direct zero-hole faces: fresh
  d108 `fa2370...` is exact-positive (83.434 s), while d110 `d8b728...`,
  d112 `3e369a...`, and d114 `c9c827...` are exact-positive cache hits.
  Exact zero-hole-negative count remains zero; no remote job is pending.

- Latest closure audit (2026-09-12): v111 (256 proposals/334 cache hits)
  added exact one-hole negatives `1b6229...` (d107), `b3945f...` (d109), and
  `cd51f5...` (d111); d109 `7eb148...` and d114 `674690...` were positive.
  v112--v114 closed every new lineage: fresh d106 `6225de...` is
  exact-positive (80.226 s), while d108 `e3e685...`, d110 `8aece1...`, and
  d112 `3e369a...` are exact-positive cache hits. Exact zero-hole-negative
  count remains zero; no remote submission is pending.

- Active continuation (2026-09-12): v115 had 340 proposals/346 cache hits and
  directly exact-checked four fresh zero-hole faces: d111 `341038...`
  (94.041 s), d116 `9e27f4...` (97.211 s) and `be8fc3...` (96.569 s), d117
  `eb3c51...` (98.549 s), all positive. Exact zero-hole-negative count is
  zero. After an idle Abacus check, sole remote v115
  `20260912T123219-40c93c2ad430` started for distinct deferred d114
  `613fa3...` (paired deferred record is the same poset and was not
  duplicated), input `5b692edf...`, control plus target, 6M/360 s and 900 s
  outer. It is running; v116 is the sole low-priority host search. Fetch and
  ingest remote output only if it is a contender; generated files untracked.

- Remote-discovery update (2026-09-12): user authorized continuous Abacus
  discovery and replay only for a zero-hole contender. v115 d114 `613fa3...`
  is primary remote exact-positive (3,515,148 states, 144.928 s; ledger
  `be371451...`); v116 d126 `23014a...` state-limited at 6M/253.901 s. v117
  cleared d13 `cfd001...` (63 states) and d14 `c6c480...` (74; ledger
  `3d621c3b...`); v118 d106 `8d9c4f...` is exact-positive (3,744,510 states,
  156.033 s; ledger `cf622f26...`). Each ran `468b1c...` control first under
  2 CPUs/8 GiB/no swap, with hashes in the ledger. None was negative, so no
  local replay. Host v117 d111--d114 and v118 d106 zero-hole faces are also
  positive; v119 is sole active low-priority host pass and the exact
  zero-hole-negative count remains zero.

- Compute expansion (2026-09-12): Codex remains sole KTT/private worker and
  local MariaDB writer. Host v120 completed its bounded pass (348 proposals,
  365 exact/cache results, 56 deferred, 142 bad-edge prunes), with no zero-hole
  negative; host v121 is now the one low-priority local process. Before remote work, recorded IDs and
  live state were checked: no nonterminal job existed and v118 was latest.
  Initial v121 `20260912T130417-3259e07c5ba4` failed before calculation because
  the command was serialized as one filename (exit 127), hence no duplicate
  result. Corrected v121r `20260912T130612-6552b61079af` runs deferred d125
  one-hole `425396...` plus control `468b1c...`, input `04545d52...`. Queued
  v122 `20260912T130721-5ab36a02c9c2` uses both audited Abacus CPUs with four
  distinct d125/d119/d117/d119 one-hole cases concurrently within its two-CPU
  affinity; input hashes
  `588b8838...`, `fc044969...`, `6ace516a...`, `c8d24af6...`, runner
  `4f6842b...`, helper `a4aae249...`, archive `bf1cfa42...`, 6M/360 s each,
  900-s outer, serial queue and 8-GiB/no-swap cap unchanged. Remote outputs
  are discovery evidence; replay is reserved for an exact zero-hole negative.
  The exact zero-hole-negative count is zero; generated inputs/logs untracked,
  no push/publication.

- Six-CPU continuation (2026-09-12): v121r `20260912T130612-6552b61079af`
  exactly completed control `468b1c...` (2,938,320 states, 117.010 s) and
  left d125 `425396...` state-limited at 6M states in 265.635 s; output
  `a2584807...`, job `979e2d9d...`, and provenance are recorded in MariaDB.
  It is nonexact, so no replay. v122 was cancelled before dispatch after an
  over-parallel command was identified and produced no result. The user
  authorized six pinned CPUs / 20-GiB hard cap / no swap / 256 tasks; smoke
  job `20260912T131630-2e3a02592ed5` verified every limit and protection.
  Following an idle check, v123 `20260912T131752-b2cbcef91a9c` began six
  distinct control-first one-hole cases concurrently, 6M/360 s each and 900-s
  outer; inputs `588b8838...`, `fc044969...`, `6ace516a...`, `c8d24af6...`,
  `47a63890...`, `cacfb367...`, runner `e88d0879...`. Host v121 is sole local
  process. Exact zero-hole-negative count is zero; replay remains reserved
only for such a result.

- Live verification: v123 has six `abacus_ideal_job.py` children at
  98.7--99.5% CPU each under nice 10 and the six-CPU affinity; host v121 is
  the sole local KTT process. CPUs 6--7 remain outside compute for control.

- Expanded zero-hole continuation (2026-09-12): host v121 completed 348
  bounded proposals (361 exact/cache, 56 deferred, 151 prunes) without a
  zero-hole negative; v122 is the sole local KTT process. Queued six-CPU v124
  `20260912T132249-3711ec0e64bf` retries six direct zero-hole cases formerly
  limited at 6M, each with control `468b1c...`, 20M states/900 s and 1200-s
  outer; inputs `a83abed1...`, `b799bfa7...`, `32f10a59...`, `601551ab...`,
  `fb1ed437...`, `79c45dea...`, runner `329e89f0...`, helper `a4aae249...`,
  archive `bf1cfa42...`. Any exact zero-hole negative gets a local replay
  before a claim. Count remains zero; generated inputs/logs untracked, no push.

- Remote-batch update (2026-09-12): v123
  `20260912T131752-b2cbcef91a9c` completed six one-hole screens: four
  360-second limits and two 6M state limits (340.883/327.400 s); each control
  was exact, per-case resource/hash provenance is in MariaDB, no replay.
  Direct zero-hole v124 `20260912T132249-3711ec0e64bf` is active after six
  exact controls; disjoint six-case v125 `20260912T133115-c6fb2da2e610` is
  queued, both 20M/900 s. Host v123 is sole local process. Zero-hole count: 0.

- Current exact-frontier update (2026-09-12): Codex remains sole KTT/private
  worker and MariaDB writer. Following a recorded-ID/live-state audit, v124
  `20260912T132249-3711ec0e64bf` completed: six formerly 6M-limited zero-hole
  targets (`23014a...`, `ec4502...`, `69bdbb...`, `795c71...`, `909c74...`,
  `376f47...`) are exact-positive (6,247,670--7,758,608 states; 375.314--
  559.849 s). Each child control matched `468b1c...` under the verified
  six-CPU/20-GiB/no-swap/256-task profile. Fetched job/output provenance is in
  primary MariaDB ledger `87f78710...`; no local replay was appropriate because
  no target was negative. v125 `20260912T133115-c6fb2da2e610` is live with six
  CPUs and distinct v126 `20260912T133501-197c3031f1d3` is queued. Host v124
  ended (365 exact/cache, 53 deferred, 139 pruned; no zero-hole negative), and
  host v125 is the sole low-priority one-core pass. Seven zero-hole state
  limits remain; exact zero-hole negatives: 0. Generated files stay untracked;
  no push. Freshly audited v127 `20260912T134726-38e0e9e0e3c4` is queued behind
  v125/v126: four distinct legacy five-second local zero-hole timeouts, inputs
  `012e4c0d...` and `8414be3b...`, now have 20M-state/900-s exact bounds and a
  1200-s outer bound; runner `329e89f0...`, helper `a4aae249...`, archive
  `bf1cfa42...`.

## One-hole exact-negative breakthrough — 2026-09-11 (active)

- Suspension checkpoint: Abacus is paused with no running KTT job, and no
  local KTT process remains. Job `20260911T202944-64c51cfc1209` was cancelled
  after starting; inspect its ID before resubmission. The first grow-then-close
  batch ended with seven four-million-state limits; the second was cancelled.
  A paired zero-hole shape/flag-plus-weight screen from certified closure
  `368eace0...` stored all 231 proposals, finding 230 exact-empty fibers and
  one equivalent row. A tested six-million-state/360-second selected retry is
  ready but was not submitted.
- Codex remains the sole KTT/private-companion worker and local MariaDB writer;
  reports/logs stay untracked, with no push or publication.
- Recovery update (2026-09-12): inspected all suspension-era Abacus IDs before
  resuming anything.  v21 first batch `20260911T202944-adebc62a5d43` is done;
  its seven useful cases were all exact 4,000,000-state limits and now have
  durable MariaDB attempt records with remote hashes/provenance.  Paired v21
  `20260911T202944-64c51cfc1209` is cancelled after its control only and was
  not duplicated.  Fetched v20 `20260911T202641-7863232114c7` matched a local
  archived-runner replay exactly: zero-hole `0e002288...` (d102, 1,503,918
  states) and `bf191ca8...` (d108, 1,511,640 states) are positive and are
  ingested in ledger `78bd9845...`.  Adapter tests pass 8/8.  Abacus was idle
  and paused, then resumed only for selected retry `20260912T062104-f403c712dc54`:
  control plus smallest unresolved zero-hole `00616831...` (d118), 6M states/
  360 s per case, 900 s outer, 2 CPUs and 8 GiB/no swap.  It is running.  A
  byte-identical local replay of one-hole seventh-flag job
  `20260911T201701-d347c91cae66` is in progress.
- Continuation update (2026-09-12): v23 `20260912T062104-f403c712dc54` made
  its control exact and d118 `00616831...` state-limited at 6M states in
  242.812 s; v24 `20260912T063005-3442a1654389` likewise limited distinct
  d120 `8b0bd1e5...` at 6M in 245.894 s. Both have durable MariaDB status and
  complete input/runner/helper/archive provenance; neither is a conclusion.
  Archived-runner local validation of v19 `20260911T201701-d347c91cae66`
  certifies d112 `36abbd66...` negative in degrees 1--3 with seven genuine
  flags, sole hole `(6,11)`, and 2,917,271 states (ledger `912ba2b3...`). Its
  direct zero-hole closure `a305b966...` is exact-positive at 3,492,125 states
  after matching job `20260911T201700-67a7fc603374` (ledger `ca8def38...`).
  After a fresh idle/unpaused queue check, only distinct v25
  `20260912T063847-0b0edc29f203` is active: d120 `ffc12142...`, control plus
  candidate at 6M/360 s per case and 900 s outer, input SHA `641070b2...`,
  verified two CPUs and 8 GiB/no swap. Codex remains sole owner/writer; inputs
  and reports are untracked and no push/publication is authorized.
- Continuation 2 (2026-09-12): v25 `20260912T063847-0b0edc29f203` matched its
  control and limited d120 `ffc12142...` at 6M states in 251.702 s; durable
  MariaDB status is `state_limited`, not an exact conclusion. Host-local v26
  independently matched control and limited d122 `544fc4d7...` at 6M in
  172.835 s, with local runner/artifact provenance stored. After an idle queue
  check, sole remote v27 `20260912T064619-d3ac01122282` started for distinct
  d122 `909c74f0...` (input `dad8891b...`, 6M/360 s per case, 900-s outer).
  Five distinct low-priority one-core host lanes are active for d123
  `37a00827...`, d124 `e8e1c5f1...`, d125 `376f47be...`/`4e987b5c...`, and
  d126 `23014acf...`; five of the 12 host CPU-equivalents remains below the
  user-set 50% cap. All logs are untracked and Codex remains sole writer.
- Continuation 3 (2026-09-12): every remaining v21 grow-close zero-hole case
  through d128 now has a selected 6M bound/provenance record; remote d122
  `909c74f0...` limited in 252.162 s, host d123--d128 in 172.835--194.428 s.
  Seven-flag closure `9944986a...` is exact-positive (3,435,640 states), and
  Abacus v36 `20260912T070101-88ae1d8fb578` exactly reproduced it in 139.503 s
  with local archived replay complete. New one-hole d110 `12c81c1e...` is
  locally exact-negative in degrees 1--2 with nine genuine flags, sole hole
  `(6,11)`, and 2,769,132 states; its zero-hole d111 closure `3f8d1ab3...` is
  exact-positive (3,308,370 states). v39 `20260912T070834-aa54b96e6755`
  exactly completed d106 `3f89f4ee...` at 1,512,790 states; matching local
  replay is active before ingestion. No flagged zero-hole negative is certified.
- Continuation 4 (2026-09-12): archived local replay exactly matched and
  ingested v39 d106 `3f89f4ee...` (1,512,790 states; ledger `b9726259...`).
  v36 exact-positive flagged closure `9944986a...` also locally matched and
  was cross-validation-reused (ledger `356ae75d...`). Fresh queue check found
  Abacus idle; sole active job v40 `20260912T071605-2e03a797c1e5` is the
  independent exact cross-check of new nine-flag one-hole negative d110
  `12c81c1e...`, input SHA `6f91a3ef...`, 6M/360 s per case and 900-s outer.
- Best certified negative `86b98937ad8a...` has one bad edge, `(a,b)=(10,11)`,
  mask `0x10000000000`, degree 109, and one negative Ehrhart coefficient
  (degree one). Its lift is `(12,1^10)/(1)`, weight `1^21`, with sole bad pair
  `(6,10)`. Abacus job `20260911T171935-1848e90938e3` and the local replay
  matched the full exact polynomial and 1,512,988 states.
- Every direct zero-hole flag swap from this bare one-equality parent is
  positive (complete degree ladder 100--110). Jobs
  `20260911T173913-a0e37431c8c2` and `20260911T173913-edff1ae44197` validate
  the locally exact hard cases.
- The next negative lineage adds flag-absorbed equalities while retaining the
  one hole. Local exact augmentation found negatives of degrees 104, 105, 107,
  and 108. Best `fd2cb24e0106...` has forbidden pairs `(6,10),(6,16)`, only
  `(6,10)` bad, and a negative linear coefficient. Abacus job
  `20260911T175147-ea1a52166b1f` matched all seven hard augmentations exactly;
  ledger `219c3781...` ingested them.
- Both direct zero-hole closures of `fd2cb24e...` are positive. The new
  degree-100 closure `237c4477...` matched locally and on Abacus job
  `20260911T180458-e3259fec5668` at 1,816,100 states; ledger `daa6118e...`
  ingested it.
- Four second flag-augmentation descendants matched locally and in Abacus job
  `20260911T181047-845c7ed0600a` (input `ea075054...`, output `86b52f0b...`);
  ledger `4f52b434...` ingested them. Degree-105 `498534b7d86a...` and
  degree-107 `47114e5e73f7...` remain negative with two flag-absorbed edges and
  sole bad pair `(6,10)`; the other two are positive.
- Complete zero-hole closures of both negatives are stored. Five hard cases
  matched locally and on serial Abacus jobs
  `20260911T181912-842f34239bad` and `20260911T182024-c44fe945633b`; all are
  positive (ledgers `7387623b...`, `885584ab...`).
- Strongest flag-heavier negative `38a1cb1c...` has degree 105, mask
  `0x80211010000000`, four valid consecutive row-2 flags `(2,9)..(2,12)`, and
  sole bad pair `(6,10)`. Job `20260911T183757-4fc05c65bfed` matched the full
  local polynomial and 1,512,108 states; ledger `f77199f5...` ingested it.
  Its three-flag predecessor `013148dd...` is negative in degrees one and two.
  All direct zero-hole closures are positive, and adding a fifth consecutive
  flag is positive. Redundant Kogan mask bits are not counted as flag edges.
- A bounded size-growth generation stored 13 one-hole descendants of
  dimensions 112--120. The two-million-state/120-second remote/local frontier
  left all 13 unresolved; bounded audit ledgers `62ac2e3a...`,`c6648387...`
  preserve every outcome. A selected four-million-state/240-second retry found
  degree-112 `560247ab...` positive and new degree-115 `97d5a35d...` negative
  in coefficients one, two, and three. The latter has four valid flags
  `(2,10)..(2,13)`, sole bad pair `(6,11)`, mask `0x40082201000000000`, and
  2,949,452 states. Job `20260911T193133-fa1ca34023e1` matched local exactly;
  ledger `b1a401e4...` ingested it. Its direct zero-hole closures are next.
- Both direct zero-hole closures are remotely exact-positive in job
  `20260911T194704-0276f54e649b`. The fifth-flag child retaining the bad edge,
  `7f22a02f...`, is remotely exact-negative in degrees one through three,
  degree 114, with 2,946,757 states (job `20260911T194804-b0ec8cf41536`, output
  `caf34a9c...`). Its flags are `(2,9)..(2,13)`, with sole bad pair `(6,11)`.
  The fifth-flag child has now matched a quiet local replay exactly in 75.169
  seconds and 2,946,757 states; local output hash is `39b987dc...`, and ledger
  `c52f1e8a...` ingested it. It is the certified structural leader with five
  genuine flags and one bad edge. Its unique sixth-flag child `468b1c0e...`
  is now also certified negative: degree 113, six genuine flags, sole bad pair
  `(6,11)`, and negative coefficients one through three. Local replay matched
  Abacus job `20260911T200614-f1f539996139` at 2,938,320 states; ledger
  `10bb3dce...` ingested it. Direct zero-hole descendants are remotely positive
  pending quiet local matches. All 518 one-step shape transports from this
  leader are stored; the two new zero-hole cases are queued as Abacus job
  `20260911T202641-7863232114c7`. Eliminating the final bad edge is primary.
  Those two one-step shape cases are remotely positive. Twelve larger
  grow-then-close zero-hole candidates (dimensions 118--128) are stored in run
  `b5978237...` and owned by jobs `20260911T202944-adebc62a5d43` and
  `20260911T202944-64c51cfc1209`. Seed-only genetic v17 also stored all 1,024
  multi-bit proposal events (786 unique, 784 nonzero-hole prunes). Its two
  zero-hole presentations duplicate closure poset `cedb1777...`; job
  `20260911T204207-212af3f7b4a7` was cancelled while queued and the equivalence
  was recorded instead of repeating the computation.
- Private genetic v16 is exact-negative-parent-only and adds cross-run exact
  reuse plus audited flag-swap/flag-augmentation operators; all 124 tests pass.

## Current negative-face reservoir bridge — 2026-09-11 (active)

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
- Two new flag-heavy ledgers added 18,278 two-hole-bounded and 19,238
  one-hole-bounded proposal slots. They reached zero-hole positive descendants
  of degrees 79 and 77 respectively, but no new admissible negative. These
  reduced-mask runs retain unit content; weight mutation is handled in the
  separate paired fixed-content lane.
- A fresh paired generation around `a2be13af...` classified 10,254 proposals
  into 4,668 nonempty exact short signatures, 3,085 dimension prunes, and
  2,328 empty outcomes. Proper degree-19 leaders `286549f5...` and
  `02909c47...` are fully exact-positive; completed Abacus jobs
  `20260911T061817-5cdaaf727fef` and `20260911T062008-54ea5ea7f1dd`
  independently matched their local dilation-13 controls and tails.
- The exact structural-tail ranker now bypasses scale-one lattice-span scans
  when the maintained strict counter is selected, and it accepts only cached
  counts backed by the same engine SHA-256. A fresh degree-19 control
  (`286549f5...`) matched all strict counts 1--6 and its prior exact h-star
  prefix/tail in 9.1 seconds. The Abacus runner now supports bounded
  multi-candidate batches for the remaining 569-class reservoir.
- Closest proven negative remains degree-84 `612334d7...`, skew hook
  `(11,1^9)/(1)`, weight `1^19`, with exactly two internal nonflag holes
  `(4,6)` and `(6,10)`; `(5,14)` is the third equality and is flag-absorbed.
- A complete zero-hole reservoir bridge directly from `612334d7...` now
  covers 30,913 first-transfer weight/flag candidates: 5,641 nonempty and
  25,272 exact-empty. Exact representatives at degrees
  28,26,22,16,8,6,4,0 are all positive; ledger `3e0e3259...` is complete.
- Two further paired shape/weight/flag generations around the small degree-8
  and degree-6 descendants classified 367 and 522 proposals. All 40 distinct
  short-signature representatives were fully reconstructed and matched two
  unused dilations; all are positive.
- Abacus job `20260911T072345-ab8e97109e4e` matched a known degree-8 flagged
  control and completed four alternate-transfer candidates of degrees
  16,22,26,28. All were positive and each matched an unused local `L(4)`
  before ingestion. Output SHA-256 is `13815cdc...`; ledger `6d14a991...` is
  complete.
- The entire 569-class second-generation proper reservoir now has exact
  structural strict counts through dilation six. Abacus computed 548 classes
  in eight jobs and all 548 dilation-six values independently matched locally
  before ingestion; no class was skipped. Global partial leader `03d0ad46...`
  is straight-shape degree 19, but its completed ordinary-9/strict-10
  polynomial is positive (normalized internal margin ~`0.0004194`).
- A new paired zero-hole generation from exact `03d0ad46...` classified all
  10,845 proposals: 5,029 new nonempty, 4,290 empty, 1,469 dimension-pruned,
  and 57 already stored, with no limited remainder. Its 434 new signature
  classes are queued in six finite Abacus structural-tail jobs beginning
  `20260911T083434-9b8c6b4b1915`; inspect existing IDs rather than resubmit.
- Degree-19 jobs `20260911T061817-5cdaaf727fef` and
  `20260911T062008-54ea5ea7f1dd` completed and both candidates are positive;
  their exact output hashes are `f420ab5f...` and `37d48e22...`.

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
