# KTT Search Handoff

## Current state — 2026-09-14 (active)

Codex is the sole KTT worker and local MariaDB writer. No counterexample is
admitted. `07975fb3...` and `fb09eaa1...` are both rejected interpolation
false positives; no negative zero-hole record remains pending. Abacus v195
`20260913T183109-69b50bf3bf77` is live on all six allocated CPUs, and audited,
disjoint v196--v200 successors are queued. Their structural
runner/engine hashes are `5564bf69...`/`f179ae7e...`; do not duplicate them.
The current read-only genetic-ledger audit found 15,353 exact negative records
but none with `nonflagged_inequalities = 0`, so no already-exact flagged
individual-face negative case exists for a duplicate Euler check.
Host discovery `v201` ran one nice-10 CPU for 300 s (`random-seed 20260914`,
three fresh zero-hole shape/weight seeds, 10M states and 30 s/candidate). It
finished cleanly with zero exact polynomials; its untracked companion report
`runs/host-v201-zero-hole-shape-weight-20260914.jsonl` is empty. No candidate
result or admission follows.

Direct user authorization now reopens Euler-local CPU discovery in this Docker
container; no supervisor queue is involved. Immediately before launch, Abacus
was audited as v195 running with v196--v200 queued, while Laplace's queue check
timed out (machine unreachable). `v202` is a read-only zero-hole shape/weight
discovery sweep capped at eight lanes on physical CPUs 0--7 (50% of Euler):
fresh seeds beginning 2026091500 replenish lanes that finish base-empty or
bounded-limited, and use
`kostka_lift_zero_hole_shape_weight_search.py` SHA-256 `df9c964d...` with
`--seeds R0 --random-seeds 1 --generations 3 --beam-width 10
--max-neighbors 120 --max-exact 24 --max-seconds 900 --candidate-seconds 60
--max-states 50000000 --require-base-point --no-db`. Generated JSONL/logs are
untracked under the companion `runs/`; no MariaDB write occurs until a result
survives the ledger and remote-reservation audit.

`v202` ended naturally before the resumed-work audit: its 24 generated reports
contain 65 distinct exact zero-hole polynomials and zero negative coefficients.
It made no MariaDB write and yielded no contender. Its nonoverlapping direct
successor `v203` is now using the same read-only command, bounds, niceness, and
eight-physical-core cap with fresh seeds beginning 2026091524; do not rerun
`v202`.

`v203` also ended naturally (13 reports, 24 distinct exact zero-hole
polynomials, zero negatives; no DB write). The first GPU-first near-one-hole
closure is likewise closed: exact-positive one-hole parent `ff64c7df...`
(`a=7,b=3`, dimension 15, sole hole `(2,2)`) yielded fresh zero-hole child
`2ca2bed6...` by `drop_hole:2:2` (`0xa0f3c11880`, dimension 16). Read-only
candidate/attempt/proposal/poset checks found no prior row; immediately before
submission Abacus v195 was live with v196--v200 queued and Laplace remained
unreachable. The individual face used its actual per-label masks
`0,0,0,0,4,8,0,32,0,0`, never a union. GPU `bits:97` is conservative for
`(4*16+1)^16`; all d1--d16 receipts are exact CRT successes, and reciprocity
interpolation has every ordinary Ehrhart coefficient positive. The full
untracked evidence is `runs/gpu-v204-2ca-interpolation-20260914.json`.
No counterexample or MariaDB write follows. A shell-control error created two
duplicate d3/d4 receipts; both independently returned zero and are retained in
the report; submit only missing dilations one at a time going forward.

The second disjoint zero-hole closure from that same honest one-hole parent is
also closed: `81113909...` is `hole_to_flag:2:2:4`, with dimension 14 and
individual masks `0,0,0,4,4,8,0,32,0,0`. Its d1--d14 Euler wide-CRT receipts
are all `ok:true`/cleaned and reconstruct exactly; d14 receipt
`20260914T140858-e907c8431490` returned `26785044` in 8.014 s. Exact
interpolation from d0--d14 has no negative coefficient (full untracked
evidence: `runs/gpu-v205-811-interpolation-20260914.json`), so it is ruled out
without a CPU replay or DB write.

A third fresh close-the-hole mutation is eliminated too: audited zero-hole
face `635a7522...` (`hole_to_flag:2:2:8`, dimension 13, masks
`0,0,0,0,12,8,0,32,0,0`) had no ledger candidate/attempt/proposal or exact
poset row. Its 13 GPU receipts all certify clean exact CRT counts; d13 receipt
`20260914T141456-f02e08718922` is `4945875` in 8.031 s. Reciprocity
interpolation has no negative coefficient; untracked complete evidence is
`runs/gpu-v206-635-interpolation-20260914.json`. No CPU replay or MariaDB
write follows.

The final single-flag closure from one-hole parent `ff64c7df...` is also
eliminated: `eb0bbfdf...` (`hole_to_flag:2:2:19`, dimension 11) is a fresh,
audited zero-hole face with masks `0,0,0,0,4,8,32,32,0,0`. Its d1--d11 bridge
receipts are all clean exact CRT evidence; d11 `20260914T141842-d8da7cb9f97b`
is `33120` in 7.979 s. Exact interpolation has no negative coefficient; full
untracked evidence is `runs/gpu-v207-eb0-interpolation-20260914.json`. Thus
all three one-flag absorption variants of this parent are ruled out; move to a
different audited one-hole near-miss family. No DB write occurred.

The first close-the-hole child from the next family is excluded even faster:
fresh zero-hole `6f9eb252...` from one-hole positive `6dd3c47a...`
(`hole_to_flag:3:4:3`, dimension 10, masks `0,0,0,2,2,0,0`) had no ledger
row, but its three clean bridge receipts (`20260914T142127-f1231b142149` etc.)
give `empty_face` evidence and exact zero. Stop this face without
interpolation; full untracked evidence is `runs/gpu-v208-6f9-empty-20260914.json`.

The broad audited one-hole closure inventory yielded 148 fresh child/poset
pairs. Its first nonempty child, `78cad543...` from parent `0181845c...`
(`hole_to_flag:4:4:13`, dimension 8, masks `0,0,0,0,0,16,0,0`), passed fresh
ledger and queue checks. Eight clean GPU CRT receipts interpolate to an
all-positive Ehrhart polynomial; evidence is untracked at
`runs/gpu-v209-78cad-interpolation-20260914.json`. No CPU replay or DB write
follows.

Active GPU closure: fresh audited `81557cce...` (parent `11c05b4a...`, margin
`118333/151009`, dimension 8, masks `0,0,0,0,0,20,24,16,0`) began exact
interpolation. Clean d1 receipt `20260914T143430-840c024273e3` is CRT value 0;
resume only d2--d8 with snapshot `e3bc68a...` after queue audit.

Closure complete: d2--d8 receipts through `20260914T143628-f4ad4842d961` all
validated as clean CRT evidence. Exact interpolation of `81557cce...` has
coefficients `1,3293/840,1835/288,539/96,377/128,227/240,35/192,13/672,1/1152`,
all positive. No DB write follows.

Active next rank: fresh `56d716e9...` (dimension 9, masks `0,0,2,2,4,0,0`)
has clean d1 CRT receipt `20260914T143823-76dea014b2dd` = 0; resume d2--d9.

Completed `56d716e9...`: all d1--d9 receipts are clean CRT evidence and exact
interpolation has coefficients `1,83/20,179/24,16523/2160,475/96,151/72,7/12,37/360,1/96,1/2160`, all positive.

The next distinct margin-ranked parent `d5161cb8...` (margin `1709/2219`)
likewise has an all-positive fresh zero-hole closure `18692508...`
(`hole_to_flag:4:4:34`, dimension 10, masks `0,0,0,6,4,0,0,0,192,128,0`).
All ten audited GPU CRT receipts interpolate positively; evidence is
`runs/gpu-v211-186925-interpolation-20260914.json`. No CPU replay or DB write
follows.

Ranking the fresh closure inventory by exact parent margin selected a closer
one-hole parent `b1bb5d60...` (margin `436669761/598740061`). Its fresh
zero-hole child `00e0dd00...` (`hole_to_flag:3:3:53`, dimension 10, masks
`0,2,2,0,0,0,0,0,0,0,512,0`) passed a fresh ledger/queue audit and all ten
GPU CRT receipts. Exact interpolation is all positive; untracked evidence is
`runs/gpu-v210-00e0dd-interpolation-20260914.json`. No CPU replay or DB write
follows.

The next fresh individual-face closure, `aee7e11c...` from the one-hole parent
`caceb1d5...` (margin `4992/6127`), is also closed. Its dimension-8 masks
`0,0,0,0,8,0,0,64,0` had eight clean CRT GPU receipts through
`20260914T144346-5b58b3c02cc1`; exact interpolation gives
`1,971/280,7019/1440,1739/480,1001/640,97/240,59/960,17/3360,1/5760`, all
positive. Evidence remains untracked at `runs/gpu-v212-aee7-interpolation-20260914.json`.
No CPU replay or MariaDB write follows.

The next audited rank, `9b1427c3...` from `43d8afd...`, is also ruled out:
eleven clean individual-face GPU CRT receipts (masks
`0,0,0,0,0,0,0,0,0,0,512,0`) through `20260914T145019-c57f47ded5d7`
interpolate to an all-positive dimension-11 polynomial. The untracked
provenance report is `runs/gpu-v213-9b1427-interpolation-20260914.json`.
No CPU replay or MariaDB write follows.

The closest fresh audited one-hole closure is now complete: `00446dee...`
from parent `0095d973...` (margin `17257827023/925938772272`) has dimension
23 and individual masks `0,0,0,0,2,2,0,0,0,0`. All 23 clean wide-CRT GPU
receipts through `20260914T150250-39cbad13a33d` use the certified GT bound
`24^23 < 2^107`; exact reciprocity interpolation has no negative coefficient.
Full untracked evidence is `runs/gpu-v214-00446-interpolation-20260914.json`.
No CPU replay or MariaDB write follows.

The second closest fresh audited one-hole closure is complete: `d39e5d15...`
from `a26cd5dd...` (margin `17147346969/519184344073`) has dimension 21 and
individual masks `0,0,0,0,0,4,4,0,0,0`. Its 21 clean wide-CRT GPU receipts
through `20260914T151236-dcf9103b38d7` use `22^21 < 2^94`; exact reciprocity
interpolation has no negative coefficient. Full untracked evidence is
`runs/gpu-v215-d39e5-interpolation-20260914.json`. No CPU replay or MariaDB
write follows.

The final fresh one-added-equality closure `95d69bc4...` is complete: all 13
clean two-prime GPU CRT receipts through `20260914T151738-ae0ba5db105a` use
`14^13 < 2^50`, and exact interpolation is all positive. Thus the audited
one-hole/single-add zero-hole closure inventory is exhausted with no KTT
counterexample and no DB write; broaden the honest one-hole mutation class.

The broadened, one-hole-preserving flag-augmentation inventory is derived from
reconstructed lifts rather than the legacy ledger `hole_count`: 198 exact
positive parents produced 344 additions, 111 quotient posets, and 50 fresh
ledger-disjoint posets after candidate/attempt/proposal/Ehrhart checks. The
first margin-ranked face is `12782cc5...` from `0095d973...` (dimension 20,
`add_flag_absorbed:31`): its one true hole is `(2,5)`, while `(5,9)` is an
endpoint flag; the individual masks are `0,0,0,0,2,0,0,0,16,0` and are never
unioned. Its certified count bound is `21^20 < 2^88`; the GPU is ready,
Abacus v195 is live with v196--v200 queued, and Laplace queue inspection timed
out. The face is otherwise fresh and will be sampled serially through d20
from this committed source snapshot; no DB write is authorized.

`12782cc5...` is instead geometrically empty: bridge receipt
`20260914T152713-85eb32a02e9a` is `ok:true`, cleaned, and carries validated
`derive_masked_interior` `empty_face` evidence, so no d2--d20 interpolation
or CPU replay is meaningful. The next fresh one-hole-preserving augmentation
is `8ed718f4...` from `a26cd5dd...` (d18, sole hole `(3,6)`, masks
`0,0,0,0,0,4,0,0,0,32`, bound `19^18 < 2^77`); it is distinct from all
Abacus work and remains unsubmitted pending this checkpoint. No DB write.

`8ed718f4...` is also geometrically empty: receipt
`20260914T152907-635992e9df11` is clean `empty_face` evidence. Before another
GPU interpolation, use the exact non-count `derive_masked_interior` prefilter
on the remaining 48 fresh faces and submit only a nonempty individual face;
this changes no ledger state and preserves the separate-mask rule.

That prefilter is complete: all 50 fresh one-hole-preserving endpoint-flag
additions return `empty:true` from `derive_masked_interior` (binary
`fe3985e...`, source `071aa415...`). This entire class is ruled out without
more GPU counts; next enumerate genuine one-hole equality swaps that move the
nonflagged equality while retaining one hole, then repeat DB/queue and
structural audits. No DB write.

The exact one-hole swap audit has 1,764 reduced swaps from 198 parents; 262
fresh quotient posets remain after ledger exclusion, and `derive_masked_interior`
leaves 40 nonempty (222 empty). The first nonempty margin-ranked face is
`0d5b93bc...` from `0095d973...`, mutation `swap_hole:2:5:6:7`: dimension 24,
sole mask vector `0,0,0,0,4,0,0,0,0,0`, unrestricted endpoint flags, and
certified `25^24 < 2^112` bound. It is pending a fresh queue/ledger audit and
serial GPU interpolation; no DB write.

`0d5b93bc...` is now closed: 24 clean four-prime CRT bridge receipts through
`20260914T154003-89b52b112993` exact-interpolate to a dimension-24 ordinary
Ehrhart polynomial with every coefficient positive. Samples are zero through
d10 and become nonzero at d11, but all receipt reconstruction and exact
Fraction back-substitution checks pass. No CPU replay or MariaDB write follows;
continue with the next fresh nonempty equality swap only after a fresh audit.

Next nonempty swap representative: `0dfd9cbf...` from the same close parent
`0095d973...` but a distinct quotient poset, with `swap_hole:2:5:6:5`, d24,
sole mask `0,0,0,8,0,0,0,0,0,0`, unrestricted flags, and `25^24 < 2^112`.
It is not in the ledger or active/queued zero-hole Abacus batches; re-audit
immediately before its first GPU count. No DB write.

`0dfd9cbf...` is closed too: all 24 four-prime CRT receipts through
`20260914T155001-b3c5b9a84ba5` validate and interpolate to an all-positive
dimension-24 polynomial (first nonzero sample d11 = 20). No CPU replay or
MariaDB write follows; continue within the 38 remaining nonempty swaps only
after fresh ledger and queue audit.

Active third nonempty swap: fresh `17e53fb7...` (`swap_hole:2:5:6:24`, d24,
sole mask `0,0,0,0,0,0,0,16,0,0`, bound `25^24 < 2^112`) passed the latest
ledger/Abacus/Laplace audit. Clean four-prime CRT receipts d1--d7 through
`20260914T155355-4b80aae0c233` are zero; resume d8--d24 only. No DB write.

`17e53fb7...` is closed: all 24 four-prime CRT receipts through
`20260914T155950-10a9e098ecc3` validate and exact-interpolate to an
all-positive dimension-24 polynomial (first nonzero sample d11 = 35). No CPU
replay or DB write follows; 37 prefiltered nonempty swaps remain.

Active fourth nonempty swap: fresh `36e05398...` (`swap_hole:2:5:6:19`, d24,
sole mask `0,0,0,0,0,0,32,0,0,0`, bound `25^24 < 2^112`) passed current
ledger/Abacus/Laplace audit. Clean CRT receipts d1--d7 through
`20260914T160332-2d29b8013ebd` are zero; resume d8--d24 only. No DB write.

`36e05398...` is closed: all 24 clean CRT receipts through
`20260914T160917-dcc1589bacf6` interpolate all-positive. Unlike the first
three swaps it starts at d10 = 3, but exact rational interpolation has no
negative coefficient. No CPU replay or DB write; 36 prefiltered nonempty
swaps remain.

Active diversified swap: fresh `25d0d36c...` from distinct parent `43d8afd5...`
(`swap_hole:11:11:5:54`, d19, mask
`0,0,0,0,0,0,0,0,0,0,1024,0`, bound `20^19 < 2^83`) passed the current
ledger/Abacus/Laplace audit. Clean three-prime CRT d1--d7 receipts through
`20260914T161425-fef3124b65bc` are zero; resume d8--d19 only. No DB write.

Diversified `25d0d36c...` is closed: all 19 three-prime CRT receipts through
`20260914T161706-5a0fcf76bac9` validate and interpolate all-positive. Its
first nonzero sample is d12 = 1; exact rational interpolation has no negative
coefficient. No CPU replay or DB write follows.

Active second diversified swap: fresh `3a4289e2...` from parent `4c0a044c...`
(`swap_hole:9:9:2:35`, d17, masks `0,0,0,0,0,0,0,0,256,0,0`, bound
`18^17 < 2^71`) passed current queue/ledger audit. Valid CRT d1--d7 receipts
through `20260914T162017-be61d7feacee` are zero; resume d8--d17. Three initial
d2--d4 parser rejections had an extra weight entry, did no computation, and
are not evidence. No DB write.

Canonical Ehrcalc `main` is now pushed at `ad49c0b`. The hardened Euler bridge
is ready (including container checks); its two end-to-end controls were read
from their JSON receipts before candidate use. `20260914T132841-d1a46a071d51`
is `ok:true`, pinned snapshot `ad49c0b...`, cleaned all containers, and
certified the six-prime wide-CRT value 1; `20260914T132904-d39fe82a325f` is
`ok:true`, snapshot-pinned/clean, and provides validated empty-face evidence 0.
They are controls only, not KTT discoveries or DB writes. For every future
bridge result inspect the JSON receipt; `ok:false` or `resource-limit` is no
count.

Euler wide-CRT exact strict count closes `07975fb3...`: receipt
`20260914T105826-fd315db8e78c` certified `L^circ(7)=0` in 16.230 s using
`43^24 = 1596772093453535767288998989560362112801` as the coordinate-box
bound and five 31-bit moduli (product exceeds twice the bound). Its stored
interpolation predicted `-2168066225856` at d7, hence is contradicted by an
exact nonnegative count. Engine source SHA-256 is `0a37e353...`; the valid
MariaDB attempt is `7c5715ec...`. Receipt `20260914T105806-7d6745d7aff1` had a
mistyped too-small bound and is non-certifying; it is retained only as audit
provenance. No overlapping union was summed.

Receipt `20260914T110145-6d2921fc01e5` applied the same certified d7
coordinate-box bound and five-prime CRT to `fb09eaa1...`, returning exact
`L^circ(7)=0` in 16.187 s, versus its interpolation prediction
`-1600194183201`. Its valid MariaDB attempt is `be0477e...`; detail includes
the receipt, residues, binaries, and source hash. Both former negative records
are eliminated; await the disjoint queued Abacus tail work for new evidence.

Ehrcalc integrated GPU checkpoint `0e1aaba` (source handoff `58d0024`) as
`5d2ecca`/`5dc8f2b` and canonical merge `dee07c2`: packed modular CPU counting now has suffix pruning, wide
counters, strict and arbitrary individual-face masks, and parser hardening.
KTT's `ComplementRowLift.forbidden_row_masks()` converts one-based candidate
pairs to the backend's zero-based per-label masks. The exact CPU control and
two packed residues match (all 1); 54 engine tests pass. Release SHA-256:
packed `19d80f5b...`, mask derivation `fe3985e0...`, reconstruction
`bcb82e2b...`, direct exact `77f24efd...`. No executable `hipMallocAsync`
path exists.

Full `cargo test --workspace` passes, as do strict-engine Clippy (`-D warnings`)
and GPU-wrapper Bash syntax checks. Following a fresh audit of the unusual
local history (901 commits ahead, zero remote-only), normal non-force push
advanced GitHub `main` from `1d53fc3` to final documented tip `73cf368` (merge
`dee07c2`); retain the feature branch.

Euler's required GPU gate passed on the host at
2026-09-14T10:07+02:00 from canonical Ehrcalc `main` `73cf368`.
`gpu-prototype/run-host-tests.sh` passed, and `gpu-prototype/run-smoke.sh`
matched the packed-modular Rust CPU reference for skew, flagged, zero-strip,
masked-face, and strict-forced-diagonal cases at both configured 31-bit
moduli (ten matches total). HIP source SHA-256 was `e0cc1690...`; the compiled
GPU binary was `73cba204...`. Strict GPU face cases are unblocked through
`gpu-prototype/run-exact-strict.sh`, subject to a certified upper bound,
sufficient CRT coverage, and an exact CPU cross-check. Count faces
individually, never by an overlapping-union sum. Abacus stays CPU-only.

Post-gate GPU control is the already-ledgered (hence non-discovery) one-hole
face `86b98937...`: its sole `(6,10)` pair maps to label-10 mask `32` in the
21-label mask vector. Exact stored reciprocity gives `L^\circ(22)=58212`, and
the new direct CPU `strict_masked_count` independently returned 58212 at d22
under 150M states in under one second. Euler's host then ran
`run-exact-strict.sh 58212 22 12,1,1,1,1,1,1,1,1,1,1 1
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1 - - 150000000
0,0,0,0,0,0,0,0,0,32,0,0,0,0,0,0,0,0,0,0,0` successfully at
2026-09-14T10:16+02:00. The analyzer reported dimension 109; one GPU batch
returned residue `58212` modulo `2147483647`, and CRT reconstructed exactly
`58212` in 8.3 seconds. External log `exact-4acd1b008d821063-batch0.log` has
SHA-256 `e1988efd...`. This closes the complete KTT mask-to-strict-GPU control;
select a new audited disjoint individual face next.

The next individual face completed successfully on Euler's host: `a=10,b=11`, mask
`0x20000000000`, candidate `551f8ccf...`, with sole forbidden pair `(7,10)`
and label-10 mask `64`. Its quotient-poset hash `91c9d071...` had no candidate,
attempt, proposal, or same-poset MariaDB row in a fresh read-only audit; local
run/certificate searches likewise had no same face (the only same-mask text is
an `a=11,b=11` case). Thus it is disjoint from prior results and the recorded
v194/v195/v196 reservations. Direct exact CPU `strict_masked_count`
(`77f24efd...`) found d22 zero and `L^\circ(23)=3738884688` at d23 under the
150M-state cap. That count is the certified bound; mask derivation is nonempty
of affine dimension 109. On host canonical `main` `8d649a1`, the supervisor ran
`gpu-prototype/run-exact-strict.sh 3738884688 23 12,1,1,1,1,1,1,1,1,1,1 1 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1 - - 150000000 0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0`.
At 2026-09-14T10:31+02:00 its two GPU residues were `1591401041` and
`1591401059`; CRT reconstructed exactly `3738884688` in 8.7 seconds, matching
the CPU certificate. External log `exact-1372b83f38e392af-batch0.log` has
SHA-256 `85bbd37f...`. HIP source is `e0cc1690...`; no union sum or DB write
occurred.

Autonomous bridge follow-up ran two more ledger-disjoint individual faces from
host `main`/`origin/main` `27ece104...` (HIP source `e0cc1690...`).
`2c03dee1...` / mask `0x40000000000` / `(8,10)` (quotient `8350fd6e...`) had
no candidate, attempt, proposal, same-poset, or local-evidence hit; exact CPU
gave d22 `11880` and d23 `1777278888` under 150M states. Its bridge receipt
`20260914T084547-ece5bf4c61f6` returned `1777278888` in 8.641 s (residue
`1777278888` modulo the first prime). `83ec8882...` / mask
`0x4000000000000` / `(7,11)` (quotient `9c085eb1...`) passed the same audit;
CPU d22/d23 counts were `52920`/`4342507596`, and receipt
`20260914T084642-c36fc4991f98` reconstructed `4342507596` in 8.353 s from
residues `47540302`/`47540338`. Both shared receipts are under
`/cargo-target/euler-gpu-bridge/results/`. They are exact count cross-checks,
not polynomial counterexamples; no union sum or DB write occurred.

Fresh remote audit: v194 `20260913T173400-b08ab8dc21ff` terminated by its
supervisor `systemd` timeout (status 15) and was fetched; v195
`20260913T183109-69b50bf3bf77` is now running and v196
`20260914T073435-1df562e27e39` remains queued. v194's six child logs validate
the pinned runner/engine/resource headers and input hashes. Their captured
ordered prefix has 458 exact counts and 23 per-case time limits; five unseen
trailing cases were not inferred or written. Sole-writer ingestion ledger
`5cb14388...` recorded 452 fresh exact attempts, six exact reuses, and all 23
time limits, with child-output SHA-256 values `d9e02b59...`, `d32143e...`,
`5a669be...`, `dd87684...`, `d6500b...`, and `2e1d0cb0...`.
The v194 inventory reservation for `07975fb3...`/`fb09eaa1...` is terminal;
retain the existing v195 disjointness audit before any new host candidate. The
strongest flagged candidate is still unadmitted `07975fb3...` (dimension 24,
zero nonflagged inequalities, four stored negative coefficients): its old
generic d25 count timed out, and its interpolation gives an impossible negative
strict d25 value. The bridge correctly rejected the rigorous `(6d+1)^24` d25
bound because its interface is u64-only; no uncertified GPU reconstruction was
accepted. Local bounded strict d25/d26 attempts likewise gave no count. A
A CPU-0/nice-10 host ordinary flagged d25 retry for `07975fb3...` is live as
PID `1576498` under `timeout 14400s` / 100M states, output
`reduced-kogan-mutation-search/runs/host-0797-flagged-ordinary-d25-4h-20260914.json`,
binary SHA-256 `377b27ea...`. With v194 terminal and recorded v195 case
disjointness rechecked, the distinct zero-hole `fb09eaa1...` d25 ordinary
replay also started on CPU 1 as PID `1628228`/`1628229` under the same bounds,
output `runs/host-fb09-flagged-ordinary-d25-4h-20260914.json`. Its certified
source record SHA-256 is `660c5afc...`; it must match
`27198291378648307039700595981` at d25, then independently match
`63657025156588451239562777670` at d26. On direct user instruction both local
ordinary replays were terminated cleanly at 2026-09-14T10:22Z without output:
`07975fb3...` had 1:23:17 CPU time and `fb09eaa1...` 4:13. No result, failure
disposition, or DB write follows from either; do not restart independent CPU
replays without new authorization. Abacus v195 is the sole active KTT compute.
Fresh read-only ledger inventory found no exact negative zero-hole row: all
15,353 exact negative genetic rows split into 293 one-hole and 15,060 two-hole
rows. Thus there is no additional already-exact zero-hole candidate to launch
outside v194's recorded reservations.
The seven archived `*flag*negative*` exact certificates were also rechecked:
each explicitly retains one or two nonflagged top-complement equalities, so
their negative polynomials are not flagged-skew witnesses.
After both queue checks (Laplace remains unreachable), v196
`20260914T073435-1df562e27e39` was queued after v195 as a 60-second,
hash-checked, non-candidate packed strict control (runner `03435442...`).
Fetch/verify it before any new packed candidate batch. No DB write beyond the
v194 verified-prefix ingestion occurred.
After an immediate v195/v196 queue audit, v197
`20260914T102537-791bb739fe51` was queued as the only disjoint continuation of
v194: its five never-observed child-18 suffix cases are `b1a8c4f1...` d10 and
`c63815d9...` d7--d10. It has input SHA-256 `4110c144...`, runner
`5564bf69...`, engine `f179ae7e...`, 100M states, 7,200 s/case, and a 28,800 s
supervisor envelope. It follows v195 (running) and v196 (queued); do not mirror
or resubmit it. Euler GPU bridge status is ready, but no new nonduplicate KTT
face currently has both the individual-face form and u64 certified bound that
its exact-strict interface requires; do not spend it on duplicate controls.

After a second immediate queue audit, v198
`20260914T104149-10548920510b` was queued behind v195/v196/v197 to keep all
six Abacus CPUs occupied once dispatched. It is a zero-hole-only,
host-disjoint structural-tail continuation: 108 signature representatives,
432 previously uncounted same-engine strict cases (dilations 7--10), and six
known d6 controls, split into six independent 73-case children. The sole KTT
worker/sole MariaDB writer audited zero candidate-key, compressed-class,
host-pass, or same-engine exact-case overlap against v194/v195/v197 and the
ledger; five already complete ordinary interpolations and 15 historic host
identities were excluded. Each child uses 1,200 seconds/case, 100M states,
and the 28,800-second supervisor envelope; runner/engine SHA-256 are
`5564bf69...`/`f179ae7e...`. Input SHA-256 values (children 01--06) are
`80415b87...`, `febb1ecc...`, `d35cf604...`, `e1dff918...`, `510e8b7b...`,
and `3a5f384d...`; generated inputs remain untracked. Fetch, hash-verify,
and use the existing exact/timeout-prefix ingestion protocol only after a
terminal result. No overlapping union is summed, no Euler GPU work was
submitted, and no DB write occurred for v198.

v199 `20260914T104629-a7df1387d2f2` is the next, distinct six-way zero-hole
tail batch behind v198: 108 further representatives, 432 new structural
strict d7--d10 cases, and six d6 controls, again at 1,200 seconds/case, 100M
states, and 28,800 seconds total. Immediate pre-submit audit found zero
candidate-key, compressed-class, host-pass, or same-engine exact-case overlap
with v194/v195/v197/v198 or the ledger. It uses the same runner/engine
`5564bf69...`/`f179ae7e...`; child input SHA-256 values are
`83a1bac1...`, `82cc65ff...`, `f7a41a98...`, `147c63e7...`, `9bba88b7...`,
and `22fd4168...`. Sole writer remains Codex; do not ingest before a terminal
fetch and the existing hash/control checks. Generated inputs remain untracked.

v200 `20260914T104857-65df7e591ef2` completes the currently audited
zero-hole tail-10 remainder behind v199: 18 final representatives, 72 new
strict d7--d10 cases, and six d6 controls, deliberately repartitioned into
six 13-case children so all six Abacus CPUs can work. Its pre-submit audit
again found zero candidate-key, compressed-class, host-pass, or same-engine
exact-case overlap. It shares the 1,200-second/100M/28,800-second profile and
runner/engine `5564bf69...`/`f179ae7e...`; input SHA-256 values are
`cd212554...`, `2c1ce160...`, `cbb0e76a...`, `0fc6e9d6...`, `49d57466...`,
and `d92a6a81...`. Codex remains sole writer; generated inputs are untracked;
fetch and verify only at terminal state.
Its argv is `python3 abacus_packed_modular_smoke.py --exact
strict_masked_count --exact-sha256 77f24efda9ded14a2a1dd592805f89285db97486955c36880db645058707b7ec
--derive derive_masked_interior --derive-sha256
fe3985e02407e73f56ffda1d47ed0dec00f90f304b93d7e951598ec66c45e42d
--packed export_modular_layer --packed-sha256
19d80f5bf30ae62fb03c82d31e7746704bc39cbd1d7045ee8295d4c9595259af`.

## Superseded pre-resume checkpoint — 2026-09-13

### Current execution (refreshed 2026-09-13)

Codex is the sole KTT worker and local MariaDB writer. `07975fb3...` (dimension 24; `pair:outer:9->11+weight:8->15`) is a structurally audited, flagged zero-hole **negative pending validation**, not an admitted or claimed counterexample. Its first independent generic `ehrcalc kostka` d25 attempt exhausted 3,600 seconds without a count; the distinct, live CPU-7 retry (PID 1313282) has 100M states, 7,200 s/sample, and a 14,400 s envelope for d25/d26. Both out-of-range values must match before admission.

Second pending candidate: `fb09eaa1...` (dimension 24; `pair:outer:9->11+weight:11->15`) completed its packed interpolation with negative Ehrhart degrees 2, 3, 5, and 6. Its reconstruction audit independently passes canonical key, partition/interval validity, derived flags, and zero nonflagged inequalities. Generic d25/d26 validation is live on CPU 8 (PID 1318383; 100M states, 7,200 s/sample, 14,400 s envelope). It likewise remains unadmitted and unclaimed pending both matches.

By direct user instruction, no Euler-local KTT computation may run: the eight active local processes (generic validation of `07975fb3...`/`fb09eaa1...`, and packed `7fd5aa4d...`/`c016205d...`/`0478452a...`/`19bb184b...`/`153f8817...`/`5c986c73...`) were terminated cleanly with their checkpoint/output evidence preserved. Do not restart them or run `ehrcalc` locally; Abacus is the sole KTT compute lane. Abacus v193 `20260913T152558-1f2918e2998b` expired its 7,500 s supervisor envelope with partial child logs; it is fetched but deliberately un-ingested. For the user-requested suspension wind-down, Abacus's queue was paused at 2026-09-13T20:23Z: v194 `20260913T173400-b08ab8dc21ff` remains running unchanged, while its audited successor v195 `20260913T183109-69b50bf3bf77` is held and must not dispatch until explicitly resumed. v195 contains 106 non-control structural tail-10 candidates, disjoint from ledger, host, v193, and v194 after excluding prior host attempts `01312ad9...`/`19bb184b...`; input SHA-256 `09f80220...`/`965e8a48...`/`bf3dcfe1...`/`47328e8b...`/`ab2cbfe3...`/`379d37e3...`, runner `5564bf69...`, engine `f179ae7e...`, six CPUs/20 GiB/256 pids, 1,200 s/case and 28,800 s envelope. Laplace is unplugged, unreachable, and has no KTT job. Generated evidence stays untracked; do not push or publish. Historical chronology follows.

- Codex remains sole private KTT worker and sole local MariaDB writer; no exact
  flagged zero-hole-negative exists. Abacus v175 and Laplace v23 were
  fetched, hash/control/profile verified, and primary-ingested
  (`b60111d6...`/`10b3b178...`): all nine d104--d102 targets are positive.
- Abacus v176 `20260913T081844-0abadaa146e5` is fetched, provenance-verified,
  and primary-ingested (`31ccaedc...`): all six pairwise-distinct d102/d101
  zero-hole closures are exact-positive (1,756,040--3,261,972 states;
  79.516--179.629 s); its only negative was the exact-known-match control.
  Inputs were `aeee2a1c...`/`adfc60e0...`/`ddb9b5ce...`/`be7626ca...`/
  `1f238ee1...`/`3863244d...`, run `130be1d9...`.
- Laplace v24 `20260913T081922-f2886ff50550` is fetched, provenance-verified,
  and primary-ingested (`95120ac2...`): its three d101/d100/d100 closures are
  exact-positive (1,628,770--3,202,500 states; 145.914--234.188 s); its only
  negative was the exact-known-match control. Inputs were `44a0884e...`/
  `77d6104a...`/`1ce3f9b9...`, run `13130c9b...`. Abacus v177/Laplace v25
  (`89897944...`/`a1c5fb2b...`) and Abacus v178/Laplace v26
  (`008407bd...`/`6e674d46...`) are fetched, provenance-verified, and
  primary-ingested: all eighteen d100--d95 targets are exact-positive. Host
  v176 completed one new exact evaluation and four state-limited cases, with
  no zero-hole contender. Abacus v179 `20260913T090630-334736a57580` is
  fetched, provenance-verified, and primary-ingested (`84195b08...`): all six
  d94/d94/d93/d92/d92/d90 closures are exact-positive. Laplace v27
  `20260913T090810-b930e2fa688a` is fetched, provenance-verified, and
  primary-ingested (`147be2f7...`): both remaining d90/d88 closures are
  exact-positive. The closure inventory is genuinely exhausted: the selector's
  512-parent bound already covers all 292 exact one-hole-negative parents.
  Host v177 completed three new exact evaluations and three state-limited
  cases, with no zero-hole contender, but yielded two fresh d114/d110 closures.
  Abacus v180 `20260913T092335-e1856906106a` is fetched, provenance-verified,
  and primary-ingested (`50e276e9...`): both are exact-positive in 6,282,430 /
  6,032,950 states (286.417 / 270.950 s). Host v178 then finished with two
  new exact evaluations and three state-limited cases, no zero-hole contender,
  and no fresh closure. Host v179 then completed three new exact evaluations
  and two state-limited cases, again without a zero-hole contender or fresh
  closure. Host v180 then completed one new exact evaluation and four
  state-limited cases, again with no fresh closure. Host v181 then completed
  two new exact evaluations and three state-limited cases, again with no fresh
  closure. Host v182 then completed two new exact evaluations and four
  state-limited cases, again with no zero-hole contender or fresh closure. Host
  v183 then completed four new exact evaluations and three state-limited cases,
  again with no zero-hole contender or fresh closure. Host v184 then completed
  two new exact evaluations and three state-limited cases, again with no
  zero-hole contender or fresh closure. The high-priority zero-hole lead
  `a7a59559...` was re-audited: its exact database row already gives a
  dimension-21 coefficient-positive polynomial, so the resumable completer
  correctly skipped it rather than duplicating counts. The disjoint paired
  shape/content/flag screen from positive `0a2c22ef...` then completed 8,153
  deduplicated proposals: 4,772 terminal three-count signatures in 528 classes,
  2,544 empty, 837 dimension-pruned, and zero resource-limited; no exact
  negative arose at this screening stage. The verified packed strict tail
  ranker (`9852ecd5...`) then certified 221 classes at depth six (77
  span-deficient classes withheld). It exposed new d21 zero-hole flagged
  candidate `10770dd0...` with negative exact prefix-plus-tail score; it was
  not in the exact ledger, and both remote queues were idle. Its full middle-
  dilation interpolation then completed after an intentional 280-second retry
  of d15: its h* vector and every Ehrhart coefficient are nonnegative, so it
  is now exact-positive. The only remaining negative-known-ends rank leader,
  distinct d21 zero-hole flagged `11a67846...`, likewise completed after the
  intentional 280-second d15 retry and is exact-positive (the same nonnegative
  h* vector as the prior d21 class). Thus every negative-known-ends leader in
  this paired frontier is resolved without a contender. The sole host writer
  now opens the next disjoint paired shape/content/flag screen from
  `11a67846...`, CPU 15/nice-10, three-count signatures, 6M states/60 seconds
  per count and a 290-second envelope; outputs are
  `zero-hole-size50-11a6-paired-v2-20260913.*`. That v2 screen completed 6,875
  terminal attempts (4,080 complete signatures in 445 classes), with 2,129
  empty, 666 dimension-pruned, and 1,844 deferred for a later resume. Its
  depth-six strict-tail pass certified 198 classes (18 span-deficient withheld)
  and yielded three new negative-known-ends d21 leads. The strongest,
  `87c10509...`, is now fully exact-positive after its intentional 280-second
  d15 retry. The next lead `1603714e...` also completed exact-positive after
  its 280-second d15 retry, and final lead `06793194...` did likewise. Thus all
  three v2 negative-known-ends leaders are exact-positive. The sole host writer
  resumed the same v2 paired screen's 1,844 deferred proposals, skipping
  terminal identities: it added 1,065 signatures (469 total classes) with no
  new limit. The resumed strict-tail pass certified eight further classes and
  exposed three new unresolved negative-known-ends leaders (two d21 and one d23). The strongest,
  `1a231fa6...`, has since completed exact-positive; its h* vector and every
  Ehrhart coefficient are nonnegative. The next `1d8be73f...` also completed
  exact-positive in 78.175 seconds, with nonnegative h* and no negative
  coefficient. The sole host writer is now continuing the final d23 v2
  negative-known-ends leader, `25586b32...`: its 60-second attempt timed out at
  d12, its 280-second/290-second aggregate retry saved d12 exactly and reached
  the aggregate limit in d13, and its enlarged 600-second/900-second pass saved
  d13 and d14. Its next 1,800-second/1,810-second pass saved d15, then recorded
  a 756.707-second deadline-bound d16 attempt. (The certified tail begins at
  index 18, so exact d16 and d17 are still required.) The 3,600-second
  per-dilation / 7,200-second envelope continuation completed d16 exactly but
  recorded a clean 3,600.328-second deadline-bound d17 attempt. Its only unsaved
  sample is d17; the sole live strict continuation is PID 1193952 (child
  1193977), CPU 15/nice-10, with a 7,200-second d17 allowance and a
  7,220-second outer bound
  (`zero-hole-size50-11a6-paired-v2-2558-completed-7200s-20260913.jsonl`). This
  has a distinct attempt-timeout key and reuses every prior exact sample; only a
  negative full result activates independent out-of-range replay. Abacus
  structural-tail v181 `20260913T143205-b5cc011aaa61` is fetched and verified
  as an engine-path launch failure before any target count; corrected v181r
  `20260913T143311-342102c4f37a` is fetched, hash/profile/control verified,
  cross-validated locally at d6, and ingested under ledger `29cfbb3d...`.
  Runner `5564bf69...`, structural engine `f179ae7e...`, and inputs
  `f3082e51...`/`55430b08...`/`89dc84ff...`/`5faf848e...`/`6298d47d...` ran
  five disjoint d23/d24 zero-hole flagged classes under the six-CPU/20-GiB/
  256-pid profile. All 35 records are exact (five repeated controls plus d1--d6
  per target); every target strict-interior count is zero through d6 and its
  known-end score is positive, so none is a contender. Abacus v182
  `20260913T143811-1677a83fd1b1` is likewise fetched, profile/control verified,
  locally d6-replayed, and ingested (`79bc161b...`) for six further disjoint
  d24--d30 classes, with inputs `13d7f56d...`/`cdc3494f...`/`e5182bdc...`/
  `97303afc...`/`a5ca4d4e...`/`99931761...`. Five have zero strict-interior
  counts through d6; d24 `2986d182...` has exact positive d4--d6 interior
  counts but still a positive known-end score. No contender arose. Abacus v183
  `20260913T144117-7e985ae054bb` is fetched, profile/control verified,
  locally d6-replayed, and ingested (`e5228382...`) for six disjoint d24
  flag-mutation classes (inputs `8cf4275a...`/`8ab6c116...`/`09e43084...`/
  `ac98a5be...`/`a7680290...`/`f1cf58b3...`). Every class has a nonzero exact
  strict tail by d3 or d4, but all six known-end scores are positive; none is a
  full exact contender. Abacus v184 `20260913T144344-0434ab2447bb` is fetched,
  profile/control verified,
  locally d6-replayed, and ingested (`1b3a9a42...`) for six additional d24
  flag mutations (inputs `45b3580a...`/`c752de38...`/`ffd9c78a...`/
  `c24ebda8...`/`cb5c5685...`/`65e7bca7...`). All have nonzero d4--d6 strict
  tails, yet every known-end score is positive; no full exact contender arose.
  Abacus v185 `20260913T144609-fb4019afc95e` is fetched, profile/control
  verified, locally d6-replayed, and ingested (`d4939b35...`) for six further
  disjoint d24 flag mutations (inputs `4acbefdd...`/`efdd1c28...`/
  `a47f824f...`/`f6ac3f1d...`/`9ff9a6b3...`/`7c9c1283...`). All have nonzero
  exact strict tails through d6, yet every known-end score is positive; no full
  exact contender arose.
  Abacus v186 `20260913T145127-90e72add27d2` is fetched, profile/control
  verified, locally d6-replayed, and ingested (`83eda51c...`) for six further
  disjoint d24 flag mutations. Its inputs `d33afe33...`/`f5bf829d...`/
  `0bd7a533...`/`8507b14a...`/`5432d767...`/`40b1b059...` yielded 42 exact
  cases including controls under the same six-CPU/20-GiB/256-pid profile; every
  known-end score is positive, so no full exact contender arose.
  Abacus v187 `20260913T145537-9b77595a5602` is fetched, profile/control
  verified, locally d6-replayed, and ingested (`9fdddcbe...`) for six further
  disjoint d23 neighbors of the host's leader. Inputs `3a07f127...`/
  `fa8bf973...`/`ffdbc648...`/`141f3713...`/`5a64cab6...`/`195dc86e...` yielded
  42 exact cases. Every known-end score is positive (even where the partial
  tail contribution is negative), so no full exact contender arose.
  Abacus v188 `20260913T145740-2c15e0aba890` is fetched, profile/control
  verified, locally d6-replayed, and ingested (`6e15df4e...`) for six further
  disjoint d23 mutations. Inputs `4a9cd53c...`/`8007ed9f...`/`ac33cab5...`/
  `cf02abfc...`/`ece32560...`/`654de4e8...` yielded 42 exact cases; every
  known-end score is positive, so no full exact contender arose.
  Abacus v189 `20260913T150008-faabdc1401a8` is fetched, profile/control
  verified, locally d6-replayed, and ingested (`c23f70e2...`) for 120 disjoint
  representatives under the verified six-CPU/20-GiB/256-pid profile. Inputs
  `a67cee89...`/`c76cda87...`/`c987a91d...`/`13a03c6b...`/`e82f7a84...`/
  `02dcffda...` yielded 725 exact remote cases (719 new plus six controls) and
  120 matching fresh local d6 checks. The only negative known-end scores are
  the already fully exact-positive d21 leaders `1a231fa6...` and `1d8be73f...`;
  no new full contender arose.
  Abacus v190 `20260913T150655-6f71cccd762f` is fetched, profile/control
  verified, locally d6-replayed, and ingested (`5b1fbb78...`) for 120 further
  disjoint representatives. The prebuilt batch containing active host candidate
  `25586b32...` was excluded and replaced by `63f1be23...`; inputs are
  `224e9c5b...`/`63f1be23...`/`bcb7e714...`/`500e170d...`/`97e7c9dd...`/
  `2cf2806a...`. All 726 remote cases are exact (720 new plus six controls) and
  all 120 local d6 checks match; every known-end score is positive.
  Abacus v191 `20260913T151119-d4c25f810bd3` is fetched, profile/control
  verified, locally d6-replayed, and ingested (`19dda7ae...`) for 120 further
  disjoint representatives. Its inputs `f5112e19...`/`38db78c5...`/
  `9061de35...`/`24946da5...`/`45c68a1c...`/`e5bef587...` yielded 726 exact
  remote cases (720 new plus six controls), with all 120 local d6 checks
  matching; every known-end score is positive.
  Abacus v192 `20260913T151522-cbdfef82afdf` is fetched, profile/control
  verified, locally d6-replayed, and ingested (`0d818a44...`) for the final 64
  prebuilt host-disjoint v2 representatives. Inputs `418cb6f6...`/
  `fbd64f77...`/`2e55372b...`/`88ad2ae7...`/`c6d4b611...`/`cc8c19b3...` yielded
  390 exact remote cases (384 new plus six controls), with all 64 local d6
  checks matching.

  All 469 v2 zero-hole signature representatives are structurally certified
  through d6. The global known-end rank has only three negative rows: already
  full-exact-positive `1a231fa6...` and `1d8be73f...`, plus live d17 leader
  `25586b32...`. A quoting error refreshed only the latter's inexpensive strict
  tail cache while its distinct expensive d17 count remained uninterrupted; no
  new full contender arose.
  The closest remaining positive-score d17 candidate `1a3b03bf...` completed a
  disjoint full packed interpolation on CPU 14 immediately, with nonnegative
  h* and Ehrhart coefficients. The host remains at two of 16 CPUs or fewer;
  only `25586b32...` is the long-running d17 count.
  The next disjoint d21 positive-score candidate `1caa386b...` completed its
  full packed interpolation in 205.974 seconds: its h* vector and every
  Ehrhart coefficient are nonnegative. The final d23 negative-known-ends
  leader `25586b32...` then completed d17 in 3,735.572 seconds: its full
  degree-23 h* vector and every Ehrhart coefficient are nonnegative, resolving
  the global rank's last negative partial-score row without a contender. Its
  exact result is recorded locally. The next eligible disjoint d23
  positive-score case `9a6b7133...` is live on CPU 14 (PID 1249112; 1,200
  seconds/sample, 7,200-second envelope). After a fresh ledger/v193/Abacus/
  Laplace audit, four further distinct positive-score cases began under the
  same limits: `f75e676b...` CPU 10/PID 1249972, `0478452a...` CPU 11/PID
  1249973, `7fe8ec33...` CPU 12/PID 1249974, and `0f819abc...` CPU 13/PID
  1249975. All five exclude every ledger row and v193 reservation; host use
  is five of 16 CPUs, below the 50% cap. A second fresh audit then admitted
  exactly the remaining three host slots: `66799c00...` CPU 7/PID 1250440,
  `1bf28b70...` CPU 8/PID 1250441, and `fb09eaa1...` CPU 9/PID 1250442,
  under the same limits. All eight are distinct from the ledger and v193;
  host use is now exactly 8 of 16 CPUs. `66799c00...` then completed exactly
  in 231.752 seconds with nonnegative h* and no negative Ehrhart coefficient;
  its exact ledger row is present. After a fresh ledger/v193 audit, CPU 7 was
  refilled by distinct `07975fb3...` (PID 1252349) under the same bound. Do not
  duplicate any active host case or add a ninth host job.
  `7fe8ec33...` subsequently completed exactly in 690.315 seconds with
  nonnegative h* and no negative Ehrhart coefficient; its exact ledger row is
  present. After a fresh ledger/v193 audit, CPU 12 was refilled by distinct
  dimension-28 `19bb184b...` (PID 1255058) under the same
  1,200-second-sample / 7,200-second-envelope policy. Host use remains exactly
  8 of 16 CPUs; do not duplicate any active case.
  `f75e676b...` subsequently completed exactly in 806.507 seconds with
  nonnegative h* and no negative Ehrhart coefficient; its exact ledger row is
  present. After a fresh ledger/v193 audit, CPU 10 was refilled by distinct
  dimension-28 `01312ad9...` (PID 1255968) under the same
  1,200-second-sample / 7,200-second-envelope policy. Host use remains exactly
  8 of 16 CPUs; do not duplicate any active case.
  `9a6b7133...` then cleanly timed out only its d15 sample at 1,200.207
  seconds; it produced no interpolated polynomial and remains absent from the
  exact ledger. Its d1--d14 samples are preserved. After checking the
  unchanged v193 reservation, the sole writer resumed that same missing d15
  on CPU 14 with a distinct 3,600-second attempt key / 7,200-second envelope
  (PID 1269031, output `...9a6b-completed-3600s...`). This is a retry of a
  resource-limited sample, not a duplicate exact evaluation; host use remains
  8 of 16 CPUs.
  `0478452a...` then cleanly timed out only d12 at 1,200.129 seconds; it also
  produced no polynomial and remains absent from the exact ledger, with d1--d11
  preserved. A fresh ledger/v193 audit cleared a same-candidate CPU-11 resume
  under its distinct 3,600-second attempt key / 7,200-second envelope (PID
  1270605, output `...0478-completed-3600s...`). This is a resource-limited
  sample retry, not a duplicate exact evaluation; host use remains 8 of 16 CPUs.
  `19bb184b...` then cleanly timed out only d11 at 1,200.133 seconds; it also
  produced no polynomial and remains absent from the exact ledger, with d1--d10
  preserved. A fresh ledger/v193 audit cleared its CPU-12 resume under the
  distinct 3,600-second attempt key / 7,200-second envelope (PID 1275566,
  output `...19bb-completed-3600s...`). This is a resource-limited sample
  retry, not a duplicate exact evaluation; host use remains exactly 8 of 16 CPUs.
  **Negative pending independent validation:** `07975fb3...` (dimension 24,
  `pair:outer:9->11+weight:8->15`) completed its packed interpolation with
  four negative Ehrhart coefficients (degrees 2, 3, 5, 6) and a negative h*
  block. It has `nonflagged_inequalities: 0`, so it is structurally a flagged
  zero-hole candidate, but it has not been inserted or claimed. The sole writer
  started the required independent general-`ehrcalc kostka` out-of-range replay
  on CPU 7 (PID 1280122): 100M states, 3,600 seconds/sample, 7,200-second
  envelope, checking d25 and d26 against the full degree-24 polynomial; output
  is `...0797-generic-verify26-20260913.json`. Only two exact matching checks
  permit database admission or a counterexample claim. Ordinary replacement
  work is paused while this validator uses the spare host capacity.
  The local structural audit independently confirms canonical-key equality,
  partition/interval validity, generated-flag equality, and zero nonflagged
  inequalities. It does not replace the pending d25/d26 count gate.
  `1bf28b70...` then cleanly timed out only d16 at 1,200.109 seconds; it also
  produced no polynomial and remains absent from the exact ledger, with d1--d15
  preserved. A fresh ledger/v193 audit cleared its CPU-8 resume under the
  distinct 3,600-second attempt key / 7,200-second envelope (PID 1277966,
  output `...1bf2-completed-3600s...`). This is a resource-limited sample
  retry, not a duplicate exact evaluation; host use remains exactly 8 of 16 CPUs.
  After an immediate
  Abacus/Laplace queue audit (Laplace remains
  unplugged and unreachable), Abacus v193 `20260913T152558-1f2918e2998b`
  started its distinct six-child structural-tail d7--d10 batch under the
  six-CPU/20-GiB/256-pid profile. Runner/engine hashes are
  `5564bf69...`/`f179ae7e...`; input hashes are `f0259454...`/
  `05adc923...`/`a4d311b9...`/`ae073bcc...`/`9d751065...`/`be43e4a2...`.
  It is disjoint from the host d17 leader, the completed d21 case, all prior
  exact rows, and every prior/active remote reservation. Fetch, verify
  profile/control/hash provenance, locally replay d10, then ingest only on
  terminal completion; do not resubmit or mirror it.
  The legacy Abacus
  hybrid runner asserts a superseded 2-CPU/8-GiB profile, so it is not submitted
  unchanged to the present 6-CPU Abacus. A negative full result still requires
  the prescribed independent out-of-range replay before any ingest or claim.
  Before
  any successor submission, recheck both queues
  and audit against host, remote reservations, and all prior candidate/poset
  identities; never mirror the six-case Abacus batch.
- The current re-audit fetched the cancelled v141 `20260912T175005-d8860a38ea79`,
  v5 `20260912T174740-ea6a08a88d40`, and v6 `20260912T175248-752c0bdc9999`
  bundles: v141 emitted metadata only, v5 only its three exact controls, and
  v6 never dispatched. Their twelve stale candidate reservations all represent
  the single already exact-positive d120 poset `c5af9f97...` (5,980,260 states),
  so they are not resubmitted.
- Verified runner/helper/archive hashes remain `e3bd01d...`/`a4aae249...`/
  `bf1cfa42...`; child limits are 30M states/1200 seconds in a 2100-second
  envelope. An exact zero-hole negative requires byte-matched local replay
  before ingest or claim. Generated output remains untracked; do not push or
  publish.

## Superseded current state — 2026-09-12

- Codex remains sole private KTT worker and sole local MariaDB writer; no
  exact flagged zero-hole-negative exists.  Fetched/verified and
-  primary-ingested Laplace v13/14/15 and Abacus v160/v162/v164 add 23
  further positives, completing this exact two-hole-closure campaign at 56
  (latest ledgers `52f1cebe...` and `64b28601...`).
- Abacus v166 `20260912T203243-b2e772b0a3ba` is fetched/verified and
  primary-ingested (`e381a915...`): its two direct d120/d118 closures of
  `9b0689...` are exact-positive. v168 `20260912T204326-f2b97a0f24c1` is
  fetched/verified/primary-ingested (`03c64315...`): all six targets positive.
  Immediately before Abacus v170 `20260912T210343-e7d1601bd9cc` was
  submitted, both queues, all
  terminal/pending candidate and quotient-poset records, and the actual host
  process were re-audited. v170 now uses all six Abacus CPUs for six distinct
  d122/d122/d120/d120/d118/d118 one-hole-derived zero-hole closures, with a
  2100-second envelope (run `5f834e56...`; input hashes `82d29e70...`/
  `ef2c3c0c...`/`61c9f548...`/`af681e15...`/`cebc1c7c...`/`f1fe96b1...`).
  It arose from a new 190-parent audit yielding 80 fresh posets. A d77
  presentation and a known v149-positive poset remain deferred duplicates.
- Laplace v16 `20260912T203916-6753c1c93869` is fetched/verified and
  primary-ingested (`3e677e5a...`): its d116 target is exact-positive in
  6,254,248 states / 465.552 s. v17 `20260912T204802-c0683b3c49ee` is also
  fetched/verified/primary-ingested (`623f17ec...`): all d126/d126/d117
  targets are positive. v18 `20260912T204820-ed13f32b6594` now runs the
  remaining disjoint d115/d114 pair within the three-CPU/12-GiB hard limit;
  neither batch mirrors Abacus.
- Both jobs use the known control, 30M states/1200 seconds per target and a
  2100-second envelope; runner/helper/archive hashes are
  `e3bd01d...`/`a4aae249...`/`bf1cfa42...`. Host v159 added a positive d107
  and fresh four-negative d119 one-hole parent `9b0689...`; its two zero-hole
  closures are v166. Host v161 additionally found fresh d117 one-hole
  negative `9297a8...`; its d118 exit was positive and d116 was v16. Host v163
  added fresh d118 one-hole negative `66e283...` (70.418 s). Host v165 is the
  sole one-CPU nice-10 CPU-15 broad-growth process, using the global-
  pending-poset guard. Both direct zero-hole exits of `66e283...` (d119/d118)
  are already exact-positive.
- Verify/fetch every terminal bundle.  An exact zero-hole negative must have
  byte-matched local replay before ingestion or any claim.  Keep generated
  output untracked; do not push or publish.

## Historical continuity

## Continuation — 2026-09-12 (active)

- Codex remains the sole private KTT worker and local MariaDB writer. There is
  no exact flagged zero-hole-negative. Abacus v135--v139 and Laplace v1--v4
  are fetched and primary-ingested; the latest exact-positive zero-hole
  ledgers are `48393908...` (v139) and `8a2f9707...` (v4). One-hole negatives
  are discovery evidence only. The verified runner/helper/archive hash to
  `e3bd01d...`, `a4aae249...`, and `bf1cfa42...`.
- Abacus v140 `20260912T174357-bb590b7f10d0` is fetched/primary-ingested
  (`c232ba5e...`): six exact-positive d119 targets, 5,450,302--5,980,253
  states and 294.222--381.618 s. Its repeated quotient posets exposed that
  genome keys were an inadequate duplicate guard. v141 was cancelled after
  111 s before a JSON case finished; v5 after exact controls but before a
  target; queued v6 before dispatch. Fetched cancelled bundles have no new
  target result.
- Private `eb7d5f2` rejects exact posets and deduplicates a batch; `97a274a`
  also reserves a poset pending in another candidate. Abacus v142
  `20260912T175829-d3d082e7c384` is fetched/primary-ingested (`d499b5e6...`):
  six novel zero-hole targets exact-positive, 3,861,858--5,980,132 states and
  195.775--339.061 s. Abacus v143 `20260912T180331-410c67f48eb7` is now
  fetched/primary-ingested (`0cb1aeaa...`): six novel d114--d111 targets are
  exact-positive (2,956,096--5,633,641 states; 134.754--316.087 s), all
  controls exact-known-match.
- Laplace v7 `20260912T175947-6e6db89bc419` is fetched/primary-ingested
  (`37e485fb...`): novel d117/d116/d115 zero-hole posets `b9b2b6...`,
  `f9dc0f...`, `afd69f...` are exact-positive (5,951,569--5,979,155 states;
  582.635--630.809 s), all controls exact-known-match. Disjoint v8 now uses
  all three Laplace CPUs under 10-GiB high/12-GiB hard/no-swap/128 tasks. Host
  v144's distinct d101 `ec9b39...` completed exact-positive in 40.906 s /
  1,828,823 states (one nice-10 CPU, run `1961de21...`). Before every
  submission, audit both queues, host, prior results, candidate keys, and
  quotient-poset hashes; never mirror Abacus. Generated artifacts remain
  untracked; no push/publication. Fetch/verify terminal output and locally
  replay/ingest only an exact zero-hole negative.
- Poset-audited v143 is complete (inputs `f405c172...`, `0f6c76c8...`,
  `a8169ca6...`, `3974c647...`, `08c225aa...`, `e416159a...`, run key
  `c6b6d500...`). Its Abacus v144 successor now runs four novel d100--d97
  direct closures. Laplace v8 `20260912T180426-761a54597c4e` is fetched/
  primary-ingested (`12b71b9e...`): d104/d103/d102 targets `fc1294...`,
  `f41cb5...`, `d32da5...` are exact-positive (1,841,770--1,846,504 states;
  121.826--195.347 s), all controls exact-known-match; inputs `e80c70ed...`,
  `7f320b7a...`, `7702041c...`, run key `ae77d0a2...`. Both completed under
  30M/1200-s/1500-s.
- Host v145 completed its one-CPU grow/flag-augmentation pass: fresh d120
  one-hole negative `42bcf8...` has three negative coefficients (3,181,332
  states), but every zero-hole closure was already exact-positive. Private
  `be263e2` rejects globally pending quotient posets during future host work.
  Abacus v146 `20260912T182808-3b1fb9e5fbc2` is fetched/primary-ingested
  (`d4b43389...`): novel d88/d89/d113 one-hole targets `254240...`,
  `725537...`, `d7e0fe...` are exact-positive (344,969--3,522,455 states;
  12.828--140.818 s), all controls exact-known-match.
- Laplace v9 `20260912T183442-531a2e456a15` reached its outer 1500-s
  supervisor deadline: controls exact-known-match; d142c `3c15b9...` reported
  its 1200-s target limit, while d142a/b `718e8d...`/`151ee8...` were
  terminated before target JSON. All three are provenance-backed `timed_out`
  attempts under ledger `1f4eef8f...`; future Laplace envelopes need
  control-time margin. Abacus v147
  `20260912T183712-603dd0ff9b63` completed: controls exact-known-match and
  hashes verified, but d137/d146/d147 targets `896c09...`, `f475e7...`,
  `068a45...` each reached the 1200-s bound without an exact result. They are
  provenance-backed `timed_out` attempts under ledger `e94f24cf...`; no retry
  is implicit.
- Host v148 has completed its sole one-CPU (nice-10 CPU 15) refreshed
  64-parent one-hole grow/flag-augmentation pass (eight 6M/360-s cases,
  1000-s outer), with both remote batches in its pending-poset snapshot. Its
  two newly computed one-hole negatives `448e36...` (d107) and `dea637...`
  (d116) close only to already exact-positive zero-hole posets `49dcb2...`
  (d108), `d551a9...` (d106), `c8f178...` (d117), and `c85ffb...` (d115);
  no contender or remote reservation resulted. Private `06d2ebd` bulk-loads
  selected full exact seed records for the next host pass (12 focused tests
  pass), preserving exact/reservation checks.
- Host v149 is now the sole local process: one disjoint one-CPU nice-10
  CPU-15 pass with seed `202609120149`, 96 negative parents, eight 6M/360-s
  exact cases, and a 1000-s outer bound. Both live queues were rechecked
  immediately before launch; it excludes Abacus v147 and Laplace v9
  candidate/poset reservations.
- The final direct closures, Abacus v144 `20260912T181310-09fd3017dc09`, are
  fetched/primary-ingested (`7df0a99d...`): `db26b1...` (d100), `fa937b...`
  (d99), `cf2e3c...` (d98), `ff6d27...` (d97) are exact-positive
  (923,615--1,790,876 states; 33.875--68.007 s), all controls exact-known-
  match. Inputs `652d8646...`, `687bb3ff...`, `1b8c6f43...`, `4c8cc4de...`;
  run key `ecf33598...`. The novel direct-closure inventory is exhausted;
  grow/mutate the one-hole negative reservoir before another remote batch.

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

- Queue correction (2026-09-12): v125 `20260912T133115-c6fb2da2e610`
  completed successfully and all six targets are exact-positive
  (6,431,425--7,719,140 states; 432.219--547.650 s), primary ledger
  `f07bdcff...`; no local replay. v126 `20260912T133501-197c3031f1d3` is the
  current single-case d118 retry. Queued v127 `20260912T134726-38e0e9e0e3c4`
  was cancelled before dispatch (zero computation) solely to split four
  independent legacy cases. Replacement v127r `20260912T135331-6489b66b8d5c`
  is queued after v126 and forks them across four pinned CPUs; inputs
  `317c8f4e...`, `b646f485...`, `7d7c050f...`, `8414be3b...`. No remote
  result was duplicated.

- Six-core queue correction (2026-09-12): v127r
  `20260912T135331-6489b66b8d5c` was also cancelled before dispatch (zero
  computation), to fill unused pinned CPUs without changing the serial queue
  or resource cap. Replacement v127rr `20260912T135946-071f0542613f` is queued
  behind v126 and forks the same four audited zero-hole legacy timeouts plus
  two distinct 6M-limited one-hole cases: d119 `1e9db4...` / `8808951b...` and
  d120 `742482...` / `bafc8b0d...`. Zero-hole inputs remain `317c8f4e...`,
  `b646f485...`, `7d7c050f...`, `8414be3b...`; every case uses 20M/900 s,
  1200-s outer, runner `329e89f0...`, helper `a4aae249...`, archive
  `bf1cfa42...`. No dispatched work was replaced.

- v126 verification (2026-09-12): single zero-hole d118 `006168...` completed
  exact-positive at 7,453,446 states in 343.901 s with an exact control;
  fetched job/result provenance is primary ledger `32ace0e3...`, no local
  replay. v127rr is live with all six child processes observed at 97--100% CPU;
  cgroup `memory.events` records zero OOMs. Host v125 finished (354
  exact/cache, 59 deferred, 140 pruned; no zero-hole negative); host v126 is
  the sole one-core low-priority process. No exact zero-hole negative exists.

- Remote continuation (2026-09-12): after an audited live queue check, v128
  `20260912T140632-4dc5ba953418` was queued behind v127rr with six distinct
  one-hole 6M-state limits (d122 `8d865a...`, d125 `425396...`, d128
  `41207b...`/`508ea3...`, d129 `9bbaee...`, d131 `cb60d9...`). They run in
  parallel at 20M states/900 s with a 1200-s outer limit; inputs
  `ba3cb175...`, `25268d42...`, `5013f04b...`, `0644ff61...`, `eedd11d3...`,
  `94d7570d...`, runner `329e89f0...`, helper `a4aae249...`, archive
  `bf1cfa42...`. No candidate is duplicated; generated files are untracked.

- Validated near-miss (2026-09-12): v127rr `20260912T135946-071f0542613f`
  completed six exact results. Four legacy zero-hole cases and d119 one-hole
  are positive. d120 one-hole `742482...` (11x13, dimension 120, sole
  nonflagged hole `(7,13)`) is exact-negative in degrees 2--4 at 8,071,584
  states. A byte-matched local replay of input `bafc8b0d...` exactly agreed on
  control and target polynomial data/DP states; validated ledger `a2ffd7d7...`.
  This is a one-hole near-miss, not the exact zero-hole flagged target;
  zero-hole exact-negative count remains 0. Host v128 is sole low-priority
  process; v128 is live on Abacus. Generated files untracked; no push.

- Direct-closure continuation (2026-09-12): v128 is terminal; after an audited
  empty-queue/candidate-state check, v129 `20260912T142101-d14372ee4b1c` was
  queued for three direct zero-hole closures of validated d120 near-miss
  `742482...`: d121 drop `01d198...` / `66c8ff94...`, d119 flag swap
  `2f0e0d...` / `074815c6...`, d121 flag swap `8fd3a5...` / `d77b3f87...`.
  Each is durable pending, runs 20M/900 s (1200-s outer), runner `329e89f0...`,
  helper `a4aae249...`, archive `bf1cfa42...`. Any negative gets local replay;
  no duplicate/push.

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
