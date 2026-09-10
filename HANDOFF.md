# Ehrcalc Handoff

## Corrected full KTT ranking — 2026-09-10 (active)

The private size-50 audit now terminates with 260 structurally valid ranked
classes, 121 durable deficient-span exclusions, 14 size prunes, and zero
timeouts.  Affine audit version `scale-one-affine-v2` prevents repeated work.
The seven-tail leader `161a292e...` is zero-hole degree 24, but its partial
known-ends score becomes positive when extended through strict dilation 10;
tail scores are being used only as heuristics.

For structural degree-25 `a2be13af...`, exact strict dilations 8 and 9 match
locally and on Abacus: `295545638645753468` and `9102474961949084714`.
Local/remote timings were 67.970/91.249 s and 224.477/319.714 s.  Dilation 10
is active locally and in bounded Abacus retry `20260910T200302-bc3142e84596`.
Its predecessor `20260910T200013-c7393f5e4055` was rejected before counting
because 2400 seconds exceeded the runner's 1800-second cap; both attempts and
the diagnosis are stored.

Abacus independently matched `42d92b82...` at
`L(17)=392796087462135314187172131` (5393.744 s, 34,921,826 peak states),
job `20260910T171434-ce38e10af4fa`, output SHA `a94dd1f3...`, completed ledger
`f1059741...`.  BEST NEGATIVE remains degree-84/two-hole `612334d7...`; best
fully certified proper near-miss remains positive degree-23 `42d92b82...`.

## Structural flagged-reciprocity repair — 2026-09-10 (superseded checkpoint)

Codex remains the sole KTT/private-companion worker and sole local MariaDB
writer.  Exact recounting exposed a correctness defect in the private tail
heuristic: scale-one lattice points need not affinely span a rational
fixed-content flagged GT polytope.  One apparent degree-25 negative
(`a2be13af...`) had structural dimension 25 but scale-one span 24; direct
ordinary counts disproved both reconstructed polynomials.  It is not a
counterexample.  A transactional quarantine archived 862 affected strict
samples, 120 zero-time derived ordinary samples, and marked 13 polynomial rows
plus affected attempts `invalid_span_deficit`; no affected row remains active.

The maintained engine now exposes propagated flagged bounds and an exact
flag-aware strict DP, and adaptive reciprocity uses it for flagged inputs.
The regression instance reports dimension 25, a genuinely restrictive flagged
example agrees between positive and reciprocal interpolation, all 38 crate
tests pass, and strict Clippy passes with `-D warnings`.  The private generic
adapter reproduces corrected `a2be13af...` strict counts at dilations 1--7:
`0,0,0,56655893,136866338728,48574493345190,5544119621688912` (last
16.106 s).  Its partial tail is strongly positive, so it does not replace the
global proper near-miss.

BEST CERTIFIED NEGATIVE remains `612334d7...`, degree 84 with exactly two
nonflag holes.  BEST PROPER ZERO-HOLE NEAR-MISS is `42d92b82...`, straight,
degree 23, certified positive, normalized smallest coefficient
`2158459137/24953681152400` (~`8.64986e-5`).  Abacus direct `42d9 L(17)` job
`20260910T171434-ce38e10af4fa` remains running; inspect rather than resubmit.
New structural control job `20260910T192616-487d1344de57` is queued behind it,
with local expected dilation-6 count `48574493345190`; runner/input/binary
SHA-256 are `4987fc9f...` / `2a1bbe2f...` / `6078110c...`, and MariaDB ledger
run is `f5b86942...`.  Submit useful structural work only after this control
matches remotely.

## Large zero-hole fixed-content genetic pivot — 2026-09-10 (superseded checkpoint)

Codex remains sole KTT/companion and local MariaDB writer.  Generated reports,
inputs, and logs remain ignored; no extra workers, account/admin changes,
pushes, or publication.  The Jochemko--Menon positivity result applies to the
ambient unsliced interval-flag face, so this lane now uses large reduced-Kogan
geometry only as scaffolding and evaluates exact fixed-content flagged Kostka
fibers.  Every evaluated paired mutation and every completed dilation is now
durably deduplicated in MariaDB.

From exact zero-hole center
`0a2c22ef64f6b6831e3644dd0e9a84ffc33d37b6f182f3c61a1413bbf4335410`,
the finite paired shape/flag plus unit-weight generation has 8,159 distinct
compressed proposals.  All are classified: 4,125 nonempty exact three-count
signatures, 2,544 exact-empty fibers, and 1,490 dimension-above-26 prunes.
Two initial 2-second timeouts completed under a 10-second retry; no pending
attempt remains.  The 4,125 nonempty rows give 395
`(dimension,L(1),L(2),L(3))` classes.  Reciprocal tails through six strict
dilations are certified for all 381 classes with `L(1)<=200000`; fourteen
larger-count classes were explicitly skipped.

The two first negative-known-ends leaders were proved exact-positive and
Ehrhart-equivalent to earlier positive near-misses.  Three further leaders of
degrees 17,17,19 are exact-positive.  New key
`62ce9739a35b5ca03aa7a57c614b8f3e555b2ec18134ed15ff263128c344c3ec`,
straight shape `(5^9,3,1,1)`, size 50, degree 21, zero nonflag inequalities,
is exact-positive with normalized low-degree margin
`189426163/1442060373600` (~1.3136e-4).  This improves the new generation but
not the global zero-hole record `2af751a0...` (~1.26228e-4).  BEST CERTIFIED
NEGATIVE remains the degree-84 9x10 reduced Kogan face `612334d7...` with two
nonflag holes; there is still no flagged-Kostka counterexample.

One negative-known-ends class remains unresolved: key `42d92b82...`, straight
size-50 shape `(6,5^7,3,3,2,1)`, degree 23, zero holes.  Exact direct samples
are stored through `L(16)`; `L(13)` through `L(16)` took 252.536, 536.630,
1076.248, and 2270.763 s locally.  The new exact value is
`L(16)=105750769630621524527579594` with peak 23,759,702 packed states.  Local
`L(17)` is active under a justified 7,200-second outer bound.  Initial Abacus
`L(17)` job `20260910T161006-faf491f3c6fc` returned `time_limited` after its
3,300-second case cap and produced no count; `L(16)` cross-check job
`20260910T163128-bbbbe2d697ec` remains active.  Diagnosed wider `L(17)` retry
`20260910T171434-ce38e10af4fa` is queued serially with 6,900/7,200-second
case/job bounds.  Inspect these recorded IDs; do not blindly resubmit them.

Abacus packed-counter compatibility and exact local matches are complete for
jobs `20260910T152301-53ee0a7009a1` (known `L(10)` control),
`20260910T152405-1877ba4bce6d` (`a7a5 L(15)`),
`20260910T154654-c47937d30808` (`0ad0 L(15)`),
`20260910T154654-ab6cd09457fc` (`035c L(15)`), and
`20260910T160516-105cdc5453d1` (`62ce L(15)`).  The widened runner/engine
SHA-256 are
`233aeb7bb8a5f1169240c92aa6de9c942b8082e69fff6c361b6d8bcaa114ea98` /
`9852ecd5569dc7ad1e108d740d6e49c2f3a08cb7d4df392755c5e966b0f3bbba`;
remote Python 3.10/glibc 2.35, CPU `[0,1]`, 8 GiB/no swap/128 tasks were
verified.  Local ledger run is `22ea3e23e6caffe6e6420abe074c05cf0c7540d257dfd6d3b7e41e23a4642594`.
The wider retry is independently recorded under ledger `f1059741...` with
input SHA-256 `911dc4b4...`.  Two concurrent zero-label-compression probes at
`L(12)` exhausted 55/175-second bounds, so active raw jobs were retained;
ledger `d8bea200...` records this non-result and prevents repetition.

Private companion source now has a self-contained packed flagged Abacus
runner plus database attempt identities for paired screenings.  Resource-bound
retries reuse exact dilation samples and skip every prior terminal candidate.
All 75 companion tests pass in 0.566 s.  Companion commits `ddd5999` and
`8254139` checkpoint database-backed screening and the finite remote retry cap.

## Flag-biased and strict one-hole genetic runs completed — 2026-09-10

Codex remains sole KTT/companion and local MariaDB writer.  Two further
resumable genetic runs and two independently validated Abacus batches are
complete.  No source algorithm, worker/account, administration, push, or
publication changes.  Generated inputs/reports/logs remain ignored.  BEST
NEGATIVE is still the 9x10 degree-84 candidate
`612334d74e3ba8f384a8647c90a4a7fcd46f129dde61c9f2999d9a1e77fab474`
with exactly two nonflag holes `(4,6),(6,10)`.  No zero/one-hole negative or
two-hole negative of lower degree was found.

Fresh flag-biased run seed `202609101435`, run
`3d30d2e6dd5e6f46378b8abe75e6f30dd37a57245a7011c0ed4ef897c535f080`:
maxholes2, elite32/local64, minside1, .05 effective bias, strength6,
40 frontier+64 random-shape immigrants, 500000 states, 5 s/case, 55 s
internal, tested 180 s outer resumptions.  Complete generation2: 3888 unique
exact attempts, 307 bad-edge prunes, zero pending/deferred, 4987 selected and
17124 duplicate proposal events; 53 new exact rows across four invocations.
No improved negative.  It exposed cheap positive frontier cases including a
one-hole degree11 and two-hole degrees5/9.

Abacus E `20260910T144045-54c6de5bc8d8`, exit0/wall32.3409616947 s,
validated parent d84 NEG control plus one-hole d11 and two-hole d5,d9,d67,
d71,d74,d75, all search cases exact NONNEG/full local matches.  Input/output
SHA `234e03798313fa10796a9e39fe210bc85cff66b03f99eb681c21b4141925a337` /
`d939fab82644e79457eb89d44b5a144980e77c57c77e7ce7e1b9b4d8ef7b3eb4`;
ledger `0ed69705631d2c2440efef87204a13a5d05f640f56c2386d907939d1e7682861`,
IDs 237654/237655/237662..237669; fetch
`/home/dev/.local/share/supervisor-compute-results/abacus/20260910T144045-54c6de5bc8d8-287w5xyb`.

Fresh strict boundary run seed `202609101450`, run
`019d86e3b6569bd6ed1faae75c1765b4efe93681ba6090c52630af6c85c560cf`:
same broad pools but maxholes1, .10 effective bias, strength5, 32 frontier+64
random immigrants.  Known two-hole negatives were mutation parents; every
new proposal above one hole was pruned before counting.  Complete generation2:
2751 unique exact attempts, 1326 bad-edge prunes, zero pending/deferred, 4370
selected and 16931 duplicate events; 16 new exact rows across two invocations.
No improved negative.

Abacus F `20260910T145339-ee8f03e04735`, exit0/wall14.1732995510 s,
validated parent d84 NEG control plus a genuine zero-hole d9 case and one-hole
d5,d7,d12,d13,d13,d14; all seven search cases exact NONNEG/full local matches.
Their tiny state counts were 9..44 (0.000827..0.012849 s remotely).  Input/
output SHA `194ccf6e7b77ed01cf975db48cb354120c888312dfd635b10b132982f4c842ca` /
`8b0e714284490e5b24c8776c8a5b2b5b3014063db35bbe64d6657def5970387f`;
ledger `1de32022d31680dbaff53a78f5ca3e3321ddab6c010d30c9cedcf167f924a124`,
IDs 237675/237676/237680..237687; fetch
`/home/dev/.local/share/supervisor-compute-results/abacus/20260910T145339-ee8f03e04735-rco5zj_a`.

For E/F the transferred runner/helper/archive hashes and verified remote
Python3.10/x86_64 CPU `[0,1]`, 8 GiB/no-swap/128-task limits were unchanged.
Bundle/source/input hashes, natural predecessor masks, independent exact h*
transforms and complete remote/local polynomials were checked before ledger
completion.  Search remains active; next deepen one-hole positive lineages and
add shape/content mutation only with terminal-status DB persistence.

## Third strict genetic seed completed; Abacus batches C/D validated — 2026-09-10

Codex retains KTT/companion ownership, private profile/model, and the sole
local MariaDB-writer role.  No extra AI workers, administrative changes,
pushes, or publication.  Generated JSON/JSONL and remote logs remain ignored
and untracked.  The active conjecture search is not complete and has no global
blocker.  BEST NEGATIVE remains candidate
`612334d74e3ba8f384a8647c90a4a7fcd46f129dde61c9f2999d9a1e77fab474`:
9x10, degree/dimension 84, three selected equalities, exactly two genuine
nonflag holes `(4,6),(6,10)`.  No zero/one-hole negative and no two-hole
negative below degree 84 was found in this checkpoint.

Completed strict genetic run seed `202609101410`, run
`4f3708dce43d0c080e79d1e9be1e8fbdc2d4dd5a8e89e37a460ec43df006ef0e`:
2 generations, 24 elite/48 local, max 2 holes, min side 1, 500000 DP states,
5 s/case, 55 s internal per invocation, effective bias .20, mutation strength
5, 32 frontier plus 48 random-shape immigrants.  Resumable passes used the
same identity and a tested finite 180 s outer cap.  Final MariaDB state is
complete with 2227 unique exact attempt rows, 496 bad-edge prunes, zero
pending/deferred, and 3250 selected plus 12615 duplicate proposal events.
Across the five invocations 83 new exact computations were added.  No improved
negative was found.  Reports
`runs/bad-edge-genetic-twohole-degree-s202609101410*.jsonl` are ignored.

Abacus batch C job `20260910T141041-882d0e27ced9` finished exit 0 in
48.5284991264 s.  The degree-84 parent control reproduced its two negative
coefficients.  Three one-hole cases of degrees 68,74,74 and four two-hole
cases of degrees 71,71,72,73 were exact NONNEG; every complete polynomial
matched the later local exact result.  Input SHA
`994925dab0ac03286104aa038cb9375fed0823686052fa38a84e786243ff0c1d`,
output SHA
`40f593774d5b04e58bb82cbe54d6dceb0a17b2abd64b7b0d5b018ca5e002b16c`,
ledger `d6c9600d2b108a1429357ff881849c938e5bdf92722221d3d8b68470be9f632f`,
prepared/receipt/validated observation IDs 237620/237621/237628..237635.
Fetched outside Dropbox at
`/home/dev/.local/share/supervisor-compute-results/abacus/20260910T141041-882d0e27ced9-fm4izwux`.

Abacus batch D job `20260910T142725-ceb26774160e` finished exit 0 in
46.499686718 s.  Parent degree 84 again reproduced exactly; seven remaining
coordinate-distinct degree-75 two-hole deferred cases were exact NONNEG and
full local matches (remote DP states 123155..173533, 4.1148..5.7919 s/case;
parent 285360 states/10.0375 s).  Input SHA
`bbbe17061bc331df37a8949a06dbedd7d183cbbc0ccbbcd17ce6db5b2ea25fa5`,
output SHA
`dd0050d6b84fb7d42c2df05ffa30d3e4d4929afd2e613da8557a4a5591bcfa96`,
ledger `f4df1cd5291ef2442f2983a1a20137bd38e75ecccf7fbdd64a45808f1f1da2e7`,
prepared/receipt/validated IDs 237639/237640/237644..237651.  Fetched outside
Dropbox at
`/home/dev/.local/share/supervisor-compute-results/abacus/20260910T142725-ceb26774160e-45c5axuw`.

For C/D, selected runner/helper/archive hashes remained respectively
`cacc046442f494dadaefb7389d1e5e479a4eeddae0d797711bb33095d09cd4a4`,
`a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a`,
and `048cb293da34c49e51d5d617f294a62e7f08f6bb455e41309f09652add521bb7`.
Bundle files, selected source hashes, natural transitive predecessors, exact
h* transforms, and complete remote/local polynomials were independently
checked before ledger completion.  Remote Python 3.10.12/x86_64, CPU `[0,1]`,
8 GiB/no-swap/128-task caps were verified unchanged.  Only trusted selected
code and nonsecret inputs were uploaded; no database or checkout.  Next:
start a fresh more flag-biased genetic seed, continue disjoint Abacus batches,
and add coordinated fixed-content/shape mutation only with attempt-complete DB
persistence.

## Strict two-hole genetic search and exact Abacus DP — 2026-09-10

Codex retains KTT/companion ownership, private profile/model and sole local DB
writer. User requested continued genetic-type search, now correctly optimizing
few holes first and low degree/size second. Compact mixed-face branch remains
paused. No extra AI workers, administrative changes, pushes or publication.
All local and remote handles terminal. Goal active; no global blocker.
BEST NEGATIVE unchanged:9x10,d84,three selected equalities,exactly two nonflag
holes[(4,6),(6,10)],key
612334d74e3ba8f384a8647c90a4a7fcd46f129dde61c9f2999d9a1e77fab474.
No negative with fewer holes or degree<84 found.

Completed upper-flag suffix neighborhood of that parent. Original audit had
90proposals:45nonreduced,1unchanged,44eligible rows. Exact scan first40 rows:
35exactNONNEG,5actual-coordinate duplicates,41.82178402203135s,
2032b9960e59ab4a97f27311a0f4cc2a0bf1fdc1d3e13356dcb83d9cfd9d3882.
Remaining distinct d76,d76,d78 cases exactNONNEG in
1b4186a65c5b97f1c101b863a456436b63175abd76b80246b6d4ccee1588da37
(4.420030815992504/3.8249582740245387/4.631791427964345s); d80 case was
already freshly NONNEG in prior c668... audit. Thus all39distinct coordinate
classes exactNONNEG and5duplicates; no extrapolation beyond this neighborhood.

Added selected-source exact companion descent-DP Abacus adapter:
private commits91d5322,6867655. It accepts natural transitive predecessor
masks,<=8cases,<=30s/case,<=500000states, verifies hash-allowlisted ZIP of
only kogan_search/{__init__,quotient,ehrhart}.py without extraction, validates
exact transforms/control, and checks remoteCPU/memory constraints. README and
tests included;72 tests passed in0.545s. No counting-algorithm changes.
RunnerSHA cacc046442f494dadaefb7389d1e5e479a4eeddae0d797711bb33095d09cd4a4,
helperSHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a,
archiveSHA048cb293da34c49e51d5d617f294a62e7f08f6bb455e41309f09652add521bb7.
SourcesSHA: __init__ b1958d8b91608437a4b72095b438bc7912bddab0056a626f03b642dcdfce1d2c,
quotient f08515b3c39e4321c0bbf30cb3053fbaf460f3e3bfb50bae10a21adfbf5f8b16,
ehrhart68dece902b753d70f460e4963dcd6ab8d4690f54b34ee0e3aa5595e3a394ecd0.

First pilot20260910T133203-196c6574e803 failed/exit1 after d40 control passed:
initial JSON rounded degree84 h* integers above2^53 through JavaScript. Same
job inspected/fetched,no blind resubmit,no search ingestion. Files verified;
outputSHA34dfa09a0b8694453af07924894ac29ba086e1b7f51f24718344e992bfbf9d66.
Ledger9c41c975e6bca7c9d4a7d44341981c44c786fec74a6c49022c41134d4d8f2f7c,
prepared237561,receipt237562,failed237569. Fixed by decimal-string h* controls
and >2^53 regression test; corrected input passed both controls locally.

Corrected pilot20260910T134228-c2185e92c0c8 done/exit0,wall38.423155546188354s.
Controlsd40NONNEG and parentd84NEG full local/remote matches. Three search
children237410d76,237431d76,237421d78 exactNONNEG remote/local full matches.
Remote seconds10.072961052006576(parent),8.42533970400109,7.060902491997695,
8.771355484990636; local5.2285941750742495,4.442786686006002,
3.954176373081282,4.64716413593851. InputSHA
3ef1ff744a114e519229ebf5b60a859fdede45b083fa49014f29dcd54d925f16,
outputSHA19d3bf5c760dc9a8432387a7139729e0b5b2b3661b3b024343c71267d6a8f90f.
Ledger92cb62bbafefdaaa8d52b14627659bbd2c7fbe6cc49d9d7ba0c4bac9d54b9d28,
prepared237571,receipt237572,validated237575..237579.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T134228-c2185e92c0c8-7e8hp92w

Strict two-hole genetic runseed202609101342,effective-bias0.25,
mutation-strength3,16elite/32local,16frontier+16random-shape immigrants,
500000states,5s/case,55s internal. Initial60s guard stopped terminal after
durable generations0/1; exact same ledger resumed with tested120s outer caps.
Complete generation2:1387exact attempt rows,208bad-edge prunes,0deferred;
51 new exact results across completed resumptions, no improvednegative.
Run cd0a3bf6715b05a1c8c61caf74bec007a6f50de52640b90a83f948a92683fe94,
2267selected+8653duplicate proposal events. Generated JSONL ignored.

Abacus deferred batchA20260910T134902-2b6515ecf539 done/exit0,
wall50.5005464553833s. Parentd84 control exactNEG; seven smallest
coordinate-distinct d71/73/74/75 genetic deferred cases exactNONNEG and full
local matches. InputSHA671ab63e7c48755c173984de73c54e71dad3d33a6c42679e68cc65e8f400eb7a,
outputSHAd53bcad47eb738f5c6714bf0eedaf97b396988c9cc02b5825ab16bf69e3905ee.
Ledger4659ce95d8b6562765a46706f3dc8a3f79cd92de3426ca8cbecd302d8ba1a00a,
prepared237583,receipt237584,validated237591..237598.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T134902-2b6515ecf539-edhjdpe7

Fresh strict-two-hole genetic runseed202609101355,effective-bias0.10,
mutation-strength4,20elite/48local,24frontier+24random-shape immigrants,
500000states,5s/case,55s internal. Initial120s outer guard stopped during
generation2 with durable state; exact same identity resumed with justified
180s outer bound after observed finite proposal overhead. Complete generation2:
2136exact attempt rows,211bad-edge prunes,0deferred; final two completed
resumptions reported24+20 new exact, no improvednegative.
Run f7f8cc1e2cda1bc810bb4f1be0a835923672ccc43033144e9d42e6993b38dd26,
2893selected+10954duplicate proposal events. Generated JSONL ignored.
Outer timeout is orchestration overhead,not candidate sign; interrupted pending
case was resumed. No untracked timeout treated as nonnegative.

Abacus deferred batchB20260910T140058-9b2d29e1551b done/exit0,
wall46.54526114463806s. Parentd84 control exactNEG; seven coordinate-distinct
d75 genetic deferred cases exactNONNEG and full local matches.
InputSHA1370f056787e4661f1b5c58bee43d26ea688686617c39cd00c816038d9668448,
outputSHAf8b1b50fa6c8a33f480f8a4362a8b1c1b9163a211d069001a462ed20f9d2ff11.
Ledger46e7a25d38fb01917c292b0cddf45e1ea0723a2854e33128d99f04ce087c5ae7,
prepared237604,receipt237605,validated237609..237616.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T140058-9b2d29e1551b-o43xph4n

For all successful remote jobs: transferred filenames/receipt hashes,input and
source geometry/predecessors,independent h*->power rational transforms and full
remote/local polynomials verified before DB ingestion. Python3.10.12,x86_64,
CPU[0,1],8GiBmemory,swap0,pids128 verified; limits unchanged. No secrets,
database/shared checkouts uploaded. Host supervisor can collect byjobID.
Next: fresh strict-two-hole genetic seeds with different flag/shape mutation
settings, use Abacus exact-DP batches on disjoint deferred/new candidates, and
explore coordinated shape/weight mutations. Do not call d84 a flagged Kostka
counterexample: it still has two genuine nonflag holes.


## Two-hole parent freshly reproduced; one-hole flag shrinks tested — 2026-09-10

User clarified: combine few bad edges AND low degree. Retain two-hole family
as primary; compact mixed-face branch paused. Previous reply was planning only;
this turn made concrete progress. Codex remains sole local DB writer and KTT/
companion owner, private profile/model unchanged. Clean Git/process checks.
No source edits, new tests, extra AI workers, admin/toolchain changes or pushes.
All local/remote handles terminal. Goal active, no global blocker.

Supplementary exact companion descent-DP freshly recomputed parent
612334d74e3ba8f384a8647c90a4a7fcd46f129dde61c9f2999d9a1e77fab474,
mask0x200000000010000001000,9x10,d84,two holes[(4,6),(6,10)]:
285360 DPstates,6.362598060979508s, negative linear/quadratic coefficients;
full cached power/h*/sign list matches after independent rational transform.
Fresh d80 single-addition neighbor
a2edfe26e15fdc1abec8383c31b8dc3a7d8582fe62c0e1afd4fdd5c87c524508,
mask0x200200000010000001000:280542states,5.802992757060565s,exact NONNEG;
full cached match. Both bounded500000states/60s outer,nice10, serialized.
DB c668ffa018c15dc6a283d801f0793a6f09f4d6d28a02b9abef02da3a29da4812.
Important provenance: these are fresh companion Python exact-descent counts,
NOT completion of the maintained-engine d84 timeout. They supplement it.
Maintained engine natural-layout d84/30s remains unresolved from prior turn.
Audited beam32/128 layouts both improve frontier cost(10,686)->(8,516);
these layouts not yet counted for d84. No claim width optimal.

Single free-equality additions retaining <=2holes and reducing d84 gave only
the above d80 face, already stored NONNEG. Broadened to full upper-flag suffix
in one original row: rows1..9,cut=row..row+9,90 proposals, all DB recorded.
Require unchanged original mask subset, reduced Kogan word, <=2holes,d<84.
45 nonreduced-pruned,1 unchanged,44 eligible proposal rows:
42 missing from genetic-candidate table,2 already stored (d80exactNONNEG and
d72bad_edge_pruned). Missing keys are NOT proof of globally new coordinate
faces. One pair row5/cuts9,10 gives identical mask/key; do not recount it.
Audit d4d0d9679901d34a8ebde2ef9337c5c3dc64d5b5341a722ee3409c760ad6fa07,
0.6852314228890464s. Includes masks/row/cut/holes/dimensions/candidate keys.
Do not extrapolate nonreduced pruning to positivity or impossibility.

Selected four distinct one-hole proposals (augmented hole[(4,6)]):
237413d40 originalrow5/cut5,
237414d45 row5/cut6,
237416d52 row5/cut8,
237417d59 row5/cut9 (237418 same mask,skip).
Abacus20260910T132148-9cf5d64b2696 done/exit0,wall80.79559898376465s.
Five serial cases incl freshcontrol,20s/case/120s whole, finite monitored bound;
samejob,no resubmission. Only selected trusted wrapper/input/engine uploaded.
Control196437 local0.22942929703276604/remote0.40569646601215936s fullmatch.
237413 remoteexact18.64310441800626/localexact10.393474254058674s NONNEG.
237414 remoteTIME LIMITED20.116597183980048/localTIME LIMITED30.005091078928672s.
237416 remoteTIME LIMITED20.104416573012713/localTIME LIMITED30.071432707016356s.
237417 remoteTIME LIMITED20.12719160100096/localTIME LIMITED30.057141047087498s.
Local maintained batch1d6163f419067aa406ac39db1062710649e1fb0573ff4276f067d5fad0498d11,
beam128,4x30s serial/150s outer,nice10,CARGO_TARGET_DIR; explicitly justified
whole bound includes tested finite per-case caps and overhead.

Supplementary exact descent-DP then resolved ALL FOUR as NONNEG:
2374130.1626300059724599s,2374140.3091991259716451s,
2374161.0754453720292076s,2374171.7844119489891455s.
Run2badf2bf8d7ace85a7318c7af32da98f79d3ef5b4df93ccffc51a051c5e7dfa1,
500000states/case,60s outer. All power/h* reconstructed over Q.
Degree40 full polynomial agrees across BOTH independent counting algorithms
and Abacus. Higher3 signs are exact companion results; maintained local/remote
timeouts remain separately recorded, not relabeled as completed computations.
This large speed difference motivates a bounded maintained exact ideal-DP
adapter or a carefully audited remote companion runner, not more identical
frontier-counting timeouts. No such source implementation exists yet.

All remote original quotient/permuted covers (triangular masks for4sources;
mixed-face control), receipt/file/header hashes, x86_64/Python3.10.12/glibc2.35/
libraries,CPU[0,1],8GiB,swap0,pids128 verified before ingestion.
Input runs/abacus-four-onehole-suffix-20260910.json SHA
48c9e85fef03f7b4a04c682caf92d069a877ebbac3bbf2042a5f4641e0766e24.
Wrapper SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a.
Engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367.
Output SHA b8a65919a535d87bb802df0c9de452be8b3da461371be2fc21fc87377d3beefa.
Completedledger4bf1b7f99cc5c277861fb5779bde815fbf8014adee8be07938c2a2bbc03d9a6e,
prepared237463,receipt237464,validated237481..237485.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T132148-9cf5d64b2696-d2uqtsaq
Host supervisor can collect byjobID. Input stays ignored/untracked.
No improved negative. Next inspect/test remaining distinct <=2hole suffix
faces with bounded exact counting, including lower degrees, then coordinated
flag/shape/weight mutations. Use DB/coordinate dedup, not missing genome keys
alone. Keep Abacus involved, preferably with the tractable exact method.


## Restore two-hole frontier; Abacus verification complete — 2026-09-10

IMPORTANT USER CORRECTION: earlier status called the compact mixed-face lineage
the best without including the older two-hole catalog. That was incomplete.
Prioritize fewer nonflag constraints, not merely smaller degree. Restore the
two-hole family to the active frontier; independent fresh count still pending.
Codex retains KTT/companion ownership, private profile/model, sole DB writer.
Previous turn progress. No source edits, new tests, extra workers, admin changes,
push/publication. All local and Abacus handles terminal; goal active, no blocker.

Fresh MariaDB query of kogan_genetic_candidates:4445 negative-marked genomes,
all hole_count2, dimensions84..88. Not4445 independent polynomials/faces.
Minimum dimension84, minimum selected equality count at that degree3.
Representative key
612334d74e3ba8f384a8647c90a4a7fcd46f129dde61c9f2999d9a1e77fab474,
9x10,mask0x200000000010000001000, recorded score978016.0.
Reconstructed KoganTriangle, checked reduced word, quotient dimension84,
complement-row lift and nonflagged_holes freshly: exactly[(4,6),(6,10)].
Three selected equalities give forbidden labels[(4,6),(5,14),(6,10)];
(5,14) is absorbed by flags. Shape(11,1^9)/(1),weight1^19.
Cached full h*->power reconstruction independently verified over Q;
degree84, negative coefficients at degrees1 and2:
-521557363663994321728331529347910631/79176774063173546519549708863744800;
-34367516038053202174706965110119606280354365131856381/3103748900391665120968760615294953306918236875136000.
This is cached-transform consistency, not fresh independent lattice counting.
Fresh maintained ehrcalc count, original natural layout,30s/case/60s outer,
nice10/CARGO_TARGET_DIR, timed out30.042443377897143s, no fresh sign evidence.
Sample24s RSS356884KiB; no memory pressure. Run
3a22b6a37c4edf7bb80f800967f155845fba05c22542e80b86d07069c291b258.
Do not repeat that identical30s attempt. Next: bounded larger count or audited
layout, then mutations preserving <=2holes. Abacus should remain in use.
Catalog/structure/transform review DB run
d00a00ea8c624d473d5892a0346ae39607f6bf3da809e482dfdd68d47088fb21,
fresh_count=false. Historical genetic score is not directly comparable with
the mixed-face extra-opposite/internal-ban metrics. Compact RAW230740 d33
seven extra/five internal remains a separate degree-focused branch.

Abacus20260910T131117-83724db95f0e done/exit0,wall64.64939546585083s.
Four distinct new five-opposite children+control,20s/case/120s whole,
established finite bound monitored under same job ID, no resubmission.
Actual-coordinate audit found pairwise distinct/no matches against
222055/203068/209610 lower-remove-two neighborhoods; scope in preparedledger.
Control196437 freshlocal0.22936847107484937/remote0.4181352949817665s fullmatch.
All four search results exact NONNEG,remote/local seconds/localrun:
235381d33 13.3384724019852/7.121153192012571,
d4d42b305ee4d0dc03ef2aaa868d94ba2827b9883cbab3a277c872475cec123a;
235959d34 15.174348983011441/7.902168761007488,
7a3649620df2701f804ec1ad79a21d340988f2e1132c177efa940b9cab03a9f9;
235939d35 17.432968821987743/9.885180006036535,
ccead28a2c95d9fa52f01d2f07ce470682fb54dd9c64d23b6c0ad58d8ca5675c;
235945d35 16.563520313997287/8.48161388700828,
caddde18b1c9ba99ddd6b452e75a3691348e2a8d4f8ea8fb62437b8d65827016.
Localbeam128/30s each60s outer, serialized. Full polynomials/h*/signs match;
all independently reconstructed over Q. All quotient/permutedcovers,
receipt/file/header hashes, runtimePython3.10.12,x86_64,glibc2.35/libraries,
CPU[0,1],8GiB,swap0,pids128 checked before DB ingestion; limits unchanged.
Only selected trusted wrapper/input/engine uploaded; no secrets/DB/checkouts.
Input runs/abacus-230740-four-fiveop-c-20260910.json SHA
d30b1434a482001bbb198fd237f4b1ef99c7e70696c5c8139cd88887ab8e8a22.
Wrapper SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a.
Engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367.
Output SHA 8d713a0b06659e342af1bfd7f5cc84daf8b94649adc25f23e43d0a604e24936a.
Completedledger feb75638face068d92e8b7ce101569a165429969fbc6dd16760265c5b19979ad;
readback prepared237165,receipt237166,validated237361..237365.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T131117-83724db95f0e-0glb3yhh
Host supervisor can collect byjobID.
CURRENT230740lower-remove-two:166exactNONNEG,4UNKNOWN234802/235346/235082/235324,
174dimprunes1166objectiveprunes21duplicates across1531proposals. Not positivity
of unknown/pruned/duplicate cases.

Before user's correction, started230740full-content-remove capd33,
NO flag-only objective filter,beam32/5s,45s internal/60s outer.
Forces multiplicity t of a label in ORIGINAL rectangle, then removes one
non-forced equality. Do not confuse original content with overall augmented
complement-row weight. Total603,offset0->93:63exactNONNEG1timeout25dimprunes
4duplicates,42.33791685895994s,
071ff8b15b3504c806662432e9815dc1180edbe1798c11e79c5ee17f687a6657.
All63 fresh exact independently reconstructed over Q. Resume93 only if this
secondary branch is selected again; pause it while restoring two-hole frontier.
Generated input and five reports remain ignored/untracked.


## Five-opposite proposal pass complete; next Abacus batch validated — 2026-09-10

Codex retains KTT/companion ownership, private profile/model and sole local DB
writer role. Previous turn was progress; fresh Git/process/remote checks found
clean repositories and no live counting workers. No source edits, new test
claims, extra AI workers, administrative changes, pushes or publication.
All handles terminal at this checkpoint. Goal active; no global blocker.
Best negative unchanged: RAW230740/cert230749 d33, seven extra opposite
equalities/five internal bans, alongside prior leaders. Counts are presentation
metrics, not a proved minimum nonflag distance.

RAW230740 lower-remove-two completed ALL1531 proposals, cap five extra
opposites/degree36, beam32/5s,45s internal batch/60s outer,nice10,
CARGO_TARGET_DIR=/cargo-target/ai-projects. All outcomes persisted in MariaDB.
642->1403:54exact NONNEG,4timeouts,83dimension prunes,608objective prunes,
12duplicates,39.00937469198834s, run
76f3e254214a0f7cbb0edd64bac7888e166fbf699cff24ce2c85c48a16863ba9.
1403->1531:98exact NONNEG,0timeouts,22dimension prunes,8objective prunes,
0duplicates,24.184841039939784s, run
79b81dfb7156e87c7aaa03fc4e9fe2325326aff7728ef08c343428efd7b878a0.
All152 fresh exact rows independently reconstructed over Q from h*.
CURRENT whole pass after wider retries:162exact NONNEG,8UNKNOWN,
174dimension prunes,1166objective prunes,21duplicates. No positivity claim
for unknown/pruned/duplicate cases. Eight unknown original sources:
234802d35,235346d34,235082d35,235324d36 already remote20/local30 UNKNOWN;
235381d33,235939d35,235945d35,235959d34 only initial5s UNKNOWN.
Next audit the latter four for a disjoint Abacus batch; do not repeat the
already exhausted identical remote/local attempts.

Abacus20260910T130429-5919f5be4432 done/exit0, wall109.02368950843811s.
Seven serial cases including fresh control,20s/case/180s whole; established
finite bound monitored under the same job ID, no resubmission.
Six sources pairwise coordinate-distinct, no actual GT-coordinate tuple
matches in older222055/203068/209610 lower-remove-two neighborhoods.
Scope/method persisted in prepared ledger, not a global isomorphism claim.
Control196437 freshlocal0.23343335511162877/remote0.4147809909773059s match.
Source,degree,remote/local seconds,local run:
235088d34 exact18.618621424015146/exact10.902950990013778,
ea07edddd16d3ebd7add45621c503315304a9f7fdc89b9bc7267a956ac723928;
235346d34 TIME LIMITED20.10252788802609/TIME LIMITED30.071056157001294,
288a8275db881e0bd2134217f76f26f7f323ba5c5ff2e9978170766c4d2e818b;
235082d35 TIME LIMITED20.114172504021553/TIME LIMITED30.01454179303255,
3d8ba452d43c3923ee2eade597e04f105832efacf909257d54ff62cab9904636;
235314d36 exact17.381742997007677/exact9.189628668944351,
9b0ccb47af3fdeaa14f7da42567b50acd6cd13e79ebc131d330aa1e36f083b6a;
235318d36 exact11.24300663502072/exact6.209928774973378,
c00bf7e0e544e9b59a88ec6cf89861f0c5f6358aa39e4e6a0d8be1addd8d3512;
235324d36 TIME LIMITED20.124271626991685/TIME LIMITED30.036559015978128,
4d819e719475de04eff17f955e64b06a5f32611a601d6b8a1d061454bac5a7a1.
Three search exact NONNEG full polynomials/h*/sign lists match fresh local,
independent rational reconstruction passed. Three timeouts sign UNKNOWN,
no inherited polynomial/sign evidence. Local beam128/30s each60s outer,
strictly serialized after proposal scan; no concurrent local DB writers.
All seven original quotient/permuted covers, receipt/file/header hashes,
Python3.10.12,x86_64,glibc2.35/libraries,CPU[0,1],8GiB memory,swap0,pids128
verified before ingestion. Installed limits unchanged. Only selected trusted
wrapper/input/engine uploaded; no credentials/database/checkouts.
Input runs/abacus-230740-six-fiveop-b-20260910.json SHA
19f4f374dc4ef47e2a7f20ec3cf32cf964ad05c5c43edd0aa872cc4a1655556d.
Wrapper SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a.
Engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367.
Output SHA 07c6fb6eb2e8423d15503481a9303cbb46655d3dc849a908f6e9f1c6dc5a5956.
Completed ledger44299407770fe9852336ccbc1228f94c8e69091dcf236aee7c1aaf20e9b7a824;
readback prepared235366,receipt235367,validated236902..236908.
Fetch outside Dropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T130429-5919f5be4432-clzpci7y
Host supervisor can collect durable originals by job ID.
Generated input and eight scan/retry reports remain ignored/untracked.


## Five-opposite search and Abacus comparison — 2026-09-10

Codex retains KTT/companion ownership, private profile/model and sole local
MariaDB-writer role. No source changes, new test claims, extra AI workers,
administrative changes, pushes or publication. All local/remote handles terminal.
Goal active; no global blocker. Best negative unchanged: RAW230740/cert230749,
degree33, seven extra opposite equalities/five internal bans, alongside prior
leaders. These are presentation counts, not a proved minimum nonflag distance.

Started RAW230740 lower-remove-two: add a lower flag and remove two opposite
equalities. Total1531 proposals, cap five extra opposites/degree36, beam32,
5s/case,45s internal batch,60s outer timeout,nice10,CARGO_TARGET_DIR.
All proposals/outcomes stored in DB; ignored reports remain untracked.
Offsets0->464: 0exact,6timeouts,43dimension prunes,410objective prunes,5duplicates,
39.00078050198499s, run
140ce26a3e7871cb90441cb65908c2bda4a080281149c068f806a843f81f022a.
Offsets464->642: 2exact NONNEG,6timeouts,26dimension prunes,140objective prunes,
4duplicates,41.25946056691464s, run
cf2ec98f617eab3b9a71c79c7988b16d1e63acb29da7e85d4059ee9c3d31ea48.
Resume at642, not a guessed requested-count boundary.
After the retries below: CURRENT7exact NONNEG,7unknown,69dimension prunes,
550objective prunes,9duplicates across642 proposals. No whole-range positivity
claim; pruned cases uncounted. Next six initial timeouts:
235082d35,235088d34,235314d36,235318d36,235324d36,235346d34.
These have not yet received wider retries or remote submissions.

First six timeouts coordinate-audited against older222055/203068/209610
lower-remove-two neighborhoods: pairwise distinct, no actual GT-coordinate
tuple matches in that scope. Not a global isomorphism claim.
Audit7b430459753627fefb59e3c8f5d3c502adf3568b5517b9059da01b12f0badc6b,
six mapping rows, completed, fresh_count=false.

Abacus20260910T125232-868f036ae0b6 done/exit0, wall107.04129672050476s.
Seven serial cases including fresh control,20s/case/180s whole; established
finite bound monitored by same job ID, no resubmission. Only selected trusted
wrapper/input/engine uploaded, no credentials/database/checkouts.
Control196437 freshlocal0.228103491012007/remote0.40476656702230684s full match.
Search source,degree,remote/local seconds,local run:
234432d34 exact14.951545223972062/exact8.32634807692375,
6c244f49c641af37be752950f939d99e1e90c5d2fa52c8cee828305249f2bc10;
234812d34 exact13.751884319004603/exact7.865094390930608,
7a4ddc105a3a2c889a5c1cb5b381caf0616d91b1c1478c55b5a065e768463ed4;
234418d35 exact17.317840076982975/exact9.489993573050015,
6d7cfb8ad21af7faa7df40600aac7326cb72898a7de4a72b0995673cc4a7a1b4;
234422d35 TIME LIMITED20.116518913011532/exact26.82099322497379,
8c1e6768aa785aafa2c466c6d9f2c458339320502f6dafd2af453e1a156a6121;
234798d35 TIME LIMITED20.10704182900372/exact13.96827619196847,
64b2779bea7563a47f16d77079b73d5beea6229e1183df498dc87991105426bf;
234802d35 TIME LIMITED20.12200795300305/TIME LIMITED30.014485885039903,
963823f7875bad27547f6f9e7cd7a42dcab6b653f6ca604a941f466e81d34732.
Five local exact results NONNEGATIVE;234802 sign UNKNOWN. Do not repeat
unchanged20s remote/30s local identities. Remote timeouts retained separately.
Local beam128/30s each60s outer, strictly serial after scan completion.
All five local exact polynomials independently reconstructed over Q from h*;
all three remote search exact polynomials/h*/sign lists match fresh local,
as does the control. All seven original quotient/permuted covers validated.
Receipt/file/header hashes, Python3.10.12,x86_64,glibc2.35/libraries,
CPU[0,1],8GiB memory,swap0,pids128 verified before DB ingestion; limits unchanged.
Input runs/abacus-230740-six-fiveop-20260910.json SHA
315258c8118b22d48b22229dbc921f84a88ef3efd49f841fbdbc0a076eb26946.
Wrapper SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a.
Engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367.
Output SHA 1c5d424d8a3877354c2fda02e3022127e9aea807f67782a16288451387e10edd.
Completed ledger c71199c55f483445c1c6f143d6911680dc0b8194a89fb3e1aefdf66faf38842c;
readback prepared234989,receipt234990,validated235359..235365 (actual DB IDs).
Fetch outside Dropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T125232-868f036ae0b6-_2s8huje
Host supervisor can collect durable originals by job ID.
Next: audit/submit next disjoint five-opposite batch to Abacus; resume local
proposal scan at642. Continue prioritizing fewer nonflag constraints.


## Joint coordinate/layout audit; first exact joint result — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,solelocal DB writer.
Previous turn progress; freshclean Git/DB/remote-idle checks.
No source edits,testclaims,extraAIworkers,admin/toolchain changes,push/publication.

Audited ALL36unresolved230740joint cases by actualGT-coordinate tuples,
compared to attemptedjoint cases from222055/203068/198861/209610.
All36pairwisedistinct,no comparisonmatches. Stored full verifiedbeam32/128
permutations,frontierprofiles,cost(maxfrontier,sumfrontier),coordinatehashes:
7c6c86b5fe7ecc9d48296891a99e51dce0fcc2a94980daa9e0fb6e212b322ccd,
3.976306793978438s,36layout_audit rows,completed,fresh_count=false.
Scope is coordinateequivalence,notabstract poset-isomorphism.
Topdegree39case232482 edge[29,132,113],bothlayouts cost(5,153).
Next232198[10,82,113] beam32(7,121)->128(6,122);
232408[29,82,113] (6,154)->(6,122);
232218[10,98,113] (7,155)->(6,123);
232428[29,98,113] (6,151)->(6,123).
Otherprofiles inDB. Rank is a computational heuristic,not sign evidence.

Abacus job20260910T124601-2d205683de6e done/exit0,wall82.7988164s.
4serialcases includingfreshcontrol,30s/case/150swhole. Larger finite
casebudget justified by auditedlayout/degree39 and testedwrapper support;
jobcap covers4x30plusstartup. Monitoredsamejob,no resubmission.
Control196437 freshlocal0.22994855500292033/remote0.40020246102358215s fullmatch.
232482d39 exactNONNEG remote21.041463179979473/local11.29968491604086s,
e1b92da2a63e9cb0fc2cb63641e9b09b3f129838b1bd20558dcd2a936b50b0e6.
232198d39 remoteTIME LIMITED30.07939386999351/localTIME LIMITED30.076924331951886s,
2acfe922d33802dae437f2f8e57f0905127104bfa6d1f58238ed0195239fad22.
232408d39 remoteTIME LIMITED30.17331276499317/localTIME LIMITED30.080410107970238s,
3512153ad657b5bc6a4d542462202538b64d8942908754124aa449b9e4cafeb7.
Lasttwo signsUNKNOWN,no inheritedpolynomial/sign evidence; do not repeat
unchangedremote30/local30 identities. Original5s attempts retained.
Localbeam128/30s each60souter/nice10/CARGO_TARGET_DIR,strictlyserialized.
Bothremote exact polynomials/h*/signlists matchfreshlocal,independent Q
reconstruction passes. All4quotients/permutedcovers andreceipt/file/header
hashes verified beforeingestion. CPU[0,1],8GiBmemory,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/libraries verified; installedlimits unchanged.
Onlytrustedwrapper/input/engine uploaded,no credentials/DB/checkouts.
Input SHA8bb6229befd14826e5fa7719be1d42a2e688ee34433a824caf35167944a27e4a
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHAd2f8b8493f7e82b9554bdeb0627b4c05c5c1081b67a12774874e06063ba34430
Completedledger0ccffc32bbbc6de8c253bb6935ba1c55662172fc1304277cf2ee4a64a0e83b13,
readback prepared234043,receipt234044,validated234051..234054.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T124601-2d205683de6e-ax84cjnf
Hostsupervisor cancollect durable originals byjobID.
CURRENT230740joint1exactNONNEGATIVE,35unknown,279dimprunes630objectiveprunes.
No wholequeue positivity claim. Sixextra/fourinternal remainsunachieved
withnegative coefficients. Paired10unknown remainsunchanged.
Ignored runs/abacus-230740-three-joint-20260910.json and3retryJSONL untracked.
Allhandles terminal. Next: newjointlayout-ranked cases or new lower-remove-two/
full-content mutations of230740; no need repeat identical exhausted attempts.
No improvednegative;goalactive,noglobalblocker. RetainRAW230740/cert230749
d33 sevenextra/fiveinternal alongsidepriorleaders; presentationcounts not
provenminimum nonflagdistance.



## Degree34 paired gaps resolved; joint proposal pass complete — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,solelocal DB writer.
Previous turn progress; freshclean Git/DB/remote-idle checks.
No source edits,testclaims,extraAIworkers,admin/toolchain changes,push/publication.

Abacus job20260910T123800-8fd5cccdc62b done/exit0,wall100.98947s.
8serialcases includingfreshcontrol,20s/case/180swhole,establishedfinitebound,
monitoredsamejob,no resubmission. Selected7degree34pairedsources had no prior
retry/remote identities; actual GT-coordinate tuples pairwisedistinct and
absent from222055/203068/198861/209610/174551 paired and230740 lower-remove
neighborhoods. Scope inpreparedledger,notglobal non-isomorphism.
Control196437 freshlocal0.22619666706304997/remote0.3990145970019512s fullmatch.
All7search exactNONNEG remote/local seconds/localrun:
23176613.917968605994247/8.050471887923777,
090c29aad4500c6fb9481be0571026b2d81a7d626604f7cadaee011698cac183;
23180814.06758540699957/8.120813749032095,
cc79808b13a32681e67e54a778c455fcc3681f0af4208911de85112e15fda764;
23185014.010850257007405/8.122453631018288,
31e3629fbfdf6c21e15c4eed70f45932e1207ab1c9f387027328d493f6c7e2f5;
23189214.100559440004872/8.14052780799102,
fe531fc7ef0ebe32d5b132eeb10dda6192d162cf7aed22984e6173fb0ec6f96e;
23193414.307148419000441/8.194831228000112,
9ff445796dfa2fa8b010b173e1cfd8738ac8c0c2b8f1609979f22078b73955d3;
23197614.179692003992386/8.114618432940915,
1734f18ad8e52ea3638cfd2b1707d98f3686c93f0fc3c409d4d77b4e49e6fd95;
23201814.023839711007895/8.157746737007983,
30d5019c9910c2423956a46f1608e2789e948ae0a74e95c5e7365360d289192f.
Localbeam128/30s each60souter/nice10/CARGO_TARGET_DIR,strictlyserialized
after jointscan terminal. All8remote fullpolynomials/h*/signlists match
freshlocal and independently reconstructed overQ.
All8quotients/permutedcovers andreceipt/file/header hashes verified before
localingestion. CPU[0,1],8GiBmemory,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/libraries verified; installedlimits unchanged.
Onlytrustedwrapper/input/engine uploaded,no credentials/DB/checkouts.
A bounded live-log preview began midJSON,causing summaryparsererror;
samejob status inspected,then fullbundle fetched. Not a failedjob or
connection-triggered resubmission. Fulloutput parses/validates normally.
Input SHA82ad41a6401a3f255c439c4f8b311b85707b08944620854346e25624f0b29196
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHAec1dcd5d5657bed778c08065d463f7be2f596fdc2c705d601f987c300deb1b11
Completedledger6b62a499cf913e0a25d1f57933aa872daa28b59b4422e55d79bbc16dd7d8efb3,
readback prepared233275,receipt233276,validated233999..234006.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T123800-8fd5cccdc62b-355_mdyx
Hostsupervisor cancollect durable originals byjobID.
CURRENT230740 paired26exactNONNEGATIVE,10unknown,
27dimprunes125objectiveprunes1duplicate. No wholequeue positivity claim.

230740 joint finished591->731->926->945 (ALL945),
samecap6extra/d40,beam32/5s,45sbatch/60souter,nice10/CARGO_TARGET_DIR,
39.062616651994176/40.45273305103183/5.620470116962679s,runs
de3d582c287f180bf2e75ee77bf63549c70de9cec630705b0808f74a9651bc67
809b2848da770a2be7f17a9ddc7c68ac46ed9654eae408fe969470be55e31974
1f35e0e6e014a5c83263f20bf2ca60d28551ad99f86d11c510b499379a6fb0bc.
CURRENTwholejointqueue36timeouts UNKNOWN,279dimprunes630objectiveprunes,
0exact/negative/duplicates. Allproposals/outcomes DBstored.
Completed proposalenumeration does NOT resolve any timedout sign.
Next: coordinate/layout audit these36six-extra/four-internal children,
prioritize tractable distinct cases forAbacus;10pairedgaps alsoavailable.
Ignored runs/abacus-230740-seven-paired-d-20260910.json,7retryJSONL and
3jointreports stayuntracked. Allhandles terminal. No improvednegative;
goalactive,noglobalblocker. RetainRAW230740/cert230749 d33 sevenextra/
fiveinternal alongsidepriorleaders; counts not provenminimum nonflagdistance.



## Seven paired gaps resolved; joint search at591 — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,solelocal DB writer.
Previous turn progress; freshclean Git/DB/remote-idle checks.
No source edits,testclaims,extraAIworkers,admin/toolchain changes,push/publication.

Abacus job20260910T123016-7e195e419eb2 done/exit0,wall111.0504465s.
8serialcases includingfreshcontrol,20s/case/180swhole,establishedfinitebound,
monitoredsamejob,no resubmission. Selected7degree33pairedsources had no prior
retry/remote identities; actual GT-coordinate tuples pairwisedistinct and
absent from222055/203068/198861/209610/174551 paired and230740 lower-remove
neighborhoods. Scope inpreparedledger,notglobal non-isomorphism.
Control196437 freshlocal0.2284415200119838/remote0.399415247986326s fullmatch.
All7local search exactNONNEG,remote/local seconds/localrun:
231902 remoteTIME LIMITED20.103281552001135/local22.93084185605403,
de985072770217d8dd96d26595931657f1ed2d3826709573836762e25cb54736;
231938 remoteexact9.88276573800249/local5.595812654006295,
5cb629e8307156e1d6332186a5257e33d9a7016b4a91df7a350bd61ca2811a1b;
231944 remoteTIME LIMITED20.11795977401198/local22.751084271003492,
f9739008e993da0fbba6009f32f4ae28c162bbcd62a9e496dbe09d54420a37ec;
231980 remoteexact9.838470154005336/local5.6384517619153485,
d3362555d407b29bfa246f1fa43dd6d26c2de5b8e049a01dea65bafc0f59172b;
231986 remoteTIME LIMITED20.114900468994165/local22.868224987993017,
868564d85390e37f207b3d06c938511568e123b730c26a5d5377345c57b070bc;
232022 remoteexact9.91034348300309/local5.5607383869355544,
14a1c1004e2e912b77e4de61a41c6b9e3cfe77d73a1a5a021782eca31d7bd0a7;
232028 remoteTIME LIMITED20.111134812992532/local22.927193508017808,
2fd675630a04a6f89a920b96f0174d7344284452c31d8000ff17fd2733ee984c.
Allremote timeouts retained distinctly from localexact.
Localbeam128/30s each60souter/nice10/CARGO_TARGET_DIR,strictlyserialized
after jointscan terminal. All7localexact independently reconstructed overQ;
3remote search exact fullpoly/h*/signlists matchfreshlocal,controltoo.
All8quotients/permutedcovers andreceipt/file/header hashes verified before
localingestion. CPU[0,1],8GiBmemory,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/libraries verified; installedlimits unchanged.
Onlytrustedwrapper/input/engine uploaded,no credentials/DB/checkouts.
Input SHA4808cbddcb10984c76739b06797286d5ad59912ea28a83176581a24900ba6f84
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA878aa7642e5e2d277990a6baa8d8fdb340ea1ac793ef0c3fffb8260893cf1124
Completedledger9ae0ea65158e2aa834a230b3febff5dacb48e73c01762e298425c37b10e26b0d,
readback prepared232451,receipt232452,validated233267..233274.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T123016-7e195e419eb2-y4eoldsq
Hostsupervisor cancollect durable originals byjobID.
CURRENT230740 paired19exactNONNEGATIVE,17unknown,
27dimprunes125objectiveprunes1duplicate. No wholequeue positivity claim.

230740 joint resumed191->391->591 (945total),samecap6extra/d40,
beam32/5s,45sbatch/60souter,nice10/CARGO_TARGET_DIR,
40.55927118496038/40.53240014205221s,runs
46d66332520d6de2edf04a66c87a4bd2af9d73754c4a52e3460f60bf084dc686
7619469a96a644bb81bd1f4afaf1b6941aa18f71bb734b5f686e07e2e9baa0b5.
Eachnewbatch7timeouts,58dimprunes135objectiveprunes,0exact/duplicates.
CURRENTfirst591:21timeouts UNKNOWN,170dimprunes400objectiveprunes.
Allproposals/outcomes DBstored. Capped,notcomplete.
NEXT resume --offset591 --count1000 withsamebounds;354proposals remain.
Ignored runs/abacus-230740-seven-paired-c-20260910.json,7retryJSONL and
2jointreports stayuntracked. Allhandles terminal. No improvednegative;
goalactive,noglobalblocker. RetainRAW230740/cert230749 d33 sevenextra/
fiveinternal alongsidepriorleaders; counts not provenminimum nonflagdistance.



## Six paired gaps resolved; joint mutations started — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,solelocal DB writer.
Previous turn progress; freshclean Git/DB/remote-idle checks.
No source edits,testclaims,extraAIworkers,admin/toolchain changes,push/publication.

Abacus job20260910T122330-28c784053ad0 done/exit0,wall101.001177s.
8serialcases includingfreshcontrol,20s/case/180swhole,establishedfinitebound,
monitoredsamejob,no resubmission. Selected7degree33pairedsources had no prior
retry/remote identities; actual GT-coordinate tuples pairwisedistinct and
absent from222055/203068/198861/209610/174551 paired and230740 lower-remove
neighborhoods. Scope recorded inpreparedledger,notglobal isomorphismclaim.
Control196437 freshlocal0.22887185600120574/remote0.4071688689873554s fullmatch.
Remote/local seconds and localrun:
231770 exactNONNEG9.842088875011541/5.756254021078348,
e189acbe7f73fe10ceecfc210be7e697379427e89ac1b2f9002fdb4f2731813c;
231776 remoteTIME LIMITED20.08599460098776/localexactNONNEG23.102830627001822,
dc7b79d12335aa6f2b5e2e8c4da63a16b7f1bfb80081d86469943a6ecb3c7795;
231812 exactNONNEG9.817727840010775/5.591686210012995,
17cd02830814b819e6f8bb502b2e9d479ec67bedbeffa364b5c77edbbd54ad7e;
231818 remoteTIME LIMITED20.143342800001847/localTIME LIMITED30.028323589940555,
b8a173c55d3303d3d56e99911198572acafbd2c793ace6233d431dfce0fef48b;
231854 exactNONNEG9.791092183993896/5.608916108030826,
c53f9307435cb42f961b985132fbba109038a7559bab938bb677c5fb271b2d4e;
231860 remoteTIME LIMITED20.11842007600353/localexactNONNEG22.770925616961904,
1d21ca53d54275dbd5ff1121cfa0d07b44f8e9bfaac70723b839c128fa058e26;
231896 exactNONNEG9.84016048599733/5.581804370973259,
41a185efd83d9ae70e9a76dc175057ea36bd092be8adffa3e092c5afc2f4e2db.
231818signUNKNOWN,no inherited polynomial/sign evidence; do not repeat
unchangedremote20/local30 identities. Otherremote timeouts remain distinct.
Localbeam128/30s each60souter/nice10/CARGO_TARGET_DIR,strictlyserialized
after localjoint handle terminal. All6localexact independently reconstructed
overQ; 4remote search exact fullpoly/h*/signlists matchfreshlocal,controltoo.
All8quotients/permutedcovers andreceipt/file/header hashes verified before
localingestion. CPU[0,1],8GiBmemory,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/libraries verified; installedlimits unchanged.
Onlytrustedwrapper/input/engine uploaded,no credentials/DB/checkouts.
Input SHA6fe24044480bb3a69a43bd4ff9af27c74a335b5b2ff305c8c35785646ba340e4
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA4b8c2ac57fbdeb1546990d373beba965e5d328ceccd8cf51e7378f2c308e3533
Completedledgerd6c26d9077554e68ce3eef465f87d705f4d59b5ba8bd59c7f2040cdaa5c7f64f,
readback prepared232045,receipt232046,validated232443..232450.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T122330-28c784053ad0-bxuhy6dz
Hostsupervisor cancollect durable originals byjobID.
CURRENT230740 paired12exactNONNEGATIVE,24unknown,
27dimprunes125objectiveprunes1duplicate. No positivity claim forwholequeue.

New230740 joint(boundaryflag+removeopposite+removeinternal) first191of945:
cap6extra/d40,beam32/5s,45sbatch/60souter,nice10/CARGO_TARGET_DIR,
40.32499490806367s,run
20b714e7402dafc6fe84c61d4f23d5b589cc77457f9217893d058005609ba0a9.
7timeouts UNKNOWN,54dimprunes130objectiveprunes,0exact/duplicates/negative.
Allproposals/outcomes DBstored. Statuscapped,notcomplete.
NEXT resume --offset191 --count1000 withsamebounds;754proposals remain.
Ignored runs/abacus-230740-seven-paired-b-20260910.json,7retryJSONL and
runs/fan230740-joint-d40-offset0-20260910.jsonl stayuntracked.
Allhandles terminal. No improvednegative;goalactive,noglobalblocker.
RetainRAW230740/cert230749 d33 sevenextra/fiveinternal alongsidepriorleaders;
presentation counts not provenminimum nonflagdistance.



## Paired proposal pass completed; first Abacus batch validated — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,solelocal DB writer.
Previous turn progress; freshclean Git/DB/remote-idle checks.
No source edits,testclaims,extraAIworkers,admin/toolchain changes,push/publication.

230740 paired ALL189proposals processed,cap6extra/d38,beam32/5s,
45sbatch/60souter,nice10/CARGO_TARGET_DIR. Resumed previousoffset56.
Newoffsets56->98->140->182->189,times41.15133439190686/
41.13813547999598/41.158707084017806/15.236805035965517s,runs:
fe4b40b2afbead016c53d33de12cd4447d9d4699bdfa23002c619b0bc1ded1e2
f73028a06575ff16fbaf0c62e04ed153203d82d153a219433cc93decd43a87ec
45bb68c4611a747dc886a2963b1b02c6db7ac83daac34c59258968f4a33bebee
e17026940ca0807b0f45961e71be9692e80c9597f1cc3f887d73e43a1c0d341b.
Initialwholequeue2exactNONNEG,34timeouts,27dimprunes125objectiveprunes,
1maskduplicate. No new exact/negative in these resumed slots; alloutcomes DB.
CURRENT after4verifiedretries:6exactNONNEGATIVE,30unknown,
27dimprunes125objectiveprunes1duplicate. No-range positivity conclusion.
Freshfirst10unknowns sorted dimension:
231770/231776/231812/231818/231854/231860/231896/231902/231938/231944 d33.
Otherunknowns inDB; lowerdimensions suitable nextremote batch.

Coordinateauditc11f17ee1aa5d9a57c7e807c9fe7e0395ab572eaca240e942b3a6db4520c6536:
first7pairedtimeout faces pairwisedistinct; noactualGT-coordinate matches in
222055/203068/198861/209610/174551 paired or230740 lower-remove neighborhoods.
Comparison mappings stored; notglobal non-isomorphism claim.
Abacus job20260910T121519-5db6e7bb6a52 done/exit0,wall109.0132837s.
8serialcases includingfreshcontrol,20s/case/180swhole,establishedfinitebound,
monitoredsamejob,no resubmission. Control196437 freshlocal0.22983030590694398/
remote0.40949617998558097s full exact match.
4search exactNONNEG remote/local seconds/localrun:
231672d33 9.776972846011631/5.4103824709309265,
1dde151aad5b17e1061f9de7ea2683a15d55cc4125209621173a853592bbafa1;
231714d33 9.857143128989264/5.6085836609127,
6d018f34addb2df8d74e88c32e7f62ed9f75dffe90e622537df6e5e40c96c2ba;
231668d34 13.772663254989311/7.955253310035914,
5cdcbff0021e364aa602f76c5679f7a4f874f4ce1fadd0103a92c566d4b59956;
231710d34 14.233264759997837/8.374426964088343,
3a1db1b733e18df5c5631be1643d41bf3c5a6d7263e57ab54f6948caecf01d99.
Remote3degree36 TIME LIMITED,signUNKNOWN:
23166420.123130988009507s,23170620.105654946994036s,
23174820.099715323012788s. No localwiderretry for these3 thisturn.
Do not repeat unchangedremote20 identities.
Local4retriesbeam128/30s each60souter,strictlyserial after proposalpass terminal.
All5remote exactfullpolynomial/h*/signlists match freshlocal and independently
reconstructed overQ. All8quotients/permutedcovers andreceipt/file/header
hashes verified beforeingestion. CPU[0,1],8GiBmemory,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/libraries verified; installedlimits unchanged.
Onlytrustedwrapper/input/engine uploaded,no credentials/DB/checkouts.
Input SHAf19a6a27cb95ac4a617e4ce93d80582f2436234055f66d74eeb5bd6f15c7de36
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA31aecc49d3cad633db7aa269a2a2b43ce8a1db3692eac1161f911f1bb5490829
Completedledger49ae08b86ccd488ad38f757b177dbf876a82470f87b75bbf3b7773c9bdf2ed4a,
readback prepared231761,receipt231762,validated232037..232044.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T121519-5db6e7bb6a52-euuymecq
Hostsupervisor cancollect durable originals byjobID.
Ignored runs/abacus-230740-seven-paired-20260910.json,4resumedproposal
reports and4retryJSONL stayuntracked. Allhandles terminal.
Next: coordinate-check disjoint paired gaps forAbacus ornewjoint/content
mutations. No improvednegative;goalactive,noglobalblocker.
RetainRAW230740/cert230749 d33 sevenextra/fiveinternal alongsidepriorleaders;
presentation counts not provenminimum nonflagdistance.



## Weight gaps closed; paired mutations started — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,solelocal DB writer.
Previous turn progress; freshclean Git/DB/remote-idle checks.
No code edits,testclaims,extraAIworkers,admin/toolchain changes,push/publication.

Coordinate audit2fbc588752ed6c9630a651e60c0f58831d3f5c001e9e96fe0237acb5de6e1f85:
fourdegree32weight gaps231585/231587/231589/231591 pairwisedistinct,
noactualGT-coordinate matches in222055/203068/198861/209610/174551
zero-weight neighborhoods. Allmappings stored; notglobal non-isomorphism.
FreshDBchecks found no previous widerretry/remotebundle identities.

Abacus job20260910T121019-999fedc1b9fc done/exit0,wall36.3522456s.
5serialcases includingcontrol,20s/case/120swhole,establishedfinitebound,
monitoredsamejob,no resubmission. Control196437 freshlocal0.22794421890284866/
remote0.4015636319818441s full exact match.
All4 searchcases exactNONNEG remotely/locally,seconds/localrun:
231585 8.859161574015161/5.1592495259828866,
97fa2394cefbda1a90cc34d55a2c5655492d25c7b43b5300dc03e04411faf328;
231587 8.926782903989078/5.146019064006396,
48f83d9c160bda81cd7cbcc43fcb4b80f40402110f0e4a4a85e691b4d8e9b096;
231589 8.923096353013534/5.120095483958721,
7c85e3f8d8da73ee87d58b90525dcfa963f3aad4a132c8231949db9f82a3db60;
231591 8.957433506002417/5.236183295957744,
38a95eaa230d0b371c3f541d61100e3927d66e870ae52e47edbaacd17a660e56.
Localbeam128/30s each60souter/nice10/CARGO_TARGET_DIR,strictlyserialized.
All5remote fullpolynomials/h*/signlists match freshlocal; independent Q
reconstruction and alloriginalquotients/permutedcovers verified beforeingestion.
CPU[0,1],8GiBmemory,swap0,pids128,x86_64,Python3.10.12/glibc2.35/libraries
verified; installedlimits unchanged. Onlytrustedwrapper/input/engine
transferred,noDB/checkouts/secrets.
Input SHA7e1517db740a43b7e4051666a617d63d77fe584f9ee435c4de914a8d0ff2969d
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHAc6972d14611d729c18af2b96b7b9c755395cb57640aedf1191f020133c93b5ec
Receipt/file/header hashes checked. Completedledger
8ded363094130e7dd98c942d09bbc6a72197a85c4124d074d310935fb001cbe1,
readback prepared231627,receipt231628,validated231749..231753.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T121019-999fedc1b9fc-c86uis6s
Hostsupervisor cancollect durable originals byjobID.
CURRENT230740 zero-weight22exact NONNEGATIVE,zeroattemptedholes,2maskduplicates.
No newnegative weightchild. Originaltimeout rows retained.

New230740 paired(addboundaryflag+removeopposite) first56of189 proposals:
cap6extra/d38,beam32/5s,45sbatch/60souter,nice10/CARGO_TARGET_DIR,
39.43396829604171s,run
e8c97218e436eea41fa5a2ca3ed3ec1e318ce0be6ba5d14e08514595d7dd7dab.
2exact NONNEGATIVE independently reconstructed overQ,7timeouts UNKNOWN,
9dimprunes38objectiveprunes,0duplicates/empty/errors. Alloutcomes DBstored.
Status capped,not complete. NEXT resume --offset56 --count200 withsamebounds.
133proposals remain; do not restartoffset0 or callrange certified.
Ignored runs/abacus-230740-four-weights-20260910.json,4retryJSONL and
runs/fan230740-paired-d38-offset0-20260910.jsonl stayuntracked.
Allhandles terminal. No improvednegative;goalactive,noglobalblocker.
RetainRAW230740/cert230749 d33 sevenextra/fiveinternal alongsidepriorleaders;
presentation counts not provenminimum nonflagdistance.



## Shrink attempted gaps closed with Abacus; weight mutations — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,solelocal DB writer.
Previous turn progress; freshclean Git/DB/remote-idle checks.
No code edits,testclaims,extraAIworkers,admin/toolchain changes,push/publication.

Actual-coordinate audit of7shrink gaps against222055/203068/198861/209610/
174551 interior-shrink neighborhoods:
5feb1c0f4b44d9e6dbdfc6bfa48ec46dd2b8bba35e1b239be682e1c6392909d8.
231285 equals212574,an original5stimeout with no wider retry. Other6distinct
within selectedbatch and absent from comparison scope. All mappings DBstored.
231285 excluded fromremote,then retried ONCE locally atnewbeam128/30budget:
exactNONNEG5.8756941159954295s,
cd72a7f929d3e4057e7fbaa3ff21470578a4b108908a7c7ead8f5ea5ce37c891.
Independent Q reconstruction passed. Do not separately redo212574 unchanged;
coordinate audit supplies identity to fresh231285 result.

Abacus job20260910T120346-ded693b6aa09 done/exit0,wall113.0280698s.
7serialcases includingcontrol,20s/case/180swhole,established finitebound,
monitoredsamejob,no resubmission. Control196437 freshlocal0.2357090211007744/
remote0.40620347901131026s exactfullmatch.
All6local search retries exactNONNEG,remote/local seconds:
231523d31 exact14.548512218985707/7.823370718979277,
0f2c4eed9e21723806090753044bc7798320cbd068facc6154277f0c8d4b006f;
231493d32 exact18.671159974997863/10.716887788032182,
68e9a39e60eaee1814dc529afe2cb8e893dbd3a77b1773af7fb583b5c8f684f1;
231315d33 remoteTIME LIMITED20.104724737990182/localexact12.699588811956346,
3e88caeeee93aaae8842f06b1ca150288fe0d8b0a8c9278339b14e9f3b846429;
231345d33 remoteTIME LIMITED20.01321775201359/localexact11.764757750905119,
48b2e4f7e641d8a83bb95499156b9605601cef9324c0c8c3d5b570ef5ee7feeb;
231387d33 remoteTIME LIMITED20.112126142979832/localexact12.703126516076736,
c5e43fc6e6f98aeefa7aa2a1aae89821a4d6880417d40b7dbb6221ebc2149ace;
231463d33 exact18.425219732976984/9.873912398004904,
7addfe1218353e11ef37ef71c16d6e727ef638fc62537207fd9a5d545725c82d.
Localbeam128/30s each60souter/nice10/CARGO_TARGET_DIR,strictlyserialized
after localweightscan terminal. All6freshlocal exact independently
reconstructed overQ. All3remote search exact fullpolynomials/h*/signlists
match freshlocal,controltoo; remote timeouts kept separate fromlocalexact.
All7quotients/permutedcovers and receipt/file/header hashes verified before
localingestion. CPU[0,1],8GiBmemory,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/libraries verified; installedlimits unchanged.
Onlytrustedwrapper/input/engine uploaded,no credentials/DB/checkouts.
Input SHA74f99e0e2d99307498ffb265196885c47f6391356af735992b657ca654bef4ad
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA002b89e28e5af76d94bdc89af7ce52f9b451e3def4099c6d6f2c21b5b6ad3490
Completedledgere09484aeba3a39f3ea30a81dd804b9100e2ada4e6e357e74b8df814442f7d797,
readback prepared231552,receipt231553,validated231614..231620.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T120346-ded693b6aa09-1y3p330e
Hostsupervisor cancollect durable originals byjobID.
CURRENT230740 interior-shrink82exact NONNEGATIVE,zeroattemptedholes,
21dimprunes8objectiveprunes20maskduplicates (131total). FreshDBanti-join0.
Prunes/duplicates not a positivity certificate or larger-range conclusion.

230740 zero-weight ALL24labels completed,dimensioncap33,no flag-only
objectivefilter,beam32/5s,45sbatch/60souter.
Offsets0->23->24,times42.56119466701057/0.03301770007237792s,runs
fe9fc2c600749229e602a440662f56c78d89ea61351f083e45d9b018a337d441
15cb886fc07d7d0b884976f68227a96316f2ecac5616227114a91b78eba81870.
18exact NONNEGATIVE independently reconstructed overQ,4timeouts UNKNOWN,
2maskduplicates,no prunes. Unretried d32 sources231585/231587/231589/231591
for zero-weight labels16/17/18/19. Allproposals/outcomes DBstored.
Ignored runs/abacus-230740-six-shrinks-20260910.json,2weightreports,
7retryJSONL stay untracked. Allhandles terminal. No improvednegative,
goalactive,no globalblocker. Next: bounded coordinate-checked weight retries
onAbacus ornew paired/joint/full-content mutations of230740.
RetainRAW230740/cert230749 d33 sevenextra/fiveinternal alongsidepriorleaders;
presentation counts not provenminimum nonflagdistance.



## Four-ban Abacus attempts and interior shrink pass — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,solelocal DB writer.
Previous turn progress; freshclean Git/DB/remote-idle checks.
No code edits,testclaims,extraAIworkers,admin/toolchain changes,push/publication.

Coordinate dedup BEFORE submission caught two repeated four-internal-ban faces:
231176 has actual full GT-coordinate signature equal to222496;
231206 equals210379. Neither counted/remotely resubmitted this turn.
All8source comparison mappings against222055/203068/198861/209610
lower-internal neighborhoods stored under
918c0f26637166789c162c40e80ae0cfc8304658f1d9c6ae206026840a6c057a.
Other6 distinct within batch and absent from that comparison scope; not a
global isomorphism claim. Initial7case preparation assertion stopped before
input creation/submission on231176; only known control had run, no searchcase.
Corrected selected6case input prepared with another fresh timed control.

Abacus job20260910T115716-c3682858f096 done/exit0,wall123.1215182s.
7serialcases includingcontrol,20s/case/180swhole,established finite wrapper,
monitoredsamejob,no resubmission. Control196437 freshlocal0.2280073369620368/
remote0.41584665299160406s exactfullmatch.
All6search TIME LIMITED (sign UNKNOWN,no polynomial evidence):
231166d37 20.114596889994573s;
231136d38 20.139758328994503s;
231146d38 20.14057400499587s;
231156d38 20.106051091017434s;
231186d38 20.131762432982214s;
231216d38 20.132144361996325s.
No local wider retries thisturn; no exactremote searchhit requiring comparison.
Original5s rows retained. Do not repeat unchanged remote20 identities.
Current230740 lower-internal remains8unknown,36dimprunes,1maskduplicate;
2of8unknowns have coordinate matches documented above,not newly certified.

All7originalquotients/permutedcovers,receipt/file/header hashes checked before
localingestion; control fullpoly/h*/signlist matched and independently
reconstructed overQ. CPU[0,1],8GiBmemory,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/resolvedlibraries verified; installedlimits unchanged.
Onlytrustedwrapper/input/engine uploaded,no credentials/DB/checkouts.
Input SHA94d92b44ea2c673b995ac9750d260e5b3e9eeebf0fd9e7233cf71ba074519543
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHAfc76d616efa37d8c7530bb3d06c7ad08051c82d9e9d65d94e8c1af570529d791
Completedledger85a95430e75345386a19131b1e749b7096e07b5847b1d527a9584a2e16ac493e,
readback prepared231274,receipt231275,validated231538..231544.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T115716-c3682858f096-e16qofoe
Hostsupervisor cancollect durable originals byjobID.

230740 interior-shrink ALL131 completed,cap7extra/d33,beam32/5s,
45sbatch/60souter,nice10/CARGO_TARGET_DIR. Strictlyserial localhandles.
Offsets0->33->74->131,times40.750477862893604/39.304696495993994/
37.00338295393158s,runs:
728800ac7fa974b3aa740a7bdeb4a1bdde652d085d615c7f0b1678f1491d8108
4c5b505309aa9030d02f65f9854bdc07682b01d1ed7bc2aa7d742c1b6ad9afac
37ba3d9e05179bb6b522a539187d0090bb1a43e1e4176bdb142a1dd8004b9a92.
75exact NONNEGATIVE,7timeouts UNKNOWN,21dimprunes,8objectiveprunes,
20maskduplicates not revalidated. All75 fresh exact independently
reconstructed overQ. Allproposals/outcomes DBstored.
Unretried5s gaps:231523d31[9,7];231493d32[8,6];
231285d33[1,6],231315d33[2,7],231345d33[3,7],
231387d33[4,13],231463d33[7,6].
These smaller gaps are useful nextAbacus candidates after coordinate checks.
Ignored runs/abacus-230740-six-lowerinternal-20260910.json and3localshrink
JSONL remain untracked. Allhandles terminal. No improvednegative;
goalactive,no globalblocker. RetainRAW230740/cert230749 d33 sevenextra/
fiveinternal alongsidepriorleaders; counts not provenminimum nonflagdistance.



## Six-opposite attempted gaps closed; corner shrinks checked — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,solelocal DB writer.
Previous turn progress; freshclean Git/DB/no-engine/remote-idle checks.
No code edits,testclaims,extraAIworkers,admin/toolchain changes,push/publication.

Abacus job20260910T115210-a2a07c6f80a8 done/exit0,wall28.2857635s.
3serialcases,20s/case/60swhole,monitoredsamejob,no resubmission.
Selectedsources231043d34/230999d35 no prior retries/remote identities.
ActualGT-coordinate tuples distinct from eachother and222055/230106
lower-remove neighborhoods; comparison scope inpreparedledger.
Control196437 freshlocal0.22799777099862695/remote0.4179625229735393s,
full exact match.
231043 exactNONNEG remote14.355611114006024/local8.620243781013414s,
ebd94f7092b037fe5d80f6fb2f615db30f95868f23e596a5660ceb86ae773164;
230999 exactNONNEG remote11.927561752992915/local6.3179653770057485s,
32574bd52e2a7557608e6c65691b6244707c3088530f64496b7b54e78aa24c54.
Localbeam128/30s each60souter/nice10/CARGO_TARGET_DIR,
strictly serialized after corner-scan terminal.
All3remote exact polynomials/h*/signlists match freshlocal and independently
reconstructed overQ. Alloriginalquotients/permutedcovers and
receipt/file/header hashes verified beforelocalingestion.
CPU[0,1],8GiB memory,swap0,pids128,x86_64,Python3.10.12/glibc2.35/libraries
verified,installedlimits unchanged. Onlytrustedwrapper/input/engineuploaded,
noDB/checkouts/secrets.
Input SHA3d536f83f3c330340b33106c942cc42aec35d5f37f7d30ccdfba4ce75694d80c
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA273c5e5bb08170d440c288a1b389f3334569056180a3479a015dd1da9cdbaea7
Completedledgerfaa0f589608a4157a08cc55fe234d2d98ac93f36c33dd0705990a7e7bb2c0e5b,
readback prepared231249,receipt231250,validated231263..231265.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T115210-a2a07c6f80a8-1oz8ek4v
Hostsupervisor cancollect durable remote originals byjobID.

CURRENT230740 lower-remove39exact NONNEGATIVE,zeroattemptedholes,
32dimprunes87objectiveprunes12duplicates (170total). FreshDBanti-join
confirms zero originaltimeouts lacking an exact retry. This does NOT certify
pruned or merely duplicated cases, nor a larger parameter range.

230740 corner-shrink ALL4 completed,cap7extra/d33,beam32/5s,
45sbatch/60souter,nice10,CARGO_TARGET_DIR,1.0952317379415035s:
54b588ea2224645dc51c5c80fefa5c10c41b1086ee5bff7a97ff46d19f9990fc.
3exact NONNEGATIVE independently reconstructed overQ,1maskduplicate,
no prunes/timeouts. Allproposals/outcomes DBstored.
Ignored runs/abacus-230740-finaltwo-lowerremove-20260910.json,
runs/fan230740-corners-d33-20260910.jsonl and2localretryJSONL untracked.
Allhandles terminal. Next: coordinate-checked four-internal-ban children
fromprevioussection onAbacus, or genuinely new interior-shrink/paired/joint/
content neighborhoods of230740. No improvednegative; goalactive,noglobalblocker.
RetainRAW230740/cert230749 d33 sevenextra/fiveinternal alongsidepriorleaders;
presentation counts not provenminimum nonflagdistance. No flaggedwitness.



## Abacus seven-case follow-up; lower-flag and internal neighborhoods — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,sole local DB writer.
Previous turn made progress; fresh clean Git/DB/remote checks. No source edits,
new AIworkers,admin/toolchain changes,limits changes,push or publication.

Abacus job20260910T114445-ee1463700eb9 done/exit0,wall88.817805s.
Established8serialcases including fresh control,20s/case/180swhole finite
bound,monitored samejob,no resubmission. Selected7sources had no prior retry
or remote identities and actual GT-coordinate signatures distinct pairwise
and from222055/230106 lower-remove neighborhoods. Comparison scope recorded
in preparedledger,not a global non-isomorphism claim.
Control196437 freshlocal0.23018697497900575s,
remote0.39758646400878206s exactfullmatch.
Search remote/local seconds,all local exact NONNEGATIVE:
230903d32 12.311902653018478/6.512050368008204,
31f1a3ee70b6b5fbfc96eb6711887e7e9abd30105514e4cca125edbb30dc5249;
230967d33 9.788157823990332/5.537146490998566,
de5de098b1f8fd1f0fc06cf359da14a4889370f7037cc92c377c668f5480d976;
230973d33 10.569637830980355/5.530390658997931,
6ab8860b9415aa4bc923e9748bf673c364959afc2b94beba2893fa430035cfa5;
231003d33 10.249090126017109/6.4621848029783,
dc1bdad865cbce886b15b14190849c3b4633f0344d8e57e3c696372d9151f688;
231047d33 10.110512382001616/6.1407442960189655,
0b4b449c0acf1d93cc8877ca54a183f9a51b1a4e679defdf5d891188feffdd6c;
231053d33 remoteTIME LIMITED20.119436348002637/localexact24.81265000498388,
9f791dad9368a2076f38ca07b113e5dc2a61cb76276e649e28517a05896c9a5c;
230963d34 13.792569932993501/8.885593403945677,
1f4442e2bab26400f32a023b1f21855bcd5a20ae93d5f627cb719f7a029369d1.
All6remote search exact polynomials/h*/signlists match freshlocal; controltoo.
All7local results independently reconstructed overQ,all8quotient/covers
reconstructed and checked beforeingestion. Remote231053 remains timeout,
not assigned the local exact result. Localbeam128/30s,60souter/nice10/
CARGO_TARGET_DIR,strictly serialized after mutation handle terminal.
CPU[0,1],8GiBmemory,swap0,pids128,x86_64,Python3.10.12/glibc2.35/libraries
verified; onlytrustedwrapper/input/engine transferred,no secrets/DB/checkouts.
Input SHA42fd1b6b226b7e6f6a25cbd45a1ef5ec8b6a3187707c691f38177614cbf8eee3
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHAda34a5566ba74c3bfb039d527d4700f11c82482137e54a8600b981c16df49e09
Receipt/file/header hashes verified. Completedledger
dded94c48dc3fa8d52c2166a752f408ed618966de2a4e5441db1d4f07b226d3c,
readback prepared231115,receipt231116,validated231239..231246.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T114445-ee1463700eb9-kjprl3sh
Hostsupervisor cancollect durable remote originals byjobID.
CURRENT230740 lower-remove37exact NONNEGATIVE,2unknown231043d34/230999d35,
32dimprunes87objectiveprunes12duplicates; lasttwo original5s only.

230740 lower-flag completed9proposals,cap7extra/d33,beam32/5s/45sbatch/
60souter,14.383622364024632s:
375699de57c02192951afa9a16a3c7ababd10f7ed4d062e747eb43e705cdb5d8.
Initial6exactNONNEG,1timeout231128d32[6,11],2maskduplicates.
Retry231128beam128/30exactNONNEG5.223243072046898s:
3ae27284affaa981e8db1b6630fc074ca2474cf7b5212240483443517f15168d.
All7exact independently reconstructed overQ; no attempted holes.

230740 lower-internal completed45proposals,cap7extra/d40,beam32/5s,
45sbatch/60souter. Offsets0->41->45,times42.02538759296294/
0.19501323509030044s,runs
cddeb3a7b2ad499384bbe60236f9d1a85bf98595976852fb1d599b97a5a5dfa6
bbe06067504e90ed016cc5392ad370a83f4cdc68f9ced9a7272df7b8240d5be5.
8timeouts UNKNOWN,36dimprunes,1maskduplicate,no fresh exact.
Four-internal-ban timeout sources:
231166d37[4,8,113];
231136/231146/231156/231176/231186/231206/231216d38
edges[1,2,113]/[2,4,113]/[3,6,113]/[5,9,113]/[6,11,113]/[8,14,113]/[9,16,113].
No wider retries yet. Allproposals/outcomes DB stored.
Ignored runs/abacus-230740-seven-lowerremove-20260910.json and local
lowerflag/lowerinternal/retry JSONL remain untracked. Allhandles terminal.
Next: lasttwo lower-remove gaps and coordinate-checked lower-internal retries
on Abacus,plus other new-parent neighborhoods. Bestnegative unchanged,
RAW230740/cert230749 d33 sevenextra/fiveinternal retained with priorleaders.
Counts not proven minimum nonflag distance. No flaggedwitness;goalactive.



## New negative branch: six-opposite neighborhood and Abacus — 2026-09-10

Codex retains KTT/companion ownership,private profile/model and sole local
MariaDB-writer role. Previous turn made progress. Fresh Git/DB/engine/Abacus
checks: clean repos,no live local count,remote queue enabled/idle.
No source edits,test claims,extra AI workers,admin/toolchain changes or pushes.

RAW230740 lower-remove completed all170proposals, <=6extra opposite,
dimension<=35,beam32/5s,45sbatch/60souter,nice10,CARGO_TARGET_DIR.
Offsets0->69->144->170, times40.01173483999446/40.84396925603505/
11.873429293977097s, runs:
1007984ea7dfc72645a845e589e90073e1d9f59b131fdf8981c44ee9f5fd6783
fb82f03f00aa096a46064e75ac5aa1b2d3370ccceeece58e1cfd0f22de3a58ef
e6dece868ad17441430b08d3b606312aeb6a680276db334537853e1262e5840a
Initial24exact NONNEGATIVE,15timeouts,32dimprunes,87objectiveprunes,
12maskduplicates not revalidated. All24 fresh polynomials independently
reconstructed overQ fromh*. All proposals/outcomes DB stored.
Six first-batch timeouts compared by full actual GT-coordinate tuples against
222055/230106 lower-remove neighborhoods: no matches; also pairwise distinct.
Prepared ledger records comparison scope; not a global equivalence claim.

Abacus job20260910T113951-55449901d20f done/exit0,wall56.4993457s.
Seven serial cases including fresh negative control,20s/case/180swhole;
established finite wrapper bound,monitored samejob,no resubmission.
Control196437 local0.2315101649146527s,remote0.4106413570116274s exactfullmatch.
Six search sources all exact NONNEGATIVE,remote/localseconds and localrun:
230793d33 10.44432532801875/5.625037853955291,
2e9629fb317290c688a66c7da33d91b9e7adeded2f89cf6a479b5d73dbb3e10d;
230867d33 4.071743664011592/2.1794164429884404,
872d8baaa6bc924882838896f50a6225e0dfad0c88423a01aa107ed80df3fd4a;
230891d33 1.3020546569896396/0.8528703670017421,
46871489b92c83732bbdb09c08da6c71b1f0127fe4b62b3c3216119f9512a0a9;
230785d34 13.20955664399662/7.720070765004493,
b4a2c699f3e44fab491fe38908559dc2141c0cb12dde22d87e27d74b3c34bb76;
230825d34 14.42082892000326/8.399915206013247,
52a5a6c2b0b9af356ffe57fc89154ced7ece66ad19d514e4987901b18a21871b;
230887d35 12.287446769012604/6.284533647005446,
75db95e8d375aed915e4d6b218efdbf18912c045ef86c4b88467e225cf560658.
Localbeam128/30s each60souter,strictly serialized after mutation terminal.
All7remote exact polynomials/h*/sign lists match fresh local; independent
rational reconstruction,originalquotient/permutedcovers verified beforeingestion.
CPU[0,1],8GiB memory,swap0,pids128,x86_64,Python3.10.12/glibc2.35/libraries
verified; installedlimits unchanged. Only trustedwrapper/input/engine uploaded.
No credentials,DB,checkouts transferred.
Input SHAe710198ac64b92efb36846986bdde2f0b36701a752bb7d307901626ed46271dc
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA609bf863c45194ba0492d480d5e169cc76e560e2f9df3512b8067a8fdf346d21
File/receipt/header hashes checked. Completedledger
7aa5104845fefd6d8e57d107197168a86d38a52ef92bada9df5913c231023088,
readback prepared230892,receipt230893,validated231108..231114.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T113951-55449901d20f-gp8_ou0_
Hostsupervisor cancollect durable originals byjobID.
Ignored input runs/abacus-230740-six-lowerremove-20260910.json,
3localproposal and6retry JSONL stay untracked. Allhandles terminal.
Sample localNI19/RSS216328 and38948KiB,not peaks.

CURRENT230740 lower-remove30exact NONNEGATIVE,9unknown,32dimprunes,
87objectiveprunes12duplicates. Remaining original5s timeouts:
230903d32;230967/230973/231003/231047/231053d33;
230963/231043d34;230999d35. No retries yet for these nine at checkpoint.
Next: coordinate-check and send disjoint subset to Abacus; other mutation
neighborhoods of230740 remain untried. No improved negative,goalactive,
no globalblocker. Retain RAW230740/cert230749 degree33 seven/five alongside
existingleaders; presentation counts not proven minimum nonflag distance.



## Abacus finds a distinct negative branch, degree33 — 2026-09-10

Codex retains KTT/companion ownership,private profile/model,sole local DB-writer
role. Previous turn made progress; fresh Git/DB/remote checks preceded work.
No code edits,extra AIworkers,admin/toolchain changes,push or publication.

NEW retained negative RAW230740 (exact retry of222063), certificate230749:
9x15,degree33,7extra opposite equalities/5internal bans, linear coefficient
-62418826945997/36100888223400. Parent209610 with added lower flag[9,15].
Vertical0x3ff01fe01fc01f801f001e001c00181f1
Raw horizontal0x100080008400840104010407040e040e00
Certified horizontal0x1000800184038407040f041f043e047e00
Fresh beam32/15s-each endpoint certification7.8853830200387165s,run
5cf1879324480709d3a1efdc2a5b23309a2235858a2d3485c04a0ee8d9bf5c67.
Both exact polynomials/h* agree, raw/expanded full-coordinate face unchanged.
Inherited exit_code124 is stale source metadata; fresh status exact and
full polynomial evidence authoritative. Not a flagged Kostka counterexample.
Counts are presentation counts, not proven minimum distance.
Actual coordinate tuple audit distinguishes230740 from174551,222055,203068,
198861,209610;230736 (retry222057) is the SAME face as174551, not a new parent.
Audit84c0349514db162be98452915ca7e2da4844eda05eaf5867011c536c22972b61,
0.04931100702378899s,stored comparisons and coordinate hashes.
230740 coordinate SHA97142bd4e79d98e5a2fb853e853490bd75668d021b3830380186639be7898091.
Use RAW230740 for new mutations, not expanded certificate230749.
Best bad-count/degree unchanged, but a distinct negative branch is available.

Abacus job20260910T113237-3a3c007a8448 done/exit0,wall38.3548287s.
Established serial5case bundle20s/case/120swhole; finite bound justified by
five explicit cases, monitored samejob. Fresh negative control196437
local0.2325494639808312s / remote0.40268018198548816s exactfullmatch.
Four disjoint previously un-retried lower-flag source IDs, all remote exact:
222044d33 NONNEG remote9.36627795000095/local5.334707473986782s,
90f1066b15c93ccc1d1a3e7eef664eb17c967cc6b63fd47c8343880b1dcf1f26;
222057d33 NEG(existing174551) remote9.412345958990045/local5.325248681008816s,
23075e75b5ec213fb19cb1b9ca90650fef90561b82bdcb2240cb8d53fc1f28e1;
222061d32 NONNEG remote9.118045446026372/local5.213911714963615s,
7547476e9e214c7542f67f41e66467d547fed5f45400d158ca7fdab101baf15a;
222063d33 NEG(new230740) remote9.76193462399533/local5.511011519934982s,
db9fa615c740348526d8a1b482843aa26ef913ffb48573e6f8adacb9abc6adb9.
Local retries beam128/30s,60souter/nice10/CARGO_TARGET_DIR, strictly serial
after local mutation handle terminal. Original timeout rows retained.
All5 full remote exact polynomials/h*/sign lists matched freshlocal results,
independent Q reconstruction checked, all reconstructed quotient/permuted
covers verified. Trusted file/receipt/header hashes checked before ingestion.
CPU[0,1],8GiB memory,swap0,pids128,x86_64,Python3.10.12/glibc2.35/libraries
verified; no limits changed or DB/credentials/checkouts transferred.
Input SHAab4730784b4697823f41113cdb8f543c930a8999fbe8790137ec5deeadd6eaa1
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA5d78c632588d363030bede93c7cf7c86f972c49d6f82ff305624130e3ecf28c4
Ledger9e9adbf787723a8477e3ea4c838876be4251cca7b422db0e3ebadce65f64661e
completed,readback prepared230686/receipt230687/validated230742..230746.
Exact local rows230734/230736/230738/230740.
Fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T113237-3a3c007a8448-79bkj6fd
Host supervisor can collect durable remote originals by jobID.

222055 zero-weight: all24labels attempted,dimensioncap33,no flag-only
objective filter,beam32/5s,45sbatch/60souter. Offset0->23->24;
41.46734027995262/0.035580222960561514s,runs
3128a55fa15058b2fa08ecc7f3c35037ee7fc78608bdee1b39b8ceb4299f757b
8f243512e79774a00fc247295067e6bc7d79e5f5860e0093c4004fb5f2c61aa3.
Initial22exact NONNEGATIVE,1timeout230719d32(label16),1maskduplicate.
Retry230719beam128/30s exactNONNEG4.707924253074452s,
cdcd659f942c12da7c1cd47454efdb52fe73daf447631ac25d47097be56657a0.
All23 exact results independently reconstructed overQ; no attempted holes.
Allproposals/outcomes stored. Generated runs/abacus-four-lowerflags-20260910.json,
localretry/zero-weight/certification JSONL remain ignored/untracked.
Allhandles terminal. Goalactive; no global blocker. Next: bounded mutations
from newRAW230740, prioritizing fewer nonflag constraints, with coordinate
deduplication before expensive repeat attempts; keep routine Abacus batches.



## Abacus lower-internal batch; equivalent mutation queue closed — 2026-09-10

Codex retains KTT/companion ownership, private profile/model and sole local
MariaDB-writer role. Previous turn was a status-only no-progress turn; this
turn executed new bounded tests and a coordinate audit. Both repos initially
clean. No extra AI workers or concurrent local counting/DB writers.

RAW230106 lower-remove: completed all156 proposals, targeting <=6 extra
opposite equalities, dimension<=35, beam32, 5s/case, 45s batch/60s outer,
nice10/CARGO_TARGET_DIR=/cargo-target/ai-projects.
Offsets0->67->129->156, times39.73422852600925/43.30629290500656/
27.80065587500576s. Runs:
effa9b4c2ea9ce8000c9cc2662e2167e0e621b425c55876ba7aeb6313263881f
acf86faed05f7f57be653849f53e33ec80155abde935e83f0019f12f3847f095
8d36622e5f4b348570c3122fa00735bee24983a9413c8bb088217d1273a5e2e7
10exact NONNEGATIVE,17 timeouts UNKNOWN,32 dimension prunes,
91 objective prunes,6 mask duplicates not revalidated. All proposals/results
persisted; all10 exact polynomials independently reconstructed over Q from h*.
Each local handle terminal before next launch.

IMPORTANT: actual full GT-coordinate signatures (equality blocks, marked
coordinates, all relations), not merely poset hashes/polynomials, show ALL156
faces match members of RAW222055 lower-remove neighborhood. No new coordinate
faces here. Do not retry this queue as novel work. Audit156 mappings stored:
9efb35dd2fd4b81d6dd27da32cb6095a323c441bc9ee1897d455af554a1652ad
completed,1.627294822013937s, fresh_count=false. Earlier insertion with a
33-character search_kind was rejected before any audit rows; corrected to
fan_neighborhood_coord_audit, no schema change. This audit establishes
equivalence only, not fresh positivity of historical/pruned/timeout rows.
Next structural neighborhoods should undergo coordinate deduplication before
expensive retries. Prefer genuinely different shape/weight or joint mutations.

Abacus job20260910T112538-2213d88be698 done/exit0,
wall143.2157594s. Seven disjoint never-remotely-attempted lower-internal children
of222055, targeting four internal bans (<=7 extra opposite), plus control196437.
Established8-serial-case wrapper,20s/case/180s whole finite bound, monitored
same job; no resubmission. Limits verified CPUs[0,1],memory.max8589934592,
swap0,pids128,Python3.10.12,x86_64,glibc2.35,ldd resolved.
Only selected trusted wrapper/input/maintained engine transferred.
Control fresh local0.23078786802943796s, remote0.410306865000166s,
full exact polynomial/h*/negative list match.
All7 remote results TIME LIMITED, no polynomial/sign evidence:
222416d38 20.121747130004223s;222426d38 20.137573893996887s;
222436d38 20.11572446100763s;222446d38 20.114479479991132s;
222466d38 20.126461156003643s;222496d38 20.12170610501198s;
222478d40 20.087925138999708s.
Do not repeat these unchanged remote20 identities. No automatic local retries
this turn; no remote exact search hit to validate. All seven signs UNKNOWN.
Parent222055 lower-internal remains2exact NONNEGATIVE,8unknown,35dimprunes.

Input SHA2ca6635ae6f3b56983617284c49f7dbf7673b9f407aa00f900f5bb4a161af1ce
Wrapper SHAa4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHAa23755efe121c2faba4ad3d2e845e1077920645a276f2be04076b98fb12de1b7
File/receipt/header hashes, all8 reconstructed quotients/permuted covers,
runtime constraints and independent rational control polynomial verified
before local ingestion. Ledger
a144de473e887c96353aad3548a832051f0ace0ea2a7695b321d33f85cae0aa6
completed; readback prepared230208,receipt230209,validated230522..230529.
Fetch outside Dropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T112538-2213d88be698-lxuc928m
Host supervisor can collect durable remote originals using jobID.
Ignored runs/abacus-lowerinternal-seven-20260910.json and
runs/fan230106-lowerremove-d35-offset{0,67,129}-20260910.jsonl stay untracked.
No code edits/test claims, DB/credentials/checkouts uploaded, admin/toolchain
changes, pushes or publication. All handles terminal.
Best negative unchanged RAW230106/cert230116 (same face222055): d33,
7extra opposite/5internal bans; presentation counts, not proven minimum
nonflag distance. Smaller d30 seven/six lineages retained. No flagged Kostka
counterexample; goal active, no genuine global blocker.



## Abacus degree36-38 batch validated; three lower-remove holes — 2026-09-10

Codex retains private profile/model,sole local DB writer,KTT/companion ownership.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no selected
retries/priorremote bundles. Abacus enabled/idle,limits unchanged.
Job20260910T111221-ee4a64f8b74d done/exit0,wall141.213577s.
Tested wrapper8serialcases,20s each,180s whole;larger finite bound retained
from prior successful batches and monitored samejob. No resubmission.
RuntimeverifiedCPUs[0,1],memory.max8589934592,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/resolvedlibraries. Only trustedwrapper/input/engine sent.
Control196437 freshlocal0.23185715707950294s,remote0.40072691498789936s,
full exact match.

Six-extra-opposite lower-remove children from222055;remote / local seconds:
222168d36 remoteTIME LIMITED20.111862635007128,localexactNONNEG15.965768123976886,
localrunb723413aa9b88a77d73290df937fcb1d31d45f194c302921b6d1632def7fa72a;
222198d36 exactNONNEG18.884157010994386 /10.261116486042738,
localrun28076d3aa975987ac25986013a368595b1359395a863e9a6072d87fd41717908;
222268d36 remoteTIME LIMITED20.105610096012242,localexactNONNEG12.60938212799374,
localrun2782f4473e4f470d9d020cc83b9bc7a14cf65cfc2d13e738106f5b32835da7f7;
222394d36 remoteTIME LIMITED20.106618478981545,localexactNONNEG12.990230366005562,
localrune42df1d544541a240494bc46cc60f28b47012e622a08aa239ad58e5e8c26c234;
222306d37 exactNONNEG19.924571993004065 /10.62312030105386,
localrun6e5d69e3f2449a87e0fe8af7efe67b63359fec5d0ff8f71a7052bb200ab419bd;
222228d38 remoteTIME LIMITED20.111757835024036,localTIME LIMITED30.023189889034256,
localrundeb9b5e1e733f1fa426a5e0e3516c8f899a66b958502adb7a6a4daa5b7177cf4;
222348d38 remoteTIME LIMITED20.129688841989264,localTIME LIMITED30.0773331039818,
localrun9eb79c5d24570eff8f70d8b4a56c29900f3810afd11c5be351a1d84ecacc8b68.
Localbeam128/30s each60souter/nice10,strictly serial fixed7-item shell loop
which waits/checks exit before next case;monitored loophandle99133 terminal0.
Original5s/remote20s retained. 222228/222348 signsUNKNOWN.
All5 localexact polynomials independently reconstructed overQ fromhstar;
2remote search exact fullpoly/hstar/sign lists match freshlocal results.
Originalquotient/uploaded beamcovers/permutations verified all8.
No inherited exact evidence on timeouts.
Lower-remove222055 CURRENT34exact NONNEGATIVE,3unknown,24dimprunes,
105objectiveprunes6duplicates. FreshDBanti-join:222132d36,222228/222348d38.
All original attempted lower-remove cases now have widerlayout/budget attempts.
Do not repeat theirremote20/local30 identities unchanged;new mutations from
simplernegative230106 or disjoint lower-internal batch remain available.

Input SHA 561b9c0c0246da1410c56aa1423d31b1cd24732d449894b92da25e13bd32b31a
Wrapper SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA 9037ec1bac757ca44431f106ebcda8984f51f23c7e2c1b4d1b4985cab0cce06e
Receipt/file/header hashes checked beforeingestion. Ledger
a090d9d8649a0aeac44b70d4d00ff285d3a276f1eae871a2d2e716a34d963ce9
completed;DBreadback prepared230184,receipt230185,validated230200..230207.
Remoteunknowns222168/222268/222394/222228/222348 keptdistinct fromlocalunknowns.
No remoteDBaccess. Dockerfetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T111221-ee4a64f8b74d-scpwcsgb
Remoteoriginals retained;hostsupervisor cancollectjobID above.
Ignored runs/abacus-lowerremove-seven-c-20260910.json and7localretryreports
untracked. Sample localNI19 RSS116372/198728/562240KiB,notpeaks.
Allhandles terminal,no localengine/remotejob remains.
No source edits/test claims,credentials/DB/checkouts transferred,toolchain/admin
changes,extra AIworkers,push/publication. Goalactive,no flagged witness.
Bestnegative same d33 seven/five;equivalent simplerRAW230106/cert230116 retained.

## Abacus degree34-36 batch validated — 2026-09-10

Codex retains private profile/model,sole local DB writer,KTT/companion ownership.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no selected
retries/priorremote bundles. Abacus enabled/idle,limits unchanged.
Job20260910T110609-d6e4a070d3c9 done/exit0,wall100.883434s.
Tested wrapper:8serialcases,20s each,180s whole;finite largerbound justified by
prior successful batch and monitored samejob. No resubmission.
RuntimeverifiedCPUs[0,1],memory.max8589934592,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/resolvedlibraries. Only trustedwrapper/input/engine sent.
Control196437 freshlocal0.22926716692745686s,remote0.4033229390042834s,
full exact match.

Six-extra-opposite lower-remove children from222055;remote / local seconds:
222274d34 exactNONNEG3.9087572380085476 /2.372880766983144,
localrundbc7075665b04b880898687aa73188a3109f4cfd6dc58c00d5a787cda04542db;
222310d34 exactNONNEG10.786184922995744 /6.872618773020804,
localrun6e006d51a02792efd9cb0425496c25446fd5a06041636a89d5e5caebaed6c249;
222400d34 exactNONNEG7.558063559001312 /5.743034165003337,
localrun6c6db9d75cd9e068da1b431ad24ffccaa407d4c435bba56ab99193c8ffcbe4d4;
222232d35 remoteTIME LIMITED20.025277102016844,localexactNONNEG26.37907601497136,
localrunc0d070d71533b1faef4feefbe4eefa94a176a6b025d4d0d6c490e2e771c85cd2;
222352d35 exactNONNEG17.51817229000153 /10.052258868003264,
localrunb06496d4c9c1407ecfe0583d9420bc8c3b69ea1ba41d2dd8d2b73c7bda54cc84;
222092d36 remoteTIME LIMITED20.11214021098567,localexactNONNEG24.31213321897667,
localrunef87c483546d7856af941ba0b05fd45b749b1f8f1e31069de00071c71f056095;
222132d36 remoteTIME LIMITED20.109142022993183,localTIME LIMITED30.004549685050733,
localrun7613e204c5340dec0b44dfa9eaf6c79ffb1aa642965b11931f38582383b80011.
Localbeam128/30s each60souter/nice10,strictly serialized by terminalhandle checks.
Original5s andremote20s retained. 222132 signUNKNOWN,not nonnegative.
All6 localexact polynomials independently reconstructed overQ fromhstar;
4remote search exact fullpoly/hstar/sign lists agree with freshlocal results.
Originalquotient and uploaded beamcovers/permutations verified all8.
No inherited exact evidence on timeout rows.
Lower-remove222055 CURRENT29exact NONNEGATIVE,8unknown,24dimprunes,
105objectiveprunes6duplicates. FreshDBanti-join unknown:
222132/222168/222198/222268/222394d36,222306d37,222228/222348d38.
Nextremote batch may take seven never-remotely-attempted sources excluding
222132;do not repeat itsremote20/local30 identities.

Input SHA 8908ad931381331707aa3c67c9f8a0857c08596ab57cce6bf4df623b3d5b7c7c
Wrapper SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA 6cf9f00519dfe35f199d227d5caeb38e811cd069c46f2374c3c0a93febcb1c8e
Receipt/file/header hashes validated beforeingestion. Ledger
97a5c7cb218614293fbfdc68e0afe7aa2b6dc7d2fc12f36b595d48cda534e671
completed;DBreadback prepared230160,receipt230161,validated230176..230183.
Remoteunknowns222232/222092/222132 keptdistinct fromlocalunknown222132.
No remoteDBaccess. Dockerfetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T110609-d6e4a070d3c9-7634aett
Remoteoriginals retained;hostsupervisor cancollectjobID above.
Ignored runs/abacus-lowerremove-seven-b-20260910.json and7localretryreports
untracked. Allhandles terminal,no localengine/remotejob remains,nopeakclaim.
No source edits/test claims,credentials/DB/checkouts transferred,toolchain/admin
changes,extra AIworkers,push/publication. Goalactive,no flagged witness.
Bestnegative same d33 seven/five;equivalent simplerRAW230106/cert230116 retained.

## Larger routine Abacus batch: seven cases resolved locally — 2026-09-10

Codex retains private profile/model,sole local DB writer,KTT/companion ownership.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no selected
retries or prior remote bundles. Abacus enabled/idle,limits unchanged.
Job20260910T110020-ea50cd722274 terminaldone/exit0,wall68.591494s.
Explicit180s whole bound justified before launch:tested self-contained wrapper,
max8 serial cases,20s/case,seven disjoint searches+knowncontrol. Monitored SAME
job while doing local validation;no ambiguous restart or resubmission.
RuntimeverifiedCPUs[0,1],memory.max8589934592,swap0,pids128,x86_64,
Python3.10.12/glibc2.35/resolvedlibraries;no installedlimit changes.
Control196437 freshlocal0.23564760095905513s,remote0.39980373601429164s,
full exact match. Only trusted unchanged wrapper,input,maintained executable sent.

Six-extra-opposite lower-remove children from222055,remote seconds / local seconds:
222278d33 EXACT NONNEGATIVE2.717654745996697 /1.5773384630447254,
localrundb38ab1c5c8a6039d912d223ceb6d3da441ff93fb1c99b543cf9732adcf31cf3;
222358d33 EXACT NONNEGATIVE9.932197702000849 /5.592892393004149,
localrun54fb3019bbed3a95b732ca33a1653011d624e7220ea7ce03996c2c7f0510495a;
222404d33 EXACT NONNEGATIVE9.626696196995908 /5.435043317032978,
localrun832b953f9b9eabbacf2ec9ca8ac455b779bcf2809a039a52e4c06dc901ce8af2;
222410d33 REMOTE TIME LIMITED20.02252765800222,localEXACT NONNEGATIVE23.213637845939957,
localrun249fd5c58a18f677856cfc0a1413d0f15fa835ddb7c5123438d22e6ad37ad418;
222098d34 EXACT NONNEGATIVE7.485064860986313 /4.668044006917626,
localruna8ec431a42d646994e7734bc81331804603a3b1979e565a28f07858f90c2581e;
222138d34 EXACT NONNEGATIVE14.196453308992204 /9.007792739081196,
localrune010cf67e183a9e4648b0a5d7af9758a3a5287b3b012761d23913381893ae58c;
222202d34 EXACT NONNEGATIVE2.134078617003979 /1.422654858091846,
localrun530f05e69481a90a9779f6024f8941f0481c8368eec49e5c459dce89c9dfbebb.
Localbeam128/30s counts each60souter/nice10,original5s records preserved.
Sequencing mistake:222098 launched before222410 handle became terminal,
causing brief overlap of these two owned,bounded counts. Both completed;
other local counts serialized. Do not repeat this;check terminalhandle before
starting next local count. No peak measurement claim,all engines now stopped.
All local polynomials independently reconstructed over Q from hstar.
Six remote fullpoly/hstar/sign lists agree with fresh local results;remote
222410 timeout preserved separately,not falsely called remoteexact.
Original quotient rebuilt and uploaded beamcovers/permutations verified all8.
Lower-remove222055 CURRENT23exact NONNEGATIVE,14unknown,24dimprunes,
105objectiveprunes6duplicates. Nextremote candidates afterfreshDB:
222274/222310/222400d34,222232/222352d35,222092/222132d36.

Input SHA eaceeebc866ac0ab559411f8fe8e612432a3b9f2627aaf0150ec840adbf1f0de
Wrapper SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA 7981e9d2b664a0447290bb5dde48c1f0f68fcc0ff07758d6c9ad71a8f0042990
Receipt/file/header hashes validated before ingestion. Ledger
72ac9e0faf4e9ef5401cd7fcbb19b4a5e5fac9139ce09a70e73a7e7bf63c7c46
completed;DBreadback prepared230136,receipt230137,validated230152..230159
(230156 remote timeout). No remoteDB access. Dockerfetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T110020-ea50cd722274-y5uy2mdg
Remote originals retained;hostsupervisor cancollectjobID above.
Ignored runs/abacus-lowerremove-seven-20260910.json and seven localretryreports
remain untracked. All handles terminal,no localengine/remotejob remains.
No source edits/test claims,credentials/DB/checkouts transferred,toolchain/admin
changes,extra AIworkers,push/publication. Goalactive,no flagged witness.
Bestnegative same d33 seven/five;equivalent simplerRAW230106/cert230116 retained.

## Second routine Abacus pair validated — 2026-09-10

Codex retains private profile/model,sole local DB writer,KTT/companion ownership.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no selected
retries/bundles. Abacus enabled/idle,limits unchanged. Submitted disjointjob
20260910T105625-e946de028630,terminaldone/exit0,wall16.161567s.
20s/case,60s whole,serialremoteCPUs[0,1],memory.max8589934592,swap0,pids128.
Runtime verifiedx86_64/Python3.10.12/glibc2.35/resolvedlibraries.
Trusted unchanged wrapper,selectedJSON,maintained executable only.
Control196437 freshlocal0.23496904701460153s,remote0.4019185670040315s,
full exact polynomial/hstar/negative list match.
Six-extra-opposite lower-remove children from222055:
222212d33 remote13.284482023009332s,EXACT NONNEGATIVE;
222238d33 remote1.5070375490031438s,EXACT NONNEGATIVE.
Fresh seriallocalbeam128/30s,60s outer,nice10 validation:
222212local7.81630170007702s,
run753497c6604427eb0268569b5a89662a813afaf3a79eb6ba9736d2f5408708ad;
222238local0.9388152329484001s,
runfbbf982adcb67738e71519d99d0dde43278c99fa496bcdf618b1f88082b558d2.
Fullpoly/hstar/sign matches,independent rational reconstruction,
rebuilt original quotient and verified uploaded beamcovers/permutations.
Source5s timeouts retained. Lower-remove222055 now16exact NONNEGATIVE,
21unknown,24dimprunes105objectiveprunes6duplicates.
Next remote-priority disjointd33 sources222278/222358,freshDB first.
Routine larger bounded bundles(up to7search cases pluscontrol,wrappermax8)
may reduce orchestration overhead;justify/monitor wholejob bounds if over60s.

Input SHA 71376fcb1090e53b591e2acd91c4aeb87b5eec5328acd19cfd6e53a257ca2911
Wrapper SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA b9c82a17a2edd6c99049c9f2c73a20d5c79529f545d58fdec6577f853861a019
Receipt/file/header hashes checked before ingestion. Ledger
492d335e93cec81bcf5a1c8bb6a2550d28d627b2a8489115ed61e65fd89fcca9
completed;DBreadback prepared230127,receipt230128,
validatedresults230133/230134/230135. No remoteDB access.
Dockerfetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T105625-e946de028630-z4d0ucsl
Remoteoriginals retained;hostsupervisor cancollectjobID above.
Ignoredinput runs/abacus-lowerremove222212-222238-20260910.json,
runs/fan-retry{222212,222238}-beam128-30-20260910.jsonl untracked.
Allhandles terminal,no localengine/remotejob remains,nopeakclaim.
No source edits/test claims,credentials/DB/checkouts transferred,toolchain/admin
changes,extra AIworkers,push/publication. Goalactive,no flagged witness.
Bestnegative remainsd33 seven/five;equivalent simplerRAW230106/cert230116 retained.

## Abacus supplied two validated degree33 results — 2026-09-10

User reiterated use Abacus as much as possible. Treat disjoint remote batches
as a routine search lane,not only occasional hardest-case retries.
Codex retains private profile/model,sole local DB writer and KTT/companion
ownership. Crossover checkpoint main165ea78/private374f6a2 completed first.
Fresh status enabled/idle,limits unchanged. Submitted job
20260910T105212-975de33e7be1,terminal done/exit0,wall16.178751s.
20s/case,60s whole,serialremoteCPUs[0,1],memory.max8589934592,swap0,pids128.
Runtime verifiedx86_64/Python3.10.12/glibc2.35/resolvedlibraries.
Only trusted unchanged wrapper,selectedJSON,and maintained executable uploaded.
Known control196437 freshlocal0.22892962803598493s,remote0.4138919540127972s,
full exact polynomial/hstar/negative list match.
Disjoint six-extra-opposite lower-remove children from222055:
222106d33 remote10.512783036989276s,EXACT NONNEGATIVE;
222180d33 remote4.203778251001495s,EXACT NONNEGATIVE.
Before remote ingestion,fresh serial local beam128/30s counts under60s/nice10:
222106local5.627315499004908s,
run987436a48e99c2e61b3e83f7b8aea34b613f3d3c53fb63462d656dfb8e6a0361;
222180local2.2383162659825757s,
run7ed2494acd5bf137b235967cf815865da7b1b6602029e9b95d89652867962df7.
Fullpolynomial/hstar/sign comparisons agree,independent rational reconstruction
passes,original quotient rebuilt and uploaded beamcovers/permutations verified.
Source5s timeouts retained. No new negative. Lower-remove222055 now14exact
NONNEGATIVE,23unknown,24dimensionprunes105objectiveprunes6duplicates.
Next remote-priority disjoint d33 sources222212/222238 (fresh DB before prepare).

Input SHA a82ab96efc630dbff460d0809fad80371a8ee7ee82bc1e50f7a24ef13d79c0c0
Wrapper SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA 627bb9c3b01a3f46ac0c67679b5d28fbc5178c31a95b6a04087ddad7c99604cf
Receipt/file/header hashes independently checked. Ledger
cc24f2c407322bdc87b16b13f7f0d4bd3fae49713601953608245e2e4d759b70
completed;fresh DBreadback prepared230118,receipt230119,
validatedresults230124/230125/230126. No remote DB access.
DurableDocker fetch outsideDropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T105212-975de33e7be1-qr3mtx0n
Remote originals retained;host supervisor can collect jobID above.
Ignored input runs/abacus-lowerremove222106-222180-20260910.json and
runs/fan-retry{222106,222180}-beam128-30-20260910.jsonl remain untracked.
All handles terminal,no local engine or remote job remains;no peak claim.
No source edits/test claims,credentials/DB/checkouts transferred,toolchain or
admin changes,extra AIworkers,push/publication. Goal active,no flagged witness.
Bestnegative same d33 seven/five;equivalent simplerRAW230106/cert230116 retained.

## Simpler negative description recovered by crossover — 2026-09-10

Codex retains sole local DB writer and KTT/private companion ownership.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
222055+196437 crossover queues. Both retained audited exactnegative9x15
parents:d33 andd30. Source unchanged.
Row-band all56 cap7extra/d33,beam32/5s/45s batch/60s outer,nice10:
2exact NONNEGATIVE(d27 observation229664,d29 observation229708),
18duplicates,26objectiveprunes,10dimprunes,0timeouts;1.6318285780726s,
runf26b6a9b98e8221a828c834ff513bf011a249008e8bb3623034f7cfb608590ae.
Column-band all182 same bounds:1exact NEGATIVE230106d33[12,13,1],
75duplicates54objectiveprunes52dimprunes,0timeouts;7.316363068995997s,
run4befeecc29a5ce8fbbc16a379cb1aa95717ab64bdf5adc39e8765ad497cdc580.
Exact negative count2.8312233670149s,linear-60428124102809/24067258815600.
Fresh DB rational reconstruction checks all3 exact outcomes.

230106 has SAME FULL GT-COORDINATE FACE,polynomial,hstar as222055:
actual tuple comparison auditfff13b7ebf34e08330e8f05a80634d4102dfd37caa000395d7ff4e125b5a91a1,
0.14821933396160603s,coordinateSHA
a167d82d1eccaf9fb585e9cf7eea597e3dbfecc910c6c97fcf1b24045eab3421.
NOT a new negative lineage or improved bad-edge/degree record.
Simpler RAW230106 removes two redundant horizontal equations(21->19):
v0x3ff01fe01fc01f801f001e001c00181f1
h0x1000800084008401840104030406040c00.
Fresh beam32/15s both-endpoint lowerflag certificate230116:
rune5042a1cce93e1af25ddcdc9759500b1f86657613060b8c5a5798dbc53d5e574,
5.0610950839472935s,full exact match and coordinate permutations verified.
Stilld33 sevenextraopposite/fiveinternal presentation counts.
Retain RAW230106 as equivalent simpler mutation parent,not expanded cert.
Reports runs/fan222055-donor196437-{rowband,columnband}-seven-d33-offset0-20260910.jsonl
and runs/fan-certify230106-beam32-15-20260910.jsonl ignored/untracked.

User reiterated maximize Abacus use. Fresh status enabled/idle,all7 previousKTT
jobs terminal. Next remote batch prepared conceptually:control196437 plus
222106/222180d33,sixextraopposite,original5s unknown;DB finds no retries.
Checkpoint before remote submission. No source edits/tests claimed,no admin
changes,extra AI workers,push/publication. Goal active,no flagged witness.

## Harder joint cases remain unknown under bounded retries — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
228306/228486 retries. Source unchanged. Bothdegree39 six-extra-opposite/
four-internal presentation candidates,not certified negative.
Serial coordinate-verified beam128/30s,60s outer,nice10:
228306[47,132,113] TIME LIMITED30.03718271793332s,
run9fbdead0db33aad62caeea8bad544352435464b06db76ef393fe115b4af9a68e;
228486[69,98,113] TIME LIMITED30.10152875096537s,
runc444d83dea9cd22955cc16c23172a82c205cc09673cd2e6ada5af66d250478a0.
228486 PID3092518 NI19 sample28s RSS1522948KiB,not peak.
Justified distinct120s retry228486:tested finite maintained engine,
similar auditedfrontier(6,121) to nearby108-110s exact counts;runtime heuristic
not sign/completion guarantee. Higher observed memory explicitly monitored.
120s percase/150s outer,nice10,TIME LIMITED120.21222357801162s,
run072a29d04acf22003fcef7fd9c2598fef029bb540657f8dcc8a1897c770f816f.
PID3094589 NI19 samples27s/44s/82s/102s
RSS1515772/1763340/2122056/3259212KiB,not peaks.
No further extension. Fresh DB readback confirms all3 statuses and budgets;
all timeout rows have no inherited polynomial/negative evidence.
Original5s/30s retained. No repeated terminal identity.
All handles terminal exit0,no local engine remains.

RAW222055 joint945 unchanged4exact NONNEGATIVE,32unknown,279dimprunes,
630objectiveprunes. Both228306/228486 remain UNKNOWN,not positivity evidence.
Do not repeat228486 beam128/120 or228306 beam128/30 unchanged.
Prefer another layout/structural approach or new lower-degree mutations from
negative222055;untried joint degree39 cases228466/228516 remain possible.
Reports runs/fan-retry228306-beam128-30-20260910.jsonl and
runs/fan-retry228486-beam128-{30,120}-20260910.jsonl ignored/untracked.
No remote submission. Best negative222055/174551 stilld33 seven/five,
presentation not minimum counts. Content-removal394exact nonnegative,
0attempted holes,165dimprunes18unvalidatedduplicates26empty.
No flagged witness;goal active,no global blocker,no source edits/test claims,
extra AI workers,admin changes,push/publication.

## Third monitored degree39 joint retry resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
228256/228276 retries. Source unchanged. Bothdegree39 six-extra-opposite/
four-internal presentation candidates,not certified negative.
Serial coordinate-verified beam128/30s,60s outer,nice10:
228256[47,82,113] TIME LIMITED30.05909109499771s,
run863ebe7441b9f6ba285cb87d5950d039a155251b12017762d81bafaba3e13714;
228276[47,98,113] TIME LIMITED30.046789149055257s,
runfc95ae9d088b8f59434846d906aa1b502aff55aca7ada9e0c716b815c4769109.
228256 PID3085825 NI19 sample28s RSS895016KiB,not peak.
Justified distinct120s retry228276:tested finite maintained engine,
audited similar frontier(6,119) to two nearbydegree39 counts finishing108-110s;
runtime heuristic,not sign/completion guarantee.
120s percase/150s outer,nice10,actively monitored;EXACT NONNEGATIVE
109.15553702099714s,
run400427f2ea4905f375e776e921fb9db2a18c47e8a85d7cc4cb1ca69210a0f0ab.
PID3087412 NI19 samples30s/48s/76s/107s
RSS675884/1187644/1279240/590984KiB,not peaks.
Fresh DB readback confirms all3 statuses and preserved budgets.
Independent rational hstar reconstruction/sign check passes exact polynomial;
timeout rows have no inherited polynomial/negative evidence.
Original5s/30s retained,no repeated terminal identity.
All handles terminal exit0,no local engine remains.

RAW222055 joint945 CURRENT4exact NONNEGATIVE
(228096,227856,228066,228276),32unknown,279dimprunes,630objectiveprunes.
227836/227886/228046/228256 remain beam128/30 unknown;no unchanged replay.
Next distinctdegree39 cases228306/228466/228486;fresh DB before action.
Reports runs/fan-retry228256-beam128-30-20260910.jsonl and
runs/fan-retry228276-beam128-{30,120}-20260910.jsonl ignored/untracked.
No remote submission. Best negative222055/174551 stilld33 seven/five,
presentation not minimum counts. Content-removal394exact nonnegative,
0attempted holes,165dimprunes18unvalidatedduplicates26empty.
No flagged witness;goal active,no global blocker,no source edits/test claims,
extra AI workers,admin changes,push/publication.

## Second monitored degree39 joint retry resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
228046/228066 retries. Source unchanged. Bothdegree39 six-extra-opposite/
four-internal presentation candidates,not certified negative.
Serial coordinate-verified beam128/30s,60s outer,nice10:
228046[29,82,113] TIME LIMITED30.047225284972228s,
runc687fed4929e212eedc8d742a7547debebb40d3be8804e3771825eca7329f1a2;
228066[29,98,113] TIME LIMITED30.06528851890471s,
run368acd2d98071d6670af5b5e989f26871f2c3d9bd13690261bf38dc39267662d.
Justified distinct120s retry228066:tested finite maintained engine,
audited same beam frontier score(6,118) as nearby227856 that finished109.8s;
this is runtime heuristic,not sign/completion guarantee.
120s percase/150s outer,nice10,actively monitored;EXACT NONNEGATIVE
108.25999167899136s,
run2b92a1f30f8abc830bf1551c8418f40ce989fdf02e912f3eab96f1c25e45a9bc.
PID3082440 NI19 samples38s/55s/83s RSS909384/720724/1396008KiB,not peaks.
Fresh DB readback confirms all3 statuses and preserved budgets.
Independent rational hstar reconstruction/sign check passes exact polynomial;
timeout rows have no inherited polynomial/negative evidence.
Original5s/30s retained,no repeated terminal identity.
All handles terminal exit0,no local engine remains.

RAW222055 joint945 CURRENT3exact NONNEGATIVE (228096,227856,228066),
33unknown,279dimensionprunes,630objectiveprunes.
227836/227886/228046 remain beam128/30 unknown;do not replay unchanged.
Next distinct degree39 cases228256/228276/228306;fresh DB before action.
Reports runs/fan-retry228046-beam128-30-20260910.jsonl and
runs/fan-retry228066-beam128-{30,120}-20260910.jsonl ignored/untracked.
No remote submission. Best negative222055/174551 stilld33 seven/five,
presentation not minimum counts. Content-removal394exact nonnegative,
0attempted holes,165dimprunes18unvalidatedduplicates26empty.
No flagged witness;goal active,no global blocker,no source edits/test claims,
extra AI workers,admin changes,push/publication.

## Monitored degree39 joint retry resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
227856/227886 retries. Source unchanged. Both are degree39 six-extra-opposite/
four-internal presentation candidates,not certified negative.
Serial coordinate-verified beam128/30s,60s outer,nice10:
227856[10,98,113] TIME LIMITED30.029845986049622s,
runc904eecfa6268228789803408b7cdbd491ee8aec80b796ffc9ca52943a38a69b;
227886[10,132,113] TIME LIMITED30.04706227593124s,
run74a5266f747b6d7413120a0fa93d5c0584e4906120d4738ca0902a64efa59f26.
Sample227856 PID3076321 NI19 at28s RSS318844KiB,not peak.

Justified distinct120s retry for227856:tested finite maintained counter,
prior30s controlled memory,directly improves both nonflag counts.
120s percase/150s outer,nice10,actively monitored;EXACT NONNEGATIVE
109.7884763389593s,
runa1a5a3853b461ef23eca60deb79b377baf062bca0e87270f8c5f8fecc1b9173e.
PID3077739 NI19 samples28s/53s/83s RSS893848/1187740/1279336KiB;
not peak. Fresh DB readback confirms all3 statuses and preserved budgets.
Independent rational hstar reconstruction/sign check passes exact polynomial;
timeout rows have no inherited polynomial/negative evidence.
Original5s/30s records retained,no repeat of terminal identity.
All handles terminal exit0,no local engine remains.

RAW222055 joint945 CURRENT2exact NONNEGATIVE (228096,227856),34unknown,
279dimensionprunes,630objectiveprunes. 227836 and227886 remain beam128/30
unknown;227856 no longer unknown. Next distinct degree39 candidate228046
or justified retry227886;fresh DB before acting. Layoutaudit7942f442... retained.
Reports runs/fan-retry227856-beam128-{30,120}-20260910.jsonl and
runs/fan-retry227886-beam128-30-20260910.jsonl ignored/untracked.
No remote submission. Best negative222055/174551 stilld33 seven/five,
presentation not minimum counts. Content-removal394exact nonnegative,
0attempted holes,165dimprunes18unvalidatedduplicates26empty.
No flagged witness;goal active,no global blocker,no source edits/test claims,
extra AI workers,admin changes,push/publication.

## Joint-candidate coordinate/layout audit and one exact count — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no selected
retries. Source unchanged. Audited all36 RAW222055 joint timeouts by rebuilding
original GT quotient equality blocks,marked blocks,and full order relations.
Actual coordinate-signature tuples compared,not just hashes:all36 distinct.
This does not rule out abstract poset isomorphism. No counting in this audit.
Verified beam32 andbeam128 coordinate permutations for every candidate,stored
full permutations/frontier profiles/scores in36 DB layout observations.
Auditkindfan_joint_layout_audit,3.772077666944824s,
run7942f44277adba6a1dbcaf250403ebdac9d1c0dd42c3472123e4fbd01b21be34.
Fresh DB confirms completed/36records/singleton coordinate groups.
Initial attempted longer search_kind failed SQL column-length check before any
case execution/insertion;used shorter kind,without changing schema/admin.
Selected improvements:227836d39 beam32(peak7,total137)->beam128(6,117);
227856d39(7,134)->(6,118);228096d39 retains identical(5,148) order
at both widths,smallest peak among this list. Width is heuristic,not runtime proof.

Serial bounded retries,coordinate-verified beam128/30s,60s outer,nice10:
227836d39[10,82,113],TIME LIMITED30.059480119030923s,
runf7f4fef36678d7d442b4409578e28ab82d4ad38db9c8b53bd7a45616f7108606;
228096d39[29,132,113],EXACT NONNEGATIVE11.261287886067294s,
rune76675eccf5e4944337ddaf135a028bd2776b12c5397e9fbf9145fc79444c166.
Original5s attempts retained. Fresh DB readback confirms both statuses,
no inherited polynomial/negative evidence for timeout;independent rational
hstar reconstruction and sign list check pass for228096.
Joint945 CURRENT1exact NONNEGATIVE,35unknown,279dimensionprunes,
630objectiveprunes;no duplicates/empty. Six extra opposite/four internal
presentation bans remain promising unknowns,not negative witnesses.
Next degree39 cases227856/227886/228046 have no known retry;fresh DB before
acting. Do not repeat227836 beam128/30 unchanged.
Reports runs/fan-retry{227836,228096}-beam128-30-20260910.jsonl ignored/untracked.
All handles terminal exit0 except initial handled SQL-length failure;
no local engine remains,no peak measurement claimed. No remote submission.
Best negative222055/174551 stilld33 seven/five,presentation not minimum counts.
Content-removal remains394exact nonnegative,0attempted holes,165dimprunes,
18unvalidatedduplicates26empty. No flagged witness,goal active,no globalblocker.
No source edits/test claims,extra AI workers,admin changes,push/publication.

## Joint flag and two-nonflag-removal queue processed — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
RAW222055 joint/internal/slide-internal queue. Source unchanged.
NEW joint mode adds one boundary flag and removes one opposite equality plus
one internal ban. Used RAW222055,not expanded certificate.
All945 proposals visited,cap6extra opposite after all implied lower flags,
dimensioncap40,beam32,5s/candidate,45s safety/60s outer,nice10.
Six serial batches,all handles exit0:
0->191,40.4005817869911s,7unknown/54dimprunes/130objectiveprunes,
run09b996cae5a5e54156d52667a1c31af058baa50e68bce04050719a028f84c3e1;
191->391,40.573395731044s,7unknown/58dim/135objective,
run32296f3c4eec1ed5e4ab1292b1515eb12ca4fd1e7199692631c2a3fe9a590a01;
391->586,40.50865961704403s,7unknown/58dim/130objective,
runa2e552b74a4fdee97b5127754e613a4285e828dffab1950090d118c4fb900670;
586->731,39.13051388191525s,7unknown/43dim/95objective,
runfe4b0fa4206cf552e6a7dfe9acf330cf77fdda8a015e9b1fcb51914ffd545800;
731->926,40.43327695305925s,7unknown/58dim/130objective,
runad2e6ae5e9e230ee67202c5b319fa08bd37529c5ec0b19a5a0077949d5a439d6;
926->945,5.612100980943069s,1unknown/8dim/10objective,
runce3f7602350c2ffc14a149f7c5eb49e03474844f1992ba69c65d1ce6ab27701f.
Fresh DB945 terminal results:36 TIME LIMITED,279dimensionprunes,
630objectiveprunes,0exact/negative/empty/duplicates/errors.
No sign conclusions. All36 attempted candidates have6extra opposite/4internal
presentation bans,27degree39 and9degree40;not certified minimum distance.
Degree39 retry priorities227836[10,82,113],227856[10,98,113],
227886[10,132,113];then boundaryindices29/47/69/95/125/159/197/239,
same removedhorizontal82/98/132,internal113.
Degree40 horizontal66/internal113,first227806.
Fresh DB before each retry;consider coordinate/layout comparison before
recounting all boundary variants. No terminal identity should be repeated.
Reports runs/fan222055-joint-six-d40-offset{0,191,391,586,731,926}-20260910.jsonl
ignored/untracked. Sample NI19 enginesRSS123532/121248/91956/204652/404756KiB,
not peaks;processes terminal,no engine remains. No remote submission.
No source edits/test claims. Best negative222055/174551 remain d33 seven/five;
none of these new six/four candidates is certified negative.
Content-remove family remains394exact nonnegative,0attempted holes,
165dimprunes18duplicates26empty. Goal active,no global blocker,no extra
AI workers,admin changes,push or publication.

## All attempted content-mutation counts resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,all EXACT NONNEGATIVE:
227150d32,5.228216855903156s,
run47dfe73fc00bf7da848c8acac74a72f7d6f021d6ba915985cf774cfee97929db;
227156d32,23.069939199951477s,
runacfabc7cd80ef9c9a15748fc389882a82ead928be1eec85c741d6b423ff922ee;
227194d32,5.445529004908167s,
run5377ffe5763dafa22032388a2aeaf07b3e9f69d9cc83fc2e36d6088a61db5c97;
227208d32,23.45812780794222s,
run82e042455ddd118693a535e4ade20ab15065f9673d599367eedc7dce30347aa8;
226662d33,6.583967222017236s,
runc4f619cec9a6597381dc289a367a9e30aae27de1df5bb59de8bd7a4d81fd6db3;
227146d33,5.786938423989341s,
run1ade675b9f3f133a06b1eb4b0ba59db03fc2b6b39f0e66a07c653be05ffa8521;
227198d33,3.219451645971276s,
runcf175024f8a503ca564bf88b8feb2062d9fe9c93f3e44c8ec07e71d1f78e21d6.
Fresh DB readback and independent rational hstar reconstruction pass all7.
Fresh full-family anti-join confirms zero original timeouts without an exact
retry. Original5s attempts retained. No peak measurements claimed.
All handles terminal exit0,no local engine remains.

RAW222055 full-content-remove CURRENT394 exact NONNEGATIVE,0attempted unknown,
165dimensionprunes,18duplicates unvalidated,26empty;all603 proposals visited.
This is NOT a positivity certificate for the full family:183pruned/duplicate
cases were not counted. New mutation neighborhood (joint flag/nonflag removal,
internal removal/slide,or crossover from222055) remains in scope after fresh DB
checks;avoid repeating completed content queue unchanged.
Reports runs/fan-retry{227150,227156,227194,227208,226662,227146,227198}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Best negative222055 and174551 remain d33 seven/five presentation counts,
not minimum distance. No flagged Kostka witness. Goal active,no global blocker.
No source edits/test claims,extra AI workers,admin changes,push/publication.

## Six degree32 content-mutation gaps resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress. Fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,all EXACT NONNEGATIVE:
227048,5.325017297058366s,
run3d00c1385c04715f56bc195a870e930dbbd1c7e82d67eff1f480421f35c3db4f;
227054,5.180230859899893s,
run4ab8b07b603ba9d6666c3b7b292643f6fc96f0f901e4fb6ad758e3db338dff30;
227060,23.954229507013224s,
runf97f0ed9e6c3d331039050955d8af36f257eb4c3b7f07768acf3783d4575e5b0;
227094,5.378203721949831s,
runa9cb99472b768bebe134d887b6ff3c7e64ddf4847371f55496770b80b67f93be;
227108,23.503679501009174s,
runf7b3e2587578dd3fb5a3f185f62704dd4a828b73a16b1dc22b1f0f8cf7875867;
227142,5.599803762044758s,
run734145da6388f7f6c9ffee59be9aaba7632b710581f43eb62163a2986774bf30.
Fresh DB readback and independent rational hstar reconstruction pass all6.
Original5s attempts retained. No peak measurements claimed.
All handles terminal exit0,no local engine remains.
222055 full-content-remove CURRENT387 exact NONNEGATIVE,7unknown,
165dimensionprunes,18duplicates unvalidated,26empty;all603 proposals visited.
Fresh DB anti-join remainingd32:227150/227156/227194/227208;
d33:226662/227146/227198. Do not repeat unchanged terminal identities.
Reports runs/fan-retry{227048,227054,227060,227094,227108,227142}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Best negative222055 and174551 remain d33 seven/five presentation counts,
not minimum distance. No flagged Kostka witness. Goal active,no global blocker,
no source edits/test claims,extra AI workers,admin changes,push/publication.

## Full603 weight-plus-equality proposals processed — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks confirmed
resumeoffset336. Source unchanged. RAW222055 full-content-remove,
dimensioncap33,beam32,5s/candidate,45s batch safety/60s outer,nice10.
No flag-only objective filter on content-modified faces.
336->357,39.837311128969304s,11exact NONNEGATIVE,2unknown,8dimensionprunes,
run99ca57c3cfe8d2b5a7ae668401dca92eecb24ede5e1d94d6ab886a1aba8d5643;
357->603,38.50387688702904s,179exact NONNEGATIVE,3unknown,38dimensionprunes,
26empty,runc43d8fdf073e50a0450a40e68c77e27e339e35db067754b7767255bbe1c27581.
All proposals/outcomes stored;fresh DB counts agree;independently reconstructed
all190 new exact polynomials from hstar over Q and verified sign lists.
Initial full603 aggregate:379exact NONNEGATIVE,15unknown,165dimensionprunes,
18duplicates not revalidated,26empty. No negative/error;all queue slots visited,
NOT a positivity certificate for skipped or timed-out cases.
New timeout sources227150d32[14,horizontal,98],227156d32[14,horizontal,132],
227194d32[15,horizontal,71],227198d33[15,horizontal,82],
227208d32[15,horizontal,132]. Original limits retained.

Fresh DB found no selected retries. Serial coordinate-verified beam128/30s
under60s outer,nice10 resolved two original5s timeouts EXACT NONNEGATIVE:
227144d31[14,horizontal,72],3.492546508088708s,
run6e6cc1ddf39b248b8799d010aad74dc1cf062296e13d35bf07b7f359517263f5;
226660d32[4,horizontal,82],5.436032091034576s,
run4e369e095f1e297f4c4b2401b8f3e9d025f0f8dbd84b14ce9234b18a4341b77c.
DB readback/independent rational reconstruction pass both.
CURRENT381exact NONNEGATIVE,13unknown,165dimensionprunes,18duplicates,26empty.
Remaining initiald32 gaps227048/227054/227060/227094/227108/227142/
227150/227156/227194/227208;d33 gaps226662/227146/227198.
No degree31 original gap remains. Do not repeat unchanged terminal identities.

Ignored reports runs/fan222055-fullcontentremove-d33-offset{336,357}-20260910.jsonl
and runs/fan-retry{227144,226660}-beam128-30-20260910.jsonl untracked.
Sample engineNI19 RSS65816/247172KiB(PIDs3050083/3051217 at0/2s),not peaks.
All handles terminal exit0,no local engine remains. No remote submission.
Best negative222055 and174551 remain d33 seven/five presentation counts,
not certified minimum distance;no flagged Kostka witness. Goal active,no global
blocker. No source edits/test claims,extra AI workers,admin changes,push/publication.

## Weight-plus-equality search advanced to336/603 — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks confirmed
resumeoffset283. Source unchanged. RAW222055 full-content-remove,
dimensioncap33,beam32,5s/candidate,45s batch safety/60s outer,nice10.
No flag-only objective filter on content-modified faces.
Serial bounded batches:
283->304,41.12765917403158s,7exact NONNEGATIVE,3unknown,11dimensionprunes,
run5c1f7d4120bba51ffda3501b343893295b483c7ec30ab78e312a5235dedb5b15;
304->317,39.35819168295711s,8exact NONNEGATIVE,2unknown,3dimensionprunes,
run17a4a76ca9c808ace391d019596106c34f239a31b1c3ceaf308f953f5ccef469;
317->336,43.03047643508762s,7exact NONNEGATIVE,3unknown,9dimensionprunes,
run48a07ab5743d50bb8873c301e4982a3af1e2ea6b3bd20cf41bf0bd8f3a911230.
No negative/duplicates/empty/errors this turn. All proposals/outcomes saved.
Fresh DB counts agree;independently reconstructed all22 new exact polynomials
from hstar over Q;sign lists agree. New5s unknowns:
227048d32[12,horizontal,71],227054d32[12,horizontal,98],
227060d32[12,horizontal,132],227094d32[13,horizontal,71],
227108d32[13,horizontal,132],227142d32[14,horizontal,71],
227144d31[14,horizontal,72],227146d33[14,horizontal,82].
Earlier226660/226662 unknown retained. All timeout signs UNKNOWN.
Aggregate DB full query:189exact NONNEGATIVE,10unknown,119dimensionprunes,
18duplicates not revalidated,0negative/empty. Resumeactualoffset336/603;
267 unprocessed proposals remain;do not replay terminal budget/layout identities.
Reports runs/fan222055-fullcontentremove-d33-offset{283,304,317}-20260910.jsonl
ignored/untracked. Sample NI19 engineRSS59592/149428/206616KiB
(PIDs3044586/3045536/3047291 at0/2/2s),not peaks.
All handles terminal exit0,no local engine remains. No remote submission.
Best negative222055 and174551 stilld33 seven/five presentation counts,
not minimum distance;no flagged Kostka witness. Goal active,no global blocker.
No source edits/test claims,extra AI workers,admin changes,push or publication.

## Weight-plus-equality search advanced to283/603 — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks confirmed
resumeoffset93. No source edits. Continued RAW222055 full-content-remove,
not expanded certificate. Dimensioncap33,beam32,5s/candidate,45s batch
safety/60s outer,nice10; no flag-only objective filter on content-modified faces.
Three serial terminal batches:
93->191,39.9817511200672s,59exact NONNEGATIVE,1timeout,38dimensionprunes,
rundc3e2ec053c2c03c74f1f0a14dc9c44b09d0e252f04061da11d68e99493d4b31;
191->255,41.070868288050406s,23exact NONNEGATIVE,24dimensionprunes,17duplicates,
run11dda56901b25df47eebd3dec11fe137c2de51e49c2468d5ad4db941544550d7;
255->283,42.01128427393269s,17exact NONNEGATIVE,10dimensionprunes,1duplicate,
run37717eaf171df2f2fe323cddb99059382d75e8060a78a93bd13a3b0b823879de.
No negative/empty/errors in these batches. All proposals/outcomes saved.
Fresh DB result-row counts agree; independently reconstructed all99 new
exact polynomials from hstar over Q and checked every sign list.
New unknown226662d33[4,horizontal,87];earlier226660d32[4,horizontal,82]
remains unknown. Timeouts are not nonnegative conclusions.
Aggregate0->283:167exact NONNEGATIVE,2unknown,96dimensionprunes,
18duplicates not revalidated,0negative/empty. 320 proposals remain.
Resume actualoffset283,never replay prior timeouts at same layout/budget.
Reports runs/fan222055-fullcontentremove-d33-offset{93,191,255}-20260910.jsonl
ignored/untracked. Sample enginePID3039058 NI19 RSS55288KiB at0s and
PID3042423 NI19 RSS113428KiB at1s;these are NOT peak measurements.
All handles terminal exit0,no local engine remains. No remote submission.
Best negative222055 and174551 stilld33 seven/five presentation counts;
no flagged Kostka witness. Goal active,no global blocker,no admin changes,
extra AI workers,push or publication. Source unchanged;no test claims.

## Weight-plus-equality mutations started; degree33 shrink gaps closed — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress. Fresh clean Git/no-engine/DB checks; no selected
retries or RAW222055 full-content-remove queue existed. Source unchanged.
Serial beam128/30s retries under60s outer,nice10,all EXACT NONNEGATIVE:
226246d33[3,7],9.969472554977983s,
run99183e187872f5eb3750a2825769461b5ade1a42ddb20782e82ca9241f516481;
226334d33[6,6],4.558099362067878s,
run3a7f2b9d15d6d7fcd06b3008b7cc44bf04e4eb049e12f46a5942734f32792963;
226364d33[7,6],6.9388118230272084s,
runcdb9be2a8675eb065363887aad91c8c03d66c410939405cef45278e2c8a3e393.
Fresh DB readback and independent rational hstar reconstruction passed all3.
Original5s attempts preserved; no measured peak claim; all handles exit0.
222055 interior-shrink now90 exact nonnegative,1 unknown226422d34
(remote20/local30 limits),18 untouched dimension prunes,8 objective prunes,
14 duplicates unvalidated. All original attempted degree<=33 gaps resolved.

Started NEW RAW222055 full-content-remove neighborhood,not expanded certificate:
603 total proposals; label then horizontal/internal equation order.
Changes original content and removes one original equation; retained mixed-face
models are NOT automatically flagged Kostka coefficients. No flag-only
objective filter applied to content-modified faces. Capdimension33,beam32,
5s/candidate,45s batch safety/60s outer,nice10,offset0->93,
40.185282062971964s. Run
e5c5446261e0d5d137d60540ce581fa1343334fc3ec0e4ef9d1430fda9c52074:
68 exact NONNEGATIVE,1 unknown226660d32[4,horizontal,82],
24 dimension prunes,0duplicates/empty/negative. Status capped,510 proposals
remain. Every proposal/outcome stored;fresh DB93 terminal rows agree.
Independently reconstructed all68 exact polynomials from hstar over Q;
all sign lists agree. Prunes/timeouts remain uncertified.
Resume actualoffset93,do not replay original timeout at same budget/layout.
Ignored reports runs/fan-retry{226246,226334,226364}-beam128-30-20260910.jsonl
and runs/fan222055-fullcontentremove-d33-offset0-20260910.jsonl untracked.
No source changes/tests claimed. No local engine remains.
No remote submission this turn; previous Abacus job remains terminal.
Best negative222055 and174551 stilld33 seven/five presentation counts;
no smaller negative or flagged witness. Goal active,no global blocker,
no extra AI workers,admin changes,push or publication.

## Abacus first-count batch validated — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Checkpointed previous local result before pilot (main e26da24/private d59ac98).
Job20260910T095405-6e6d8ec8c9a1 completed exit0,wall22.190928s.
Selected previously uncounted dimension-pruned226422,cut[9,6] from RAW222055:
8x14,d34,five extra opposite/four internal bans (presentation,not minimal distance).
Fresh mask query found only original started/pruned rows,no prior retry.
Known negative196437 was freshly locally recounted before submission (timing
not separately measured); remote0.4043310659908457s matched full exact
polynomial/hstar/negative list. Trial226422 remote20.13912266799889s TIME LIMITED.
Serial local coordinate-verified beam128/30s retry also TIME LIMITED,
30.102538823033683s,runc7b475cbb60d7bd2a3f028f3b2eba6c994a1a5955f440445b8f462893c1a0f49.
Sample localPID3031243 at10s NI19 RSS853156KiB;not peak. All handles terminal,
no local engine remains. Sign226422 UNKNOWN,no exact cache/promotion.
222055 interior-shrink now87 exact nonnegative,4 unknown (three originald33
plus226422d34),18 untouched dimension prunes,8 objective prunes,14 duplicates.
No negative improvement or flagged Kostka counterexample.

Remote limits freshly verified:CPUs[0,1],memory.max8589934592,swap0,pids128;
x86_64,Python3.10.12,glibc2.35,all executable libraries resolved.
20s/case,60s whole;two selected cases,zero additional proposals skipped.
Trusted wrapper/engine unchanged,only wrapper/input/executable uploaded.
Input SHA256 fc7560723e0c1924be7900ed89144c7616318270d14ae7cf8feb9ad9a3c3367d
Wrapper SHA256 a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
Engine SHA256 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
Output SHA256 2b11100cb4f5b4bcd55151234096370e7702ea83852973de78e979c633140960
Validated receipt/file/header hashes,independent rational hstar reconstruction,
baseline exact match,and rebuilt original quotient/beam covers for both cases.
Ledger2752c87417cd2e278966c6712a157a76302c11b428ea6a178058d760008eed8b
completed,DB readback confirms prepared226463,receipt226464,validated control
226467 and timeout226468. Local retry226465 retained separately.
Fetch initially refused while job running; inspected SAME job,then fetched
terminal result,never resubmitted. Durable Docker copy outside Dropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T095405-6e6d8ec8c9a1-neh35qqu
Remote originals retained;host supervisor can collect job ID above.
Ignored runs/abacus-shrink226422-20260910.json and
runs/fan-retry226422-beam128-30-20260910.jsonl remain untracked.
No source edits/tests claimed,no credentials/DB/checkouts transferred,no admin
changes,extra workers,push or publication. Best negative222055 and174551 remain
d33 seven/five. Next disjoint lower-degree nonflag mutations or remaining
226246/226334/226364d33 gaps;do not repeat terminal budget/layout identities.
Goal active,no global blocker.

## Abacus batch preparation and recovered local result — 2026-09-10

Codex retains KTT/private companion ownership and sole local DB-writer role.
Previous status-only turn made no search progress; revalidated current state.
Both repositories clean, no live local engine. Recovered terminal226216 retry:
beam128/30s, exact nonnegative,10.324849889962934s,
runbf82ba811052fde68b4e861bb1ccfbc369d3676b15e31f8c0d601f6940756871.
Fresh DB readback agrees with report; original5s timeout retained.
222055 interior-shrink now87 exact nonnegative,3 unknown,19 dimension prunes,
8 objective prunes,14 duplicates. No improved negative or flagged witness.
Abacus freshly available, all six previous KTT jobs terminal, queue idle.
Preparing previously uncounted226422d34,8x14,five extra opposite/four internal
presentation bans,cut[9,6] from RAW222055. Fresh DB mask lookup finds only
started/dimension_pruned source rows, no retry. Remote control196437 plus
this disjoint first count,20s/case,60s whole; no new source changes.
No MariaDB MCP available; protected existing Python DB adapter used.
Remote README/INSTALL read, compatibility and bundle hashes to be verified;
no credentials/database/checkout upload, administration change or extra agents.
Generated inputs/reports remain untracked. Goal active; no global blocker.

## Remaining degree32 shrinking cases resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,both EXACT NONNEGATIVE:
226276d32[4,7],5.361237645032816s,
runbb0bd5cc1c6a309230e6ed6da6542ee4dddf89d3d5fe681f0007b8fb84570220;
226394d32[8,6],6.59636452794075s,
runa2d9fef036ef028f5dc45da59ab5855fda836830d734b01a60c8fc0b63fb68d1.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Both finished before resource samples;no peak claim.
All handles terminal exit0,no engine remains.
222055 interior-shrink now86 exact nonnegative,4 unknown,
19 dimension prunes,8 objective prunes,14 duplicates not revalidated.
Remaining initial d33 gaps226216[2,7],226246[3,7],226334[6,6],226364[7,6].
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{226276,226394}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No smaller negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Two degree32 shrinking cases resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,both EXACT NONNEGATIVE:
226228d32[2,13],11.204734291997738s,
runb6cf0d49ea6a25b5e9a17d7e6ff6db823cb4e6f48c42709cc50fc25b187c6ecf;
226258d32[3,13],11.548042502021417s,
run2dd7e397935f1a6343507eca6f540eb7d4834aafb7854afb082c04b5caa911ac.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Both finished before resource samples;no peak claim.
All handles terminal exit0,no engine remains.
222055 interior-shrink now84 exact nonnegative,6 unknown,
19 dimension prunes,8 objective prunes,14 duplicates not revalidated.
Fresh DB remaining d32 cuts226276[4,7],226394[8,6];d33 cuts226216[2,7],
226246[3,7],226334[6,6],226364[7,6]. Do not repeat terminal identities.
Reports runs/fan-retry{226228,226258}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No smaller negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Three more smaller-shape exact results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/full DB next-gap query
found never-retried selected sources. Source unchanged.
Serial coordinate-verified beam128/30s,60s outer,nice10,ALL EXACT NONNEGATIVE:
226318d31[5,13],6.113725772011094s,
runbb8af4990cddea21a699a8eeb550d667a99b6602dd052273ffeb8fe7e3425336;
226424d31[9,7],5.882796018035151s,
run9137d7fe6413e32662be2b0c525eabf928c2fdb6205dc08c8c13410fb28c8651;
226186d32[1,6],5.901621041004546s,
rund1240e22c62602292e0ec759269859a4fb776a63cf9a1435fe849f042a76b027.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. All finished before resource samples;no peak claim.
All handles terminal exit0,no engine remains.
222055 interior-shrink now82 exact nonnegative,8 unknown,
19 dimension prunes,8 objective prunes,14 duplicates not revalidated.
Initial d31 gaps resolved;next d32 sources226228[2,13],226258[3,13],
fresh full query before further work. Do not repeat terminal identities.
Reports runs/fan-retry{226318,226424,226186}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No smaller negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Four smallest interior-shrink gaps resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,ALL EXACT NONNEGATIVE:
226220d30[2,9],1.8343608169816434s,
run078a7a6e5f3b84d6715df96b81f0de47a1056e6757173c273f741c82a1dcd2d4;
226218d31[2,8],2.222233423963189s,
run0584ea308d2c92555ebba137c008c6c63d8820f4d27e7a73a25988f7e03b113d;
226248d31[3,8],3.6233510290039703s,
run1b9a09638243540b990e33038afc00abb364827f7e19abdb4fd9a6ce864ab95c;
226288d31[4,13],6.180804442963563s,
runfca237f8d88a039aba05e447772d73b8aa417fbf569662dd3356a5e5bf5cf669.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. All finished before resource samples;no peak claim.
All handles terminal exit0,no engine remains.
222055 interior-shrink now79 exact nonnegative,11 unknown,
19 dimension prunes,8 objective prunes,14 duplicates not revalidated.
Next smallest known gap226318d31[5,13];fresh full query before further work.
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{226220,226218,226248,226288}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Retain negative222055/cert222064 and174551/cert174566,d33 seven/five,
presentation counts not minimum distance. No smaller negative or flagged
Kostka counterexample. Goal active,no global blocker;no push/publication,
extra AI workers or admin changes.

## New negative lineage interior-shrink neighborhood completed — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no222055
interior-shrink queue. Source unchanged. Mutated RAW222055,not certificate.
All131 row/column cuts excluding four corners,cap7 extra opposite/d33,
verified beam32,5s/case,45s batches,60s outer,nice10,serialized.
Actual offsets0->29->51->116->131,no repeats:
0 batch39.07786673400551s,17 exact nonnegative/5 timeouts/4 dim/1 objective/
2 duplicates,rundd789cf3cfc037e4cd9f95920c6c1fb023885df696fac9d74bc8158dfa3fa134;
29 batch39.35564699897077s,13 exact nonnegative/4 timeouts/4 dim/1 objective/
0 duplicates,run79c73fd05550a989d5835f87578c0f39cdb6e00b623ab34c099331b4337305c1;
51 batch39.3342923700111s,36 exact nonnegative/5 timeouts/8 dim/4 objective/
12 duplicates,rune75686a63665a2bd6b1bbf1ac5812db4ce7f9d8b11f040a222d177fe6b6cbe75;
116 batch8.614275423926301s,9 exact nonnegative/1 timeout/3 dim/2 objective/
0 duplicates,run4fbcc8bf3b00d9caceadf9de9bb5766caff518b183cd4fc183589f30666ef1c4.
Total75 exact nonnegative,15 unknown,19 dimension prunes,8 objective prunes,
14 duplicates not revalidated;0 negative/empty/errors.
Fresh DB aggregation confirms131 terminal outcomes plus131 started records.
Next smallest unknown226220d30[2,9];d31 sources226218[2,8],226248[3,8],
226288[4,13],226318[5,13]. Fresh query before retries.
Samples(not peaks),NI19:PID3008080 under1s CPU100%,RSS80032KiB;
PID3010381 at1s100%,RSS60756KiB;PID3011439 at3s100%,RSS75928KiB.
All handles terminal exit0,no engine remains.
Reports runs/fan222055-interiorshrink-seven-d33-offset{0,29,51,116}-20260910.jsonl
ignored/untracked. No remote submission this turn.
Prior paired queue36 exact nonnegative,no attempted gaps;prunes uncounted.
Lower-remove-two still unknown223762d35 after120s;do not repeat unchanged.
Retain negative222055/cert222064 and174551/cert174566,d33 seven/five,
presentation counts not minimum distance. No smaller negative or flagged
Kostka counterexample. Goal active,no global blocker;no push/publication,
extra AI workers or admin changes.

## All attempted paired counts resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,both EXACT NONNEGATIVE:
226052d36[197,66],23.139749318012036s,
run9485d75b44de4b9c9e84f62e29dfd33edcede8182043ac629f45a0f67ca6a778;
226094d36[239,66],22.639588807011023s,
run6a659a0e25749ec7e51bd6dd21d0ab73a022ecb9fae34b4eb7af51ba05430468.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Full DB anti-join confirms zero original paired timeouts without
an exact retry. Samples(not peaks):PID3005028 at11s CPU100%,RSS254464KiB;
PID3005442 at8s CPU99.8%,RSS286552KiB;bothNI19.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now36 exact nonnegative,0 attempted unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Not a positivity certificate for all189 proposals:prunes/skips are uncounted.
Next new mutation neighborhood (joint/internal/shape/weight from222055),
or selected pruned case with fresh bounds;query DB before starting.
Do not repeat completed queue/terminal retry identities unchanged.
Reports runs/fan-retry{226052,226094}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
Other neighborhood lower-remove-two still unknown223762d35 after120s.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Two more degree36 paired gaps resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,both EXACT NONNEGATIVE:
225968d36[125,66],22.710884660016745s,
run789bdcd64ab5e876d77a72f53b5ff6be70a26233f3656a34de5f36345ab37a99;
226010d36[159,66],23.06931304198224s,
run4f00e9f8eddc1e0c2edae72e25478305fb292e883fe55ed1b8bd2c344631fb41.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Samples(not peaks):PID3003146 at10s CPU99.9%,RSS235836KiB;
PID3003549 at15s CPU99.9%,RSS344744KiB;bothNI19.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now34 exact nonnegative,2 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Remaining d36 sources226052/226094;fresh query before further work.
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{225968,226010}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Two further degree36 paired exact results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,both EXACT NONNEGATIVE:
225884d36[69,66],22.105030659004115s,
run855a98d8bea98826f0fae36b4120553f36f0d7c90b1ac6ff8517ff8005ac8d27;
225926d36[95,66],24.61860132799484s,
runef83ff438e1017f517008ed3245b0f22f975b00978ba4302c3c5d0a05945c4b5.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Samples(not peaks):PID2998119 at10s CPU100%,RSS228580KiB;
PID2998655 at15s CPU99.9%,RSS360056KiB;bothNI19.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now32 exact nonnegative,4 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Remaining d36 sources225968/226010/226052/226094;fresh query before further
work. Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{225884,225926}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Two degree36 paired exact results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,both EXACT NONNEGATIVE:
225800d36[29,66],21.843502103933133s,
run80744bf6ef425f41991cdf9cfeaf9f88ead27bb6f97e53f8c11cf80412b8a3c0;
225842d36[47,66],22.474368239054456s,
runf6f1e1b653d76b7c3116c9552111f76facf4fc0cb1cc5561395ba41f752813cf.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Samples(not peaks):PID2995563 at12s CPU99.9%,RSS93384KiB;
PID2996961 at8s CPU99.8%,RSS287400KiB;bothNI19.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now30 exact nonnegative,6 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Remaining d36 sources225884/225926/225968/226010/226052/226094;
fresh query before further work. Do not repeat terminal identities.
Reports runs/fan-retry{225800,225842}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Last degree34 and first degree36 paired gaps resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,both EXACT NONNEGATIVE:
226100d34[239,82],4.2636244220193475s,
run498ff57b397cdb9ef581c300ce38002b479bba9dac5e9de914542d37596c1618;
225758d36[10,66],12.800480149919167s,
rune3919ee12d2b480029a8660ba5a1bd17c5b5e4418c2ed506db2a0ad53fb52616.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Both finished before resource samples;no peak claim.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now28 exact nonnegative,8 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
All initial d34 timeouts resolved. Next d36 sources225800/225842/225884;
fresh full query before further work. Do not repeat terminal identities.
Reports runs/fan-retry{226100,225758}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Five further degree34 paired exact results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,ALL EXACT NONNEGATIVE:
225890d34,4.426516245934181s,
run25adf6fb38aebe10c01de97e76aa6110e2c8a764758b8f56ce41159c6d7246b8;
225932d34,4.463943185051903s,
run1367c8f3dad031962376d8c96e564de9a5a190283c6029aa73228043aed8d32f;
225974d34,4.240782696055248s,
runb74261bb1e0aba6c4f2d02065a35e5f3978583270f9e30563294673bf8b3a960;
226016d34,4.2570748190628365s,
runbd1d8606eb5419c2d8e74c811824104f4f62c152b91728a31a457835181f2939;
226058d34,4.270357314962894s,
run7e3106d7eea2001a52012839e9a7f221e682e873c3824a87962c6b5cc5d4c45b.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. All finished before resource samples;no peak claim.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now26 exact nonnegative,10 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Fresh full query next never-retried226100d34[239,82];then d36 sources
225758[10,66],225800[29,66],225842[47,66],225884[69,66].
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{225890,225932,225974,226016,226058}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Retain negative222055/cert222064 and174551/cert174566,d33 seven/five,
presentation counts not minimum distance. No improved negative or flagged
Kostka counterexample. Goal active,no global blocker;no push/publication,
extra AI workers or admin changes.

## Three degree34 paired exact results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,ALL EXACT NONNEGATIVE:
225764d34[10,82],7.663365467102267s,
rundc84f86e71d13fc8cf420f0ad446c177cc37b4efcde817679894dbaadf3b9d1a;
225806d34[29,82],7.787344016018324s,
run348070493db6348358902c36427a3acb26650d02bcf63f139c7d8fb01d39bb8a;
225848d34[47,82],4.190836225985549s,
run1afe190bf79511c74f354efbcf9641c64bce622c1da6ed90df5288ab00a40172.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. All three finished before resource samples;no peak claim.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now21 exact nonnegative,15 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Fresh full query next never-retried d34 sources225890[69,82],225932[95,82],
225974[125,82],226016[159,82],226058[197,82].
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{225764,225806,225848}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Remaining degree33 paired children resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks and full
next-gap query found never-retried selected sources. Source unchanged.
Serial coordinate-verified beam128/30s,60s outer,nice10,ALL EXACT NONNEGATIVE:
226062d33[197,98],5.26006815303117s,
run9f63419af447190ceaa2adc6de5dabe37e4aa87ffdffa4eb5597e55dd56603e7;
226068d33[197,132],22.797635232913308s,
run991a28446874710a713910b7bee2b0603b1f94a1bdd4f0a495b42c8fdba288a2;
226104d33[239,98],5.334174069925211s,
run86efc3d85e1309d16c492abc78980d79a2a89531a0b201869e624530f98a5a64;
226110d33[239,132],23.195089519023895s,
runb6a716d425bbd503fac37b8b41df8440b2bf652334bd3433e7ad4395f186f034.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Samples(not peaks):PID2984578 at8s CPU99.8%,RSS443692KiB;
PID2986735 at10s CPU99.9%,RSS572704KiB;bothNI19.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now18 exact nonnegative,18 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
All initial d33 timeouts resolved. Next never-retried d34 sources
225764[10,82],225806[29,82],225848[47,82];fresh full query before more.
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{226062,226068,226104,226110}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Retain negative222055/cert222064 and174551/cert174566,d33 seven/five,
presentation counts not minimum distance. No improved negative or flagged
Kostka counterexample. Goal active,no global blocker;no push/publication,
extra AI workers or admin changes.

## Two more degree33 six-opposite results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,both EXACT NONNEGATIVE:
226020d33[159,98],5.224159801960923s,
runf6aaf0e98432c6a760da53e9951beef471626d0a957c09ad242edc58e233b8b1;
226026d33[159,132],22.621771650970913s,
runc9daa9be6708d58f5c10d5ca5cf12427351ae5583f25ba96cab46978ad40cb62.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. SamplePID2983163 at9s CPU99.8%,RSS572628KiB,NI19,not peak.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now14 exact nonnegative,22 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Next never-retried d33 sources226062[197,98],226068[197,132],
226104[239,98];fresh full query before further work.
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{226020,226026}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Two degree33 paired exact results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,both EXACT NONNEGATIVE:
225978d33[125,98],5.2586524749640375s,
runb729d1a72a147abf3b6b8bb741b39b22c76d5dbd7af3867149dcc9e6688e76f5;
225984d33[125,132],23.463017511065118s,
run7b4faf12be5e96f023b2fa43bfbeb31bc9ee6b97c1d5db9c6df5764c0d5262f7.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. SamplePID2981288 at9s CPU99.8%,RSS572816KiB,NI19,not peak.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now12 exact nonnegative,24 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Fresh full query next never-retried d33 sources226020[159,98],
226026[159,132],226062[197,98],226068[197,132],226104[239,98].
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{225978,225984}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Three more degree33 paired children resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/full DB next-gap query
found never-retried selected sources. Source unchanged.
Serial coordinate-verified beam128/30s,60s outer,nice10,ALL EXACT NONNEGATIVE:
225900d33[69,132],22.59351926506497s,
run819dd1eec13c8f62d362e30fa6f5de9647422e8c985e6a3b6a91effc4cff40c7;
225936d33[95,98],5.228341378970072s,
run48a9bfd193ca9520a73a17d16a25e06bf1d23c7b9581e6828ca5f58e4b24fde5;
225942d33[95,132],22.815395847079344s,
run5ba42346aaccc6df0a57ccd75e0b551bb8f4cfe1cefd84386d01b37a33348d6b.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Samples(not peaks):PID2979258 at13s CPU99.8%,RSS456504KiB;
PID2979904 at13s CPU100%,RSS774668KiB;bothNI19.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now10 exact nonnegative,26 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Next smallest never-retried225978d33[125,98],225984d33[125,132];
fresh full query before further work. Do not repeat terminal identities.
Reports runs/fan-retry{225900,225936,225942}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Retain negative222055/cert222064 and174551/cert174566,d33 seven/five,
presentation counts not minimum distance. No improved negative or flagged
Kostka counterexample. Goal active,no global blocker;no push/publication,
extra AI workers or admin changes.

## Four degree33 paired children resolved exactly — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,ALL EXACT NONNEGATIVE:
225768d33[10,98],2.7859753200318664s,
runa4ea92b282e6c1f4d5cb980b9c006d88f5a2c521a6a71e5536414d76aa9faa3e;
225810d33[29,98],2.8500081499805674s,
run27a6b1d8ec3fd3f9f05c4c6c9c7b7d88abf10ebc9fcfb1c6884dc3a147ca9cb3;
225852d33[47,98],2.878967994940467s,
runece929d959147ecfe21ee089c39f17d7d4f05e84a91c1d4caea4891be9e01ad0;
225858d33[47,132],22.632266194093972s,
run0d7a1fcb9b816665ba299b17ddc237a75a1dc9c8053e7e9e06e9ff649d251a14.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. SamplePID2978109 at10s CPU100%,RSS572700KiB,NI19,not peak.
All handles terminal exit0,no engine remains.
222055 paired neighborhood now7 exact nonnegative,29 unknown,
27 dimension prunes,125 objective prunes,1 duplicate not revalidated.
Next smallest unknown225900d33[69,132];fresh full query before further work.
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{225768,225810,225852,225858}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Retain negative222055/cert222064 and174551/cert174566,d33 seven/five,
presentation counts not minimum distance. No improved negative or flagged
Kostka counterexample. Goal active,no global blocker;no push/publication,
extra AI workers or admin changes.

## New lineage paired boundary-flag/removal neighborhood completed — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no222055
paired queue. Source unchanged. Mutated RAW222055,not expanded certificate.
All189 paired boundary-flag plus opposite-removal proposals,cap6 extra opposite,
dimension38,verified beam32,5s/case,45s batches,60s outer,nice10.
Actual resume offsets0->55->97->139->181->189;no timeout repeats:
0 batch40.06692993408069s,2 exact nonnegative/7 timeouts/9 dim/37 objective,
runedb889d46b646cbcd3a206e01577889dfabe6b8eaa62c470992742bfc3b277bb;
55 batch39.06709304894321s,1 exact nonnegative/7 timeouts/6 dim/28 objective,
runf57f26f53ba513558e9bee0690f2e62154bdb35abd74d5548c912e646d5072d0;
97 batch41.12517149199266s,0 exact/8 timeouts/6 dim/28 objective,
runbf733be6b4ab17b4e944cb7df5c5ab0d40256b6c39267d6592b3da979c47cc14;
139 batch41.12330464599654s,0 exact/8 timeouts/6 dim/28 objective,
run4b91f9b1140dc42bed60095e67907c67543601d5372d30c7d9b5c55622f473c1;
181 batch15.233621210092679s,0 exact/3 timeouts/0 dim/4 objective/1 duplicate,
run03bf43a5ae95fd215766ad8c53bc92dec8fb8ae208f83a444a8659b564cd3a67.
Total3 exact nonnegative,33 unknown,27 dimension prunes,125 objective prunes,
1 duplicate not revalidated;0 negative/empty/errors. Fresh DB aggregation
confirms189 terminal outcomes plus189 started records.
Next smallest d33 timeouts225768[10,98],225810[29,98],225852[47,98],
225858[47,132],225900[69,132];fresh query before retry.
Samples(not peaks),NI19:PID2973860 at1s99.3%,RSS195140KiB;
PID2975083 at4s99.7%,RSS23612KiB;PID2976140 at1s99.3%,RSS175384KiB.
All handles terminal exit0,no engine remains.
Reports runs/fan222055-paired-six-d38-offset{0,55,97,139,181}-20260910.jsonl
ignored/untracked. No remote submission this turn.
Previous lower-remove-two remains33 exact nonnegative,1 unknown223762d35
after120s;do not repeat it unchanged. Pruned cases are not sign conclusions.
Retain negative222055/cert222064 and174551/cert174566,d33 seven/five,
presentation counts not minimum distance. No improved negative or flagged
Kostka counterexample. Goal active,no global blocker;no push/publication,
extra AI workers or admin changes.

## Both remaining degree36 gaps resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found only30s
selected attempts. Source unchanged. Serial coordinate-verified beam128/120s,
150s outer,nice10. Finite extension justified by nearby31-48s completions and
maintained120s support;no global cap change.
225172d36,EXACT NONNEGATIVE31.45088953000959s,
run623c183c43c6f00e057669a7de8e268e2bac923776adc38d17f94de18ece8eec;
225178d36,EXACT NONNEGATIVE31.63335230294615s,
run876dac288ff9de0a54dda0812512d296a98cbf78422af237a355cc2d6c077573.
Fresh DB readback confirms120s exact results alongside preserved30s timeouts;
original5s attempts retained. Samples(not peaks):PID2970224 at8s CPU99.8%,
RSS129908KiB;PID2970906 at10s CPU100%,RSS218276KiB;bothNI19.
Same handles monitored to terminal exit0,no engine remains.
222055 lower-remove-two now33 exact nonnegative,1 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Only attempted unresolved source223762d35 has a120s beam128 bound.
Do not repeat it unchanged. Next distinct verified layout or new mutation
neighborhood from retained negative222055;pruned cases remain uncertified.
Reports runs/fan-retry{225172,225178}-beam128-120-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Remaining degree35 cases:one exact,one120s bound — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found only30s
selected attempts. Source unchanged. Serial coordinate-verified beam128/120s,
150s outer,nice10. Finite extension justified by nearby31-44s completions
and maintained120s support;no global cap change.
223762d35,time_limited120.06626558396965s,sign UNKNOWN,
run3a2076075e72a17dd45706267488518f552ddcbf3fdc0890f4be099ce501d6af;
225634d35,EXACT NONNEGATIVE47.544351110002026s,
run3b8fd44d91a9c17704da5d7850fc23ffb55f5cb64d6ecbf8a039b97b6223a496.
Fresh DB readback confirms statuses alongside preserved30s timeouts;
original5s attempts retained. Timeout has no exact cache/negative list.
Samples(not peaks),allNI19:
PID2966933 at10s CPU99.8%,RSS261724KiB;37s99.8%,887032KiB;
69s99.8%,585920KiB;96s99.9%,2257568KiB.
PID2968947 at6s CPU100%,RSS165764KiB;34s100%,725064KiB.
Same handles monitored throughout,to terminal exit0,no engine remains.
222055 lower-remove-two now31 exact nonnegative,3 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Remaining unknown223762d35 now120s bound;225172/225178d36 only30s bounds.
Next selected finite longer d36 count or distinct verified layout/mutation;
do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{223762,225634}-beam128-120-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Two degree35 gaps resolved beyond previous time limit — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found only30s
attempts for selected sources. Source unchanged. Serial coordinate-verified
beam128/120s,150s outer,nice10. Finite extension justified by smallest remaining
five-opposite d35 cases and preceding d34 successes;maintained120s support,
no global cap change.
223258d35,EXACT NONNEGATIVE31.184147646999918s,
rune543e8c0f2eb1a5b0d2c7ce13dbff338748985c62ea45f772ff68afcb9ad8499;
223530d35,EXACT NONNEGATIVE31.71710367000196s,
run86b64dd0d9a79e14928b740811e8e8795325c22f9747a2719bae65a441bb36a4.
Fresh DB readback confirms120s exact results alongside retained30s timeouts;
original5s attempts retained. Samples(not peaks):PID2964905 at10s CPU99.9%,
RSS386572KiB;PID2965695 at6s CPU99.5%,RSS153288KiB;bothNI19.
Same handles monitored to terminal exit0,no engine remains.
222055 lower-remove-two now30 exact nonnegative,4 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Remaining30s unknowns223762/225634d35;225172/225178d36.
Next selected finite longer count or distinct mutation;do not repeat terminal
budget/layout identities unchanged.
Reports runs/fan-retry{223258,223530}-beam128-120-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Both remaining degree34 gaps resolved with finite longer counts — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found only30s
attempts for selected sources. Source unchanged. Two serial coordinate-verified
beam128/120s retries,150s outer,nice10. Longer finite bound justified by these
smallest remaining five-opposite d34 cases and maintained tested120s support;
first finished44s,then same bound used for second. No global cap changes.
225214d34,EXACT NONNEGATIVE43.68497831199784s,
rundb557a597224db671e3eaa52728219bd000c7f22b5db733e64d38b3995e0fe36;
225648d34,EXACT NONNEGATIVE35.66983627201989s,
runfc322b04f7ba9760acc8bd9c780c3dfa823768ab011f5868fecd0d879005f00b.
Fresh DB readback confirms120s exact results alongside preserved30s timeouts;
original5s attempts also retained. Samples(not peaks):
PID2962472 at9s CPU100%,RSS165244KiB;at36s99.9%,RSS606980KiB;
PID2963315 at10s CPU99.9%,RSS572740KiB;allNI19.
Same handles monitored to terminal exit0,no engine remains.
222055 lower-remove-two now28 exact nonnegative,6 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
No attempted d34 gap remains. Remaining30s unknowns:
223258/223530/223762/225634d35;225172/225178d36.
Next selected longer finite count or distinct mutation;do not repeat terminal
budget/layout identities unchanged.
Reports runs/fan-retry{225214,225648}-beam128-120-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## All first longer retries completed in five-opposite neighborhood — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10:
225168d36,EXACT NONNEGATIVE13.580820686067455s,
rundc61b66131740f3200787539fbed7a2a650aab9b8737700ea0e86aadae34bcc0;
225172d36,time_limited30.004414355033077s,
run3d7c1c35219fad7248a04a2b622d11185ce3853ed62b0683d3875736c4b901cc;
225178d36,time_limited30.01146365806926s,
rund104da40a5ddb67135344924efed84a09a8b33c1ea4d6466092440ab81d3c449.
Both timeouts sign UNKNOWN,no polynomial/negative list or exact cache assigned.
Fresh DB readback confirms statuses;original5s attempts retained.
Samples(not peaks):PID2959611 at8s CPU99.8%,RSS165440KiB;
PID2960219 at8s CPU99.8%,RSS122324KiB;
PID2960722 at11s CPU99.9%,RSS43572KiB;allNI19.
All handles terminal exit0,no engine remains.
222055 lower-remove-two now26 exact nonnegative,8 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
All31 original5s timeouts now have a30s beam128 retry.
Fresh DB unresolved list:225214/225648d34;
223258/223530/223762/225634d35;225172/225178d36.
Next use selected justified longer finite counts or distinct verified layouts,
or a new mutation neighborhood;do not repeat terminal identities unchanged.
Reports runs/fan-retry{225168,225172,225178}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Three degree36 five-opposite results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,ALL EXACT NONNEGATIVE:
224022d36,22.43934472592082s,
runce02b5fc4bf13d68aa3a5cdaf2f2084da920297f62e30323d59a85b605203201;
224026d36,22.040210079983808s,
runb1d86b703481ffb7a7976368fd780536cc6acfc36f8ae2791061842d3c061eaa;
224032d36,2.492270046961494s,
run552734658eb1eea036755912c3776213d1db757037d4a7b39e39c3d39c81a9e5.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. Samples(not peaks):PID2957814 at10s CPU99.8%,RSS322276KiB;
PID2958140 at9s CPU100%,RSS227064KiB;bothNI19.
All handles terminal exit0,no engine remains.
222055 lower-remove-two now25 exact nonnegative,9 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Next never-retried d36 sources225168/225172/225178. Other six unknowns
have30s bounds,not sign certificates. Do not repeat terminal identities.
Reports runs/fan-retry{224022,224026,224032}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Last first-retry degree35 cases — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10:
224758d35,EXACT NONNEGATIVE11.152163307997398s,
run2fc393d198fed99978259b5cf9fb1f17368d79a64098d7aecf5f824709c8f317;
225628d35,EXACT NONNEGATIVE6.4389412510208786s,
run155ea5c1b8a78e1fdd0e5b729b6084573178732752c65e8c0707ab3475cae542;
225634d35,time_limited30.036868914030492s,sign UNKNOWN,
runc8d42ea5f2d19cd8d77a956b7e17205206d593b876aaab0e2209bd8aa0300f5f.
Fresh DB readback confirms statuses/sign fields;original5s attempts retained.
Timeout has no polynomial/negative list and no exact cache assigned.
Samples(not peaks):PID2955637 at9s CPU99.7%,RSS118132KiB;
PID2956228 at10s CPU100%,RSS208568KiB;bothNI19.
All handles terminal exit0,no engine remains.
222055 lower-remove-two now22 exact nonnegative,12 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Fresh full DB query:all d35 initial timeouts now have a30s beam128 attempt.
Six never-retried d36 sources remain:224022[5,10,66,82],224026[5,10,66,98],
224032[5,10,66,132],225168[8,14,66,82],225172[8,14,66,98],
225178[8,14,66,132]. Other six unknowns have30s bounds,not sign certificates.
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{224758,225628,225634}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Two further degree35 exact results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Two serial coordinate-verified beam128
30s counts,60s outer,nice10,both EXACT NONNEGATIVE:
224748d35,8.195542234927416s,
runee53f0f926d290031bf7ca46cfba2435b7229a91d0a4786c8496dc9db810fe72;
224752d35,24.49179301399272s,
run8ddc8594fa070989f24afb23b02a2173ac520a66e67767045f4ce9eda5b1851f.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. SamplePID2953972 at8s CPU99.8%,RSS90176KiB,NI19,not peak.
All handles terminal exit0,no engine remains.
222055 lower-remove-two now20 exact nonnegative,14 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Fresh DB next never-retried d35 sources224758[7,13,66,132],
225628[9,15,82,98],225634[9,15,82,132];then d36 sources224022/224026.
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{224748,224752}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Two degree35 results and one bound — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10:
223762d35,time_limited30.06342749495525s,sign UNKNOWN,
runb36b00b757ae1dfa4204f310cfc885dc36fe8087b9d703e7af2e6f33b2f2059e;
224368d35,EXACT NONNEGATIVE4.047469667973928s,
run45c052febda0c17f8a55bd46639fb36cc39e04d2a4c2f6ced3d81784331bc4ea;
224374d35,EXACT NONNEGATIVE2.446285195997916s,
run7c508141316fcdbacefb424814a239e935fed04aa817d35c6aaac795049bd621.
Fresh DB readback confirms statuses/sign fields;original5s attempts retained.
Timeout has no polynomial/negative list and no exact cache assigned.
SamplePID2951792 at7s CPU99.8%,RSS385776KiB,NI19,not peak.
Same handles monitored to terminal exit0,no engine remains.
222055 lower-remove-two now18 exact nonnegative,16 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Next never-retried d35 sources224748/224752;fresh query before further work.
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{223762,224368,224374}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Two degree35 counting bounds recorded — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Two serial coordinate-verified beam128
30s counts,60s outer,nice10,both sign UNKNOWN:
223258d35,time_limited30.014433222007938s,
runf48f215bbc7ad22aae9b1207e6b780521741cae382013c294efe14c5d8a5ad1e;
223530d35,time_limited30.011964987032115s,
run45ca5331e82a04172745f75278ce7990259671d843eeb1d740d089723b5e5fc4.
DB readback confirms time_limited and absent negative lists;no exact cache
assigned. Original5s attempts retained. Samples(not peaks):
PID2949489 at10s CPU99.9%,RSS309236KiB;
PID2950060 at13s CPU99.6%,RSS324548KiB;bothNI19.
Same handles monitored to terminal exit0,no engine remains.
222055 lower-remove-two still16 exact nonnegative,18 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Fresh DB next never-retried d35 sources223762[4,8,82,132],
224368[6,11,82,98],224374[6,11,82,132],224748[7,13,66,82],
224752[7,13,66,98]. Distinct layouts or justified longer finite counts
also remain available for timeouts;do not repeat terminal identities unchanged.
Reports runs/fan-retry{223258,223530}-beam128-30-20260910.jsonl untracked.
No remote submission this turn. Retain negative222055/cert222064 and
174551/cert174566,d33 seven/five,presentation counts not minimum distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Three degree35 five-opposite results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no prior
selected retries. Source unchanged. Serial coordinate-verified beam128/30s,
60s outer,nice10,ALL EXACT NONNEGATIVE:
222874d35,6.433748017065227s,
run02117e2da75440f9f6440a44fb4e42832868e8c8f7a6a9b8b432d2ff40a815e8;
222878d35,27.321325698983856s,
run1a78f28052472c69e9c87353b086237c75c4f614b50fc119f9ce61cbfa9963d1;
223254d35,13.472556616994552s,
runecc166687503be2c0749684fb4c1d2be36061c45411815975c345e4db5162cff.
Fresh DB readback confirms statuses/empty negative lists;original5s attempts
retained. SamplePID2947497 at9s CPU100%,RSS223488KiB,NI19,not peak.
All handles terminal exit0,no engine remains.
222055 lower-remove-two now16 exact nonnegative,18 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Next d35 sources223258/223530;d34 gaps225214/225648 remain unknown after30s.
Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{222874,222878,223254}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Retain negative222055/cert222064 and174551/cert174566,d33 seven/five;
presentation counts,not certified minimum bad-edge distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Remaining degree34 five-opposite retries — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
selected retries. Source unchanged. Three serial coordinate-verified
beam128/30s counts,60s outer,nice10:
225208d34[8,14,82,98],EXACT NONNEGATIVE9.273635040037334s,
run9fa1ba1db0d2a795d28d157fd83a57168afb1a8cc17897f78e4fa66bb5d921fd;
225214d34[8,14,82,132],time_limited30.02190796099603s,
run1c0288aa5f2da543ced2d511a87b029f5593c1986681bea21e79593affa406dd;
225648d34[9,15,98,132],time_limited30.0752152960049s,
run8ad1401f6cd9af79465905495d8b60f0ee23c5112d57d1515d2d89b1cb132ff0.
Both timeouts sign UNKNOWN,no polynomial/negative list assigned.
Fresh DB readback confirms all statuses. Original5s attempts retained.
Samples(not peaks):PID2944610 at7s CPU99.8%,RSS254636KiB;
PID2945259 at8s CPU99.8%,RSS572684KiB;bothNI19.
Same handles monitored to terminal exit0,no engine remains.
222055 lower-remove-two now13 exact nonnegative,21 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
All original degree34 timeouts have now received a30s beam128 retry;
225214 and225648 still unresolved. Degree35 sources222874/222878/223254/
223258/223530 remain eligible;or justified longer bound/layout audit for
the two d34 gaps. Do not repeat terminal budget/layout identities unchanged.
Reports runs/fan-retry{225208,225214,225648}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Best negative222055/cert222064 and174551/cert174566 remain d33 seven/five,
presentation counts rather than certified minimum bad-edge distance.
No improved negative or flagged Kostka counterexample.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Five more five-opposite counts resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress. Fresh clean Git/no-engine/DB checks; no prior
retry identities for selected sources. Source unchanged.
Five serial beam128/30s retries with verified coordinate permutations,
60s outer,nice10,ALL EXACT NONNEGATIVE:
223268d34,7.833549699978903s,
runf042af6b9e31d81f7d9685b3182b2c5d395c0fda0a99d5b457360ae3b5bfad08;
223536d34,10.347564200055785s,
run0335938c2c7cc63cb2de86ab82b7b0e5181c8f496c85f3a20b058066da9d9cb5;
224062d34,0.8892337959259748s,
runca9c9dddd70bc7da94c24e0f6b3be9493510d6e46b6476b48ac238dc804e3ab3;
224068d34,1.0149184799520299s,
run1aff4d4a516b054d075ff5dc185bb83d9e8f90f8d0763393bd58299ea5b6df50;
224388d34,2.253091130987741s,
run46bbf93795004ee1c5b85273870d4a7976703e2bceba8dff9d088a76975fce99.
Fresh DB readback confirms all statuses/empty negative lists.
Original5s attempts retained. Wider layout improved several runtimes;
no general runtime guarantee inferred. SamplePID2941493 at9s CPU99.8%,
RSS59808KiB,NI19,not peak. All handles terminal,no engine remaining.
222055 lower-remove-two now12 exact nonnegative,22 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Fresh full DB query corrects preliminary shortlist:three further d34 unknown
225208[8,14,82,98],225214[8,14,82,132],225648[9,15,98,132].
Thus degree34 is NOT exhausted. Next d35 unknown222874/222878/223254/
223258/223530. Do not repeat terminal budget/layout identities.
Reports runs/fan-retry{223268,223536,224062,224068,224388}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Best negative222055/cert222064 and174551/cert174566 remain d33 seven/five;
no improved negative or flagged Kostka counterexample. Metrics remain
presentation counts,not certified minimum bad-edge distance.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Three five-opposite gaps resolved exactly — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made progress;fresh clean Git/no-engine/DB checks found no
prior retries for these candidates. Source unchanged.
Three serial verified beam128/30s retries,60s outer,nice10:
224794d33[7,13,82,132],EXACT NONNEGATIVE12.26539908000268s,
run39f9d433e74440ac8f5091ce08d7fa86ff7dfe27ed82a61b97fe0f2f30e7e7d7;
225228d33[8,14,98,132],EXACT NONNEGATIVE24.31065720692277s,
run66662ace3436e6b8f016f0ef3378f94f35010a13dce9e0cec2ddba292d3c5a5f;
222888d34[1,2,98,132],EXACT NONNEGATIVE8.43492414499633s,
run5407797de980c35aa8d40515c08fbcb2366e41eec5e704db9bc033b5f2a69a15.
Fresh DB readback confirms all three statuses and empty negative lists.
Original5s timeouts retained;no duplicate attempt identity repeated.
Samples(not peaks):PID2938854 at9s CPU100%,RSS335040KiB;
PID2939305 at9s CPU100%,RSS572644KiB;bothNI19.
All handles terminal exit0,no engine remains.
222055 lower-remove-two neighborhood now7 exact nonnegative,27 unknown,
153 dimension prunes,1376 objective prunes,9 duplicates not revalidated.
Next d34 unknown223268/223536/224062/224068/224388;consider Abacus
disjoint retry batch after fresh baseline/compatibility validation.
Reports runs/fan-retry{224794,225228,222888}-beam128-30-20260910.jsonl
ignored/untracked. No remote submission this turn.
Retain222055/cert222064 and174551/cert174566,d33 seven extra opposite/five
internal bans;no improved negative and no flagged Kostka counterexample.
Metrics are presentation counts,not certified minimum bad-edge distance.
Goal active,no global blocker;no push/publication,extra AI workers or admin changes.

## Complete new-lineage five-opposite neighborhood — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made concrete progress;fresh Git clean/DB/no-engine checks.
No source edits. Resumed RAW222055 lower-remove-two at465,not prior timeouts.
All1572 proposals now covered,cap5 extra opposite/d36,verified beam32,
5s/case,45s batches,60s outer,nice10,serialized:
465->761,39.5423513710266s,1 exact nonnegative/6 timeouts/40 dim prunes/
245 objective prunes/4 duplicates,
runbf0d7eb45f193a7c3d3ec56d985f042c56d6dba8aaa749af7835aa0ad2236518;
761->1121,42.62244090298191s,1 exact nonnegative/7 timeouts/33 dim/
319 objective/0 duplicates,
run531b66d0c9464c2caf3df5350c3da8fc3d26bf917262abde90627d6abeb556cf;
1121->1349,42.50756799499504s,1 exact nonnegative/7 timeouts/15 dim/
203 objective/2 duplicates,
runca63f953e251662bf92a96cccbe26483f78b9bf60f6775024364f59e0876f2b7;
1349->1572,29.26848475006409s,0 exact/5 timeouts/18 dim/200 objective/
0 duplicates,run826a132659a156467745843ba71eed76038da7caf990da948bcc3be022d97fea.
With previous0->465:3 initial exact nonnegative,31 timeouts,153 dimension
prunes,1376 objective prunes,9 duplicates not revalidated;0 negative/empty/
errors. Fresh DB aggregation confirms1572 terminal plus1572 started records.
Prunes and duplicates are not sign conclusions;timeouts remain unknown.

Smallest gap224808d32 edge[7,13,98,132],fresh verified beam128/30s retry:
EXACT NONNEGATIVE21.809044553898275s,
runada0370cb2a0c72e17a74115f82b80be49d1000abfb780a4dbd50f0f79047169.
Fresh DB readback confirms;original5s attempt retained.
Neighborhood now4 exact nonnegative,30 unknown,prunes/skips unchanged.
Next smallest unknown224794d33[7,13,82,132],225228d33[8,14,98,132],
then222888/223268d34;eligible for distinct-budget local or Abacus batches.
Do not repeat completed queue or terminal retry identities unchanged.
SamplePID2937817 at7s CPU100%,RSS186408KiB,NI19;not peak.
Reports runs/fan222055-lowerremovetwo-five-d36-offset{465,761,1121,1349}-20260910.jsonl
and runs/fan-retry224808-beam128-30-20260910.jsonl ignored/untracked.
No engine remains after terminal monitoring. No remote submission this turn.
Retain negative222055/cert222064 and174551/cert174566,d33 seven/five;
no improved nonflag negative certified. Counts are presentation metrics,
not proven minimum bad-edge distance;no flagged Kostka counterexample.
Goal active,no global blocker. No push/publication,extra AI workers,
administration changes or unrelated edits.

## Four-internal retries and new five-opposite queue — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous status-only turn was no progress; fresh clean Git and DB checks allowed
continued computation. No global blocker. No source changes.
Recovered completed222456d37 retry without resubmitting: beam128/30s exact
NONNEGATIVE23.684528828016482s,
run89bf3c39d29fd50d1abc45949bb842c227139c7c37d18b1894ab8597144d64fe.
Next four-internal-ban child222486d37, same verified layout method/30s:
time_limited30.066539549967274s, sign UNKNOWN,
runf66e2d85f497922dc5d3df9eb71b78e3e8267048258417f1f1b81ea630218e6b.
Both original5s attempts retained; DB readback confirms statuses.
222055 lower-internal neighborhood now2 exact nonnegative,8 unknown,
35 dimension prunes. SamplePID2927921 at7s CPU99.8%,RSS624920KiB,NI19,
not peak. Reports runs/fan-retry{222456,222486}-beam128-30-20260910.jsonl
remain ignored/untracked.

Started previously absent RAW222055 lower-remove-two neighborhood:
1572 total proposals,cap5 extra opposite/d36,beam32 verified layouts,
5s/case,45s batch,60s outer,nice10. Requested count1500,actual offset0->465
in39.00811602897011s:6 timeouts,47 dimension prunes,409 objective prunes,
3 duplicates not revalidated;0 exact/negative/empty/errors.
runa5ab36b4f56a1b02d0c021b4b9bc38e82e2291956635f9f7998a8ceb8b77cf2c.
DB aggregation confirms465 terminal outcomes plus465 started records.
Resume at465,not0; six attempted polynomials remain unknown.
Report runs/fan222055-lowerremovetwo-five-d36-offset0-20260910.jsonl untracked.
SamplePID2929217 at1s CPU99.4%,RSS103988KiB,NI19,not peak.
No engine remains after monitored terminal exit.
Abacus status freshly confirmed enabled,queue active but all six KTT jobs
terminal;no remote job submitted this increment. Existing job IDs/results
remain in prior ledger sections. CPU0,1/8GiB limits unchanged.
Best negative lineages222055/cert222064 and174551/cert174566 retained:
d33,seven extra opposite/five internal bans,not minimum-distance certificates
and not flagged Kostka counterexamples. Goal active;no push/publication,
extra workers,administration changes or unrelated edits.

## New lineage internal-removal neighborhood and two exact retries — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn completed a new mutation neighborhood(progress);no global blocker.
Fresh Git clean/no-engine checks;DB found no222368 retry or222055 lower-internal
queue. Source unchanged.
Smallest prior six-opposite gap222368d32 edge[8,14,132],verified beam128/30s,
EXACT NONNEGATIVE21.652309602941386s,
run9595034670f863760b5fc03d9fcec990fde915d8a7afd20b8a9a669f4d861048.
Original5s timeout retained;222055 lower-remove neighborhood now12 exact
nonnegative,25 unknown,24 dim prunes,105 objective prunes,6 duplicates.
SamplePID2918932 at17s CPU100%,RSS898196KiB,NI19,not peak.

Raw222055 lower flag plus internal-ban removal:all45 proposals,cap7 extra
opposite/d40,beam32 verified layouts,5s/case,45s batches,60s outer,nice10.
Offsets0->32->45,41.6412309280131s/10.64648766501341s:
first8 timeouts/24 dimension prunes,
run82941bd3ecdf3e6d5b5f9027ee4fa957f701a38764ddba9b24dc5f41fadc64d0;
last2 timeouts/11 dimension prunes,
run8a70a5d29cf62022d9173fcb03938c6e397b39e772ed51f65cccb3b20be94198.
Total10 initial unknown,35 dimension prunes;no exact/negative/duplicate/
objective prune/empty/error. Every proposal/outcome persisted,actual offsets
resumed without repeating timeouts. Fresh DB aggregation confirms.
Smallest4-internal-ban child222476d36 edge[7,13,113],verified beam128/30s,
EXACT NONNEGATIVE13.257966834004037s,
runbdc448211925416f1c3c73eb1bd68e789b73e45c2a0e6de629b4f56eebad3b5e.
DB readback verifies both retries;original5s attempt retained.
Lower-internal neighborhood now1 exact nonnegative,9 unknown,35 dim prunes.
Next d37 unknown222456[5,10,113],222486[8,14,113];or new five-opposite
lower-remove-two neighborhood from222055. Do not repeat terminal identities.
Reports runs/fan-retry{222368,222476}-beam128-30-20260910.jsonl and
runs/fan222055-lowerinternal-seven-d40-offset{0,32}-20260910.jsonl untracked.
All counts60s outer/reduced priority;no engine left at terminal check.
Retain222055/cert222064 and174551/cert174566 as negative starting points,
both d33 seven/five;no reduced-nonflag negative certified this turn.
No push/publication,extra workers,admin changes or remote job.
Goal active,no global blocker.

## New negative lineage:complete six-opposite removal neighborhood — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn certified a new negative lineage(progress);no global blocker.
Fresh Git clean/no-engine checks;DB showed only222055's certificate,no scans.
Mutated RAW222055 (not expanded certificate222064):lower flag plus one
opposite removal,cap6 extra opposite after all implied flags,dimension cap38,
beam32 full-coordinate-verified layout,5s/case,45s batch,60s outer,nice10.
All172 proposals completed with actual resumed offsets0->57->104->146->172:
0 batch43.791144440067s,5 exact/7 timeouts/9 dim/33 objective/3 duplicates,
rund47451ea98cc548693fa6afc2cc5645f394586a4516ccf1152a60625bcff4a47;
57 batch42.79074245202355s,2 exact/8 timeouts/8 dim/28 objective/1 duplicate,
rund388629856aeb4f2988c9db60817974f1ee147251979999a7c595dccb72d6f24;
104 batch39.64060102205258s,2 exact/7 timeouts/4 dim/27 objective/2 duplicates,
run2ed0a9ebba4af59f1688e3ad16f5a87dc185bb087236e150197906751bdc129e;
146 batch27.820302470005117s,1 exact/5 timeouts/3 dim/17 objective/0 duplicates,
runee3e21d8b0423eaf30e34c1a51c419d4c2689640037f44f6b2d71762d86455c6.
Total10 initial exact nonnegative(degrees31..34),27 unknown(degrees31..38),
24 dimension prunes(d39..48),105 objective prunes,6 duplicates not revalidated.
No negative,empty or engine errors. Fresh DB aggregation confirms totals.
Every proposal/start/outcome persisted;timeouts not revisited during resume.
SamplePID2914202 under1s CPU98.4%,RSS121040KiB,NI19,not peak.

Smallest gap222326d31 edge[7,13,132] got verified beam128/30s retry,
EXACT NONNEGATIVE14.153200346045196s,
runf3dda1d2cc70405defb63ebb4ea039866aaa5ea3c4925a769e515b07ef41d090.
DB readback confirms exact status/signs;original5s timeout retained.
Neighborhood now11 exact nonnegative,26 unknown,prunes/skips unchanged.
Next smallest unknown222368d32[8,14,132];d33 gaps222106/222180/222212/
222238/222278/222358,or new222055 lower-remove-two/lower-internal queue.
Do not repeat completed172-proposal identity or terminal retries unchanged.
Reports runs/fan222055-lowerremove-six-d38-offset{0,57,104,146}-20260910.jsonl
and runs/fan-retry222326-beam128-30-20260910.jsonl ignored/untracked.
Retain negative222055/cert222064 as active lineage:degree33,seven/five,
linear-60428124102809/24067258815600. No six-opposite negative certified yet.
Primary174551/cert174566 also retained;neither is flagged Kostka witness.
Source unchanged,no live engine at terminal check,no push/publication,
extra workers,admin changes or remote submission. Goal active,no global blocker.

## New certified degree33 negative lineage from lower flags — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn recorded bounds/layout evidence(progress);no global blocker.
Fresh clean Git/no-engine checks;DB found no selected standalone lower-flag
queues. Source unchanged. New raw negative222055,certificate222064,9x15,d33:
linear coefficient -60428124102809/24067258815600.
Seven extra opposite constraints/five internal bans (not minimum distance),
ties174566 on degree/count metrics,NOT flagged Kostka counterexample.
Derived from negative209610 by adding lower flag at edge[5,9].
Raw masks:v0x3ff01fe01fc01f801f001e001c00181f1,
h0x100080008400840184010407040e040c00.
Initial exact2.848657105001621s. Independent source and explicit-all-implied-
flag endpoint recounts beam32/15s agree fully with source polynomial/hstar,
full coordinate signatures agree;certificate4.977142857038416s,
run8b4522efd72d835730f6f7d74ffe0151da4d82c42adf3e2ab87b927d927058dd.
Certificate expanded h0x1000800184038407840f041f043e047c00.
Use RAW222055 for mutations,not expanded certificate mask.
Fresh full-coordinate comparison proves different face from primary174551,
0.011812653043307364s,
runde243ddefaa12793b411cd9c7ba7eeab0a41fab187f101dff453bc25a8eeb2a6;
new signatureSHA a167d82d1eccaf9fb585e9cf7eea597e3dbfecc910c6c97fcf1b24045eab3421.
DB search of matching negative linear coefficient finds only new222055/222064.
Keep BOTH primary and this lineage;next prioritize removing nonflag equalities
from222055 (lower-remove/lower-remove-two/lower-internal) under finite caps.

Six standalone lower-flag queues completed,52 proposals total,cap7 opposite,
beam32/5s cases/45s batches/60s outer,nice10,serialized,all outcomes in DB:
182122 all8,d30cap,7 exact nonnegative/1 duplicate,1.2056674460181966s,
rundae0038ec9dc87dbdb5863e9ccd5d1177f8f5eb99197f81c78ddc98fa6fc8ab1;
182773 all8,d30cap,7 exact nonnegative/1 duplicate,1.1984715909929946s,
run736e0af701d66a55ecd226b6cf98c0db81dce292a88f9c1758709e738fd7827d;
174551 all9,d34cap,8 exact nonnegative/1 duplicate,10.049030096968636s,
run8b5e5671220a96b3923dfb33188d036c2ef094a5a4ce2c6b8e23dfc9b93f88db;
198861 all9,d34cap,8 exact nonnegative/1 duplicate,11.322404764010571s,
run09a298869d0299cf496d3df82cf4e3d36ef97f04bc87a52a998832ab1ab7bf1f;
203068 all9,d34cap,6 exact(1 negative)/1 timeout/2 duplicates,12.664114860002883s,
run39692782ad8e6abbafb9fc4288dd9203391a41f96fd44c5df6f33dee9d699878;
209610 all9,d34cap,4 exact(1 negative)/3 timeouts/2 duplicates,23.98279121692758s,
run5b8de500a4115048a5ba80b525dc6280ebeceadb000af11c263984797ca31849.
Total40 exact(38 nonnegative,2 negative),4 unknown,8 duplicates not revalidated;
no prunes,empty or errors. DB aggregation confirms totals.
203068 negative222030d33 edge[2,3],2.278300748905167s,linear matchesprimary.
Fresh full-coordinate audit proves SAME FACE as174551,not new lineage,
0.011902331025339663s,
runa1908684dfa2b82bd548ce3354da5636a877cc90485182a724cca49701e7b7dc.
Unresolved222044d33 edge[9,15];222057d33[6,11],222061d32[8,14],222063d33[9,15].
All timeouts unknown,all duplicate skips uncertified;do not repeat queues.
Reports runs/fan{182122,182773}-lowerflag-seven-d30-offset0-20260910.jsonl,
runs/fan{174551,198861,203068,209610}-lowerflag-seven-d34-offset0-20260910.jsonl,
runs/fan-certify222055-beam32-15-20260910.jsonl ignored/untracked.
No live engine at final check;no push/publication,extra workers,admin changes
or remote submission this turn. Goal active,no global blocker.

## Four-internal-ban gaps:30s attempts and layout comparison — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn recorded new mutation bounds(progress);no global blocker.
Fresh Git clean/no-engine checks;DB found no prior retries221921/221941.
Two serial full-coordinate-verified beam128/30s attempts,60s outer,nice10:
221921d40 time_limited30.0308859430952s,
rune4ecac6d76d7a1b8c985259d7db92ebaf51759c61d81bdca7f649be4b3e2926b;
221941d40 time_limited30.09130217100028s,
run6d583d349e2c6eda339c6c0a7a9bcbae76fbf1e1d70449c541fc314d42814828.
Original5s attempts retained. With prior22193130s timeout,all three
four-internal-ban candidates remain sign UNKNOWN. No exact cache/polynomial
assigned to these failures. Fresh DB readback confirms all three statuses.
Observed samples(not peaks):PID2904974 at14s CPU99.9%,RSS664284KiB;
PID2905684 at25s CPU99.9%,RSS986104KiB;NI19.
Same handles monitored to terminal exit0,no engine remaining.
Reports runs/fan-retry{221921,221941}-beam128-30-20260910.jsonl untracked.

Fresh beam32/128 layout audit,full coordinate-permutation verification:
221921 identical orders,peak/total(6,121),0.1025418060598895s;
221931 identical orders,(6,118),0.107245501014404s;
221941 identical orders,(6,123),0.1017903620377183s.
Full orders/metadata persisted,layout-only,no polynomial count,
run8134cc60f45c7f3e20ee7f29b94019ff88320b41224a9e76b6a0db639eba5f00.
No redundant beam32 count warranted;frontier scores are not optimality proof.
Next distinct mutation or layout,or selected justified finite longer bound;
do not repeat terminal budget/layout identities unchanged.
Best negative remains174566d33 seven/five,not flagged Kostka witness.
Source unchanged,no push/publication,extra workers,admin changes or remote job.
Goal active,no global blocker;these local counting gaps are not a global impasse.

## Direct internal-ban removal from seven-opposite parents — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn established a new runtime bound and layout audit(progress);
no global blocker. Fresh Git clean/no-engine checks;DB found no internal
queues for198861/203068/209610. Source unchanged.
All5 direct internal-ban removals per parent,offset0->5,cap7 extra opposite,
dimension cap40,beam32 coordinate-verified layouts,5s/case,45s batch,
60s outer,nice10,serialized. Each queue:1 timeout,4 dimension prunes,
no exact count,negative,duplicate,objective prune,empty or error.
198861,5.2613710089353845s,
run6dc3245f47a8e66a6246ffbd6369386e860262d1c49a07ffd5e60594c10507b4;
203068,5.2783546190476045s,
run8b1ba166e4f08cea5c828f628579d8a56e24fbb6e25ace8477218606b5599ee9;
209610,5.283071193960495s,
run9620c330c9483cd1fb81d54affc8ffae6315c3d66d8a0bd76f5bd3f84960881d.
Each direct removal reduces presentation internal bans5->4.
d40 edge113 timeout children221921/221931/221941 respectively.
Remaining four per parent degrees44/47/49/50,edges128/144/161/179,
remain uncounted. No sign inference from prune/timeout.
Selected column-parent child221931 got fresh beam128/30s attempt,
time_limited30.094048516941257s,
run862559e685587ee256557fcd047b919b496072c382f1ec922175d05590b0bbbb.
Unknown sign,no polynomial/exact cache assigned. Original5s attempt retained.
SamplePID2903192 at10s CPU100%,RSS739612KiB,NI19,not peak.
Same handle observed terminal exit0,no engine remaining.
Fresh DB aggregation/readback confirms all three queues and retry.
Reports runs/fan{198861,203068,209610}-internal-seven-d40-offset0-20260910.jsonl
and runs/fan-retry221931-beam128-30-20260910.jsonl ignored/untracked.
Next distinct verified layouts or selected bounded attempts for221921/221941;
do not repeat completed identities. Three four-internal-ban candidates are
not verified negative and do not improve certified best.
Best negative remains174566d33 seven/five,not flagged Kostka witness.
No push/publication,extra workers,admin changes or remote job this turn.
Goal active,no global blocker.

## Degree37 five-opposite longer bound and layout audit — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks;DB confirmed221647 had only beam128/30 timeout before new attempt.
Fixed finite d37 poset and tested helper,prior bounded memory justify one
120s count with150s outer/nice10,serialized. Same session monitored throughout.
221647 time_limited120.20272205001675s,
runf6b7d162ed215a21b68cd70477b56fee602d647d77286269a976ef287cfaf17e.
Sign UNKNOWN;fresh DB readback confirms no polynomial evidence. Original30s
attempt retained. No exact cache or positivity inference from timeout.
Observed samples PID2899435,NI19 (not peaks):
12s CPU98.6%,RSS595044KiB;42s CPU98.7%,RSS1649740KiB;
54s CPU99.0%,RSS1230088KiB;95s CPU99.4%,RSS1117876KiB;
115s CPU99.5%,RSS3447856KiB. Engine exited under internal timeout,
helper terminal exit0,no remaining engine. No resource-limit changes.
Report runs/fan-retry221647-beam128-120-20260910.jsonl ignored/untracked.

Fresh exact coordinate-permutation layout comparison before further counting:
221647 beam32/128 identical order,peak/total(6,125),0.08588308899197727s;
221677 beam32/128 identical order,peak/total(6,126),0.08224989997688681s.
Both full orders and metadata persisted,layout-only(no polynomial count),
run0d64cd7b595cf1b540fd908c6aef736bf3a61503f6fd1a046e3beff8bf4c27b5.
No redundant beam32 recount warranted. Scores are heuristic,not optimality.
Next distinct mutation/counting layout,or selected new bounded attempt for
another unresolved candidate;do not repeat221647120s terminal identity.
198861 interior-shrink counts unchanged:87 exact nonnegative,3 unresolved,
15 original dimension prunes uncounted,26 duplicate skips not revalidated.
Best negative remains174566d33 seven/five,not flagged Kostka witness.
Source unchanged,no push/publication,extra workers,admin changes or remote job.
Goal active,no global blocker;this finite failure is not a global impasse.

## Degree35 seven-opposite smaller-shape candidates — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks;DB verified three original dimension-pruned d35 children,no retries.
Selected as possible smaller-shape negative intermediates,not an improvement
in opposite count:all7 extra opposite,198861 children8x14.
Serial beam128 full-coordinate-verified30s counts,60s outer,nice10:
221661 cut[2,13] EXACT NONNEGATIVE14.02740180096589s,
run77d5414f2448f8e399fadece8484a8995cbaacf4add8726f424ec9fdc4fd108e;
221691 cut[3,13] time_limited30.006701662088744s,sign UNKNOWN,
run82d0eba7395e9d4434d9dd3cb042fdb5ed7ef2ddb199fda8c8183e27c82bd2ee;
221721 cut[4,13] EXACT NONNEGATIVE25.320868573035114s,
runfff5f7a859c02b647ca9f8e9debf0386305ec1fe3f5c8dab8ac486f266cb8405.
Fresh DB readback confirms statuses/timings/exact sign lists. No polynomial
assigned to timeout;original dimension-prune observations retained.
Resource samples(not peaks):PID2895540 at11s CPU95.0%,RSS167608KiB;
PID2896930 at16s CPU96.4%,RSS232016KiB;NI19.
All handles monitored through terminal exit0,no engine remaining.
198861 interior-shrink neighborhood now87 exact nonnegative,
3 unresolved counted cases221691d35,221647/221677d37;
15 original dimension prunes still uncounted,26 duplicate skips unrevalidated.
No sign claims for skipped/pruned/time-limited cases. No new negative lineage.
Reports runs/fan-retry{221661,221691,221721}-beam128-30-20260910.jsonl ignored.
Do not repeat terminal identities;next distinct mutation/layout or selected
new finite bounds on unresolved fewer-nonflag candidates.
Best negative remains174566d33 seven/five,not flagged Kostka witness.
Source unchanged,no push/publication,extra workers,admin changes or remote
submission this turn. Goal active,no global blocker.

## Monitored degree34 five-opposite gap resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks;DB confirmed221855 only prior beam128/30s timeout,not120s.
Selected d34 five-opposite shrink child cut[9,6],fixed finite poset and tested
helper justify new120s count under150s outer/nice10,serialized.
Same live session monitored throughout,no duplicate restart on observation wait.
EXACT NONNEGATIVE116.05111328093335s,
run4ee05eda923add16ff128cd3f760922ca2fe02ddb197db29496ff3b24408212d.
Fresh DB readback confirms exact sign evidence and preserves original30s
time_limited30.099427095032297s. No negative witness.
Resource samples PID2891039,NI19 (not measured peaks):
9s CPU100%,RSS853148KiB;30s CPU100%,RSS1688724KiB;
58s CPU100%,RSS1939460KiB;77s CPU99.9%,RSS3208120KiB;
105s CPU99.9%,RSS67220KiB.
Local cgroup check during run:memory.max25769803776 bytes,
memory.current19189448704 bytes. No resource-limit/admin changes.
Engine terminal exit0/no remaining process confirmed.
Report runs/fan-retry221855-beam128-120-20260910.jsonl ignored/untracked;
all started/outcome evidence in local DB,full-coordinate-verified beam128 layout.
All seven d34 five-opposite children now exact nonnegative.
Entire198861 interior-shrink neighborhood now85 exact nonnegative,
2 unresolved counting cases221647/221677d37,18 original dimension prunes
still uncounted,26 duplicates not revalidated. No claims for prunes/skips.
Next distinct mutation/layout or selected finite retries of d37 gaps;do not
repeat terminal221855120s identity. Best negative remains174566d33 seven/five,
not flagged Kostka witness. Source unchanged.
No push/publication,extra workers,admin changes or remote submission this turn.
Goal active,no global blocker.

## Degree37 five-opposite counts and monitored longer retry — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh clean Git/no-engine
checks;DB verified original dimension-pruned d37 children and no prior retries.
All have5 extra opposite constraints. Serial full-coordinate-verified beam128,
30s cases/60s outer/nice10. All first attempts time_limited,unknown signs:
221647 cut[2,6],30.06255820696242s,
run69e3104a020ebef7e8666a08caf9b1db22471e58ac80a0837e33e82b05a5f816;
221677 cut[3,6],30.06485442398116s,
runff32be8a8f22f83716c4a95e7b00d67a83c32b060bf3c8ff8647b9bf8c83b0de;
221735 cut[5,5],30.023728611995466s,
run8151b830dc922e306770f9ba2e93db4cf2c97c006dd8728c4c0ec29fd46f1ed6.
Stored layout peak/total respectively(6,125),(6,126),(5,115),heuristics only.
Observed samples(not peaks),allNI19:
PID2886794 at12s CPU99.8%,RSS562196KiB;
PID2887746 at12s CPU99.9%,RSS562152KiB;
PID2888384 at26s CPU99.9%,RSS462332KiB.

Selected lower-frontier221735 for new120s attempt using tested helper,
150s outer/nice10;finite fixed poset,prior controlled memory and lower frontier
justify extended bound. Monitored SAME session through exit0,not restarted.
SamplesPID2889201 at16s CPU99.9%,RSS273500KiB;at35s RSS527356KiB,NI19.
EXACT NONNEGATIVE57.38682211807463s,
runc47a4543fb7ea2bcadabe760c3bc2006a2e174c4432d08fb0e037d7948f1eb2d.
Fresh DB readback confirms all4 statuses/times/sign evidence. Original30s
timeouts retained;221647/221677 unknown,no polynomial assigned to timeouts.
198861 interior-shrink neighborhood now84 exact nonnegative,3 unresolved
counting cases(221855d34,221647/221677d37),18 original dimension prunes
still uncounted,26 duplicates not revalidated. No sign claims for skips.
Reports runs/fan-retry{221647,221677,221735}-beam128-30-20260910.jsonl
and runs/fan-retry221735-beam128-120-20260910.jsonl ignored/untracked.
Next distinct mutation/layout or selected finite longer gap attempt;do not
repeat terminal identities. Best negative remains174566d33 seven/five,
not flagged Kostka witness. Source unchanged,no live engine at final check.
No push/publication,extra workers,admin changes or remote submission this turn.
Goal active,no global blocker.

## Four remaining degree34 five-opposite attempts — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks;DB found no prior retries for four selected198861 shrink children.
All d34,5 extra opposite,serial beam128 full-coordinate-verified30s counts,
60s outer,nice10. Source unchanged.
221737 cut[5,6] EXACT NONNEGATIVE14.735108566004783s,
run9fea4539866b788b153670103d21ac50b93f90326307aa0a10ac77984ee1d833;
221767 cut[6,6] EXACT NONNEGATIVE15.08138761704322s,
run1eddb688ae995c133f07f87810f6f917e5862fab247853693969d8cfee4dcd50;
221797 cut[7,6] EXACT NONNEGATIVE20.6611030040076s,
run5b5215f3c1cf5e3c8c1eed7ba1d64cb6d2477ad4b2327755980aea50b1116a58;
221855 cut[9,6] time_limited30.099427095032297s,sign UNKNOWN,
run4bb9f4361692ed3da2e0843a7f87a85ef17188235f2b8ce2082041a0c7581c77.
Fresh DB readback confirms statuses/timings/sign evidence. No polynomial/cache
assigned to timeout. Original dimension-prune observations retained.
Samples(not peaks):PID2883388 at9s CPU100%,RSS279348KiB;
PID2883737 at16s CPU100%,RSS206668KiB;NI19.
All handles monitored to terminal exit0,no engine at final check.
The seven d34 five-opposite children now6 exact nonnegative,1 unknown.
Entire198861 interior-shrink neighborhood:83 exact nonnegative,
1 newly counted timeout,21 original dimension-pruned cases still uncounted,
26 duplicate skips not revalidated. No sign claims for skips/prunes/timeouts.
Next uncounted d35 seven-opposite221661/221691/221721;d37 five-opposite
221647/221677/221735,or changed verified layout/new finite bound for221855.
Reports runs/fan-retry{221737,221767,221797,221855}-beam128-30-20260910.jsonl
ignored/untracked. Do not repeat terminal layout/budget identities unchanged.
Best negative remains174566d33 seven/five,not flagged Kostka witness.
No push/publication,extra workers,admin changes or remote submission this turn.
Goal active,no global blocker.

## Three degree34 five-opposite shrink children resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks;DB found seven198861 interior-shrink dimension-pruned d34 children
with5 extra opposite constraints,no prior retries. Selected first three
under serial beam128/30s counts,full-coordinate verification,60s outer,nice10.
All EXACT NONNEGATIVE:
221649 cut[2,7],19.028558501042426s,
rund02dfdc85ba984e522b0dad605f6615f330f0204dad7171837e48b7704192159;
221679 cut[3,7],22.490116961998865s,
runb88f316006f120b0fb71c3bcb2ee9af1eb3c3464e8160d787e9124826e8d7c11;
221709 cut[4,7],18.740208237082697s,
rundc4556d72add8ae51c068a1ab2eb0d2eaa1840f0b8377e8d182501af5d81ef5c.
Fresh DB readback confirms exact statuses,timings,empty negative lists.
Observed resource samples(not peaks):PID2881019 at13s CPU99.9%,RSS305064KiB;
PID2881808 at14s CPU100%,RSS323468KiB;NI19. Same live handles monitored
through terminal exit0,no remaining engine at final check.
Original dimension-prune observations retained;not reclassified scan counts.
198861 interior-shrink neighborhood now80 exact nonnegative,25 original
dimension prunes uncounted,26 duplicates not revalidated. No sign claims for
uncounted/pruned/skipped cases. No negative witness.
Remaining d34 five-opposite children for next bounded attempts:
221737 cut[5,6],221767 cut[6,6],221797 cut[7,6],221855 cut[9,6].
Reports runs/fan-retry{221649,221679,221709}-beam128-30-20260910.jsonl
ignored/untracked;all started/outcome records in DB. Do not repeat terminal
budget/layout identities. Source unchanged.
Best negative remains174566d33 seven/five,not flagged Kostka witness.
No push/publication,extra workers,admin changes or remote submission this turn.
Goal active,no global blocker.

## Row-parent shrink timeouts resolved and corners completed — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Clean Git/no-engine checked,
DB found no prior retries for221857/221811/221827 or198861 corner-shrink queue.
Source unchanged. Three serial full-coordinate-verified beam128/30s retries,
60s outer/nice10,all EXACT NONNEGATIVE:
221857d31 cut[9,7],3.759768240037374s,
rund1ef312d1c9d29fcacbf6fda53b5f586e847c39a91f43085c1615ddeaf781784;
221811d32 cut[7,13],8.083508216892369s,
rund322518aa4f7c6d501c85c8ac5911da936a113ab3060729ba48ea41c531e50f6;
221827d33 cut[8,6],13.286143208970316s,
run23c51dd54f124b351a151d80ee158826079ba8bd98cacda3abcaee6b3a513c55.
Fresh DB readback confirms exact statuses,timings,empty negative lists.
Original5s timeouts retained. Raw198861 interior-shrink neighborhood now
77 exact nonnegative,0 unresolved counting cases,28 original dimension prunes,
26 skipped duplicates not revalidated. No sign conclusion for pruned/skipped cases.

Complementary corner shrink all4 first/last row-column choices,children8x14,
cap7 extra opposite/d33,beam32/5s cases/45s batch/60s outer,nice10:
offset0->4 completed0.9629950610687956s,3 exact nonnegative,1 duplicate
not revalidated,no prunes,timeouts,empty,negative or errors,
run787387cf9dac0fd25efe5f4d195e04373c1e4e9e565db163074c15710b59a677.
DB aggregation matches. All proposals/results stored. No in-flight resource
sample retained;same live handles observed terminal,engine absent at final check.
Reports runs/fan-retry{221857,221811,221827}-beam128-30-20260910.jsonl and
runs/fan198861-cornershrink-seven-d33-offset0-20260910.jsonl ignored/untracked.
Do not repeat completed shrink queues or terminal retries unchanged.
Next uncounted dimension-pruned fewer-nonflag children or distinct mutations.
No improved negative;primary174566d33 seven/five remains,not flagged witness.
No push/publication,extra workers,admin changes or remote submission this turn.
Goal remains active,no global blocker.

## New band crossovers and row-parent shrink neighborhood — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks,DB inventory confirmed no selected band-cross or198861 interior-shrink
queues. Source unchanged. All scans5s/case,45s batch,60s outer,nice10,
beam32 full-coordinate-verified layouts when counted,all proposals persisted.

Crossovers cap6 extra opposite,dimension cap38,all finite bands:
198861+203068 row56:17 duplicates,39 objective prunes,0.8893096370156854s,
runbb1e11c9ce446aedd4a7695597e0005f18791b5cd780d0de09607f3bb70b1c55;
column182:113 duplicates,69 objective prunes,2.849870938109234s,
run9f47183379b79ce67a7a28da6baeedd8d5556cad70d8ef8a3c6ad6821bca05f7.
209610+198861 row56:41 duplicates,15 objective prunes,0.8796999440528452s,
run3029e4e8c30a943d7024796c14098f55171baed10bf59de25e0223f150b444d7;
column182:145 duplicates,37 objective prunes,2.8751443709479645s,
runc6e53a945d2a1dc8325a5cd817f5d9b196def23798d3b76294ad42078cbc01f8.
All476 proposals completed:316 duplicates not revalidated,160 objective prunes,
no counts. DB aggregation matches. No polynomial-sign claims for skips.

Raw198861 interior-shrink all131 noncorner row/column cuts,children8x14,
cap7 extra opposite/d33. Resumed actual offsets0->37->95->131:
0 batch40.15738877898548s,23 exact,9 dimension prunes,5 duplicates,
run430592bfdb270c742307906e2c9f1819971dd28545ba84c1af0e19137e66f26e;
37 batch39.45952826610301s,36 exact,14 dimension prunes,8 duplicates,
run61b7d8ea21cde2f2fbee693289c57d289cfe9ded0684baf0d592592921f144b2;
95 batch23.35509730700869s,15 exact,3 timeouts,5 dimension prunes,13 duplicates,
runfb5ec25f4fc4d85aefcbd78ae9a2f025ef8f154dcc68d9ee76f9f383eb01f0ad.
Total74 exact nonnegative(degrees15..33),3 unknown,28 dimension prunes,
26 duplicates not revalidated;no negative,objective prunes,empty or errors.
Fresh DB aggregation confirms totals and gaps:
221811d32 cut[7,13],221827d33 cut[8,6],221857d31 cut[9,7].
SamplePID2872240 under1s CPU98.4%,RSS77916KiB,NI19,not peak.
No repeated batches;all started/outcomes stored,original timeout signs unknown.
Reports runs/fan{198861-203068,209610-198861}-{rowband,columnband}-six-d38-offset0-20260910.jsonl
and runs/fan198861-interiorshrink-seven-d33-offset{0,37,95}-20260910.jsonl
ignored/untracked. Next selected221857d31/221811d32 retries or new mutation;
do not repeat completed queues unchanged.
No improved negative;primary174566d33 seven/five remains,not flagged witness.
No live engine at terminal check;no push/publication,extra workers,admin changes
or remote submission this turn. Goal active,no global blocker.

## Degree40 removal bounds and corner shrink scans — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks and DB checks showed no prior retries of182128/220627 and no dedicated
corner-shrink queues for182122/182773. Source unchanged.
Selected d40 edge98 internal-ban removals(5 internal bans),verified beam128,
30s internal/60s outer,nice10,serialized:
182128 time_limited30.09010555408895s,
run405867b3065c1914e3ea6206637a4453ad5eef0a7544e1dfeefab721c8c949ff;
220627 time_limited30.089447073987685s,
runc07504009e417b48f78b5390e11d4f2bbe0f248100660c4e0a21e94d8c28f17e.
Both signs remain UNKNOWN;original shorter attempts retained,no exact cache.
Observed samples(not peaks):
PID2866369 at9s CPU99.7%,RSS737104KiB,at27s CPU99.9%,RSS1497212KiB;
PID2866720 at17s CPU99.8%,RSS867740KiB;bothNI19.
Same handles monitored to terminal exit0;no retry beyond30s this turn.
Fresh DB readback confirms unknown status and missing polynomial evidence.

Corner shrink complements prior116 interior cuts:all4 combinations of first/
last row and first/last column per8x15 parent,children7x14,offset0->4.
Cap7 extra opposite,dimension cap32,beam32/5s cases/45s batch/60s outer,nice10.
182122:3 exact nonnegative(degrees10..25),1 duplicate not revalidated(d10),
0.20651885296683758s,
runbf08fcc75e99b84164dd998de8f14468c95b62574b17032c58dfca26ded182be.
182773:3 exact nonnegative,1 duplicate not revalidated,
0.20270695898216218s,
run1b3d4a81fdebe98de1835ddd0d73627604668a84cdac8dd424e34ba69f8a89e7.
No timeouts,prunes,empty faces,negative or errors in corner scans.
Fresh DB aggregation verifies both summaries. Six exact outcomes added;
skipped duplicates are not sign certificates. All proposals/results persisted.
Reports runs/fan-retry{182128,220627}-beam128-30-20260910.jsonl and
runs/fan{182122,182773}-cornershrink-seven-d32-offset0-20260910.jsonl untracked.
No improved negative:primary174566d33 seven/five remains,not flagged witness.
Next distinct mutation or verified counting layout for expensive removal gaps;
do not repeat completed corner scans or terminal budget/layout identities.
No live engine at final check;no push/publication,extra workers,admin changes
or remote submission this turn. Goal active,no global blocker.

## Smaller-parent internal-ban moves and removal — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh clean Git/no-engine
checks;DB found no slide-internal queues for182122/182773. Source unchanged.
Both slide-internal queues complete8 moves,offset0->8,cap7 extra opposite,
dimension cap38,beam32 verified coordinates,5s/case,45s batch,60s outer,nice10.
182122:2 exact nonnegative(d35,d36),6 dimension prunes(d40..49),
8.619659341988154s,
run68f6fc0554c6321aeff3380e55fb14693b43bf0a67e4431f2122f880a117c63c.
182773:2 exact nonnegative(d35,d36),6 dimension prunes(d40..48),
8.643010539002717s,
run82ddf1dbf2eedec8a241acd4fc458e2d6f880ffa057b64e5b617913c609a45d6.
No negative,timeouts,duplicate,objective prunes,empty or errors.
Tested moves[85,73]/[85,84] retain6 internal bans;not a distance improvement.
Exact result IDs220593/220595 and220609/220611. All outcomes stored.

Earlier182122 direct removal182126d36 already had exact beam32/30 retry
11.186896767001599s;not recounted.182128d40 had no retry.
DB showed no182773 direct-internal queue;ran all6 removals with cap7/d40,
same count/time bounds.10.30475068895612s,
rune55ab7a4809f53bd95d7ce31c00d2486f3e80a51797cdb29126fe070faf0fcd7:
220625d36 edge85 and220627d40 edge98 timed out5s;
220629d43,220631d46,220633d48,220635d49 dimension-pruned.
All direct removals reduce internal bans to5 in this presentation.
Selected220625 beam128/30s fresh exact retry NONNEGATIVE11.64237505197525s,
runb91aa61608006af7b9f40529bfb46e45b0a0511f3dc1e26e122da592d4fd5f3a.
SamplePID2865210 at11s CPU99.9%,RSS36784KiB,NI19,not peak.
Original5s timeout retained;220627 remains unknown,not positive.
Fresh DB aggregation/readback confirms three queues and selected retry.
Reports runs/fan{182122,182773}-slideinternal-seven-d38-offset0-20260910.jsonl,
runs/fan182773-internal-seven-d40-offset0-20260910.jsonl,
runs/fan-retry220625-beam128-30-20260910.jsonl ignored/untracked.
No sign claims for pruned/skipped cases. Next selected5-internal d40 gaps
182128/220627 or distinct mutation;do not repeat finished identities.
Best negative remains174566d33 seven/five,not flagged witness.
No live engine at terminal check;no push/publication,extra workers,admin
changes or remote submission this turn. Goal active,no global blocker.

## Row-parent weights and all band-parent weight timeouts resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks;DB found no198861 zero-weight queue or selected retry identities.
Source unchanged. All scan counts beam32/5s,45s batches,60s outer,nice10;
dimension cap34,no flag-only objective cap for weight-forced equalities.
Raw198861 all24 labels,offset0->23 capped39.06860043306369s:
22 exact nonnegative(degrees26..33),1 duplicate not revalidated(d29),
run395ba7c3d98e4702df3083ae6874fb23bf97dc57a53cd5153f4013be19a5e0b4.
Resume23->24 completed0.035504037979990244s:
1 duplicate not revalidated(d34),
run033c6256d88f2599cdf9f394f3d089ea9a8df49b688077452c7ece1bf2050467.
No timeouts,prunes,empty faces,negative or errors. All starts/results stored;
fresh DB aggregation matches both batches. Resource sample:
PID2860673 at3s CPU99.6%,RSS99448KiB,NI19,not peak.
Reports runs/fan198861-zeroweight-d34-offset{0,23}-20260910.jsonl untracked.

All remaining209610 zero-weight d33 timeouts got fresh verified beam128/30s
retries,serial60s guards/nice10. All EXACT NONNEGATIVE:
220459 label16,5.372656755964272s,
run78a4ea7de9569aaacce7e25cbe3c2c549932139df1e067fb2ff43bb747ea229c;
220461 label17,5.566493866965175s,
run2ff0f9a2da480a2a27638c37c73c1ed5ba3a0cd75b4e7dfa555924533eed7196;
220463 label18,5.497388293966651s,
run901dcaae3fcbeb702bcc12ffba30143befc3001b2ea22022a98c44640cef06a6;
220465 label19,5.485145371989347s,
run89c69aad16cff11f2de3f5505fad20caf36c34d7d30d274167d7dd0f4d5d42b0;
220471 label22,5.377710090950131s,
runec7d394cdf2a3f98a4417b8544043aeeefdf174cc89588f5a91da9d5aa80fd4a;
220473 label23,5.36563292494975s,
run54f8050376eff87e614790cf00c78924507cfa41194c52a433c11e45fa51b69f.
Fresh DB readback confirms all6 statuses/times/empty negative lists.
Original5s timeouts retained.209610 zero-weight neighborhood now23 exact
nonnegative,0 unresolved counts,1 skipped duplicate not revalidated.
Reports runs/fan-retry{220459,220461,220463,220465,220471,220473}-beam128-30-20260910.jsonl
ignored/untracked. No sign claim for skipped duplicate;no negative to compress.
Do not repeat completed zero-weight neighborhoods unchanged.
Next distinct mutation family or unresolved fewer-nonflag candidates;best
negative remains174566d33 seven/five,not flagged Kostka witness.
No engine live at final check;no push/publication,extra workers,admin changes
or remote submission this turn. Goal active,no global blocker.

## Column-parent zero-weight neighborhood exactly resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous goal turn made exact progress;no global blocker. Fresh clean Git/
no-engine checks and DB checks found no220469 retry or203068 zero-weight queue.
Source unchanged. First209610 child220469d32,label21,beam128/30s
EXACT NONNEGATIVE5.165876041981392s,
run1250e2dc7f16e0bb379591b99c008bfe1f1fbc67db647979c6d93af9f8a147c4.
Original5s timeout retained.209610 zero-weight neighborhood now17 exact
nonnegative,6 unresolved,1 duplicate not revalidated.

Raw203068 zero-weight all24 label choices,offset0->24,dimension cap34,
beam32 verified coordinates,5s/case,45s batch,60s outer,nice10.
No flag-only objective filter on weight-forced equalities.
Completed34.85214651399292s:21 initial exact nonnegative(degrees26..33),
2 timeouts(d33,label18/19),1 duplicate not revalidated(d34),
no prunes,empty faces,negative or engine errors,
run266a53a3c2bc4873e5e031c11650dd6857d75def544449ac27305ef83abb2b64.
Resource sample during scan:PID2857259 at2s CPU100%,RSS31672KiB,NI19,
not peak. All starts/results recorded and batch summary checked against DB.

Both timeout children resolved with serial beam128/30s retries:
220515d33 label18 EXACT NONNEGATIVE5.350675379973836s,
run29fd1b2155ffde894687699ca6dc48646608e70827b243954bf605744cfe2392;
220517d33 label19 EXACT NONNEGATIVE5.329993774998002s,
run85ab283686d8ad5c904e715a8d2a181d7527d915e988d2baed2f3e33cee1b37c.
All counts60s outer/nice10;fresh DB readback confirms three retries.
203068 neighborhood now23 exact nonnegative,0 unresolved counting cases,
1 skipped duplicate not revalidated. Original timeout observations retained.
Reports runs/fan203068-zeroweight-d34-offset0-20260910.jsonl and
runs/fan-retry{220469,220515,220517}-beam128-30-20260910.jsonl ignored/untracked.
No negative to compress/certify. Do not infer sign from duplicate skip.
Next distinct198861 zero-weight queue or remaining209610 d33 gaps
220459/220461/220463/220465/220471/220473;do not repeat terminal attempts.
No improved negative;primary174566d33 seven/five remains,not flagged witness.
No live engine at final check;no push/publication,extra workers,admin changes
or remote submission this turn. Goal active,no global blocker.

## Primary and band-parent zero-weight neighborhoods — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks. DB found no zero-weight queues for174551/209610/203068/198861;
selected first two this turn. Source unchanged.
All24 zero-label choices per9x15 parent,dimension cap34,beam32 verified
coordinate layouts,5s/case,45s batches,60s outer,nice10,serialized.
No flag-only objective filter applied to weight-forced equalities.
174551 offset0->24 complete,33.91702242102474s:
23 exact nonnegative(degrees25..32),1 duplicate not revalidated,
no timeouts,dimension prunes,empty faces or negatives,
run3749f502805575e85c8ac15686c17dd37fe141158e7234e5ad0051e9e4dffd5b.
209610 offset0->21 capped,39.27003852289636s:
15 exact nonnegative(degrees26..33),6 timeouts,
run265612f56ae432c888453862560b905b1a2a86a59ea3de5d6e430853a0656c94.
Resumed at21->24 completed,10.11105577996932s:
2 timeouts,1 duplicate not revalidated,
run42863c5f6cad8ac931474b6542cffe9dd81ddbf40cac7b48378f896c880c6ae0.
No repeated first batch;total15 initial exact,8 unknown,1 duplicate;
no prunes,empty faces,negative or engine errors.
Timeout IDs/labels:220459/16,220461/17,220463/18,220465/19(d33);
220467/20,220469/21(d32);220471/22,220473/23(d33).
Sample during209610:PID2852828 at1s CPU99.4%,RSS64956KiB,NI19,not peak.
Fresh DB aggregation confirms all three batch outcomes.

Selected220467d32 label20,beam128/30s retry EXACT NONNEGATIVE
5.082296273089014s,
run950899c4d3b5861af41caa82e8608ac3f7beff22def31580f1f3032b50544bcf.
Fresh DB readback confirms exact status/sign list. Original5s timeout retained;
209610 neighborhood now16 exact nonnegative,7 unknown,1 duplicate.
Reports runs/fan174551-zeroweight-d34-offset0-20260910.jsonl,
runs/fan209610-zeroweight-d34-offset{0,21}-20260910.jsonl,
runs/fan-retry220467-beam128-30-20260910.jsonl ignored/untracked.
All starts/outcomes/prunes stored;no compression/certificate promotion without
a fresh negative. Duplicate skips do not establish polynomial signs.
Next220469d32 or distinct203068/198861 zero-weight queues;do not repeat finished
identities. No improved negative;primary174566d33 seven/five remains,
not flagged Kostka witness. No live engine at terminal check.
No push/publication,extra workers,admin changes or remote submission this turn.
Goal active,no global blocker.

## Six degree37 paired candidates exactly resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh Git clean/no-engine
checks and DB inspection found six original dimension prunes,no prior retries.
Raw182773 paired children220086/220112/220138/220164/220190/220216,
edges[15,50]/[29,50]/[47,50]/[81,50]/[109,50]/[141,50],
all d37 with6 extra opposite constraints after implied lower flags.
Serialized beam128 full-coordinate-verified30s counts,60s outer,nice10.
All EXACT NONNEGATIVE:
220086,12.850411259103566s,
run837abfcbb6eef437fdb9e2987aaba25e30ebefc2522bd7b26332a754eced9ef5;
220112,12.867493455996737s,
run93baba29ba5c7eaa829df73dc142bdbf4ed0668fa4c050cffe1ef033d031b7bf;
220138,13.233919081976637s,
run2e1947c2aad07981de2e82ed009c08cfb6be395aed2396f6dae4c1fa0fbb2b14;
220164,13.097988888970576s,
runb5caf8a9071cbd5aa60146c4d199f9ae7e8aa28f900db39d1fb6c1907ad5b89f;
220190,12.834477162105031s,
runf673b42bde13c3f1e906c37abfb426def7324a10e02ee69a17ac4b8afdb61b15;
220216,12.429944441071711s,
run4332bae99f4e9a91fd6e3bba4d784fe27514fe593206ebd15b58dced7aee4eab.
DB readback confirms all6 statuses,timings,empty negative lists.
Original prune observations retained. Reports
runs/fan-retry{220086,220112,220138,220164,220190,220216}-beam128-30-20260910.jsonl
ignored/untracked. One resource sample:PID2849168 at7s CPU99.8%,
RSS213972KiB,NI19,not peak. All live handles monitored to terminal exit0.

Fresh full-coordinate-signature audit of six source faces:
run6951d44c9aac036b3eeb13be4f8f71403bbe5582f438200b917555f9b3957947,
0.0340579180046916s,all six signatures distinct,hashes persisted in DB.
Includes equality blocks,marked blocks and full order;not a polynomial recount.
Distinct coordinate faces do not rule out abstract poset isomorphism.
Raw182773 paired neighborhood now38 exact nonnegative counting outcomes
(32 initial+6 retries),18 original dimension prunes still uncounted,
48 objective prunes. No sign claim for uncounted/pruned cases.
Do not repeat terminal identities;next distinct mutations or higher-degree
selected gaps. No improved negative;primary174566d33 seven/five remains,
not flagged Kostka witness or minimum-distance certificate.
Source unchanged,no push/publication,extra workers,admin changes or remote job.
Goal remains active,no global blocker.

## Paired and single mutations of smaller negative parents — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Fresh clean Git/no-engine
checks and DB inventory found no paired queues for182122/182773;earlier inventory
also showed no single queues. Source unchanged. All serial counts beam32 with
full coordinate permutation verification,5s/case,45s batch,60s outer,nice10.
Every proposal/result/prune persisted;all four queues completed,no timeouts,
engine errors,empty faces or negative results.

Paired add_flag_boundary + remove_opposite,all104 proposals per parent,
offset0->104,cap6 extra opposite after all implied flags,dimension cap36:
182122:31 exact nonnegative(degrees30..34),24 dimension prunes,
48 objective prunes,1 duplicate not revalidated;26.108534681028686s,
rund4addd0a1c97c18e69f514b47a9ff0c0fb8e770e13a5bb1aa8609d4b3d778695.
182773:32 exact nonnegative(degrees30..33),24 dimension prunes,
48 objective prunes,0 duplicates;25.178286058013327s,
run8c323985592656d805b97b9b879ff2d0c17305cabed8c57236b7cc8486f2f958.
Minimum uncounted dimension38 for182122;37 for182773.
182773 d37 gaps220086/220112/220138/220164/220190/220216,
edges[15,50]/[29,50]/[47,50]/[81,50]/[109,50]/[141,50].
Sample during first batch:PID2844014 under1s CPU88.8%,RSS11076KiB,NI19;
not a peak measurement.

Single remove_opposite or add_flag_boundary,all21 proposals per parent,
offset0->21,cap7 extra opposite,dimension cap32,allowing smaller intermediates:
182122:11 exact nonnegative(degrees29..32),4 dimension prunes,
5 objective prunes,1 duplicate not revalidated;4.315979539998807s,
runb744d92f5f96c59dd4ba53e10445a6086587947f15290e56af838ece24afe15b.
182773:11 exact nonnegative(degrees29..32),3 dimension prunes,
5 objective prunes,2 duplicates not revalidated;4.3009465779177845s,
runea9a555b9cbc4c7984c4337758ce6be4ec79870cd3cda08514d9c38eda9e29fe.
Fresh DB aggregation matches all four reports. Total85 fresh exact nonnegative
counts;duplicates and prunes do not certify signs or geometric equivalence.
Reports runs/fan{182122,182773}-{paired-six-d36,single-seven-d32}-offset0-20260910.jsonl
remain ignored/untracked. Do not repeat these completed identities unchanged.
No improved negative:primary174566d33 seven/five remains,not flagged witness.
Next selected d37 paired gaps or distinct shape/weight/flag mutation.
No engine live at final check;no push/publication,extra workers,admin changes
or remote submission this turn. Goal active,no global blocker.

## Degree36 shrink retries and zero-weight neighborhoods — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Clean Git/no live engine
checked before launch. DB confirmed no prior retry of selected children and no
zero-weight queue for smaller parents182122/182773. Source unchanged.
Raw182773 five-opposite7x14 d36 children,beam128/30s,60s outer,nice10:
219514 cut[1,5] EXACT NONNEGATIVE12.397391972015612s,
rune97e49b9b77055a31468a7f593eaf293c7d18d9c0f542ec514ec7e920c927677;
219662 cut[6,5] EXACT NONNEGATIVE1.702865373925306s,
run67c58e17f5557ccc424237d74ac4233bf4e7eb7c130868146927339fc9b6c1d0;
219720 cut[8,5] time_limited30.005849412991665s,sign unknown,
run5be5524b64c7fff8071ae29a8e8694cddbf48fa29c34b6207144a24ae4e9a43f.
Full-coordinate-verified permutations and all outcomes persisted.
Observed samples (not peaks):PID2839994 at8s CPU100%,RSS233404KiB,NI19;
PID2840869 at22s CPU99.9%,RSS443760KiB,NI19.
Same handles monitored to terminal exit0;no engine left at final check.
Raw182773 original16 dimension prunes now7 exact nonnegative,1 timeout,
8 still uncounted;retain original prune rows and timeout unknown.
Reports runs/fan-retry{219514,219662,219720}-beam128-30-20260910.jsonl untracked.

Zero-weight mutation queues:all23 label choices for each8x15 parent,
offset0->23,dimension cap32,beam32/5s cases/45s batch/60s outer,nice10.
No flag-only objective cap applied to weight-forced equalities.
182122:22 exact nonnegative,degrees22..29;1 duplicate not revalidated;
no dimension prunes,empty faces,timeouts or negative;2.8320253029232845s,
run75bd86aea1cbd566f2fec846876d695d3fcfd35e13f5a159f983993c6b7cbfe5.
182773:22 exact nonnegative;1 duplicate not revalidated;
no dimension prunes,empty faces,timeouts or negative;2.7985916800098494s,
run9c8b0df44201dd11605fd9e10b0e7b518bfacf96c5494563a045111f8297adde.
Fresh DB aggregation/readback checks both scan summaries and three retries.
Reports runs/fan{182122,182773}-zeroweight-d32-offset0-20260910.jsonl untracked.
Every proposal/result including skipped duplicate is stored. No negative
weight mutant to compress/certify;duplicates do not certify polynomial signs.
No improved negative:primary174566d33 seven/five remains,not flagged witness.
Next distinct paired flag/removal mutations on smaller parents (DB inventory
showed no paired queues),or selected unresolved counts. Do not repeat finished
zero-weight queues or terminal retry identities unchanged.
No push/publication,extra workers,admin changes or remote submission this turn.
Goal remains active,no global blocker.

## Remaining degree35 five-opposite children resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous goal turn made exact progress;no global blocker. Fresh clean Git/no
live engine checks and DB checks found no prior retries for selected children.
Source unchanged. Raw182773 interior-shrink children219602 cut[4,5],
219632 cut[5,5],219692 cut[7,5] all7x14,d35,5 extra opposite constraints.
These were original dimension prunes,not previously counted.
Serialized beam128 full-coordinate-verified counts,5s initial bounds,
new30s bounds only for timeouts;each60s outer guard,nice10.
219602 first time_limited5.019781149923801s,
runc54f303e67ae11b836c82e9878d999d93ef0d91675a2abecca20e1355f34969e;
then EXACT NONNEGATIVE8.424408695078455s,
run6623bb153615f60f6d443a622db4c76c028cdae325a3b65f6940df58ad81d470.
219632 first time_limited5.020413286983967s,
runbf6c049d7cbf248b441ec9e253c43648a1ff690b0bdebe54cb2ba7e7259a66dd;
then EXACT NONNEGATIVE8.536955339019187s,
runcc054d99f0d985a2e4b3f4a1c0082c90ba53fb152cd5fe11a88d8f0de953f9e6.
219692 EXACT NONNEGATIVE3.0481031199451536s on first5s attempt,
runa746aa31342b8a3a7c08cb5dee73fada28f34818d2d1f45dea9a707c62ee49f2.
Fresh DB readback confirms all5 statuses/times/exact sign lists. Preserve
original prunes and timeout observations;no polynomial assigned to timeout.
Reports runs/fan-retry{219602,219632,219692}-beam128-{5,30}-20260910.jsonl
(actual five attempted combinations only) ignored/untracked.
Live handles observed through terminal exit0;engines had already finished at
resource checks,no retained in-flight RSS sample. No engine at final check.
Raw182773 original16 dimension prunes now5 resolved exact nonnegative,11
uncounted;fresh exclusion query confirms next d36 IDs219514/219662/219720.
Raw182122 still14 uncounted original dimension prunes. No sign claims for them.
Next distinct selected pruned cases or new shape/weight/flag mutation;
do not repeat terminal retry identities unchanged.
No improved negative;primary174566d33 seven/five remains,not flagged witness.
No push/publication,extra workers,admin changes or remote job this turn.
Goal active,no global blocker.

## Four degree-pruned five-opposite shrink children resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous turn made exact progress;no global blocker. Both worktrees clean and
no engine live at launch. Fresh DB checks found no local retry of selected
children before first count. Source unchanged.
Selected7x14 interior-shrink children all have5 extra opposite constraints:
182122 children219312d35 cut[2,6],219342d35 cut[3,6];
182773 children219544d34 cut[2,6],219574d34 cut[3,6].
All had been dimension-pruned under32 cap,not counted in that scan.
Full-coordinate-verified beam128 layouts,serialized5s then new30s counts,
each under60s outer guard,nice10. All started/results persisted in DB.
5s attempts remained unknown and are retained:
219312,5.018988959025592s,
run8bc6ed2a1d95f57c45bad63cc4a1dff16b5d39796bbe54416f6e2a3b056284ca;
219342,5.0181585809914395s,
runf0e225a10c207e431ba12c2d376fdf522dabdff3e3421ecdcec1c4785ecc16e1;
219544,5.018454781034961s,
run66f3931bcbe930685f0d2863591649fc25c592ad3abbb630eb6cf42054796ccf;
219574,5.017965027014725s,
runf9912f0e6a7671259dacbbc695816e3c848cdf788c2cc018372902ebd14d698e.

All four30s attempts EXACT NONNEGATIVE:
219312,8.566590850008652s,
run2612d752dd132590e0683fd98cbd1c5c182c59389ad234768cec6ac646b01658;
219342,8.293287552078255s,
run407584bd70ee36eef239e59e36b30b3d7e0a93ca3f0c7814cf6545f57be33cde;
219544,7.708932022913359s,
runc8cfd368bf34479ebfa71c0f18ebb6c8ee05dce9722e0e6bceb101fbe94507ad;
219574,7.853076440980658s,
run6fb5f80fbb7db864acf22e5a2f946e9bc63876bce1708e59bfe646ef8dbe0f79.
Fresh DB readback confirms all8 statuses,times,and exact sign lists.
Observed checks found engines already terminal;no retained in-flight RSS sample.
Beam128 frontier peak/total:219312(5,100),219544(5,94),heuristic scores only.
No geometric equivalence inferred from matching polynomial or shape.
Reports runs/fan-retry{219312,219342,219544,219574}-beam128-{5,30}-20260910.jsonl
ignored/untracked. Prior dimension prunes retained,not reclassified scan counts.
Remaining original dimension-pruned cases:14 per smaller-parent neighborhood.
Next low-degree five-opposite gaps include219602/219632/219692d35 from182773,
and219370/219400/219430/219460d36 from182122. Do not repeat terminal identities.
No improved negative;primary174566d33 seven/five remains,not flagged witness.
No push/publication,extra workers,admin changes or remote submission this turn.
Goal remains active;no global blocker.

## Smaller-parent interior shrink neighborhoods completed — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Previous goal turn made exact progress;no global blocker. Fresh Git checks clean,
no engine live before launch. DB mode inventory showed no prior interior-shrink
queue for either raw negative182122 or182773. Source unchanged.
Both complete queues enumerate116 row/column cuts (all8x15 cuts except four
corners), producing7x14 children. Bounds:dimension<=32,extra opposite<=6
after all implied lower flags,beam32 coordinate-verified layout,5s/candidate,
45s batch,60s outer,nice10. All proposal/result/prune rows persisted flush1.
182122 offset0->116,20.87110741296783s:
89 exact nonnegative (degrees9..32),0 timeouts,16 dimension prunes,
8 objective prunes,3 duplicates not revalidated.
Run8d7bf8e7e37fff3677dc1e7af5294043dae73c321e178120551914a9e0c790b1.
182773 offset0->116,17.921435066033155s:
80 exact nonnegative (degrees9..31),0 timeouts,16 dimension prunes,
8 objective prunes,12 duplicates not revalidated.
Run401ff4a83329c51115103ce5d8ed7f6cc0fcdbf3b20386b669aecd92ef83c0fe.
Fresh DB aggregation matches reports;no negative/empty/error outcomes.
Reports runs/fan{182122,182773}-interiorshrink-six-d32-offset0-20260910.jsonl
remain ignored/untracked. These are bounded scans,not exclusions of pruned cases.

Two smallest objective-pruned children both retain7 extra opposite constraints
but have degree28:219504 from182122 and219736 from182773,cut[8,13].
Selected fresh beam32/5s retries (60s outer) both EXACT NONNEGATIVE:
219504,0.20489783200901002s,
run9ef87a059479df997420c964116070e20187ca4d4a6233e6dbfef07f3837a15f;
219736,0.20350221206899732s,
rundaf1453f909e269bbbc0795a496c007af194efb240d8e879d43c4708a2c05012.
DB readback confirms statuses/times/empty negative coefficient lists.
Reports runs/fan-retry{219504,219736}-beam32-5-20260910.jsonl untracked.
Original prune observations retained,not rewritten as fresh scan counts.
Do not infer same face from equal polynomial;no coordinate-equivalence claim.

Uncounted dimension-pruned five-opposite children of182122 start atdegree35:
219312 cut[2,6],219342 cut[3,6];degree36:219370/219400/219430/219460.
Other pruned candidates remain in DB. Smaller rectangle need not lower degree.
Next distinct shape/weight/flag changes or selected pruned-case exact attempts;
do not repeat these completed queue identities unchanged.
No improved negative;primary174566degree33 seven/five and smaller degree30
seven/six remain. Neither is a flagged Kostka counterexample or minimum-distance
certificate. No engine live at terminal check. No push/publication,extra workers,
admin changes or remote submission this turn. Goal remains active.

## Two remaining30s five-opposite gaps resolved — 2026-09-10

Codex owns both handoffs and bounded local KTT counts;sole local DB writer.
Previous status-only turn was no new search progress;this turn adds fresh exact
evidence. No global blocker. Source unchanged. Selected finite d36 posets had
prior30s timeouts;tested120s helper and controlled prior runs justify serial
120s counts with150s outer guards,nice10/effective engineNI19.
215363 beam32 EXACT NONNEGATIVE32.60179156099912s,
run2559d42c63883bf7c2276f80d1a1f4ee50d44dfcb338e3e44f885bea3320ec43;
216775 beam128 EXACT NONNEGATIVE33.09586167591624s,
runcad57597b481384760a6b6522158abe3c26077780b3de0f33d8b88ee986625e4.
Same live handles monitored to exit0;resource samples,not peaks:
PID2827129 at10s CPU100%,RSS222088KiB;
PID2828102 at14s CPU99.9%,RSS393340KiB,at31s RSS32888KiB.
Both prior30s timeouts retained. Reports
runs/fan-retry{215363,216775}-beam{32,128}-120-20260910.jsonl ignored.
Raw209610 lower-remove-two bounded neighborhood now29 exact nonnegative,
one unresolved counting case215123d35 (already beam128/120 timeout),
plus159 dimension prunes,1218 objective prunes,1 duplicate not revalidated.
This is not a positivity proof over pruned/skipped cases.

Smaller-shape follow-up212722d34,8x14,interior-shrink edge[6,6]:
DB confirmed only dimension_pruned original,no prior local exact retry.
Fresh beam128/5s time_limited5.01145396695938s,
run3219f0ea1f4bae88b79e176e3682ccac1211e045f8bcd79d1589ac19dc0a333e;
new30s bound EXACT NONNEGATIVE8.32254731701687s,
run13816ab7d7ee768061187118cfdc9227d6adfbf45937c74bd0597d8db606da8a.
Both60s outer/reduced priority;sample first enginePID2828911 at1s
CPU100%,RSS49048KiB,NI19. Full-coordinate-verified layout before counting.
DB readback confirms all three exact results this turn and preserves timeouts.
Reports runs/fan-retry212722-beam128-{5,30}-20260910.jsonl untracked.
No improved negative:primary174566d33 seven/five remains,not flagged witness.
Next distinct shape/weight/flag mutation or unresolved neighborhood,not repeated
terminal attempts. No live engine at terminal check;goal active,no global blocker.
No push/publication,admin changes or extra workers.

## Validated Abacus retry and two local five-opposite results — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
The intervening user status turn was not a new search increment. Clean worktrees
and no live engine confirmed before the next retry; no global blocker.
Fresh DB inspection confirms local exact NONNEGATIVE results:
215377 beam128/120s,37.656594596919604s,
run7d2e537084d8e93b2f85b2d3de2ecc5f7d39d178f70f62c3b1b267846cd58507;
216789 beam128/120s,38.04084962699562s,
runa5b5d328dc7b827cbdad94c8db8e8d87c65a456a24b752edbe7f239183b6359b.
Both had prior30s timeouts, retained. Reports
runs/fan-retry{215377,216789}-beam128-120-20260910.jsonl remain untracked.
No in-flight resource sample was retained for these two runs.
Raw209610 two-removal neighborhood now27 exact nonnegative,3 unresolved,
plus159 dimension prunes,1218 objective prunes,1 unrevalidated duplicate.
Remaining215123d35 already timed out120s;215363/216775d36 have30s gaps.

Abacus job20260910T061625-2b372ab61d07 completed exit0 in32.27877736091614s.
Known negative196437 matched freshly recomputed local full polynomial/hstar:
remote0.4018353859864874s,local0.27388003293890506s.
Candidate212634 remained time_limited30.115326583996648s,sign unknown.
This was a new30s bound after20s timeout, not a duplicate submission.
Input runs/abacus-five212634-30-20260910.json SHA256
f042c3019e90684a7aab952aa684a9ca292e6774d1d8daf305859a5133e0df52;
wrapper SHA256 a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a;
engine SHA256 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367;
output SHA256 7da615f63ec73d2d03daf2a61b10ccf454dd3fe925ab77657d38e13fcee09801.
Fetched outside Dropbox under
/home/dev/.local/share/supervisor-compute-results/abacus/20260910T061625-2b372ab61d07-14f9putm.
Manifest/input/code/engine hashes, exact baseline, independent rational transform,
case IDs, exit status, resolved libraries and effective CPU0,1/8GiB limits were
validated before local ledger ingestion. Ledger
8dd3a54b33f0698920d4cbc9c97d78a8a504f545cd4706479d2770f270e40634
contains receipt and both validated outcomes, completed; no exact cache for
the timeout. Fresh DB inspection confirms both results. No credentials/DB/
shared checkout uploaded; no admin changes. Host supervisor can fetch this job
ID durably; remote originals retained. Latest compute status enabled, no live job.

Primary negative remains174566d33 seven extra opposite/five internal constraints,
not a flagged witness or a certified minimum distance. Source unchanged.
Next selected215363/216775 finite120s retries are owned by this session;
retain all timeout identities and monitor serialized counts.

## Three monitored longer retries resolve five-opposite gaps — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Source unchanged. Three selected finite dimension36 posets
with verified peak-five orders had prior30s timeouts;tested120s helper and
controlled prior runs justify serialized120s counts/150s outer guards.
All reduced priority,effective engineNI19;same live handles monitored,never
restarted on observation timeout. Results EXACT NONNEGATIVE:
214309 beam128,39.791392s,
runc3679f8c2bf01e47db6896c98d0618bdaa03100933ddd99e2ceff7a73fa8ff8f;
214651 beam32(same order as128),45.377098s,
run54dea688e082c1a60500b4db8a6431098304213c577e3808c326126b4d088882;
214891 beam128,45.094058s,
runccea055c216e91bc98c7229d0068228756412229a2fe684b9f772662e2e9558f.
Observed resource samples(not peaks):
PID2803885 at33s CPU100%,RSS385228KiB;
PID2805052 at35s CPU99.9%,RSS498572KiB;
PID2805985 at29s CPU100%,RSS534564KiB.
Each terminal check confirmed no engine remaining. Original30s timeouts kept.
Reports runs/fan-retry{214309,214651,214891}-beam{128,32,128}-120-20260910.jsonl
remain ignored/untracked;all count outcomes in DB.

Raw209610 two-removal neighborhood now25 exact nonnegative,5 unresolved
counting cases,plus159 dimension prunes,1218 objective prunes,1 duplicate.
Fresh DB exclusion query confirms remaining:
215123d35 [4,8,82,132],already timed out beam128/120;
215377d35 [5,9,98,132],beam128/30 timeout;
216789d35 [9,15,98,132],beam128/30 timeout;
215363d36 [5,9,82,132],unchanged beam32/128,beam32/30 timeout;
216775d36 [9,15,82,132],beam128/30 timeout.
Next a selected new120s bound for these30s gaps or new mutation/layout;
do not repeat215123's terminal120s identity. No sign exclusions from timeouts.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
No live scan,push/publication or extra workers;goal active,no global blocker.

## Extended finite retry bound tested and monitored — 2026-09-10

Codex owns private retry helper/tests/README and both handoffs;sole local DB
writer unchanged. Clean worktrees/no live scan verified at adoption. Previous
turn made progress;no global blocker. Extended optional --seconds range1..120,
default5 unchanged. Existing version1 identities unchanged;budget already part
of identity. Tests verify120s forwarding and recorded config using beam128,
and reject0/-1/121/noninteger before DB access. All68 tests pass0.468s.
README documents serialization,monitoring and150s outer guard for120s count.
Maintained engine and primary scanner unchanged;Abacus wrapper still caps30s.

Selected215123d35 five-opposite child previously timed out beam32/30 and
beam128/30;fixed finite poset and prior controlled runs justify one120s retry.
Launched with beam128,timeout150s outer,nice10 wrapper (engine effectiveNI19).
Observed engine PID2801596 CPU99.9%,RSS1091496KiB at31s,907128KiB at64s,
2088224KiB at102s. These are samples,not measured peak memory.
Monitored same live handle throughout;no duplicate restart or unbounded scan.
Engine exited at internal timeout;helper stored time_limited120.051361s,
rune4f54461ed0b08c81a51270d0cbda73d4716867fa12e31d3f7be5bc29db01052.
No remaining engine process on terminal check. Original shorter attempts
retained;215123 signs remain unknown. Do not repeat this terminal identity.
Report runs/fan-retry215123-beam128-120-20260910.jsonl ignored/untracked.
Eight two-removal counting gaps remain;no positivity exclusion from timeout.
Next other selected finite longer retry or new mutation/layout;not a global
blocker and not permission for unbounded/resource-unmonitored computation.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
Source increment verified,all results in DB,no live local scan,push/publication
or extra workers. Goal remains active.

## Five-opposite retries leave eight unresolved counts — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Source unchanged. All counts30s internal/60s outer,
reduced priority;original5s attempts retained.
Changed-order beam128:
215993d36 EXACT NONNEGATIVE19.554080s,
run5d5f4d186f33bc557b525b2691c559f98312c2f6835c29aac6673874a422d242;
216775d36 time_limited30.038127s,
runde0d785c4620b9562af9afb458dcc09c7fe3e32d4ed9ed9d7b896979c58234c4.
Unchanged-order beam32,new30s bound (not redundant beam128):
215363d36 time_limited30.035267s,
run363b8da7947834a8cd01a4a64d00488d23c56ac073d9e3b1392d0ece7bb9ab4a;
215983d36 EXACT NONNEGATIVE9.071180s,
rund90bf0fdd817b0641cfdeab89d1d81c3792dd59c69b334c822a4116cd7bd7bc5;
215987d36 EXACT NONNEGATIVE29.128969s,
run9b0eb15cc0f29ecea0e051509305c1f7d598f963146e5e58dfdebe9103ddcf4c.

Raw209610 two-removal neighborhood now22 exact nonnegative,8 unresolved
counting cases,plus159 dimension prunes,1218 objective prunes,1 duplicate.
Fresh DB query excluding exact retries confirms unresolved IDs:
215123d35 [4,8,82,132];215377d35 [5,9,98,132];
216789d35 [9,15,98,132];214309d36 [1,2,82,132];
214651d36 [2,4,82,132];214891d36 [3,6,82,132];
215363d36 [5,9,82,132];216775d36 [9,15,82,132].
All have now had at least one30s exact attempt;signs unknown.
Do not repeat terminal layout/budget identities. Next a justified longer
finite retry would require extending/testing the helper's current30s limit,
or try a distinct verified layout/shape/weight mutation. Not a global blocker.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
All results stored,reports ignored,no live scan,push/publication or extra workers.
Goal remains active.

## Four dimension36 five-opposite gaps resolved — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Source unchanged. Original5s attempts retained.
Unchanged-order214651 beam32/30s remains time_limited30.045254s,
run8ad6e476979244c8ae118278c59615da01353ca95551ad0683062889db57eed7.

Fresh full-coordinate-verified beam32/128 layout audit,all dimension36:
215357(6,118)->(5,104),0.084194s;
215363(6,127)->(6,127),0.079860s,identical;
215629(6,124)->(5,105),0.083924s;
215635(6,133)->(5,114),0.083089s;
215983(5,115)->(5,115),0.097322s,identical;
215987(5,117)->(5,117),0.098584s,identical;
215993(6,134)->(6,122),0.095254s,changed;
216769(6,127)->(5,108),0.091373s;
216775(6,136)->(6,136),0.088275s,changed.
Layout ledgerfc3f09865a64f894eb9b35c5aaafc8d55154d0b36db9180593520624f21fc126,
both full orders stored. Scores are heuristics,not optimality/sign proofs.
Identical orders not recounted as beam128;unchanged-peak changed orders deferred.

Four peak-reducing beam128/30s retries resolve EXACT NONNEGATIVE:
215357,8.574119s,run1951618882deb38f801ee4974356edbeac0efe228da6c018f17a9980b0f0f029;
215629,18.477250s,run88caa1bb36695868e0550cb13d9e6a5aea38804ac222d445f52b7bdce6c0e8f1;
215635,12.392566s,run833ea9d8c62ea4e5da2defd1fad66511f973688c73fc6b17de897d633c25cb63;
216769,14.358402s,run999ba094b928459c555ae7ced35f163bfff6ee7c5e21b576e21f962e223a82ce.
All counts60s outer/reduced priority. Raw209610 two-removal neighborhood
now19 exact nonnegative,11 unresolved counting cases(DB exclusion query confirms),
plus159 dimension prunes,1218 objective prunes,1 unrevalidated duplicate.
Next deferred215993/216775 changed-order retries or new30s bounds for
215363/215983/215987 unchanged orders;do not repeat terminal identities.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
All outcomes stored,reports ignored,no live scan,push/publication or extra workers.
Goal remains active,no global blocker.

## Three five-opposite gaps resolved; dimension36 layout audit — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Source unchanged. Original5s attempts preserved.
Distinct beam32/30s retries for unchanged-order cases:
216389d35 EXACT NONNEGATIVE10.889843s,
rundaa5ec4c49ff04698008095d0f1fd98b43873eceb623230f92ebb9419474f971;
214305d36 EXACT NONNEGATIVE13.927481s,
run27820dc1bffdc73774e9468009f170074110c00e69d1ba13794b77a03d9e0aff.
These were new time bounds,not redundant beam128 attempts on identical orders.

Fresh full-coordinate-verified beam32/128 comparison for214647/214651/214891,
layout ledgerd2f964ed251369350816bfe075f2f399db5d70409a1dda6e6a8d8091cc02921f:
214647d36 peak/total(6,119)->(5,104),0.082206s;
214651d36(5,101)->(5,101),0.087505s,identical order,not retried this turn;
214891d36(6,114)->(5,101),0.079557s.
Full permutations stored;layout scores do not certify optimality or signs.
Changed-order beam128/30s retries:
214647 EXACT NONNEGATIVE19.472256s,
run99a4e5d148927cbed3dd4a2288c416ddb9fc5de6e51d64b966b7d8fdd1193aaf;
214891 time_limited30.036768s,
runae833afbe7159aebf2f9cec20c39146352703bd098b1248fb59733f1a0b28077.
Every count under60s guard/reduced priority. Timeouts remain unknown.
Raw209610 two-removal neighborhood now15 exact nonnegative,15 unresolved
counting cases,plus159 dimension prunes,1218 objective prunes,1 duplicate.
Next unchanged-order214651 may receive new30s bound;remaining d36 gaps
215357/215363/215629/215635/215983/215987/215993/216769/216775 need layout
audit or selected retries. Avoid repeating terminal layout/budget identities.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
All outcomes in DB,reports ignored,no live scan,push/publication or extra workers.
Goal remains active,no global blocker.

## Deferred five-opposite retry and new donor crossover — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Source unchanged. Deferred changed-order216395d35 five-
opposite child resolves EXACT NONNEGATIVE beam128/30s in21.594916s,
run59e8d6ddaaeec3b8941c2f9345a3a5e56a1685e1112556294b817f3186d0a69f.
Raw209610 two-removal neighborhood now12 exact nonnegative,18 unresolved
counting cases,plus159 dimension prunes,1218 objective prunes,1 duplicate.

DB check found no earlier174551+173215 crossover runs before launch.
Row-band all56,0->24->48->56,40.666345/40.921167/0.721547s,cap7/d38,
beam32/5s cases/45s batches/60s guards.
DB confirms2 initial exact nonnegative,16 limits,30 objective prunes,8
duplicates not revalidated,no other outcomes.
Runs7f23106193d83b4b9263518856d237cc011a4f2285a52be145f492ab78677c43,
bad84ad1043db52e847d53075f23003adddf9190e82c656db876fa21c433ad6e,
d25605fb64617b2a86647154c8c21d199e9573959dc2efc76d2b1353be7748a4.
Column-band all182,cap6/d36,same count/time bounds,112 duplicates,
70 objective prunes,no counts,2.840861s,
run6e72e05d4e01a4d3b2c1dd105750b3d70ac950cad5c752785b97a27f09fd447a.
All started observations separate;skips/prunes do not certify signs.

Small row gap218788d34 band[3,4),direction1 resolves EXACT NEGATIVE
beam128/30s in5.692205s,result219209,
runddda089f5afb8d6e6f4ad9186ae8d71287551e2df7aef612165f23e0d5057c0d.
Linear-11685727793017/8022419605200. Fresh full-coordinate comparison
with209610 proves SAME FACE,not new lineage;not inferred from polynomial.
Audit0.010788s,run8d66102d1e1efb2377df8797cd7ea72b2722e7f53c95a7ac9383d8b9fec58355,
signatureSHA da48c5b6a7403f27b9ce5add94c9dceb2356e19760051e2781c2c07795354475.
Original timeout retained;row neighborhood now3 exact(one negative),15
unresolved counting cases. Nearby218792/218812/218816d34 remain unknown.
Do not repeat completed donor queues unchanged. Coordinate-equivalence cache
would avoid these redundant-presentation recounts;implementation still deferred.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
All outcomes in DB,reports ignored,no live scan,push/publication or extra workers.
Goal remains active,no global blocker.

## Seven layout comparisons and four five-opposite retries — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Source unchanged. Selected unresolved two-removal children
by DB query excluding completed exact retries,then freshly compared beam32/128
orders with full coordinate-permutation checks. Layout ledger
3a465fc9d23329bb571cc740137c8c122a2057e2d6e102e8e4c0e2bb920bfda6.
Peak/total scores and layout times:
215377d35:(6,132)->(6,94),0.078333s;
215649d35:(6,133)->(5,110),0.081175s;
216389d35:(5,106)->(5,106),0.084540s,identical order,not recounted;
216395d35:(6,135)->(6,135),0.086416s,different order,deferred;
216789d35:(6,136)->(6,98),0.085690s;
214305d36:(5,104)->(5,104),0.082889s,identical order,not recounted;
214309d36:(6,118)->(5,102),0.078908s.
Full orders stored in DB,layout-only results do not assert coefficient signs.

Four distinct beam128/30s retries,60s guards/reduced priority:
215649 EXACT NONNEGATIVE7.787433s,
run574955832c65d1def6add52955c4f8eaa400c789ed6c87835a1ebe6016f28f41;
214309 time_limited30.040278s,
run084f6754afcf58218e47b59e3826d23b095e569236611c5de42d104bc2af8199;
215377 time_limited30.068244s,
run5b4d78b7f616102f124bfbd8f60033eca56576fb38824aa3a1b9f5aa47377a87;
216789 time_limited30.043183s,
run0170458ffed93973d0301770cbc0f43d03bfe446b3d3491c110a8f41d97a1fb5.
Originalbeam32 attempts retained;timeouts unknown,do not repeat terminal
layout/budget identities. Raw209610 two-removal neighborhood now11 exact
nonnegative,19 unresolved counting cases,plus159 dimension prunes,
1218 objective prunes,1 unrevalidated duplicate.
Next deferred changed-order216395 or audit remaining d36 gaps;shape/weight
mutation remains in scope. Smaller frontier did not guarantee completion.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
All outcomes stored,reports ignored,no live local scan,push/publication or
extra workers. Goal remains active,no global blocker.

## Verified beam128 retries implemented; two gaps resolved — 2026-09-10

Codex owns private retry helper/tests/README and both handoffs;sole local DB
writer unchanged. Clean worktrees/no live scan verified at adoption. Previous
turn made progress;no global blocker. Added --relabel-beam-wide (128 ideals/
level) to exact retry helper only,mutually exclusive with existing layouts.
Distinct beam128-peak-total-frontier-v1 identity;default/beam32 identities
unchanged. Same maintained engine and complete coordinate-permutation check.
Tests cover128 forwarding,distinct identity,original-poset cache,cleared
inherited evidence and stored frontier/permutation/timing. All67 tests pass0.466s.
README updated;public Rust CLI and primary scanner unchanged.

Fresh layout-only audit of214897/215123/212604/212634 stored in DB,
runea624212a400919e798a6c8676a56ed817ea85a51e388503e725ce7686272e44.
Beam32->128 peak/total frontier scores:
214897:(6,120)->(5,91),layout0.074967s;
215123:(6,113)->(6,107),0.071774s;
212604:(6,119)->(5,110),0.064759s;
212634:(5,110)->(5,110),0.063343s,IDENTICAL order so not recounted.
Both full permutations persisted;layout scores are heuristics,not optimality proofs.

Distinct beam128/30s retries,each60s outer/reduced priority:
214897d35 five-opposite two-removal EXACT NONNEGATIVE10.965115s,
run70a482531a61bc5ac011d4b9c56f579cdf9432819a6c2146afda32f34cd816d1;
212604d34 five-opposite smaller-shape child EXACT NONNEGATIVE21.643083s,
run11412c7c11db08239bd62c58e7e214df93f3b37b60e3a5dc549683f901f9bd0b;
215123d35 five-opposite two-removal time_limited30.068188s,
run9adaad303fc183f1064e2b11342b854fd6b3313d3e74e92432f614989644697d.
All original beam32 timeouts retained. Wider order resolved two prior30s
timeouts;215123 remains unknown,not evidence against coefficient negativity.
Raw209610 two-removal neighborhood now10 exact nonnegative,20 unresolved
counting cases,plus159 dimension prunes,1218 objective prunes,1 duplicate.
212604 was a dimension-pruned shrink child,not part of that1408 neighborhood.
Next audit wider layouts for other selected five-opposite gaps before counting;
Abacus may receive explicit verified changed-order covers under its same limits.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
All outcomes in DB,reports ignored,no live local scan,push/publication or
extra workers. Source increment verified;goal remains active,no global blocker.

## Three dimension35 five-opposite retries — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Three raw209610 lower-remove-two children independently
reconstructed and retried beam32/30s,each under60s guard/reduced priority:
214661d35 [2,4,98,132] EXACT NONNEGATIVE8.262669s,
runa2e8379121f13525fba4c4cef7614d36bc0ffbb6f901c166b202e3f9fc2d2113;
214897d35 [3,6,98,132] time_limited30.044952s,
runc66d0fba030353babe41ea72cf277993eafa23e7a14c1e82ffcc089daad550c1;
215123d35 [4,8,82,132] time_limited30.074922s,
runb05db70066017f1f15efd1811cc321512dcc637053876d8f5f9954ea04dc68a0.
DB confirms source dimensions/edges/five extra opposite constraints and terminal
retry statuses. Original5s attempts retained. Timeouts have no sign conclusion.
That1408 neighborhood now9 exact nonnegative,21 unresolved counting cases,
plus159 dimension prunes,1218 objective prunes,1 unrevalidated duplicate.
Do not repeat these terminal30s layout/budget identities unchanged.

Before starting smaller-parent two-removal work,checked DB and handoff:
raw182122 lower-remove-two was already completed undercap5/d36,beam5,
offsets0->230->446->completion,runs64ec27f9518d0523662612295eea3ee90262da414000cce49e0e312270f3bf49,
7d02f8adf29abe5180f78bc64826fe3235ec2703c6310817553be0cf4754ebd4,
0d8a67304cca8a1908a1bb2780f51a54cd24c093f7817b5183bc55c61c41828c.
No duplicate scan launched;these historical ranges were checked for ownership/
planning,not promoted to fresh coefficient verification. Raw182773 also has
documented completed two-removal work;read its detailed entry before proceeding.
Next distinct negative-donor crossover,shape/weight mutation or different verified
counting layout for timed-out five-opposite gaps. No global search blocker.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
All outcomes in DB,reports ignored,source unchanged,no live local scan.
No push/publication/extra workers;goal remains active.

## Smaller-parent band queues and three five-opposite retries — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Checked earlier smaller-parent work:only prefix crosses
were completed before this turn;these interior-band queues are disjoint.
Pair182122+182773 (8x15,d30 negative parents),cap six extra opposite/d36,
beam32/5s cases/45s batches/60s guards:
row-band all42,30 duplicate-not-revalidated,12 objective prunes,0.653153s,
rundf5a3f297cff6bc04b4adc45d0881495ab42c93ba312f906d2652d5dbf7b548d;
column-band all182,169 duplicates,13 objective prunes,3.049563s,
run748a5d1c170e419bab09a66e2b7a528294b1a44c377f9f0e85a308a375a2fb7e.
DB confirms all224 outcomes,no counts/other outcomes. Started rows separate;
duplicate status is not fresh verification. Do not repeat queues unchanged.

Raw209610 lower-remove-two five-opposite gaps resolve exact NONNEGATIVE
under distinct beam30 retries,original5s timeout records retained:
216015d34 [7,13,82,132],21.005840s,
rundda56d812810e630e64a6fa33221bc79e5bf7b1fb62be9e626eb9ab8e0b110b0;
216409d34 [8,14,98,132],11.855464s,
run730372268ab6e0f9fef4d7c1f36dff3a139d65d4a8623987f01f26824d313471;
214319d35 [1,2,98,132],10.778449s,
rund6d5f2a0b28edd6151904499275a6ae6fef841121d4836a57563a628df8a34c0.
That1408 neighborhood now8 exact nonnegative,22 unresolved counting cases,
plus159 dimension prunes,1218 objective prunes,1 unrevalidated duplicate.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
Next target remaining five-opposite d35 gaps214661/214897/215123 or
broaden shape/weight mutations beyond these redundant negative-parent bands.
All observations stored,reports ignored,source unchanged,no live local scan.
No push/publication/extra workers;goal active,no global blocker.

## Six-opposite band queues and two five-opposite exact retries — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Four disjoint queues completed under cap six extra opposite/
dimension36,beam32/5s cases/45s batches/60s guards:
- column-band174551+173399 all182:111 duplicate-not-revalidated,71 objective
  prunes,2.833406s,run37222217da6c085ea342adb554a2cdb46160445151b0308d3780b75ccfacd7bd.
- column-band174551+173929 all182:156 duplicates,26 objective prunes,
  2.825230s,runf228fe559feeaf3b0597dff01cdda6e7a8aa1b02d3789c3c36c551ce8683ab97.
- row-band209610+203068 all56:14 duplicates,42 objective prunes,
  0.875337s,run0650cf3263ba591ca945fa9cd51887d65dd06b808c464d02f9c1213275ebbf7e.
- column-band209610+203068 all182:94 duplicates,88 objective prunes,
  2.840769s,run2a5c2b104b783a13045dbb362c52dac5f83083150035bdc6610c81216cb4f80a.

DB confirms all602 outcomes;no new exact counts,empty,dimension prunes or
engine errors. Started rows retained separately. Skips do not certify signs.
Do not repeat these four queues unchanged.

Raw209610 lower-remove-two gaps freshly resolve exact NONNEGATIVE beam30:
215137d34 [4,8,98,132],26.722668s,
runf1ed4b37d3b0282fe59ebf572c57fa3dea6a14b2bf595587ece7f9f273e0aed2;
216009d34 [7,13,82,98],9.522487s,
run5b9bd584a192957192b40964f5e1fbc697c55654a34f1e94b7677d296b04d0bd.
Both satisfy five-extra-opposite bound;original5s timeouts retained.
That1408 neighborhood now5 exact nonnegative,25 unresolved counting cases,
with159 dimension prunes,1218 objective prunes,1 unrevalidated duplicate.
Nearby216015/216409d34 remain unknown,as do selected Abacus candidates.
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
Next selected five-opposite retries or new mutations of smaller negative parents;
current negative-parent band combinations show heavy presentation redundancy.
All observations stored,reports ignored,source unchanged,no live local scan.
No push/publication/extra workers;goal active,no global blocker.

## Internal slides and two row-band crossovers audited — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Source unchanged,all outcomes in DB,reports ignored.
Raw209610 slide-internal all7 dimension-pruned undercap7/d38,0.346278s,
runfff7765088b8b2f1a1537eae2e90e99258e8727fc485a3210bc3f5cd8a9c20ae.
No exact count/sign conclusion for those slides.

Row-band174551+173399 all56,0->24->56,40.695518/23.511514s,
cap seven extra opposite/d38,beam32/5s cases/45s batches/60s guards.
DB confirms2 exact(one negative),12 time limits,30 objective prunes,
12 duplicates not revalidated,no other outcomes.
Runs54ab23e796ea51e51e8cb2ccef25495c688e728807fadf2109eaf8ace2058c55,
b1d0ae26ad05b6c5a55a53ee3cc2608618d4ec4485a252cd137a8e06cc6cfff3.
Exact negative216891d33,band[4,6),direction1,2.486032s,linear
-15109773697/66853496710. Fresh full-coordinate comparison with174551
proves SAME FACE,not new lineage. Audit0.011331s,
run80b84b25c5d5cd0f8c84e21ee51240126c1654d532532b8eab5c432093e7822d;
signatureSHA f71e7f12d937d669b606edf2364ddc050bc8408f7b1cd1f17a1965e0b99df0b7.
Masks differ by redundant equation;mere polynomial equality was not used as proof.

Timed-out216867d34,band[3,4),direction1,fresh full-coordinate comparison
proves same face as retained negative209610. Original timeout remains;
equivalence audit does not claim a new exact engine run. Audit0.010807s,
runff5e512d417f4516f25ca3b43933b127fd73a10f0c3655e56c532ff6297bd868;
signatureSHA da48c5b6a7403f27b9ce5add94c9dceb2356e19760051e2781c2c07795354475.
Thus one of12 timeouts has resolved geometric identity;11 others not resolved
by this comparison. Nearby216871/216875/216879d34 remain unknown.

Row-band174551+173929 all56,0->24->56,40.687797/20.725270s,samebounds.
DB confirms12 time limits,21 objective prunes,23 duplicates not revalidated,
no exact/empty/dimension-prune/errors.
Runs1941dfdea70126e453bab8a96266ce14de16fd8a2f993219b0aa13e35c4bf018,
83ff275385c566d0b9cecf0bc9b6ab641256db3df5420deb4468d7b7fca28b3a.
Do not repeat either completed row-band queue unchanged. Next disjoint column
bands or lower-budget-gap retries;consider tested full-coordinate deduplication
to avoid recounting redundant presentations (current cache remains mask-based).
No improved negative;primary174566d33 seven/five retained,not a flagged witness.
No live scan,no push/publication/extra workers;goal active,no global blocker.

## Band-parent five-opposite two-removal queue completed — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Raw209610 lower-remove-two adds a lower flag and removes
two opposite equations,cap five extra opposite/dimension36,beam32/5s cases/
45s batches/60s guards. Completed0->453->692->1007->1395->1408.
Times43.783882/39.590382/41.240230/42.702724/10.270729s.
Runs8b226431d8bbe6de7cf9107c1a4adde08033f504dbfca9e00587ca89ba6c6ca0,
f62f06a9eea8735825ee193344ad868d467d7b645ff6cb94410d5e144a5cc638,
41b502358f8a2ca54f73643b03a07e0eddff761822cde1549f2b06e3474339fe,
0974321a0fb3cfab42b0cd67879102336b5e7b3a9357aa41b072df24394926be,
7f93a2582814e67a5442da3ca2f78c24248999a8fb6e9e5fa80585c25a25456b.
DB confirms all1408:2 initial exact nonnegative,28 time limits,159 dimension
prunes,1218 objective prunes,1 duplicate not revalidated,no empty/errors.
Started observations are separate from these outcomes;prunes are not sign proofs.
Smallest five-opposite gap216029d33 [7,13,98,132] resolves exact nonnegative
beam30 in7.437028s,
runf6d273cd2c39316a25564fba3b176f58c686d5affb5632b012426410ae9e9567.
Thus3 exact,27 unresolved counting cases. Original timeout retained.
Nearby215137/216009/216015/216409d34 and214319d35 remain unknown.
Do not repeat this completed1408 queue unchanged. Next selected five-opposite
larger-budget gaps (possibly Abacus),internal slides,or disjoint band crossovers.
No improved negative;primary174566d33 seven extra opposite/five internal retained,
not a certified minimum distance or flagged-Kostka witness.
All outcomes in DB,reports ignored,source unchanged,no live local scan.
No push/publication/extra workers;goal active,no global blocker.

## Band-parent weight-removal neighborhood completed — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Raw209610 full-content-remove completed311->325->339->
351->580,40.412459/40.550347/40.458893/32.907991s. Capdimension35,
beam32/5s cases/45s batches/60s guards;no flag-only objective filter.
New runsf06b6e859894f8dcc4925434fd73810ef096fc32a4416ab620d4d36803eacc5d,
78c81b30db0f8e19554bbf202396fedd507bce5a34db92e177ad1cec2630cb40,
78093672a99f66484aa9cc92fb8234544e3b5e72f66ab71e15261cacb9acb1e0,
226b448645631ec969f2df07ea71d5cd33a64d82fcbd6e516a8a3b88d1aa0e54.
DB join on seed/mode/dimension cap confirms all580 outcomes:
332 initial exact nonnegative,62 time limits,161 dimension prunes,25 empty,
no duplicates/errors.555 started records are separate (empty has no start).
All original-content constraints retain multiplicity t and forced-mask metadata.

Small gap213470d32,label14/removehorizontal26,resolves exact NONNEGATIVE
beam30 in4.952140s,
run6b0b55e8106417858b8675482c9aa7317982bfed67cbb90d602b075c43df73fa.
Together with213010 and213282 retries:335 exact,59 unresolved counting cases.
Original timeout rows retained;no sign conclusion for unresolved or pruned cases.
Nearby213290/213308/213320/213326/213332/213354/213374/213380d32 unknown.
No improved negative;primary174566d33 seven extra opposite/five internal retained.
This is a completed finite neighborhood,NOT an exhaustive no-counterexample result.
Do not repeat this580 queue unchanged. Next raw209610 lower-remove-two toward
five opposite constraints,disjoint band crossover,or selected unresolved retries.
All observations in DB,reports ignored,source unchanged,no live local scan.
No push/publication/extra workers;goal active,no global blocker.

## Abacus resumed; five-opposite result validated — 2026-09-10

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress.
Raw209610 full-content-remove resumed256->276->294->311/580,capdimension35,
beam32/5s cases/45s batches/60s guards,no flag-only objective filter.
Times42.761242/40.450828/40.898581s. DB confirms8 new exact nonnegative,
20 time limits,27 dimension prunes,no other outcomes.
Runs7654374306c2b2ef9fa0bbcf7978db70ab3bdf0dada148109ddd99d6bad25f4b,
427bab3a60a5cbfe199fd92b30ac146c0c7741409f82e2e5cac8fab0843c1576,
c4a54a2d1e256098f6911ab99d046ff2161b49f02e9895349afda35e0cb06200.
Cumulative162 initial exact,38 initial time limits,111 dimension prunes.
Small gap213282d32,label10/removehorizontal25,resolves exact nonnegative beam30
4.917055s,runffdadaeaa807889d33095f3a83a69e25d0a64316b7030f1b7fb65c44e2c3b821.
Including previous213010 retry gives164 exact,36 unresolved counts. Resume311/580.
Reports retain launch-date20260909 in filenames;subsequent retries use20260910.

User reported Abacus awake after suspension. Status confirmed enabled/idle;
prior jobs terminal,not resubmitted. Same trusted wrapper/engine hashes verified.
Job20260910T051743-2f98ecb7f70a:known negative196437 plus disjoint previously
dimension-pruned five-opposite d34 children212634/212692;20s each/60s whole.
Original quotient/beam permutations freshly reconstructed and compared before
upload. No credentials/DB/checkouts transferred. CPU0,1/8GiB limits unchanged.
Code SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a;
engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367;
input SHA 3e7cf94d264c8fb9b48fec7a2921c9fd9c2c9a83b5b087a5154c4638bec5f48b.
Job done/exit0,36.299557s. Baseline full polynomial/hstar/sign matches fresh
local0.276577s vs remote0.413092s. Child212634 time_limited20.071760s,
unknown. Child212692 exact NONNEGATIVE15.508762s,full rational polynomial/
hstar exactly matches fresh local beam30 count8.025394s,
runb2163ea66f87679999923710b64ed6e7f1206bf045279f813af5b68d0373c068.
Metadata,manifest file hashes,header,case IDs,GLIBC2.35/libraries and actual
CPU/memory/swap/PID limits checked before remote result ingestion. Exact hstar
transform independently checked. Local retry supplies exact candidate cache.
Ledger a5f7561947e5ec6cebc9199fa97714772aef995d1a9e70916f0621f1c73ac311.
Output SHA 880a61dafaadd4f76a4df1e2ff39aead821e06494a5a822cd3dfb4339f9d595d.
Fetched outside Dropbox:
 /home/dev/.local/share/supervisor-compute-results/abacus/20260910T051743-2f98ecb7f70a-x32dnywv.
Remote originals retained for host collection by ID. No connection failures.
No improved negative;174566d33 seven/five remains primary,not a flagged witness.
No live local/remote scan at checkpoint;source unchanged,all results stored,
reports ignored,no push/publication/extra workers. Goal active,no global blocker.

## Weight-mutation continuation to256/580 — 2026-09-09

Codex retains ownership and sole local DB writer;clean checkouts/no live scan
verified at adoption. Previous turn made progress;no global blocker.
Raw209610 full-content-remove resumed160->197->224->237->256/580,
39.101037/40.286158/40.590390/39.860766s;capdimension35,beam32/5s cases/
45s batches/60s guards,no flag-only objective filter. New DB outcomes:
50 exact nonnegative,11 time limits,35 dimension prunes,no duplicates/empty/errors.
Runs55a876bd569ce2fa662a38ca26fb660d04f63db2e55fda1ea057510232a0ca04,
2622df9706077e0e0587f754676d2e2e81550836d4ce6f47ac5b285a8c40d919,
39ff34540c508d7a715ef07b526024f695346e59930f95d255a237f1a5300cc4,
318d2d5a2571e8028c854b4e824760d68b5cfe0524da649834136fc7b7004223.
Cumulative154 initial exact,18 initial limits,84 dimension prunes;including
previous exact retry213010 gives155 exact and17 unresolved counting cases.
Small new gaps213282/213290/213308d32,label10/removehorizontal25/40/132;
213304d33,label10/remove103. Do not interpret timeouts as nonnegative.
Resume256/580. User requested best-candidate status at this safe checkpoint:
DB rows174566 and182123 re-read;primary174566d33 seven extra opposite/
five internal remains retained,smaller182123d30 seven/six. No improved negative.
All outcomes stored/reports ignored;no live scan/source edits/publication.
Goal remains active;no blocker.

## Five-opposite retry and weight-mutation queue — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress;
no global blocker. Five-opposite d34 child212604 remains unknown after beam30
30.059278s,run2436f9dba26fdd3911a41c8ab04755f04bf917159e82def229c1caabaf505275.
Original5s timeout retained. Do not repeat identical terminal retry.

Raw209610 full-content-remove queue0->76->111->160/580,dimension cap35,
beam32/5s cases/45s batches/60s guards,40.505348/43.403880/39.907346s.
No flag-only objective filter applied to original-content constraints before
compression. Each label is forced into every original tableau column,paired
with removal of one non-forced equation;payload retains original multiplicity t.
DB confirms104 initial exact nonnegative,7 time limits,49 dimension prunes,
no duplicates/empty/errors. Started observations retained separately.
Runsfca3f3c5d95e5c167f68270682e7beb9b595d1d0a68949ad64a0b4814d26def2,
15b8c97e0dede028bd74ec221d68415794e60800c855277f401d3108a2ae0860,
da1d1cc9e8fb4e79f2a32ff7dff1e4f8d012c51596d74580365b3bc6762a5a8a.
Smallest unresolved213010d33,label4/remove horizontal82,resolves exact
nonnegative beam30 in5.527803s,
runfeb931a8d1a5e47e2fa59776cfa3071f1f54b5cba976ca53a57bf9222fe2a760.
Thus105 exact,6 unresolved among counting cases through160. Original timeout
retained. Gaps213012d34,212876/212926/213006/213054/213102d35 remain unknown.
Resume160/580 with identical bounds;do not restart earlier offsets.
No improved negative;primary174566d33 seven extra opposite/five internal retained.
No minimal-distance claim or flagged-Kostka witness. All observations in DB,
reports ignored;source unchanged,no live scan,no push/publication/extra workers.
Goal remains active;further weight/shape/flag mutations available.

## Band parent joint and smaller-shape queues completed — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live scan verified at adoption. Previous turn made progress
by finishing and validating Abacus retries; no global blocker. No source edits.
Raw209610 joint all900:585 objective prunes,315 dimension prunes,no counts,
25.775531s,run88ffe968342bc62de03e27fac26e347c4e17c763780b01c54b215f719fcfab93.
Cap six extra opposite/dimension38,beam32/5s cases/45s batches/60s guard.
Smallest qualifying dimension40 gaps210906/210926/210956/211106 remain uncounted.

Raw209610 interior-shrink all131 creates8x14 children;cap seven extra opposite/
dimension33,beam32/5s cases/45s batches/60s guards. Actual slices0->31->86->131,
41.439932/43.585077/31.470582s. DB confirms73 initial exact nonnegative,
11 time limits,27 dimension prunes,8 objective prunes,12 duplicates not
revalidated,no empty/errors. Started observations retained separately.
Runs0c3de5bda9ca7ef7c9ca0798788ee4039753a6ecd9c073084e02ae134c911b44,
cf66a96cf9414ac7e435ce8ca3a4a5f6f9e63a26a96b09e66a97e3cff7804ffa,
cb7770b9502d055af7ddf1f2dcab58d0012bb5d3fa8fc894d97d2ce653526b84.
Six-opposite dimension32 gap212592 [2,1] resolves exact nonnegative under
beam30 in4.963220s,run3afd77a1f90b91209d0b8f1c5593d9ae2d64c9bf61b0e8a94a0d93a221df71f3.
Thus74 exact,10 unresolved counting cases;original5s timeout retained.
Nearby212606/212622d32 and212574d33 remain unknown.

Selected five-opposite dimension34 child212604 [2,7],previously dimension
pruned,was first counted beam5:time_limited5.013594s,
run65352386b1f1f71e45551fe7953648ff78973ee530b8bc3db5d88b94b1a3ea07.
No sign conclusion. Similar five-opposite d34 gaps212634/212692/212722
remain uncounted. These are priority larger-budget candidates,possibly Abacus.
Completed neighborhoods must not be rerun unchanged. Next full-content-remove
on209610 or selected five-opposite retries;source shape/weight may mutate.
Primary174566d33 seven extra opposite/five internal remains best retained;
no minimum-distance claim or flagged-Kostka counterexample.
Focused frontier tests48 pass0.448s. All outcomes in DB,reports ignored.
No live scan at checkpoint,no push/publication/extra workers;goal remains active.

## Abacus follow-up and completed paired queue — 2026-09-09

Codex retains ownership and sole local DB writer. Raw209610 paired queue is
complete180/180: DB confirms2 initial exact nonnegative,34 time limits,
27 dimension prunes,117 objective prunes. Cap six extra opposite/dimension38,
beam32/5s cases/45s batches/60s guards. Resume slices53->93->133->173->180
used43.925719/43.562639/42.028897/15.711504s. Original started observations
are retained separately from these180 outcomes. Retry210433 beam30 remains
unknown30.056855s,runbc39b52408c5e6a036434d36c8b691101598d364d959e16b89556eafb02c6d18.
No improved negative; primary174566 degree33 seven extra opposite/five internal
remains retained, not a certified minimum distance or flagged-Kostka witness.

User reaffirmed Abacus. Remote docs re-read; queue idle before submission,
serial CPUs0,1/8GiB limits unchanged. Prior exact compatibility pilot retained;
wrapper/engine SHA freshly match its tested binaries. Two disjoint unresolved
cases210473/210513d34 (six extra opposite) submitted with20s each/60s job:
20260909T201132-1b8bd791b489. Original quotient and beam permutations freshly
reconstructed and compared before upload. Only wrapper/input/engine transferred.
Code SHA a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a;
engine SHA 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367;
input SHA 215caf339cf3ac8fb2d5848d8246c305474601a61410d0ae723d8c227ad18ee8.
Local ledger964a8d1c9da5f052348fe176d7d0642bb9497cb3e7182e39d5bf565488411225
records submission. Job was running on inspection; early fetch refused because
not finished. Inspect this ID, never resubmit on ambiguous connection. Results
must be validated before ingestion; remote originals retained for host collection.
Generated reports remain ignored. No source edits, extra workers or publication.

Job finished done/exit0,wall42.346215s. Both cases time_limited20.124329/
20.126899s; signs unknown. Fetched outside Dropbox to
/home/dev/.local/share/supervisor-compute-results/abacus/20260909T201132-1b8bd791b489-5lvaiot8.
Manifest file hashes/header/case IDs/GLIBC2.35 libraries/CPU affinity/memory,
swap and PID limits validated before local ledger result ingestion. No exact
cache rows inserted. Output SHA
93dee086672ec39220329237d9fbc0f35d6c75a6cc76eab65c91edab733ec92e.
No live local/remote scan at checkpoint; selected larger-budget cases remain
open, not a global blocker. Do not repeat these20s attempts unchanged.

## Band-negative internal removals; paired scan started — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress; no global blocker. Raw209610 lower-internal all45 recorded0->36->45,
41.842370/5.460726s;cap7/dimension40,beam32/5s cases/45s batches/60s guards.
DB confirms9 initial time limits,36 dimension prunes,no exact/other outcomes.
Runs66f518e747d1da9c261daa8d8004e0755181ea9739837dc4bb9a075d6e0436e8,
2d7ac838e28d5d0c83f9272365270dda0267417cb2936be6015ad150d003a566.
Small seven/four gap210369d37 [7,13,113] resolves exact nonnegative beam30
14.829513s,run108c4b4b7fb879ec371a48fff163b26dc1ef658c53b5e4842aef8b39a3efdbac.
Thus1 exact,8 unresolved counts;original timeout retained. Nearby210339/
210379d38,210309d39 remain unknown. No negative-sign exclusion.

Raw209610 paired0->53/180:2 exact nonnegative,7 limits,9 dimension prunes,
35 objective prunes42.754086s;cap six extra opposite/dimension38,beam32/5s/
45s/60s,runc8080fbef6413f7b9b6df0b10f1b7863d386ea25628a02ff1a08111c09b9e05a.
No duplicates,empty or errors. Resume53/180 with unchanged bounds.
No improved negative;primary174566d33 seven/five and band209610/cert209614 retained.
All67 tests pass0.710s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Band-negative lower-remove queue completed — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous turn made progress:
successful fresh certificate209614 and new mutation coverage,no global blocker.
Raw209610 lower-remove completed86->123->163,43.187336/41.120477s under
cap six extra opposite/dimension38,beam32/5s cases/45s batches/60s guards.
New runs5f94e438572f0c365689e824bef08e8a5639e2ede623c32d965d485189bb4ee4,
8281e0481181024939f3c49a418f22d45f37002d7c52ff4b7e4a6b86bf3847ae.
DB confirms whole163:5 initial exact nonnegative,30 limits,26 dimension
prunes,100 objective prunes,2 duplicates not revalidated,no empty/errors.
Small six/five gap209861d32 [7,13,132] resolves exact nonnegative beam30
9.286722s,run80635ae46a6a60f38096f8508d386db33ad26359108d4d4443c0ec581af3c6d1.
Thus6 exact,29 unresolved counting cases;original timeout retained.
Nearby209755/209851/209895/209901d33 remain unknown.

Disjoint column-band174551+196437 all182 recorded2.907096s:119 duplicates
not revalidated,63 objective prunes,no count/other outcomes;cap six extra
opposite/dimension36,beam32/5s/45s/60s,run
e83b614d593be3a862a0feca4c61aa6465a81b1137e42763a73503b3292340e4.
No positivity conclusion for skipped children. No improved negative;
primary174566d33 seven/five and band209610/cert209614 retained.
Next paired/internal/content mutations of209610 or disjoint band parent pairs;
do not repeat completed lower-remove or these column bands at unchanged bounds.
Selected larger-budget nonflag gaps remain open,including possible Abacus.
All67 tests pass0.472s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Band-negative flag certificate completed — 2026-09-09

Follow-up raw209610 lower-remove0->49->86/163,40.057557/43.936482s:
4 exact nonnegative,14 initial limits,15 dimension prunes,51 objective prunes,
2 duplicates not revalidated. Cap six extra opposite/dimension38,beam32/5s
cases/45s batches/60s guards. Runs
5b026d91d7d1c509a3ce08ebd1c43f5287262f48cf3b84b1e2863904a5b15a36,
3ad090dd872668547878772de9fbfd4f0a00622dc7700842cb7a41f206a316a4.
No improved child;all outcomes in DB,reports ignored,no live scan.
Resume86/163 with unchanged bounds. Certificate209614 remains retained.

Codex owns private certificate helper/tests/README and both handoffs;sole local
DB writer unchanged. Clean checkout/no live scan verified. Previous turn made
progress with tested band crossover and exact negative polynomial.
Extended --endpoint-seconds accepted range1..30 (default5 unchanged). Existing
version2 identity includes budget;old20s failed attempt retained and not repeated.
Tests cover30s wiring to BOTH endpoint calls and rejection of31/zero/fractional
bounds;all67 tests pass1.034s. README documents monitored90s outer allowance
for two bounded30s endpoints plus overhead after prior smaller attempts.
Maintained exact engine unchanged.

Fresh beam30 certificate209614 for negative209610 PASSES24.031952s,
run765d57cbd5550591d7c605e8218b193aaf7d60fc5fef4eaca59663f06a3256f5.
Both fresh endpoint rational polynomials match source;full-coordinate signature
and both counting permutations verified. Degree34,seven extra opposite/five
internal bans;linear-11685727793017/8022419605200.
Raw209610 (exact retry of209553) is the mutation parent;certificate209614
has expanded implied-flag mask and should not replace raw for mutations.
Original failed209612/20s retained as historical unknown verification outcome.
This resolves the local certificate gap,NOT the global KTT goal. Primary174566
degree33 seven/five remains best. No claim of minimal nonflag distance or
flagged-Kostka counterexample.
Source/README/tests verified;all results in DB,reports ignored,no live scan.
Next mutate209610 toward six opposite constraints or try disjoint column bands.
No push/publication or extra workers.

## Interior-band crossover implemented; negative candidate retained — 2026-09-09

Codex owns private frontier/tests/README and both handoffs; sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous turn made progress.
Added row-band-crossover and column-band-crossover:strictly interior zero-based
[lo,hi) intervals,reciprocal children,(m-1)*(m-2) proposals for axis lengthm.
Stable lower-cut/upper-cut/direction order;distinct mode/ordering identities.
Shared exact-negative parent validation,matching rectangles and content guards;
at least3 rows/columns required on the selected axis. Prefix identities unchanged.
Both parent IDs,band parent,and axis-specific interval survive empty/skipped
children;parent signs never inherited as child evidence. README updated.
All67 tests pass1.718s,cover bitwise inheritance,bounds,small axes,required
donor,exact small-poset CLI comparison,lineage,resume,timeout,empty and duplicates.
Maintained Rust engine unchanged.

First row-band174551+196437 all56 recorded0->24->56,42.729881/22.592440s.
Cap seven extra opposite/dimension38,beam32/5s cases/45s batches/60s guards.
DB confirms2 initial exact nonnegative,12 limits,19 duplicates not revalidated,
23 objective prunes,no other outcomes. Runs
1c45a384dbb0a3d6c0a3a131d6231cb79f1f4c8fb5a9e363b8146b7d810fa7be,
f882371baa563b32c82a09eafd4216f6853b18ec1dfba03eee99505d15183996.
Gap209553d34 band[3,4),direction1 (band parent196437) resolves EXACT NEGATIVE
on beam30 in16.372457s,result209610,run
4dbfa4e3053729c3ac45a0907d26b85636e447adf9f5f9fd6ece6e4899a44aaa.
Linear-11685727793017/8022419605200;9x15,seven extra opposite/five internal.
v0x3ff01fe01fc01f801f001e001c00181f1,
h0x100080008400840104010407040e040c00.
Original timeout retained;now3 exact(one negative),11 unresolved counting cases.
This exact polynomial is retained,NOT a certified flagged presentation.
Fresh beam20 flag-certificate209612 failed at first endpoint timeout20.207482s,
rune33dad3fe355de38ea8259996fd18e9fa120c5185bc4b9e0dfdb624a306846ca.
Failure persisted without polynomial evidence;do not repeat same terminal
certificate identity. Full certificate needs a distinct adequately bounded
attempt; current helper caps endpoint budget20s. No global blocker.
Primary174566d33 seven/five remains best;209610 is an unpromoted negative
mutation parent. Nearby209557/209561/209565d34 remain unresolved.
Next certify209610 under a new supported finite budget/layout,or run disjoint
column-band search. No claim of minimum nonflag distance or KTT counterexample.
All outcomes in DB,reports ignored,no live scan,no push/publication/workers.

## Targeted five-opposite retries and internal slides — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress; no global blocker. Ledger checked for previous retries before work.
Five-opposite lower-remove-two gap208545d33 [6,12,82,132] resolves exact
nonnegative beam30 in5.178064s,run
039b4203c67c897429529eef23b72078d256c4d67e50a1dcd15b975b42bed0a5.
Gap208861d33 [7,13,98,132] remains unknown beam30 in30.071895s,run
271dbd85431c1bc0d54063ff437163c8de4444cff6a863f84601434421cefcb8.
Original timeouts retained. With earlier208555,whole1220 queue now7 exact
and26 unresolved counting cases; do not repeat30s budgets for208861/207337.

Raw203068 slide-internal all7 recorded under cap7/dimension38,beam32/5s/
45s/60s;all dimension-pruned0.334661s,no other outcomes,run
124d4ce2a7cf8fab4ee707fe04130386eb810a5144bee355091443bfe5aba806.
Smallest209483d39 slide[113,99],then209485d40 [113,112].
First bounded count209483 beam5 remains unknown5.018588s,run
70a99b16a5677538d0130a94dc3fbbc8f37f84057e4f751c5c1d789dd48d8287.
No improved negative;primary174566d33 seven/five and column203068/cert203079 retained.
Next structural diversification could exchange interior row/column bands
between negative parents (two cuts,not just prefixes); this is NOT implemented.
Require bounded stable order,parent validation,lineage and exact CLI/resume
tests before use. Existing selected nonflag gaps also remain available.
All66 tests pass0.451s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers. Presentation
counts are not minimum distance; pruning/timeouts do not exclude negative signs.

## Five-opposite removal queue completed — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress; no global blocker. Raw203068 lower-remove-two completed607->897->
1212->1220 in42.606978/39.692236/10.217770s;cap five extra opposite,
dimension36,beam32/5s cases/45s batches/60s guards.
New613 proposals:4 exact nonnegative,14 initial limits,66 dimension prunes,
529 objective prunes. Runs
ab242752644ff5896fa1da89b35b0606da8bc545331bf08477780d80e006ce1a,
505ce43dd4d21249505427abc12854833b138336c9fffaccc2b49996dc046bc2,
90e19414747f23e88735a9a07a413f40997056a2db5e4448ee8f397681eca75c.
DB confirms all1220:5 initial exact nonnegative,28 initial time limits,
156 dimension prunes,1031 objective prunes,no duplicates/empty/engine errors.
Small gap208555d32 [6,12,98,132] resolves exact nonnegative beam30
7.620829s,run2e87f6723d7c01018c897b4df8ec6aa495fc067d46d3e3bf5ed333c0da155bed.
Thus6 exact,27 unresolved counts;original timeout retained. Previous207337
still unknown after30s. Nearby208545/208861d33,208851d34 remain unknown.
Bounds/pruning do not exclude negative signs; five-opposite metric is not
minimum bad-edge distance. No improved negative;primary174566d33 seven/five
and column203068/cert203079 retained.
Next diversify crossover structure beyond prefixes or alternate negative
parents; tested prefix crosses are duplicate-dominated. Selected longer-budget
nonflag gaps remain open; do not repeat completed queue at unchanged bounds.
All66 tests pass0.446s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Smaller-parent crosses and five-opposite removals — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous turn made progress;
no global blocker. Checked smaller8x15 parent pair182122+182773 against ledger.
Column all28:1 exact nonnegative,26 duplicates not revalidated,1 objective
prune1.915821s,run542f89b4536e95351876b92ccb6d883cd12ddb5f46d3c17f6c0536aa77d92810.
Row all14:12 duplicates,2 objective prunes0.220786s,run
409dc5f543fbcc6624663f8033c3071907f01ebc1f85406e278064eef27fabe5.
Both cap six extra opposite/dimension36,beam32/5s/45s/60s,no other outcomes.

Raw203068 lower-remove-two targets five extra opposite by strengthening a lower
flag and deleting two horizontal equations. New queue0->421->607/1220,
43.511318/41.881359s,dimension36,beam32/5s cases/45s batches/60s guards.
DB confirms1 exact nonnegative,14 initial time limits,90 dimension prunes,
502 objective prunes,no duplicates/empty/engine errors.
Runs3707781f1b5dbc8d5d2271ac9b937374885960f5cb181456f335344730a25d29,
ee7c00324538657ddf0c1f165b76f8a89c0d62991b7fd39e66eb6389d6013ee5.
Gap207337d34 [1,2,98,132],five extra opposite,remains unknown on beam30
30.062129s,runf348be2c656bb1907978ed8e9d0ef8ef38d94ded50e8c9f5194b2ac16a005b51.
Original timeout retained; do not repeat same layout/budget. Nearby207323/
207327/207643 dimension35 remain unknown. Resume607/1220 with same bounds.
No improved negative;primary174566d33 seven/five and column203068/cert203079 retained.
Counts are presentation metrics,not minimum distance; prunes do not exclude signs.
All66 tests pass0.453s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Cross-lineage recombination and targeted gap retries — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous turn made progress;
no global blocker. Fresh pair198861+203068 checked against ledger before runs.
Column all28:21 duplicates not revalidated,7 objective prunes0.432396s;
run b0456f7b9210c316920f784362b36c9c8a1d443296428dbc3e70cc99cbebd5a2.
Row all16:7 duplicates,9 objective prunes0.254106s;run
4b4eebe35b9692500c583e1507ab082add0c938f05b1dd40d8365aefd11b173e.
Cap six extra opposite/dimension38,beam32/5s cases/45s batches/60s guards.
No count or sign conclusion for these skipped children; both lineages retained.

Targeted raw203068 lower-remove gaps203209d33 [4,8,98] and203213d33
[4,8,132],six/five,resolve exact nonnegative on beam30:
5.754610s/run7a79eea11ed2d88182f92e24cb77046a34faaca2e77438fbb182a271f3fe2247,
7.325870s/run2df5c7b1f9ed83947e18492498830a2c455eac219fae50382f722433fab30443.
With earlier203113,that152-proposal queue now has19 exact and17 unresolved
counting cases;initial16 exact/20 timeout observations retained.
Internal-ban gap203446d38 [1,2,113],seven/four,remains unknown on beam30
30.069439s,runa8daea23390447ba29bfe0a290f45d1d5017b113f02af9d04f90363ed9adba41.
Do not repeat that same budget/layout; no negativity exclusion.
No improved negative;primary174566d33 seven/five and column203068/cert203079 retained.
Next diversify compatible smaller8x15 parents182122/182773 or other negative
parents,or audit a new bounded crossover neighborhood. Current tested prefix
crosses are heavily duplicate-dominated; no new crossover mode added this turn.
All66 tests pass0.450s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Column-descendant content-removal queue completed — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress; no global blocker. Raw203068 full-content-remove completed281->298->
314->324->534,40.502403/40.637982/40.239267/23.600295s. Same dimension35,
beam32/5s cases/45s batches/60s guards,no flag-only objective filter.
New253 proposals:158 exact nonnegative,19 initial limits,53 dimension prunes,
23 empty,no duplicates/errors. Runs
aa8c522f05251a73a19e85d9e68997bbf74ef65c2376a78882970d96149e81ab,
d556a491b6927975c24b94a58b70092eec7f21629cbeb5ad444c5e97a8d44eb3,
273e1e0b303e04b02212ddfcaf4c4be753b13a582477ec240e09c4d6a30c04f8,
645bbd30bca04ef62f9e94549e1ec8ff9276feac143744e45f4816e66f664033.
DB confirms all534:311 initial exact nonnegative,49 initial limits,
151 dimension prunes,23 empty,no duplicates/engine errors.
Gap206394d32,label14/remove horizontal41 resolves exact nonnegative beam30
4.945611s,run142beb6cf285deff5c5dc3cacc093c0e1958fc5a255149c1fdff7f7a068d5d15.
After retries205974,206256,206394:314 exact,46 unresolved counting cases.
Original timeouts retained. Nearby206426/206432/206438d32 remain unknown.
No new negative;primary174566d33 seven/five and column203068/cert203079 retained.
Next choose disjoint negative-parent crossovers or selected larger-budget
nonflag-removal gaps,including Abacus; do not repeat completed content queue.
Bounds/prunes do not exclude negative signs; counts are presentation metrics.
All66 tests pass0.451s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Column-descendant content mutations through281 — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress; no global blocker. Raw203068 full-content-remove resumed217->240->
260->281/534,42.408746/40.010943/39.277588s. Same dimension35,beam32/5s
cases/45s batches/60s guards,no flag-only objective filter before compression.
New64 proposals:17 exact nonnegative,17 initial limits,30 dimension prunes.
Runs6676b97f1f223047ba6badb58e89771634c778a0f51385f42fbe1cfe9e4cd1b2,
4410fbf9cb4db7dc4c8047ad608f7f20ccde3b3fa8e5b1069b1d331558ee84ab,
729bd808edb9a288149435d661539d234503d44a28474b276a94e1cedd9f11d2.
DB confirms all281:153 initial exact nonnegative,30 initial limits,
98 dimension prunes,no duplicates/empty/errors.
Gap206256d33,label11/remove horizontal10 resolves exact nonnegative beam30
5.375357s,run e38b1109e6e645561c14a75c5f8e6258c712421449d7f2096006221c087fdd20.
Fresh full-coordinate comparison with old202283 was unequal; that older
resolved case was not reused as evidence. Original timeout retained.
After retries205974 and206256:155 exact,28 unresolved counting cases.
Nearby206262/206268/206286/206298d33 remain unknown. Resume281/534.
No new negative;primary174566d33 seven/five and column203068/cert203079 retained.
Bounds/prunes do not exclude negative signs; counts are presentation metrics.
All66 tests pass0.448s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Column-descendant content mutations through217 — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress; no global blocker. Raw203068 full-content-remove advanced0->73->
103->164->217/534,40.247402/41.769731/39.016753/39.368889s.
Dimension35,beam32/5s cases/45s batches/60s guards,no flag-only objective filter
before possible full-content compression. DB confirms136 initial exact
nonnegative,13 initial limits,68 dimension prunes,no duplicates/empty/errors.
Runs9dbfdc984a024b3395296e136b594a364db1ef906926bc4197f95fef44b24922,
4cd4ad4f969b3d688a0bcb9ca8baa5783a3413a0dfdb2255891c439503180490,
901d84abcaa80252db51bbb3227f201446c012d1992383c91f207828da112557,
1cb2774e6bc42e0c1f37ce2b248dfbaab673fc4fcdd6d06a25cacd00aec35205.
Small gap205974d31,label4/remove horizontal98 resolves exact nonnegative
beam30 in6.662140s,run
661f7c1fc38db5e73a6efbc30db8625a210abc2eb8f891de27e19a76c509df11.
Thus137 exact,12 unresolved counting cases;original timeout retained.
Nearby205970/205976/206020/206066d32 remain unknown.
Resume217/534 with unchanged bounds. No new negative;primary174566d33
seven/five and column203068/cert203079 retained. Bounds/prunes do not exclude
negative signs,and presentation metrics are not minimum bad-edge distance.
All66 tests pass0.446s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Joint removals and smaller-shape neighborhood completed — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress; no global blocker. Raw203068 joint all810 recorded23.675498s under
cap six extra opposite/dimension38,beam32/5s/45s/60s. DB confirms495 objective
prunes,315 dimension prunes,no counts/other outcomes;run
da66c2025829feb4600f95aae034d9e2363d39e9c23b890b1b532b27bb127375.
Smallest eligible cases dimension40:204050[10,82,113],204070[10,98,113],
204090[10,132,113],204230[29,82,113]. First count204050 beam5 remains
unknown5.016588s,run2334b1776a98dc54199770d8223bb9c29a3046465b9d9a87b36c26a0815ebe3b.

Raw203068 interior-shrink all131 recorded0->42->109->131,
40.124753/43.032224/15.975956s;cap seven extra opposite/dimension33,
beam32/5s cases/45s batches/60s guards. New8x14 faces,not equivalences.
DB confirms86 exact nonnegative,5 initial limits,28 dimension prunes,
12 duplicates not revalidated,no empty/objective prunes/engine errors.
Runsc9501161c9ade91f0a31b723071fcb1f30916cc91ab85d94b4b06e884a7c8e52,
539e57ec8f7026f618ca9e0fa2a01665f68ed2de9abc2158458e39c0e8e1db57,
9e29295c867663a39a48f0d6c98fe0ec24bb0aded93e66094f1d04453a1611cb.
Gap205582d32 shrink[2,8] resolves exact nonnegative beam30 in5.161946s,
runb5b5260f2aecbc8d214da9a8b87162ce4739ae7160ddd0855e681dfad74caeef.
Thus87 exact,4 unresolved counts:205612/205642/205758/205788,all dimension32.
Original timeouts retained; bounds/prunes do not exclude negative signs.
No improved negative;primary174566d33 seven/five and column203068/cert203079 retained.
Next content/removal mutations of203068 or disjoint crossover parents; completed
paired/lower-remove/lower-internal/joint/interior-shrink queues stay closed
under these budgets. Presentation counts are not minimum bad-edge distance.
All66 tests pass0.450s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Column-descendant paired removals completed — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress; no global blocker. Raw203068 paired all162 completed in slices
0->48->86->122->158->162,42.733692/42.820158/41.021353/39.610346/8.662820s.
Cap six extra opposite/dimension38,beam32/5s cases/45s batches/60s guards.
DB confirms6 initial exact nonnegative,30 initial time limits,27 dimension
prunes,99 objective prunes,no duplicates,empty cases or engine errors.
Runs4e078d7a59bd66c3355c379fcabab2786548a57761f185e2b592b67ab678bec8,
0f33636acffe73f7ee13c632202fd6573f0334834c1912f4707492640d70cb50,
2f7e0d585399b60170881710389afa48209c887a8f72f71f824ac73abc5c2282,
173333f06fbd84fd5659ec35025789412ba90c707c8c5ee6d9092be54e30edcb,
56e6dc217d2876c2e84407c5a7dcdaadd837c58be0904c7c286b96767c0f97b5.
Small gap203628d34 [10,132] resolves exact nonnegative beam30 in7.673346s,
run d0a7fa670cd14111e5cf8c0e9040fcbf0eaab87b5f28af221e10c318f3e7c625.
Thus7 exact and29 unresolved counting cases;original timeout retained.
Nearby203664/203700/203736/203768d34 remain unknown. Bounds/prunes are
not sign exclusions; six-opposite counts are presentation metrics.
No new negative;primary174566d33 seven/five and column203068/cert203079 retained.
Next joint or shape/content mutations of203068; do not repeat completed paired,
lower-remove or lower-internal queues. Selected larger-budget gaps remain open.
All66 tests pass0.447s. Source unchanged,all outcomes in DB,reports ignored,
no live scan at checkpoint.
Goal active; focused documentation commits,no push/publication or extra workers.

## Column-descendant internal removals completed — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous turn made progress;
no global blocker. Raw203068 lower-internal all45 completed0->31->45 in
39.199719/10.701942s;cap seven extra opposite/dimension40,beam32/5s cases/
45s batches/60s guards. Targets four internal bans; no minimum-distance claim.
DB confirms1 initial exact nonnegative,9 limits,35 dimension prunes,
no objective prunes,duplicates,empty cases or engine errors.
Runsfe0897a511b50661debaa7a89a5e10cc100999b4e7b72f91756376d1a1e7e839,
de42b338ba6de1a1fc90428be808b30026f3c228783bc49ba9d4523547bdb311.
Small gap203506d37 [7,13,113] resolves exact nonnegative beam30 in7.785546s,
run37dc0823761803b9ed1d4e61cda15b3914515c2cd5f077da30fa63961e2fde0c.
Fresh full-coordinate comparison with old199747 was unequal,so that old
resolved case was not used as its evidence. Original timeout retained.
Thus2 exact,8 unresolved counting cases; nearby203446/203476/203516d38.
Bounds/pruning do not exclude negative signs.

Column cross174551+173929 all28:26 duplicates not revalidated,1 objective
prune,1 time limit203572d37 cut10/direction0,no fresh exact result;5.692718s,
run eaa7e15167c51dbd7a5eee434a5693bb4f91e30a570469ab422e51a91514c55c.
Cap7/dimension38,beam32/5s/45s/60s. No new negative.
Primary174566d33 seven/five and column203068/certificate203079 retained.
Next paired mutations of203068 targeting six extra opposite,or disjoint
shape/content reductions. Completed lower-remove/lower-internal queues should
not be restarted; retain selected longer-budget gaps,including possible Abacus.
All66 tests pass0.445s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no push/publication or extra workers.

## Completed first column-descendant removal queue — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress: new certified negative lineage and tested column-crossover source.
Raw203068 lower-remove resumed48->90->134->152 in39.011932/40.530862/
20.527730s;cap six extra opposite/dimension38,beam32/5s cases/45s batches/
60s guards. DB confirms whole152:16 initial exact nonnegative,20 limits,
25 dimension prunes,89 objective prunes,2 duplicates not revalidated.
No empty cases or engine errors. New runs
d6e005928bc12cd850e1c9497f18cc1563a7b53b820377c8d232464a63d5c719,
21f68a2d2b34e2103b44cd692d5884d9ab8d5659b5a56d73875933d7a317fe53,
087be821ac331c27975d99c10a20a3ddd7714b43b474229714143ed1b7369324.
Small six/five gap203113d33 [1,2,98] resolves exact nonnegative beam30
11.337169s,runf7ebb4fd9b3843b3ccf52e17d94923562e14df3932d4613542bb9205bf49155b.
Thus17 exact,19 unresolved counting cases;original timeouts preserved.
Nearby203209/203213d33 remain unknown. Coverage is not negativity exclusion.

Disjoint column cross174551+173399 all28:1 exact negative,3 limits,
21 duplicates not revalidated,3 objective prunes19.094447s;cap7/dimension38,
beam32/5s/45s/60s,run
b520a6db7fc302413d3becf9e591396bb0acd0c8a3dfb9be176fcc1f26297c6f.
Negative203437d34 has the same polynomial as203068. Fresh full-coordinate
signature equality confirms same face,not a new geometric lineage;stored
equivalence a8308a3952423647cff1c178c7c5fc8c3ea50b6a7651f6b216d655c1ed53c100,
0.010858s,not another polynomial count. Both parent IDs and mask pairs retained.
Primary174566d33 seven/five remains best;203068/certificate203079 retained.
Next paired/internal/content mutations of203068 or column cross with173929;
do not repeat completed lower-remove queue. Metrics are not minimum distance.
All66 tests pass0.445s,source unchanged,all outcomes in DB,reports ignored.
No live scan,goal active,no global blocker,push/publication or extra workers.

## Column crossover verified: new negative lineage — 2026-09-09

Follow-up raw203068 lower-remove0->48/152:1 exact nonnegative,8 limits,
9 dimension prunes,30 objective prunes43.456629s,cap six extra opposite,
dimension38,beam32/5s cases/45s batch/60s guard,run
7916075b3d325e7296f7fe82181b6bc118d1109e54c3bc344f05b49351be4abf.
All outcomes in DB,report ignored,no new negative and no live scan.
Resume48/152; use raw203068,not expanded certificate203079,for mutations.

Codex owns private frontier/tests/README and both handoffs; sole local DB writer.
Clean checkout/no live scan verified before source edits; previous turn made progress.
Added --mode column-crossover with required donor,shared parent validation and
exact counting. Reciprocal column prefixes give2*(b-1) proposals in original
rectangle coordinates. Axis-specific queue identity/model/cut metadata preserve
both parents,including skipped/empty children. Row mode identities unchanged.
All66 tests pass0.458s,including column inheritance/bounds,one-row rectangle,
CLI exact small-poset comparison,lineage,resume,timeout,empty and duplicates.
README updated; maintained Rust engine unchanged.

Fresh row cross174551+173929 all16:1 exact nonnegative,12 duplicates not
revalidated,3 objective prunes0.558915s,run
f813a4e318c2a31551374a07c0bbaaaaf34617016391c4a72455109f14ed00cc.
Column cross174551+196437 all28:2 exact negative,2 limits,21 duplicates,
3 objective prunes17.136852s,run
b9d05fa0dd677573e98d0823315b3addd059f891654d19e0f7785d4e6161abf8.
Both cap seven extra opposite/dimension38,beam32/5s cases/45s batch/60s guard.
Negative raw203068(cut12,prefix174551) and203072(cut13) have degree34,
linear-296662246687/1266697832400,seven extra opposite/five internal bans.
Full-coordinate equality freshly reconstructed and persisted,not a recount:
runff11728e04218dd01d9d641214667e432485551b3f63b7e05272e7ae445e58c0,
0.010341s. Thus two mask presentations of the same face,not two independent
geometric lineages. Raw203068 uses
v0x3ff01fe01fc01f801f001e001c00181f1,
h0x1000000084008401040184030406040c00.
Fresh beam15 flag certificate203079 checks both endpoint polynomials and
full-coordinate equality6.657198s,run
efb7bb1f97ce3623b62fc4c3a56ca3d922835909fc5816cbf602d5efa1682b5f.
Retain this new negative for mutations;primary174566 degree33 seven/five
remains best. Counts are presentation metrics,not minimum bad-edge distance,
and no flagged-Kostka witness is claimed. Gaps203060d37/203064d34 remain unknown.
All outcomes in DB,reports ignored,no live scan. Next mutate raw203068 toward
six opposite constraints,or additional disjoint column crosses.
No push/publication or extra workers.

## Completed crossover-parent content-removal queue — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous turn made progress;
no global blocker. Raw198861 full-content-remove completed372->392->583->626,
39.012268/44.014796/1.526895s. Same dimension36,beam32/5s cases/45s batches/
60s guards,no flag-only objective filter before potential content compression.
New254 proposals:185 exact nonnegative,11 initial limits,31 dimension prunes,
27 empty,no duplicates/errors. Runs
21a5d1ab12285ebfecbd08fec9ba02ba6d6f6d01908c265e36e29dfe06a4f73e,
4a7c4171caf04a8cf36aeb495d21297e81a3538ad1cd95303001d7779140e92d,
6bea6cf2758c5a26313f7bde31f223999f4ea8b7b869538967fb864ec52b5ec8.
DB confirms all626:286 initial exact nonnegative,71 initial time limits,
131 dimension prunes,111 duplicates not revalidated,27 empty,no engine errors.
Gap202511 dimension32,label15/remove horizontal66 resolves exact nonnegative
beam30 in5.009284s,run
ac57d2ce7b57e5b496252096ed84c23539c98c120b10abe0f6ab48e7df8193b1.
After retries202283,202431,202511:289 exact,68 unresolved counting cases.
Original limits retained; bounded coverage is not a sign exclusion.
Nearby202525 dimension33,202515/202521 dimension34 remain unknown.
No new negative;primary174566 degree33 seven/five and crossover198864 retained.
Next choose a disjoint negative-parent crossover or new column-prefix crossover
implementation (not yet implemented),rather than repeat this completed queue.
Longer-budget unresolved cases remain eligible,including Abacus with local
validation and sole local DB ingestion. Counts are presentation metrics,
not minimum bad-edge distance or a flagged-Kostka witness.
All65 tests pass0.444s,source unchanged,all outcomes in DB,reports ignored.
No live scan at checkpoint; focused documentation commits,no push/publication.

## Content-removal scan advanced through372 — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live scan verified at adoption. Previous goal turn made
progress; no global blocker. Raw198861 full-content-remove resumed319->335->
346->361->372 under dimension36,beam32/5s cases/45s batches/60s guards.
No flag-only objective filter before potential content compression.
Slices39.017042/40.430978/43.584903/42.299716s;14 exact nonnegative,
22 initial limits,17 dimension prunes,no duplicates/empty/errors.
Runs3a8373608ecbb1047f045a476ff5cf2f70537696079915c2b296fb078c09ff79,
4cc0ece86f9ac37e006b2b25994cd12bed4ea211ed08a42b7d400304972e6ecc,
9726bc30201c63c480e4789552d4cf9ea2a84c0026903073c0a0d49e9871a409,
59eaa16f6396a3d78e45233c25f65f58b47667df33d8514fb0fe9c66ef3ab5f2.
Gap202431 dimension32,label14/remove horizontal11 resolves exact nonnegative
beam30 in5.216309s,run
415478e858880ebddd7dd6deeee4c984bee45a8c1a01fde41c0de4dd391fe3d4.
Original timeout retained. Nearby202437/202439/202447/202453 dimension32
remain unknown. DB confirms all372:101 initial exact nonnegative,60 initial
limits,100 dimension prunes,111 duplicates not revalidated,no other outcomes.
After retries202283 and202431:103 exact,58 unresolved counting cases.
Resume372/626 with unchanged bounds. Prunes/duplicates are not sign exclusions.
No new negative;primary174566 degree33 seven/five and crossover198864 retained.
All65 tests pass0.448s,source unchanged,all outcomes in DB,reports ignored.
No live scan at checkpoint; focused documentation commits,no push/publication.

## Content-removal scan advanced through319 — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live local scan verified at adoption. Previous goal turn
made progress; no global blocker. Raw198861 full-content-remove resumed under
unchanged dimension36,beam32/5s cases/45s batches/60s guards. No flag-only
objective filter before potential full-content compression.
Slices206->248->268->287->300 took40.217543/39.423961/42.037858/41.396145s:
47 exact nonnegative,16 initial limits,31 dimension prunes,no other outcomes.
Runs2143aceeb0264d93e703d75728f267f301e33528571ebb16b1165e35ad78184c,
2a3fb35f17b332692f3b134c0ef4230b85c4f723de227ddf5ddbe9ca8b62d99e,
de15a0c02fcd9de2cb10c70539ff0ed8cd476c1fe475b52adae85c3cde151f0f,
88ca681abe8b0ff76fca480ece24f07122895f39f9e678fd6ca5d777caaa1f11.
Gap202283 dimension33,label11/remove horizontal10 resolves exact nonnegative
beam30 in5.692979s,run
f1ada866d3cd2df047011ae5b26d8eaf58ac6545b6ba90e00c808bfb3a6e5530.
Original timeout retained. Nearby202289/202295 dimension33 remain unknown.
No new negative;primary174566 degree33 seven/five and crossover198864 retained.
300->319:6 exact nonnegative,5 limits,8 dimension prunes43.227547s,
rund2133775b819bf8cc6eccfbd3e49cf546de1f3a5068c9c5dce73bb1a78acc1d4.
DB confirms all319:87 initial exact nonnegative,38 initial limits,83 dimension
prunes,111 duplicates not revalidated,no empty cases or engine errors.
After retry202283:88 exact,37 unresolved counting cases; skipped cases are
not positivity exclusions. Resume319/626 with unchanged bounds.
All65 tests pass0.450s. All outcomes in DB,reports ignored,source unchanged.
No live scan at checkpoint; focused documentation commits,no push/publication.

## Content-mutation continuation — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkout/no live local scan verified at adoption. Previous turn made
progress with new bounded nonflag-removal coverage; no global blocker.
First count of joint candidate199939 dimension40,six extra opposite/four
internal,remains unknown after beam5 in5.018321s,run
bc9d0612a092b2c807044f6f6e824530b9b63a51f35941ab782071109112418b.
Original dimension-pruned observation retained; this is not a sign exclusion.
Raw198861 full-content-remove cap dimension36,beam32/5s cases/45s batches/
60s guards,without flag-only objective pruning before possible compression:
0->96/626:0 exact,8 limits,15 dimension prunes,73 duplicates not revalidated,
42.648147s,run44574e18564ed49f73781ffd7c937c4e1c8e27f826559f60f6937736426ac144.
96->163:3 exact nonnegative,7 limits,19 dimension prunes,38 duplicates,
39.892710s,runde693d169c54eef9deb66cc22e22d1e0a1565e8bf95032c700ab8df152025bbe.
All outcomes in DB,including skipped genomes; no cached polynomial promoted
to fresh evidence. Small first-slice gaps201807/201861 dimension34 involve
content labels1/2 and internal-ban removal128;201949 dimension34 label4/h66.
163->206:31 exact nonnegative,2 limits,10 dimension prunes,no duplicates,
42.413706s,run3f107cdca37dcb2afc316e4e59272d954e13d9c8cfa9b710f76146e117d58440.
DB confirms combined206:34 exact nonnegative,17 limits,44 dimension prunes,
111 duplicates not revalidated,no empty cases or engine errors. Resume206/626
with the same bounds; preserve distinct run identities and actual next offset.
All65 tests pass0.453s,source unchanged. No live scan at checkpoint.
Generated reports ignored. Focused documentation commits,no push/publication.
No new negative; primary174566 degree33 seven/five and crossover198864 retained.

## Internal-ban reduction neighborhood — 2026-09-09

Codex retains KTT/private companion/handoff ownership and sole local DB writer.
Clean checkouts/no live local scan verified at adoption. Previous goal turn
made progress: recorded additional bounded Abacus attempts, not a global blocker.
Raw198861 lower-internal all45 completed in slices0->36->45,
43.340981/5.456248s. Cap seven extra opposite,dimension40;beam32/5s cases,
45s batches/60s guards. These proposals strengthen a lower flag and remove one
internal ban, targeting seven extra opposite/four internal presentation counts.
DB confirms1 exact nonnegative,9 initial time limits,35 dimension prunes;
no duplicates,empty cases,objective prunes or engine errors.
Runs a8a3e713f4b81d8441ee8bb748905780f92e6c8651fa41200df1311947ad5c0c
and36b62641ae1ad1c4d0501b0e625d82a3468f4e297aa7f5c0e74015bf594f14e8.
Smallest gap199747 dimension37 [7,13,113] resolves exact nonnegative on
beam30 retry10.281711s,run
02d3cbfd37d3f5fb38bf658a65461522c44491cf8afac3cc724b3fb4d8039793.
Thus2 exact and8 unresolved counting cases after retry;original limits retained.
Remaining gaps include199717/199757 dimension38,199687/199697/199707/
199727 dimension39,199739 dimension40. Pruning is not sign exclusion.
All65 tests pass0.443s. Source unchanged,all attempts in DB,reports ignored.
No new negative;best174566 degree33 seven/five and crossover198864 retained.
Raw198861 joint all990 completed26.616525s under cap six/dimension38,
beam32/5s cases/45s batch/60s guard. DB confirms675 objective prunes and
315 dimension prunes,no counts or other outcomes;run
1581fe49a5639c8e16440319748215f3c5609350c3a742d455a26c3d22f62b19.
Smallest objective-eligible candidates have dimension40:199939[15,82,113],
199959[15,98,113],199989[15,132,113],200159[29,82,113],
200179[29,98,113]. Six extra opposite/four internal presentation counts.
Next consider bounded first counts of these dimension40 cases,including on
Abacus,or disjoint shape/content mutations. None was counted in this queue.
No live local scan at checkpoint. Goal active; no counterexample or global
blocker. Focused documentation commits only,no push/publication.

## Additional Abacus batch — 2026-09-09

User authorized further Abacus work. Codex retains existing private profile,
KTT ownership and sole local DB-writer role; no additional AI workers.
Submitted job 20260909T181922-14fba127f303: pending paired children
199322 and199328 of raw198861,dimension34,six extra opposite/five internal.
Both original 5s attempts remain unknown; original quotient and beam32 order
freshly reconstructed, no matching poset-hash exact cache row before submission.
20s per case,60s outer bound; installed serial CPU0,1/8GiB limits unchanged.
Only the previously verified wrapper,binary,and selected covers input uploaded.
Input SHA256 0afaa3e6f4972641f14b4021fcf4cb1eb2fc2f292871c2b51a4c099a6c56f14c.
Wrapper/binary hashes unchanged from pilot below.
Local prepared ledger 6d98995ddd4c106efd742f6b847f44277de5306afd8977642bb08be9ec156ba7.
Job done exit0 in40.332241s; both cases time limited,20.119551/20.087735s.
Validated receipt/file/header hashes, CPU/memory limits, exact input IDs and
fresh original quotient/beam covers before persisting both unknown outcomes.
No polynomial returned, hence no exact-sign claim or Ehrhart cache insertion.
Output SHA256 9963839ffe893cb9ff5d9444890ce0db8d7450a8d2bc16f47eb5e037755521ff.
Fetched outside Dropbox to /home/dev/.local/share/supervisor-compute-results/
abacus/20260909T181922-14fba127f303-b4xsantg; remote original retained.
Host supervisor may collect this job ID durably. No connection failure.
Inspect this exact ID on ambiguity; do not repeat this budget automatically.
No remote DB access or unvalidated result ingestion. No live job at checkpoint.

Local raw198861 paired queue also completed all198 under cap six extra
opposite/dimension38,beam32/5s cases,45s batches/60s guards.
DB confirms2 exact nonnegative,34 initial limits,27 dimension prunes,
135 objective prunes;198 separate started proposal records retained.
Slices0->44->88->132->176->198 took41.146374/39.568224/39.557930/
41.122356/20.561935s. No duplicates,empty cases or engine errors.
Both remote retries are among these34 limits and remain unknown.
All65 tests pass0.436s; no engine/source changes. Reports/inputs ignored.
Primary174566 degree33 seven extra opposite/five internal remains best;
negative crossover198864 degree34 seven/five retained. Counts are presentation
metrics, not minimum bad-edge distance or a flagged-Kostka witness.
Further Abacus batches authorized; choose disjoint unresolved cases or new
mutations, preserve serial limits and local validation/DB ownership.

## Abacus remote-compute pilot completed — 2026-09-09

User explicitly selected this KTT worker for the pilot. Prior crossover source
and documentation were committed; both checkouts clean and no live local scan
before pilot. Codex retains private profile/model, existing file ownership,
and sole LOCAL MariaDB-writer role. Read supervisor README Remote compute and
remote/INSTALL.md; used only supervisor-tool compute. No extra workers,
credentials/DB/checkouts uploaded, toolchain installs, administration changes,
pushes, or publication.

Submitted only trusted experiments/abacus_exact_job.py, one selected JSON input,
and maintained /cargo-target/ai-projects/release/ehrcalc (3,084,600 bytes).
ELF requires GLIBC2.34; remote verified x86_64,Python3.10.12,GLIBC2.35,
all dynamic libraries resolved,successful exact execution on both tested cases.
Kernel affinity[0,1],memory.max8589934592,swap.max0,pids.max128 verified.
Installed serial queue/limits unchanged. No CPU-compatibility claim beyond
tested executable paths. No dependency blocker or connection failure occurred.

Jobs (both done,exit0;60s outer limit,20s/case):
- 20260909T180144-b387aeb75fea: known negative196437,degree30.
  Fresh local0.276174s,remote0.401477s. Full rational polynomial,hstar and
  negative linear-573717288983/155272637520 matched exactly.
- 20260909T180330-3adc4ab17ded: disjoint crossover198861+173929,index7,
  degree31,remote0.484587s,local validation0.327867s;exact nonnegative.
  All16 proposals recorded locally:one prepared/count case,nine poset-hash
  duplicates not revalidated,three coordinate duplicates,three objective
  prunes. Cap seven extra opposite/dimension38,max two remote cases;only one
  eligible unique case survived. No unverified result or remote timeout.

SHA-256:
- wrapper a4aae249eec359b0b04255b554f7df1a61cd975060fd0bdae1d9ef5b677d885a
- executable 8b09017a36c7274b38c82e585fb41015ed1dbebb36f230cbba7af8cdce08b367
- baseline input 98c040549f6d1d7e8502bb5f249d8fa5ba1714a4e7172b699e6877b628e55261
- search input 333f7c4c3b7d718862ec895327752cf781c191b2978d3da5db6a43c06c1b22c0
- baseline output 848fb4126d8b06f967c6d2abb407a3946f3bea4b9fc21dafa3ef35e422c4ac22
- search output 887faebb47c0d719a796bbab267bd9e96b622a0ba948d89b71218a3241e7ad6e

Before result ingestion, checked receipt/file/header hashes,terminal job status,
full exact fresh-local comparisons,and independent rational hstar reconstruction.
Rebuilt original search quotient and beam permutation/covers against the prepared
proposal. Local validation ledger
4f2dcffd8d79115aa1ae96aa927c62cbc8a12b95d3eccf7abd199a4e4ff118fa
contains baseline199282/result199283. Search ledger
d279127b41ecad2182dbfedaf399ba089f853ba928508b8094c4fd9ad7292c36
contains exact result199284 and all skipped proposals. No remote DB access.

Docker fetch copies are outside Dropbox under /home/dev/.local/share/
supervisor-compute-results/abacus/ with directories
20260909T180144-b387aeb75fea-kqk3ncdx and
20260909T180330-3adc4ab17ded-c_g7l7qz. Remote originals retained; host supervisor
can durably collect both job IDs with compute fetch. Inspect recorded IDs on
connection failure;never blindly resubmit. Generated local inputs/logs ignored.
Added self-contained wrapper,exact corruption/transform test,and README notes;
all65 tests pass0.453s. Source/README/tests/handoffs owned by Codex for this
increment. No live pilot job. Pilot complete,global KTT goal remains active:
best174566 degree33 seven/five unchanged,new negative198864 retained.

## Completed first crossover-child removal queue — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress:new certified crossover
negative and tested source implementation. Raw198861 lower-remove resumed
54->95->161->183,42.830202/42.972843/20.567803s:13 exact nonnegative,
16 limits,16 dimension prunes,84 objective prunes. Cap six extra opposite,
dimension38,beam32/5s cases/45s batches/60s guards,serialized.
DB confirms entire183:15 exact nonnegative,23 initial limits,25 dimension
prunes,120 objective prunes,no duplicates/empty/errors.
Smallest six/five gap199175 dimension32 [7,13,132] resolves exact nonnegative
beam30 in8.907779s,run
7af8c2664c2454bdc9b6f345b37142a92354f12f8de6fce1f954bba8bedef674.
Thus16 exact and22 unresolved counting cases after retry;original timeouts
retained. Small gaps199019/199219 dimension33;do not promote from metrics.
Alternated another row crossover174551+173399 (previously fresh-certified
negative180617),all16:1 exact nonnegative,4 duplicates not revalidated,
11 objective prunes0.528947s,same cap7/dimension38/beam32/5s bounds.
No new negative. Child198861/certificate198864 degree34 seven/five retained;
primary174566 degree33 seven/five remains best. All lineages/gaps preserved.
All64 tests pass0.436s,source unchanged,all outcomes inDB,reports ignored,
no live job. Next paired/content mutations of198861 or other compatible
negative-parent crossovers. Column-prefix crossover is a possible future
extension for diversity,but is NOT implemented. Completed queues are bounded
coverage,not negativity exclusions;counts are not minimum bad-edge distance.
Ownership/goal active,local commits,no push/publication or extra workers.

## Bounded crossover implemented: certified negative descendant — 2026-09-09

Follow-up raw198861 lower-remove0->54/183:2 exact nonnegative,7 limits,
9 dimension prunes,36 objective prunes41.703379s;cap6/dimension38,
beam32/5s/45s/60s,same serialized bounds. All results inDB,report ignored,
no live job. Resume54/183;no improved child. Private source checkpointf553e4b.

Codex owns private frontier/tests/README and handoffs; clean checkout/no live
scan verified. Previous turn made progress. Added --mode row-crossover with
required --donor: matching explicit rectangles, exact negative parent rows,
content-modified parents rejected until compression. Reciprocal prefixes of
both masks at each interior row cut give exactly2*(a-1) stable proposals.
Donor/config ordering distinguish DB identity; existing mode identities stay
unchanged. Both parent IDs/cut/direction survive empty/skipped/duplicate rows;
parent polynomials never inherited as child evidence. Invalid donors recorded
as validation failure. All64 tests pass0.464s,cover inheritance/bounds,
validation,exact adapter integration,lineage,empty/duplicate/timeout/resume.
README updated; maintained Rust engine unchanged.
First live cross174551+196437,9x15,all16:3 exact(one negative),3 duplicates
not revalidated,10 objective prunes,no limits/errors/empty/dimension prunes,
3.885647s. Cap7/dimension38,beam32/5s cases/45s batch/60s guard.
Negative raw198861 from cut8,prefix parent174551,count3.263524s:
degree34,linear-115236644371/164094946470.
Fresh beam15 certificate198864 passes both endpoints/fullcoordinate identity
6.641688s,run26e9fc099f7cf9a246b8bbb6f4aecb18a4941d9f443983834f6d3fa89acbede9.
Seven additional opposite/five internal bans,9x15. Retain new negative lineage;
not a promotion over primary174566 degree33 seven/five,not a flagged witness.
Presentation metrics are not minimum distance. All outcomes inDB,reports ignored.
Next mutate raw198861 toward six opposite constraints,or cross other compatible
negative parents. Ownership/goal active,local source checkpoint,no push/workers.

## Completed alternate joint queue; crossover next — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Raw173903 joint resumed
685->835->1069->1080,39.800859/42.100212/5.569595s;new395 outcomes:
15 initial limits,152 dimension prunes,228 objective prunes,no exact/other.
Cap seven extra opposite,dimension38,beam32/5s cases/45s batches/60s guards,
serialized. MariaDB confirms entire1080:36 initial limits,396 dimension
prunes,648 objective prunes,no exact/duplicates/empty/errors. Completed means
bounded coverage,not a negativity exclusion. First dimension37 seven/five
gap191962 [15,98,99] stays unknown on beam30 retry30.038981s,run
3829b550cb81bba8a8429dec30cd851e75572f1ebdb654a68b3fd832137da3ec.
Original timeouts retained. No improved negative;primary174566 degree33
seven/five and all alternative lineages preserved.
Inspected frontier integration point for a new bounded genetic crossover:
compatible9x15 negative raw174551 and196437 could exchange row prefixes
of both equality masks,with reciprocal children at each interior cut.
This is a proposed next implementation,NOT an implemented or tested mode.
Require explicit donor ID,both parent IDs in DB,matching rectangle/model,
stable bounded queue,resume/dedup tests,and exact maintained-engine evaluation.
Reuse current scanner counting/recording;never infer sign from parent scores.
Next source work should add/test this small crossover mode rather than
blindly repeating completed one-parent neighborhoods. No source edit yet.
All60 tests pass0.428s,all outcomes inDB,reports ignored,no live job.
Presentation counts are not minimum distance. Ownership/goal active,local
commits,no push/publication or extra workers.

## Completed simplified-parent queue and equivalence links — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Raw196437 lower-remove-two
continued410->508->690->798,39.693421/39.349985/23.825026s.
New388 outcomes:75 exact nonnegative,14 initial limits,90 dimension prunes,
209 objective prunes,no other outcomes. Cap six extra opposite,dimension35,
beam32/5s cases/45s batches/60s guards,serialized. DB confirms entire798:
94 exact nonnegative,22 initial limits,200 dimension prunes,482 objective
prunes,no duplicates/empty/errors. Completed means bounded coverage,not
exclusion of skipped/limited cases.
Avoided redundant retries by freshly reconstructing fullcoordinate signatures:
197463 dimension29 equals exact retry195951 (old source194886),and197457
dimension30 equals exact retry197260 (old source194880). Compared full
signatures,not just hashes;equalities include blocks,marks,and full order.
Persisted both coordinate_equivalent_not_recounted records in MariaDB,
search_kind fan_coordinate_equivalence_audit,run
d314e6efd35a93ba04e491d2daa19518f27f0c370628636dff4080593b08ba80,
elapsed0.031455s. Payloads retain target/reference IDs,both mask pairs,
signature hashes,and fresh_count_performed=false. These are geometric
equivalence checks,not fresh Ehrhart counts;original timeouts preserved.
No improved negative. Primary174566 degree33 seven/five,smaller alternatives,
and simplified negative196437/certificate196438 retained. Counts are
presentation metrics,not minimum distance. All60 tests pass0.425s,source
unchanged,all attempts/outcomes inDB,reports ignored,no live job.
Next old173903 joint685/1080 or distinct negative lineage. Check fullcoordinate
equivalence before retrying simplified-parent gaps;mask-only cache misses
equivalent faces. Preserve lineages/gaps,ownership/goal active,local commits,
no push/publication or extra workers.

## Redundancy audit and simplified negative presentation — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Raw173903 joint457->685
adds7 limits,83 dimension prunes,138 objective prunes42.048466s;cap7,
dimension38,beam32/5s cases/45s batches/60s guards. DB entire685/1080:
21 limits,244 dimension prunes,420 objective prunes,no exact/other outcomes.
Audited redundancy before further mutations: raw173903 internal-first greedy
prune accepts5/26 deletions,all horizontal;6 internal and15 horizontal
deletions change the fullcoordinate face and remain uncomputed. Both fresh
5s endpoints agree;pruning loop0.315405s. Final196437 preserves the entire
negative degree30 polynomial,linear-573717288983/155272637520.
Prune run d29ee67c56845d8e149ddc5248bf9329bca50504755e0d7644752173a976971a.
Fresh beam15 certificate196438 passes0.486420s,both endpoints/fullcoordinate
identity,run624985b9820f1d387f8591c277e5a00de16c2b642e93aca6c68816de3cb1e214.
Still eight additional opposite/six internal:five removed equations do NOT
constitute an improved nonflag count. Retain simpler raw196437 for mutations.
Raw196437 lower-remove-two0->210->410/798,cap6,dimension35,same bounds:
19 exact nonnegative,8 limits,110 dimension prunes,273 objective prunes,
41.739865/40.151419s,no other outcomes. DB independently confirms totals.
Earlier gap194880 dimension30 resolves exact nonnegative beam30 in1.519968s,
run534fe210cb69486668d6915ad4c35449488398efcd5f8657b822ae2ca33cd2d5.
Old173903 two-removal queue now165 exact/21 unresolved counting cases after
two resolved retries;original timeouts and193874 unknown30s retained.
Primary174566 degree33 seven/five unchanged;all negative lineages retained.
Presentation counts are not minimum distance. All60 tests pass0.432s,source
unchanged,all deletion/scan outcomes inDB,reports ignored,no live job.
Next simplified196437 two-removal410/798 or old173903 joint685/1080.
Ownership/goal active,local commits,no push/publication or extra workers.

## Completed alternate-parent two-removal queue — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Raw173903 lower-remove-two
continued581->858->962->1221->1341->1414,42.494774/39.002017/42.433450/
40.113431/16.983551s. New833 outcomes:149 exact nonnegative,16 initial
limits,124 dimension prunes,544 objective prunes,no other outcomes.
Cap six extra opposite,dimension35,beam32/5s cases/45s batches/60s guards,
serialized. MariaDB confirms entire1414:163 exact nonnegative,23 initial
limits,212 dimension prunes,1001 objective prunes,15 duplicates not
revalidated,no empty/errors. Completion is bounded coverage,not a sign
exclusion of pruned/limited cases.
Smallest gap194886 dimension29 [6,11,114,132] resolves exact nonnegative
beam30 in1.599233s,run
23ffd18242cf0c13f2bf7977daca0f00fb2af2543158f5e7ec9014601dedbd57.
Thus164 exact and22 unresolved counting cases after this retry;original
timeouts preserved. Small gaps194880 dimension30,193880/195570 dimension31;
193874 dimension32 remains unknown30s. No improved negative. Primary174566
degree33 seven/five,smaller alternatives and verified190585 retained;
presentation counts are not minimum bad-edge distance.
All60 tests pass1.006s,source unchanged,all outcomes inDB,reports ignored,
no live job. Next raw173903 joint457/1080,selected smaller unresolved cases,
or a distinct negative lineage;do not repeat completed two-removal queue or
terminal retry identities. All lineages/gaps retained,ownership/goal active,
local commits,no push/publication or extra workers.

## Alternate-parent combined and two-removal continuation — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Raw173903 joint229->457:
7 limits,83 dimension prunes,138 objective prunes41.910549s;cap seven extra
opposite,dimension38. DB confirms joint[0,457)/1080:14 limits,161 dimension
prunes,282 objective prunes,no exact/other outcomes.
Raw173903 lower-remove-two0->377->581/1414,cap six extra opposite,
dimension35:14 exact nonnegative,7 limits,88 dimension prunes,457 objective
prunes,15 duplicates not revalidated,40.524071/41.159205s.
Both scans beam32/5s cases/45s batches/60s outer guards,serialized.
Smallest initial two-removal gap193874 dimension32 [2,3,98,132],six/six,
remains unknown on beam30 retry30.029736s,run
3c3f0c48d47144249c30a42ff79d65fdfabc4f3ed6f8f4397d0064ba7e8d9cf2.
Other initial gaps193480 dimension33 [1,2,82,132] and193860 dimension34
[2,3,82,132]. Original timeout evidence retained;none promoted from metrics.
MariaDB independently confirms all totals/no engine errors. No improved
negative;primary174566 degree33 seven/five,smaller alternatives and verified
alternate190585 retained. Counts are presentation metrics,not minimum distance.
All60 tests pass0.472s,source unchanged,all outcomes inDB,reports ignored.
No live job. Next raw173903 lower-remove-two581/1414 and/or joint457/1080;
retain all negative lineages/gaps and do not repeat terminal retry identities.
Ownership/goal active,local commits,no push/publication or extra workers.

## Alternate-parent nonflag-removal checkpoint — 2026-09-09

Codex retains KTT/private DB and handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Raw173903 joint0->229/1080:
7 initial limits,78 dimension prunes,144 objective prunes,41.822992s,no exact.
Cap seven extra opposite,dimension38,beam32/5s cases/45s batches/60s guards.
Raw173903 lower-internal all54:3 exact nonnegative,6 initial limits,
45 dimension prunes,36.884063s,cap eight opposite/dimension38,same timing.
Allow opposite count to stay eight while reducing internal bans as an
intermediate-lineage strategy,not a promotion over primary174566.
Eight/five gaps192288 dimension34 [6,11,99] and192300 dimension35 [7,12,99]
resolve exact nonnegative beam30 in5.370280/6.163518s,runs
90bea0a2ad37c110bb2bd1bd8580c1d394fd865c1df7d4bdc2e90cd28cfa817e
and f84b729b447664526b2d57fddbe9a8aa37a2e39e2d4d2b94046f041f6d698def.
Four remaining lower-internal limits192240/192276/192312/192324 dimension36;
original timeout evidence retained. Raw173903 lower-remove all163 in
0->138->163,39.095121/3.241731s,cap seven opposite/dimension35:
50 exact nonnegative,3 limits,31 dimension prunes,76 objective prunes,
3 duplicates not revalidated. Unknowns192532 dimension34 [6,11,66],
192362/192570 dimension35;seven/six presentation,not certified negative.
MariaDB independently confirms all totals/no engine errors. No improved
negative;primary174566 degree33 seven/five,smaller alternatives and freshly
certified190585 retained. Presentation counts are not minimum bad-edge distance.
All60 tests pass0.431s,source unchanged,all outcomes inDB,reports ignored.
No live job. Next raw173903 joint229/1080,paired or two-opposite mutations,
or distinct retained negative parent;avoid repeated terminal retry identities.
All lineages/gaps preserved,ownership/goal active,local commits,no push,
publication,or extra workers.

## Diversified negative parent and completed content queues — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Raw182773 content-remove
all424 in0->142->232->424,40.256238/42.283849/21.554942s.
DB confirms278 exact nonnegative,5 initial limits,122 dimension prunes,
19 empty,no duplicates/errors. Dimension34,beam32/5s cases/45s batches/
60s guards,serialized,no flag-only cap before content compression.
Fresh beam15 certificate190585 of previously unmutated raw173903 passes
both endpoints/fullcoordinate identity in0.464764s,run
5849441b9cc119a75901f2b37766f70d3d39397022c366ebd2fefed37ba30850.
9x15,degree30,eight additional opposite/six internal bans;linear coefficient
-573717288983/155272637520. Retain this distinct negative polynomial as
a diversified parent,not a promotion over primary174566 seven/five degree33.
Raw173903 content-remove dimension35,same timing/layout bounds,all604 in
0->181->292->475->604,39.054001/39.399551/39.014394/5.676961s.
DB confirms426 exact nonnegative,5 initial limits,126 dimension prunes,
26 empty,21 exact-genome duplicates not revalidated,no engine errors.
No new negative child. Queues complete only under stated bounds,not exclusion
of timeouts/skips. Degree30 seven/six alternates182123/182774 retained.
Presentation counts are not certified minimum bad-edge distance.
All60 tests pass0.426s;source unchanged. Every outcome inDB,reports ignored,
no live job. Next nonflag-reduction mutations of freshly verified raw173903
(e.g. joint cap7/dimension38),or a distinct retained negative parent;do not
repeat completed content queues or terminal retry identities.
All negative lineages/gaps preserved,ownership and goal active,local commits,
no push/publication or extra workers.

## Completed alternate two-removal and smaller content queues — 2026-09-09

Codex retains KTT/private DB/handoff ownership; clean worktrees/no live scan
verified at adoption. Previous turn made progress. Raw182773 lower-remove-two
completed535 in slices0->245->357->535,39.620916/43.141205/38.660454s.
DB confirms37 exact nonnegative,15 initial limits,116 dimension prunes,
367 objective prunes,no duplicates/empty/errors. Cap five extra opposite,
dimension36,beam32/5s cases/45s batches/60s guards,serialized.
Diversified to raw182122 full-content-remove,dimension34,same timing/layout
bounds,without flag-only objective cap before content compression.
All424 in slices0->117->214->424,39.007162/41.692304/33.133150s.
DB confirms273 exact nonnegative,5 initial limits,127 dimension prunes,
19 empty,no duplicates/errors. Smallest content gap189316 dimension32,
label11/vertical85 resolves exact nonnegative beam30 in4.852808s,run
f358b28cdc0e47f7827a00a42f5e02f17a46364faabee5c95f8fd043e3d098ec.
Thus274 exact and4 unresolved counting cases after retry;original timeout
retained. Remaining content gaps189388/189426 dimension33,189050/189428
dimension34. Completed means bounded coverage,not exclusion of skipped cases.
No negative child or improved certified presentation. Primary174566 degree33
seven/five and alternate182123/182774 degree30 seven/six retained;presentation
counts are not minimum bad-edge distance. All60 tests pass0.430s,source
unchanged,all attempts/outcomes inDB,reports ignored,no live job.
Next alternate raw182773 full-content-remove dimension34,or selectively
resolve small five/six gaps;avoid repeating terminal layout/budget identities.
Negative lineages preserved,ownership/goal active,local commits,no push,
publication,or extra workers.

## Completed second joint and first two-removal queues — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Raw182773 joint resumed
289->433->570->624,39.480183/39.010332/21.467494s. New335 outcomes:
4 exact nonnegative,14 initial limits,167 dimension prunes,150 objective
prunes. MariaDB confirms entire624:4 exact,28 limits,304 dimension prunes,
288 objective prunes,no duplicates/empty/errors. Cap six extra opposite,
dimension38,beam32/5s cases/45s batches/60s guards,serialized.
Diversified to raw182122 stronger lower flag + two opposite removals,
cap five extra opposite,dimension36,same timing/layout bounds.
All535 in slices0->230->446->535,39.020252/39.789975/24.812842s.
DB totals:37 exact nonnegative,11 initial limits,120 dimension prunes,
367 objective prunes,no other outcomes. Five/six gaps187086 dimension32
and187286 dimension31 resolve exact nonnegative on beam30 retries,
20.924159/5.541523s respectively;run keys
47f13583d5cc71e65d50e8c178eb123fb84a88a31d5d670b8e70f5f56fe21696
and f1614885c53a13b8ceeeaf9adea47b750dad8e299425842b4d85b4abb04110d2.
Thus39 exact and9 unresolved counting cases after retries;original timeouts
retained. Other small gaps187376/187846 dimension32;186916/187272/187676
dimension34. Completed means bounded coverage,not exclusion of pruned/limited
cases. No negative child or promoted presentation. Primary174566 degree33
seven/five,alternates182123/182774 degree30 seven/six unchanged;presentation
metrics are not minimum bad-edge distance. All60 tests pass0.432s,source
unchanged,all outcomes inDB,reports ignored,no live job. Next alternate
raw182773 lower-remove-two cap5/dimension36,or diversified content/weight
mutations of retained negative parents;preserve lineages and unresolved cases.
Ownership/goal retained,local commits,no push/publication or extra workers.

## Completed first smaller-parent joint queue — 2026-09-09

Codex retains KTT/private DB and handoff ownership; clean worktrees/no live
scan verified at adoption. Previous turn made progress. Continued serialized
joint flag addition + opposite/internal removal scans of raw182122/182773,
max-extra-opposite6,dimension38,beam32,5s cases/45s batches/60s guards.
182122 slices151->301->445->589->624:39.785373/39.912006/39.488094/
16.004473s;4 exact nonnegative,20 initial limits,233 dimension prunes,
216 objective prunes. Entire624 now DB-confirmed:8 exact nonnegative,
24 initial limits,304 dimension prunes,288 objective prunes;no other statuses.
182773 slices0->145->289:39.791139/39.833133s;14 initial limits,
137 dimension prunes,138 objective prunes,no exact/other outcomes.
Separate retry184704 (dimension36,six/five,edge[57,83,85]) resolves exact
nonnegative beam30 in11.255248s,run
398e6830f59b8c2c438341574fab7919ef53b4aee6311b29b62dc94a0c95d9fa.
Thus first queue has9 exact nonnegative and23 unresolved counting cases
after this retry; original timeouts retained. Its completion is bounded
coverage,not a sign exclusion for pruned/limited cases. Other small unknowns
184728/184740/184860 dimension36,six/five;184426 remains unknown30s.
No new negative or improved certified presentation. Primary174566 degree33
seven/five and alternate182123/182774 degree30 seven/six unchanged.
Presentation counts are not certified minimum distance. All outcomes inDB,
reports runs/fan-parent182122-joint38-offset*-extra6-beam5-20260909.jsonl
and corresponding182773 reports ignored. All60 tests pass0.424s,source
unchanged,no live job. Next resume182773 joint289/624,then diversify mutation
neighborhoods rather than repeatedly recount terminal layouts/budgets.
All negative lineages/gaps retained;goal active,local commits,no push or workers.

## Opposite-removal and joint checkpoint — 2026-09-09

Codex retains KTT/private DB and handoff ownership; no live scan at checkpoint.
Adopted own prior dirty companion handoff after diff/ownership inspection.
Previous strategy-only turn was no progress; this turn adds exact DB evidence.
Raw182122 and182773 lower-remove queues both complete96 under cap six
extra opposite,dimension38,beam32/5s cases/45s batches/60s outer guards.
Each:34 exact nonnegative,6 initial limits,16 dimension prunes,40 objective
prunes; no duplicates/empty/errors. Slices0->76->96 took43.642723/10.156720s
and41.890286/9.885093s respectively. Gap184020 dimension36,six/six,
resolves exact nonnegative beam30 in9.228539s,run
26890d5296063577db2cef2e902155478520a57f49739cea9d917a811463ef5a.
Raw182122 joint0->151/624:4 exact nonnegative,4 initial limits,71 dimension
prunes,72 objective prunes43.884013s,same bounds. Six/five unknowns:
184426/184582 dimension36;184366/184522 dimension38.
184426 beam30 remains unknown30.048378s,run
9d6813f2cabfcf4b9c42a83a7ccba618a8155fdfc5d89fac278122ca6f0bb0f3.
No new negative or improved certified presentation. Best174566 degree33
seven/five; alternates182123/182774 degree30 seven/six retained.
These are presentation metrics,not certified minimum bad-edge distance.
MariaDB independently confirms preceding content totals:173399 all604,
448 exact(two negative),124 dimension prunes,6 limits,26 empty;173929
all581,426 exact nonnegative,124 prunes,5 limits,25 empty,1 duplicate.
Their final slices took20.817997/36.747808s. Prior182849 beam30 resolves
nonnegative7.700391s. All60 tests pass0.429s;source unchanged.
All outcomes persisted inDB,reports ignored. MCP registry inspected;no MariaDB
MCP,existing protected MariaDbStore used. rust/CLAUDE.md absent,AGENTS applies.
Next resume182122 joint151/624 or alternate raw182773 joint; preserve all
negative lineages and unknowns. No extra workers,no push/publication;goal active.

## Second smaller negative placement — 2026-09-09

Codex retains KTT/private ownership;clean worktrees/no live scan verified.
Previous turn made progress:new certified negative alternate.182122 lower-
internal48 complete:5 exact nonnegative,32 dimension prunes,11 initial limits,
39.356989/29.995367s;cap7/dimension40/beam32/5s. Seven/five gap182212
dimension34 resolves nonnegative beam30 7.762448s.173929 content171->281:
70 exact nonnegative,39 prunes,1 limit39.724174s.173399 content269->368:
64 exact(one negative),31 prunes,4 limits40.082665s. Content caps34/36,
beam32/5s/45s batches/60s guards,serialized. New negative182534 content13/
horizontal66,degree30,linear-90004117681/155272637520,count0.223968s.
Fresh compression182655 and pruning182773 (94/113 equivalent deletions,
loop1.299187s) pass fullcoordinate/lattice and both fresh5s endpoints.
Fresh beam15 certificate182774 passes0.492935s:8x15,degree30,seven/six.
Exact coefficient arrays equal182123,but fullcoordinate quotient signatures
differ. Retain as alternate placement,not a new polynomial/improved distance.
Primary174566 degree33 seven/five unchanged. DB totals173399 content368/604:
273 exact(two negative),90 prunes,5 limits;173929 content281/581:211 exact,
67 prunes,2 limits,1 duplicate not revalidated. All60 tests pass0.429s,source
unchanged,all outcomes inDB,reports ignored,no live job. Next alternate content
173929 offset281,173399 offset368,and raw182773/182122 nonflag mutations.
Lineages/gaps retained,goal active,local commits,no push/publication.

## Diversified content search: new negative alternate — 2026-09-09

Codex retains KTT/private ownership;clean worktrees/no live scan verified.
Previous turn made progress. Alternated content-plus-equation-removal scans
between two freshly certified negative parents.173399 content[0,269)/604:
209 exact(one negative),59 dimension prunes,1 limit,41.771866/39.907223s.
173929 content[0,171)/581:141 exact nonnegative,28 prunes,1 limit,1 duplicate
not revalidated39.130514s. Dimension caps36/34,beam32/5s cases/45s batches/
60s guards. No flag-only objective cap before content compression.
Fresh certificate181467 verifies parent173929 degree29,eight/six0.391719s.
New negative181988 from173399:content label11,horizontal removal82,degree30,
linear-90004117681/155272637520,exact count0.255365s. Exact full-content
compression182003:8x15,shape(16,1^8)/(1),weight1^23;both fresh5s endpoints
and fullcoordinate lattice map agree. Pruning182122 accepts95/114 equivalent
deletions,loop1.291227s,then fresh beam15 flag certificate182123 agrees
0.492645s:seven extra opposite/six internal bans. Smaller negative alternate,
not a promotion over primary174566 degree33,9x15,seven/five.
182122 internal all6:4 dimension prunes,2 limits10.311258s.182126 dimension36
resolves nonnegative on beam30 retry11.186897s;182128 dimension40 unknown5s.
All60 tests pass0.434s,source unchanged,all attempts inDB,reports ignored,
no live job,no extra workers. Next alternate content173399 offset269 and
173929 offset171,and flag/internal mutations of smaller raw182122. Lineages/
gaps retained,goal active,local documentation commits,no push/publication.

## Completed two-removal queue and alternate parent — 2026-09-09

Codex retains KTT/private ownership;clean worktrees/no live scan at adoption.
Previous turn made progress.174551 queue finished1745->1764->1786:
2 exact nonnegative,14 limits,25 objective prunes43.981388/33.303796s.
MariaDB confirms full1786:257 initial exact nonnegative,345 initial limits,
267 dimension prunes,917 objective prunes,no duplicates/empty/errors.
First137 unfiltered,remaining1649 max-extra-opposite6;dimension40,beam32,
5s cases/45s batches/60s guards. Seven separate retries resolve seven initial
gaps;completion is bounded coverage,not mathematical exclusion of all cases.
Fresh beam15 certificate of alternate173399 is180617:degree31,eight
opposite/six internal,two negative coefficients;fullcoordinate identity and
both fresh endpoints agree0.506828s. Raw173399 previously unmutated.
Diversified to173399 joint cap7/dimension40/beam32/5s:229/1080 proposals,
144 objective prunes,78 dimension prunes,7 unknown limits42.073186s.
Seven/five unknowns180856/180880/180892/181096/181120 dimension38,
180820/181060 dimension39. Best174566 degree33 seven/five unchanged.
All60 tests pass0.422s,source unchanged,all attempts inDB,reports ignored,
no live job,no extra workers. Next alternative-parent combined/shape/weight
branches;all lineages/gaps retained,goal active,local commits,no publication.

## Strategy discussion checkpoint — 2026-09-09

User asks whether to adjust/parallelize. Codex completed already-running
174551 lower-remove-two1666->1715->1728->1745:24 limits,11 dimension
prunes,44 objective prunes,41.421831/40.542539/40.604168s,no exact/other
outcomes. Max-extra-opposite6,dimension40,beam32/5s cases/45s batches/
60s guards. MariaDB confirms filtered[137,1745):195 exact nonnegative,
294 initial limits,227 dimension prunes,892 objective prunes;unfiltered137:
60 exact,37 initial limits,40 dimension prunes. Separate retries resolve five
filtered/two unfiltered gaps. No improvement;selected174566 degree33 seven/
five retained. All60 tests pass0.431s,source unchanged,reports ignored,
all outcomes inDB,no live job. Pause launches to answer strategy question;
resume1745/1786 if continuing current queue. Goal remains active;diversified
negative populations,hard-gap counting,structural/proper-flag lanes are
proposals,not authorization for extra workers. No push/publication.

## Filtered1666 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress.174551 lower-remove-two
1507->1533->1573->1643->1666 adds17 exact nonnegative,22 limits,20
dimension prunes,100 objective prunes;39.479635/41.019248/41.985051/
40.693079s,no other outcomes. Max-extra-opposite6,dimension40,beam32/5s
cases/45s batches/60s guards,serialized. MariaDB confirms filtered[137,1666):
195 exact,270 initial limits,216 dimension prunes,848 objective prunes.
Five earlier retries resolve five initial filtered gaps;old timeouts retained.
No improved negative;selected174566 degree33 seven/five retained. All60
tests pass0.428s,source unchanged,all attempts inDB,reports ignored,no live
job. Resume1666/1786 with cap6;lineages/gaps retained,ownership retained,
goal active,focused local documentation commits,no push/publication.

## Filtered1507 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress.174551 lower-remove-two
1334->1402->1466->1492->1507 adds27 exact nonnegative,18 limits,31
dimension prunes,97 objective prunes;40.786847/39.651097/43.817570/
40.545163s,no other outcomes. Max-extra-opposite6,dimension40,beam32/5s
cases/45s batches/60s guards,serialized. MariaDB confirms filtered[137,1507):
178 exact,248 initial limits,196 dimension prunes,748 objective prunes.
Five earlier retries resolve five initial filtered gaps;old timeout observations
retained. No improved negative;selected174566 degree33 seven/five retained.
All60 tests pass0.430s,source unchanged,all attempts inDB,reports ignored,
no live job. Resume1507/1786 with cap6;lineages/gaps retained,ownership
retained,goal active,focused local docs commits,no push/publication.

## Filtered1334 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress.174551 lower-remove-two
1164->1233->1261->1279->1334 adds43 exact nonnegative,16 limits,16
dimension prunes,95 objective prunes;39.913782/40.736891/40.118888/
43.043410s,no other outcomes. Max-extra-opposite6,dimension40,beam32/5s
cases/45s batches/60s guards,serialized. MariaDB confirms filtered[137,1334):
151 exact,230 initial limits,165 dimension prunes,651 objective prunes.
Five earlier retries resolve five initial filtered gaps;old timeout observations
retained. No improved negative;selected174566 degree33 seven/five retained.
All60 tests pass0.431s,source unchanged,all attempts inDB,reports ignored,
no live job. Resume1334/1786 with cap6;lineages/gaps retained,ownership
retained,goal active,focused local docs commits,no push/publication.

## Filtered1164 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress.174551 lower-remove-two
1002->1029->1050->1083->1164 adds33 exact nonnegative,19 limits,21
dimension prunes,89 objective prunes;42.945725/39.732500/41.362651/
39.350416s,no other outcomes. Max-extra-opposite6,dimension40,beam32/5s
cases/45s batches/60s guards,serialized. MariaDB confirms filtered[137,1164):
108 exact,214 initial limits,149 dimension prunes,556 objective prunes.
Five/five179252 dimension32 resolves exact nonnegative on distinct beam30
retry12.764061s. Together with177472/177914/178184/178492,five filtered
gaps resolved;old timeouts retained. No improved negative;selected174566
degree33 seven/five retained. All60 tests pass0.422s,source unchanged,
all attempts inDB,reports ignored,no live job. Resume1164/1786 with cap6;
lineages/gaps retained,ownership retained,goal active,focused local docs
commits,no push/publication.

## Filtered1002 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress.174551 lower-remove-two
888->942->973->1002 adds13 exact nonnegative,21 limits,22 dimension
prunes,58 objective prunes;42.301564/41.043865/41.047511s,no other
outcomes. Max-extra-opposite6,dimension40,beam32/5s cases/45s batches/
60s guards,serialized. MariaDB confirms filtered[137,1002):75 exact,
195 initial limits,128 dimension prunes,467 objective prunes. Four earlier
retries resolve four initial filtered gaps. Five/five178802 dimension34
remains unknown on distinct beam30 retry30.049980s. No improved negative;
selected174566 degree33 seven/five retained. All60 tests pass0.946s,
source unchanged,all attempts inDB,reports ignored,no live job. Resume
1002/1786 with cap6;lineages/gaps retained,ownership retained,goal active,
focused local documentation commits,no push/publication.

## Filtered888 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Five/five178478 dimension34
remains unknown on distinct beam30 retry30.068662s.174551 lower-remove-two
802->816->831->849->888 adds10 exact nonnegative,29 limits,2 dimension
prunes,45 objective prunes;42.214991/40.542677/44.016168/39.666528s,
no other outcomes. Max-extra-opposite6,dimension40,beam32/5s cases/45s
batches/60s guards,serialized. MariaDB confirms filtered[137,888):62 exact,
174 initial limits,106 dimension prunes,409 objective prunes. Four earlier
retries resolve four initial filtered gaps;timeouts remain recorded. No improved
negative;selected174566 degree33 seven/five retained. All60 tests pass0.434s,
source unchanged,all attempts inDB,reports ignored,no live job. Resume
888/1786 with cap6;lineages/gaps retained,ownership retained,goal active,
focused local documentation commits,no push/publication.

## Filtered802 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress.174551 lower-remove-two
658->673->688->709->802 adds10 exact nonnegative,29 limits,23 dimension
prunes,82 objective prunes;43.250102/43.842231/42.343363/39.198719s,
no other outcomes. Max-extra-opposite6,dimension40,beam32/5s cases/45s
batches/60s guards,serialized. MariaDB confirms filtered[137,802):52 exact,
145 initial limits,104 dimension prunes,364 objective prunes. New five/five
178492 dimension33 resolves exact nonnegative on beam30 retry17.172581s.
Separate177472/177914/178184/178492 retries resolve four filtered gaps.
Five/five178478 dimension34 remains unknown at5s. No improved negative;
selected174566 degree33 seven/five retained. All60 tests pass0.434s,
source unchanged,all attempts inDB,reports ignored,no live job. Resume
802/1786 with cap6 or retry178478;lineages/gaps retained,ownership retained,
goal active,focused local documentation commits,no push/publication.

## Filtered658 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Five/five177914/178184,
both dimension34,resolve exact nonnegative on distinct beam30 retries
8.045741/8.644466s.174551 lower-remove-two536->549->618->658 adds
10 exact nonnegative,21 initial limits,25 dimension prunes,66 objective
prunes;40.221227/41.833056/39.522989s,no other outcomes. Max-extra-
opposite6,dimension40,beam32/5s cases/45s batches/60s guards,serialized.
MariaDB confirms filtered[137,658):42 exact,116 initial limits,81 dimension
prunes,282 objective prunes. Separate177472/177914/178184 retries resolve
three filtered gaps;prior timeout observations retained. No improved negative;
selected174566 degree33 seven/five retained. All60 tests pass0.430s,
source unchanged,all attempts inDB,reports ignored,no live job. Resume
658/1786 with cap6;lineages/gaps retained,ownership retained,goal active,
focused local documentation commits,no push/publication.

## Filtered536 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress. Five/five177898 dimension35
remains unknown on distinct beam30 retry30.060831s.174551 lower-remove-two
414->485->523->536 adds12 exact nonnegative,21 limits,25 dimension
prunes,64 objective prunes;43.663551/40.238936/43.843488s,no other
outcomes. Max-extra-opposite6,dimension40,beam32/5s cases/45s batches/
60s guards,serialized. Filtered[137,536):32 exact,95 initial limits,56
dimension prunes,216 objective prunes;separate177472 retry resolved one
initial gap. New five/five177914 dimension34 remains unknown at5s. No
improved negative;selected174566 degree33 seven/five retained. All60 tests
pass0.439s,source unchanged,all attempts inDB,reports ignored,no live job.
Resume536/1786 with cap6 or retry177914;lineages/gaps retained,ownership
retained,goal active,focused local documentation commits,no push/publication.

## Filtered414 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress.174551 lower-remove-two
344->367->378->401->414 adds10 exact nonnegative,29 limits,31 objective
prunes,no other outcomes;43.250184/40.560889/43.498163/40.484115s.
Max-extra-opposite6,dimension40,beam32/5s cases/45s batches/60s guards,
serialized. Filtered[137,414):20 exact,74 initial limits,31 dimension
prunes,152 objective prunes;separate177472 retry resolved one initial gap.
Five/five177476 dimension35 remains unknown on distinct beam30 retry
30.048831s. New five/five177898 dimension35 remains unknown at5s.
No improved negative;selected174566 degree33 seven/five retained. All60
tests pass0.429s,source unchanged,all attempts inDB,reports ignored,no live
job. Resume414/1786 with cap6 or retry177898;lineages/gaps retained,
ownership retained,goal active,local docs commits,no push/publication.

## Filtered344 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress.174551 lower-remove-two
168->187->204->290->344 adds8 exact nonnegative,29 initial limits,
31 dimension prunes,108 objective prunes;40.313400/40.568494/42.919267/
41.760862s. Max-extra-opposite6,dimension40,beam32/5s cases/45s batches/
60s guards,serialized. MariaDB confirms filtered[137,344):10 exact,
45 initial limits,31 dimension prunes,121 objective prunes,no other outcomes.
Five/five177472 dimension35 resolves exact nonnegative on beam30 retry
7.599663s. Five/five177488 dimension34 remains unknown on beam30 retry
30.054894s;old timeout observations retained. No improved negative;
selected174566 degree33 seven/five retained. All60 tests pass0.440s,
source unchanged,all attempts inDB,reports ignored,no live job. Resume
344/1786 with cap6;all lineages/gaps preserved,ownership retained,goal
active,focused local documentation commits,no push/publication.

## Nonflag-budget filter checkpoint — 2026-09-09

Codex retains KTT/private ownership;clean worktrees/no live scan verified
at adoption. Previous turn made progress.174551 lower-remove-two99->137
adds13 exact nonnegative,14 dimension prunes,11 limits39.820038/39.969731s.
Private commitb3386f5 adds optional --max-extra-opposite,sharing the existing
all-implied-lower-flags helper with certificate code;fresh certificate
semantics/default DB identities unchanged. Distinct capped runs persist all
proposal/result skips as objective_pruned,without a sign/minimality claim.
All60 tests pass0.430s,including identity/DB records/filter bypass and bounds.
Cap6 slices137->156->168:2 exact nonnegative,13 objective prunes,16 limits,
43.391666/40.504565s;dimension cap40/beam32/5s cases/45s batches/60s
guards,serialized. MariaDB confirms unfiltered137:60 exact,40 dimension
prunes,37 initial limits;filtered31 counts above. Older177140/177130
retries resolve two initial gaps. New five/five presentation177384 dimension40
stays unknown on distinct beam30 retry30.079775s;177388/177392 also
five/five unknowns. No improved negative;selected174566 degree33 seven/five
retained. Resume168/1786 with cap6;all skipped cases/lineages/gaps retained,
all outcomes inDB,reports ignored,no live job,goal active,local commits only.

## Two-removal99 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress.174551 lower-remove-two
35->68->84->99 adds27 exact nonnegative,21 prunes,16 initial limits;
39.207917/43.772275/42.822085s. Cap40/beam32/5s cases,45s batches/60s
guards,serialized. MariaDB confirms initial99:47 exact nonnegative,26
prunes,26 timeouts,no duplicates/empty/errors. Distinct beam30 retries
177140(dimension33) and177130(dimension34),six/five presentations,resolve
nonnegative5.654387/5.490103s. Coverage after retries49 exact nonnegative,
26 prunes,24 unresolved;old timeout observations retained. No improvement;
selected174566 degree33 seven/five unchanged. All58 tests pass0.433s,
source unchanged,every attempt inDB,reports ignored,no live job. Resume
99/1786;nonflag reduction primary,all lineages/gaps preserved,ownership
retained,goal active,focused local docs commits,no push/publication.

## Combined nonflag-removal continuation — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live scan
verified at adoption. Previous turn made progress,not blocked.174551 joint
21->37 adds8 prunes/8 unknown limits40.995649s;total37/990:21 prunes,
16 limits,no exact results. Diversified to lower-remove-two: stronger lower
flag plus two opposite removals,cap40/beam32/5s cases,45s batches/60s
guards. Slices0->17 and17->35 take41.356326/43.528540s;MariaDB confirms
20 exact nonnegative,5 prunes,10 unknown limits,no other outcomes.
Seven/four presentation176966 dimension37 resolves exact nonnegative on
distinct beam30 retry17.407517s;earlier5/15 limits retained. No improved
negative candidate;selected174566 degree33 seven/five retained. All58 tests
pass0.426s,source unchanged,all attempts inDB,reports ignored,no live job.
Next lower-remove-two35/1786 or remaining seven/four gaps,joint37/990
paused. Nonflag count primary;all lineages/gaps preserved,goal active,
ownership retained,focused local docs commits,no push/publication.

## Nonflag-first mutation checkpoint — 2026-09-09

User redirects effort from shrinking to fewer nonflag constraints. Codex
retains KTT/private search ownership. Selected174566 remains degree33,
seven additional opposite constraints/five internal bans; no minimum-distance
claim or KTT witness. Pause remaining shrinking retries176720/176808:
176838 beam15 resolves nonnegative10.026271s;176868/176660/176690
remain unknown15s limits. Shrink coverage94 exact,22 prunes,5 gaps,10
duplicates not revalidated. Older six/five gap174695 beam30 resolves
nonnegative5.821469s. New174551 joint[0,21)/990:13 prunes,8 limits,
41.250757s;paused next21. Private commit5838bd1 adds lower-internal mode:
strengthen a lower flag and remove an internal ban, stable finite product,
DB records and exact engine unchanged. All58 tests pass0.422s. First12/45
mutations:4 prunes,8 limits,40.712677s;next12 running. Cap45 allows dimension
growth from removing constraints;beam32 ordering,5s cases,45s batches/60s
outer guards,serialized. Every attempt inMariaDB,reports ignored. Maintained
Rust engine unchanged; focused local checkpoints only, no push/publication.

Lower-internal queue now complete45; MariaDB confirms28 unknown timeouts,
15 prunes,2 duplicates not revalidated,no exact/empty/engine errors. Remaining
slices40.863361/40.605395/20.470402s. Read-only seven/four presentation
176966 dimension37 remains unknown on distinct beam15 retry15.015975s;
no negative claim. Older six/five gap174765 beam30 resolves nonnegative
3.286994s. No improvement;selected174566 retained. All58 tests pass,
all attempts persisted,reports ignored,no live job at checkpoint. Next
joint21/990 or lower-remove-two mutations,not shrinking;ownership retained,
goal active,no push/publication. All negative lineages and gaps preserved.

## Seven/five alternatives checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption. Older six/five gaps174619/175099 resolve exact nonnegative
on separate beam30 retries6.757393/17.591838s.174551 internal-slide7:
5 prunes,2 unknown limits10.377979s,no exact results. Interior-shrink131
fully visited:MariaDB confirms93 exact nonnegative,22 prunes,6 unknown
limits,10 duplicates not revalidated,no empty/errors. Three slices
39.743819/39.048202/18.659768s. Scans cap40(slide)/33(shrink),beam32,
5s cases,45s batches/60s guards,serialized. No improvement;selected174566
degree33 seven/five retained. All56 tests pass0.432s,source unchanged,
all outcomes inMariaDB,reports ignored,no live job. Next shrinking gaps
or other negative lineages/combined removals;nonflag reduction primary,
ownership retained,goal active,no push/publication.

## Seven/five content queue completed — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 full-content-remove626 fully visited;MariaDB confirms
441 exact nonnegative,68 prunes,90 unknown limits,27 empty,no duplicates/
engine errors. This turn[375,626) added193 exact,16 prunes,15 limits,
27 empty;cap40/beam32/5s slices43.413449/39.007269/35.342525s,
45s batches/60s guards. Older six/five degree33 gaps174615/175105
resolve exact nonnegative via distinct beam30 retries11.000469/22.109663s;
prior15 timeouts preserved. Selected174566 degree33 seven/five unchanged.
All56 tests pass0.435s,source unchanged,every outcome inMariaDB,reports
ignored,no live job. Next remaining promising six/five gaps or new
shape/internal-slide mutations;nonflag reduction primary,ownership retained,
goal active,no push/publication. No KTT witness/minimality claim.

## Seven/five content375 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 full-content-remove[328,375)/626 added28 exact
nonnegative,8 prunes,11 unknown limits,no duplicates/empty/errors.
Four cap40/beam32/5s slices39.515040/40.516758/39.144828/40.780599s,
45s batches/60s guards,serialized. No improvement;selected174566 degree33
seven/five retained. All56 tests pass0.422s,source unchanged,every outcome
inMariaDB,reports ignored,no live job. Next375/626,nonflag reduction
primary,ownership retained,goal active,no push/publication.

## Seven/five content328 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 full-content-remove[274,328)/626 added28 exact
nonnegative,9 prunes,17 unknown limits,no duplicates/empty/errors.
Four cap40/beam32/5s slices41.340029/39.501620/43.052831/42.281137s,
45s batches/60s guards,serialized. No improvement;selected174566 degree33
seven/five retained. All56 tests pass0.424s,source unchanged,every outcome
inMariaDB,reports ignored,no live job. Next328/626,nonflag reduction
primary,ownership retained,goal active,no push/publication.

## Seven/five content274 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 full-content-remove[150,274)/626 added85 exact
nonnegative,21 prunes,18 unknown limits. Four cap40/beam32/5s slices
42.139952/39.786788/43.444527/42.942150s,45s batches/60s guards.
MariaDB confirms total274:192 exact nonnegative,35 prunes,47 unknown
limits,no duplicates/empty/errors. No improvement;selected174566 degree33
seven/five retained. All56 tests pass0.426s,source unchanged,reports
ignored,no live job. Next274/626,nonflag reduction primary,all negative
lineages preserved,ownership retained,goal active,no push/publication.

## Seven/five content-removal checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 full-content-remove[0,150)/626:MariaDB confirms107
exact nonnegative,14 prunes,29 unknown limits,no duplicates/empty/errors.
Five cap40/beam32/5s slices41.547731/39.419849/43.350704/41.925913/
39.085954s,45s batches/60s guards,serialized. These mutations vary
content and remove constraints;no candidate improvement,selected174566
degree33 seven/five retained. All56 tests pass0.432s,source unchanged,
all outcomes inMariaDB,reports ignored,no live job. Next offset150/626;
nonflag reduction primary,ownership retained,goal active,no push/publication.

## Seven/five paired queue completed — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 paired198 fully visited;MariaDB confirms106 exact
nonnegative,18 prunes,74 unknown limits,no duplicates/empty/errors.
This turn[119,198) added39 exact,6 prunes,34 limits. Scans cap40,beam32,
5s cases,45s batches/60s guards,serialized;full slice timings in private
handoff/database. No improvement;selected174566 degree33 seven/five
retained. All56 tests pass0.439s,source unchanged,reports ignored,no live
job. Next content/removal174551 to vary weights and target fewer nonflag
constraints;all earlier lineages/gaps preserved,ownership retained,
goal active,no push/publication. No KTT witness/minimality claim.

## Seven/five paired119 checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 paired[68,119)/198 added26 exact nonnegative,6 prunes,
19 unknown limits,no duplicates/empty/errors. Four cap40/beam32/5s slices
40.152422/42.215096/43.297421/41.027570s,45s batches/60s guards,
serialized. No candidate improvement;selected174566 degree33 seven/five
retained. All56 tests pass0.433s,source unchanged,every outcome inMariaDB,
reports ignored,no live job. Next paired119 or content/removal174551;
nonflag reduction primary,ownership retained,goal active,no push/publication.

## Seven/five paired coverage continuation — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 paired[36,68)/198 added16 exact nonnegative,2 prunes,
14 unknown limits,no duplicates/empty/errors. Three cap40/beam32/5s
slices41.448020/43.702834/40.040608s,45s batches/60s guards.
Read-only degree33 six/five presentations175099/175105 remain unknown
after separate beam15 retries15.036647/15.054606s. No sign/metric
certificate or candidate improvement;selected174566 degree33 seven/five
retained. All56 tests pass0.432s,source unchanged,all attempts inMariaDB,
reports ignored,no live job. Next paired68 or content/removal174551,
nonflag reduction primary,ownership retained,goal active,no push/publication.

## Four-internal flags and paired continuation — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174569 lower-flags9:1 exact nonnegative,8 initial timeouts,
41.880050s. Degree36 gap174967 resolves nonnegative on beam15 retry
7.005323s;seven other signs unknown.174551 paired[0,36)/198:25 exact
nonnegative,4 prunes,7 limits,no duplicates/empty/errors,three slices
41.113569/39.550446/42.174833s. Scans cap40,beam32,5s cases,45s
batches/60s guards,serialized;full bounds/IDs in private handoff andDB.
No candidate improvement;selected174566 degree33 seven/five retained.
All56 tests pass0.428s,source unchanged,reports ignored,no live job.
Next paired36 or content/removal on174551;nonflag reduction primary,
all negative lineages preserved,ownership retained,goal active,no push.

## Seven/five removal queue completed — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 lower-remove183 queue now fully visited;MariaDB confirms
118 exact nonnegative,17 prunes,48 initial timeouts,no duplicates/empty/
engine errors. This turn completed[78,183),bounded cap40/beam32/5s cases,
45s batches/60s guards;full slice timing in private handoff and database.
Read-only six/five ranking selected174815 degree30 and174729/174735
degree32;separate beam15 retries resolve all nonnegative in3.294742/
2.591569/6.576709s. Thus121 exact nonnegative,45 signs remain unknown.
No candidate improvement;selected174566 degree33 seven/five retained,
not KTT witness/minimal-distance proof. All56 tests pass1.334s,source
unchanged,reports ignored,no live job. Next paired/content-removal on
174551 or flags on fewer-internal174569;nonflag reduction primary,
ownership retained,goal active,no push/publication.

## Seven/five bounded continuation — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174551 internal5:3 prunes,2 unknown limits(174569 degree39,
174571 degree43),10.293882s. Lower-remove[0,78)/183:52 exact nonnegative,
8 prunes,18 unknown limits,no duplicates/empty/errors. Four slices
40.828876/42.885961/40.917262/42.228583s,cap40,beam32,5s cases,
45s batches/60s guards. Read-only ranked degree33 six/five presentations
174615/174619 both remain beam15 limited15.045846/15.029235s;
neither sign nor metric certified. Selected174566 degree33 seven/five
unchanged. All56 tests pass0.422s,source unchanged,all outcomes inMariaDB,
reports ignored,no live job. Next lower-remove78 or compensating flags
on174569;nonflag reduction primary,ownership retained,goal active,no push.

## Certified seven/five degree33 improvement — 2026-09-09

New primary174566:9x15,degree33,SEVEN additional opposite/FIVE internal
bans,linear `-15109773697/66853496710`. Source174551 is174320
lower/remove[6,11,114];fresh full coordinate/flag certificate and both
beam15 exact endpoints agree4.556579s. Shape `(16,1^9)/(1)`,weight
`(1^24)`. Improves opposite8->7 and degree35->33;not KTT witness or
minimal bad-edge distance proof. Preserve all previous negative lineages.
174320 lower-remove[32,108)/162 added76 slots:44 exact(1 negative),
11 prunes,18 initial limits,3 duplicates not revalidated,no empty/errors.
Degree34 seven/five gap174485 separately resolves nonnegative beam15
10.139339s. Full bounds/timings/IDs in private handoff and MariaDB.
Scans cap40,beam32,5s cases,45s batches/60s guards,serialized. All56
tests pass0.421s,source unchanged,reports ignored,no live job. Codex
retains ownership;next174551 nonflag mutations or oldqueue108/162,
goal active,no push/publication.

## Eight/five bounded continuation — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.174320 lower-flags9:5 exact nonnegative,1 limit,3 duplicates
not revalidated,15.897578s. Gap174343 resolves nonnegative beam15
7.700799s. Lower-remove[0,32)/162:18 exact nonnegative,6 prunes,
6 initial limits,2 duplicates not revalidated,42.053093/41.706796s.
Lowest-degree gap174383 resolves nonnegative beam15 in11.451411s;
5 new signs remain unknown. No empty/errors;selected174324 degree35
eight additional opposite/five internal unchanged,not KTT witness.
Scans cap40,beam32,5s cases,45s batches/60s guards,serialized;every
outcome inMariaDB,full timing/IDs in private handoff,reports ignored.
All56 tests pass0.420s,source unchanged,no live job. Next lower-remove
offset32,nonflag reduction primary,ownership retained,goal active,no push.

## Certified eight/five nonflag improvement — 2026-09-09

New primary selected174324:9x15,degree35,eight additional opposite/FIVE
internal bans,linear `-14848560603761/24067258815600`. Shape
`(16,1^9)/(1)`,weight `(1^24)`. Lineage173391 ->paired173929 ->internal
174289(remove99) ->lower-flag174317(row9 bound14) ->exact beam15 retry
174320 ->fresh beam15 certificate174324. Both fresh endpoint polynomials
and full coordinate/flag equality verified12.419535s. Internal6->5 improves
primary objective despite degree29->35. Not KTT witness/minimal-distance proof.
Retain degree29 eight/six backup174024 (fresh certificate0.681592s).

173391 paired180 complete:MariaDB confirms144 exact(9 negative),18 prunes,
18 unknown limits.173929 internal6:3 limits/3 prunes;174289 lower-flags9:
7 exact nonnegative,2 initial limits. Separate beam15 retries resolve174315
nonnegative7.739759s and174317 negative8.966838s. All attempts inMariaDB.
Private certificate now supports --endpoint-seconds1..20(default5) and
--relabel-beam;old identities unchanged,new budgets/layouts distinct.
Existing verified beam32 helper and full two-endpoint checks retained.
Greedy15 certificate174322 failed15.019606s;beam15 succeeds174324.
CLI budget/layout plumbing and invalid-bound tests pass,all56 tests0.426s,
README updated,no maintained Rust/public API changes. Private budget source
checkpoint9111fc3;beam/runtime checkpoint follows. Reports ignored,no live job.
Codex owns next174320 mutations with nonflag reduction primary;all negative
lineages preserved,goal active,no push/publication.

## Degree30 removal coverage checkpoint — 2026-09-09

Codex retains KTT/private DB/handoff ownership;clean worktrees/no live worker
at adoption.173391 internal6:4 prunes,2 limits(173567 degree37,173569
degree42),11.156238s. Lower-remove164 fully visited:MariaDB confirms128
exact(6 negative),18 prunes,18 unknown limits,no duplicates/empty/errors.
Six negative lineages retained;none provisionally improves best eight/six
degree30,not freshly flag-certified. Selected173480 unchanged.
173567 compensating lower-flags9:5 exact nonnegative,4 limits32.597858s;
degree34 gap173919 resolves nonnegative via beam15 retry6.256115s.
Three flags still unknown. Scans cap40/45,beam32,5s cases,45s batches/
60s guards,serialized;full timing/IDs in private handoff and MariaDB.
All55 tests pass2.417s,source unchanged,reports ignored,no live job.
Next paired or content/removal on173391;nonflag reduction primary,
ownership retained,goal active,no push/publication. No KTT witness.

## Certified degree30 eight/six — 2026-09-09

New selected173480:9x15,degree30,eight additional opposite/six internal,
linear `-5180116423/1350196848`. Source173391 is173215 lower/remove
[4,7,88];fresh full coordinate/flag certificate and both greedy5 exact
endpoints agree0.866715s. Shape `(16,1^9)/(1)`,weight `(1^24)`.
Degree-only improvement;not fewer constraints/KTT witness/minimality theorem.

Fewer-internal173247 lower-flag9:3 exact nonnegative,6 initial limits;
lowest-degree173269/173271 resolve nonnegative on separate beam15 retries
11.030218/13.960918s. Four remaining signs unknown.173215 lower-remove142
fully visited:MariaDB confirms92 exact(14 negative),24 prunes,14 unknown
limits,12 duplicates not revalidated,no empty/errors. Every negative lineage
retained,including degree30 alternatives not freshly flag-certified.
All scans cap40,beam32,5s cases,45s batches/60s guards;full timing/IDs
in private handoff and MariaDB. All55 tests pass0.428s,no source changes,
reports ignored,no live job. Codex retains ownership;next nonflag mutations
of173391,older parents preserved,goal active,no push/publication.

## Smaller certified eight/six degree31 — 2026-09-09

New selected173244:9x15,degree31,eight additional opposite/six internal,
linear `-674599361503/84446522160`,quadratic
`-13884924050228981/2432784680726400`. Shape `(16,1^9)/(1)`,weight
`(1^24)`. Source173215 from171666 lower/remove[6,10,71];fresh full
coordinate/flag certificate and both greedy5 exact endpoints agree0.989857s.
Reduces smaller backup's nine opposites to eight;improves best size/degree
tie-breaks,not overall best nonflag counts. Not a KTT witness/minimality proof.

172425 paired[38,88)/230 added33 exact nonnegative,6 prunes,11 limits.
Diversified to171666:internal6 gave4 prunes/2 unknowns;lower-remove[0,94)
/139 gave57 exact(10 negative),19 prunes,10 limits,8 duplicates not
revalidated. All negative siblings retained,one freshly certified above.
New173215 internal6 likewise4 prunes/2 unknowns (173247 degree38,
173249 degree43),10.335941s,no exact. Full bounds/timings/IDs in private
handoff and MariaDB;cap40/45,beam32,5s cases,45s batches/60s guards.
All55 tests pass0.426s,no source changes,reports ignored,no live job.
Codex retains ownership;next nonflag removal on173215 or compensating
flags on173247. Older queues preserved,goal active,no push/publication.

## Alternative nonflag queue completion — 2026-09-09

Codex owns KTT/private DB/handoff continuation;clean worktrees/no live scan
at adoption. Completed172425 lower-remove207 queue with cap40,beam32,
5s cases,45s batches/60s guards. Independent MariaDB aggregation:142 exact
(6 negative),28 prunes,27 unknown timeouts,10 duplicates not revalidated,
no empty/engine errors. This turn added132 slots;new negative172807/172819
degree32,provisional eight/six only,no certified primary-metric improvement.
Switched to paired boundary flag/opposite removal on172425:[0,38)/230,
25 exact(1 negative),6 prunes,7 limits,no other outcomes. Negative172915
degree32 retained,not freshly flag-certified. Full bounds/timing/IDs in
private handoff and MariaDB. Selected172248/172452 remain degree32 eight/six,
not a KTT witness/minimal-distance theorem. All55 tests pass0.430s,source
unchanged,reports ignored,no live job. Next paired172425 offset38 or
172187 lower-remove94;ownership retained,goal active,no push/publication.

## Degree32 alternative lineage continuation — 2026-09-09

Codex owns KTT/private DB/handoff continuation;worktrees clean at adoption.
172187 lower-remove [0,94)/198:67 exact(4 negative),15 prunes,12 limits,
no duplicate/empty/engine errors;three45s batches/60s guards,cap40,beam32,
5s cases. Fresh certificate172452 of172425:10x15,degree32,eight additional
opposite/six internal,linear `-263100760739/80224196052`,both greedy5
endpoints/full coordinate equality verified8.910616s. Additional parent,
not fewer nonflag constraints or KTT witness. Direct internal172251 remains
unknown after separate beam30 retry30.101829s.
172425 lower-remove [0,75)/207:52 exact(4 negative),12 prunes,10 limits,
1 duplicate not revalidated,no empty/engine errors. Negative descendants
retained;no count improvement certified. Full timing/IDs in private handoff
and MariaDB,all55 tests pass0.422s,no source changes,reports ignored.
No live job at checkpoint;next172425 offset75 or172187 offset94.
Ownership retained,goal active,nonflag reduction primary,no push/publication.

## Degree32 eight/six tie-break improvement — 2026-09-09

New selected172248,source172187 from170880 lower-flag/remove [9,14,71]:
10x15,degree32,eight additional opposite/six internal,linear
`-4147071266201/4813451763120`. Fresh full coordinate/flag certificate
and both maintained exact greedy5 endpoints agree,7.455316s. No fewer
nonflag constraints yet;not a KTT witness/minimal-distance certificate.
170880 lower-remove196 fully visited with beam32,cap40,5s cases,
45s batches/60s guards:143 exact(16 negative),28 pruned,25 unknown limits,
no duplicates/empty/engine errors. All lineages/outcomes inMariaDB,
reports ignored. Prune171904 final endpoint failure separately persisted;
not certified. All55 tests pass0.429s,no source changes. Full slice timing
and IDs in private handoff. Codex owns direct internal-removal continuation
on172187;nonflag reduction primary,older branches retained,goal active.

Direct internal queue6 completed10.393457s:4 pruned,2 unknown5s limits
(172251 degree39,172253 degree44),no exact results. Selected172248 remains;
next flag-plus-removal mutations,no live job at checkpoint,ownership retained.

## Nonflag-first continuation — 2026-09-09

Codex owns KTT/private mutation DB and handoff continuation. User prioritizes
nonflag removal over further shrinking. Selected170883 remains degree33,
10x15,eight additional opposite/six internal constraints,not a KTT witness.
Content/remove170880 [0,116)/654 produced three degree31 negatives among69
exact cases;32 pruned,15 limited,no duplicate/empty/scan errors. Smaller
backup171667 certified9x15,degree31,nine opposite/six internal,negative
linear and quadratic coefficients;therefore not the primary improvement.
Full coordinate compression171530/prune171666/fresh certificate171667
verified with maintained Rust exact engine. Failed transpose171483 target
timeout preserved,not certified. Full bounds/timing/lineages in private
handoff and MariaDB;reports ignored,all55 tests pass0.434s,no source changes.
Next bounded joint mutations on170880 add a flag while removing opposite
and internal constraints;cap45,beam32,5s cases,45s batches/60s guards.
No push/publication;goal active,older branches paused.

Joint170880 [0,73)/1260:57 prunes,16 time limits,no exact result in two
42.540810/42.235956s batches. Degree39 child171706 also beam15 limited
15.059097s. Signs unknown;resume73. No candidate improvement/no live scan
at checkpoint. Next direct opposite/flag-plus-removal work;ownership retained.

## Verified beam mutation scanner wiring — 2026-09-09

Private scanner now accepts --relabel-beam using the tested32-state helper.
Full coordinate equivalence/permutation/profiles/layout time retained with
original geometry;maintained exact Rust engine unchanged. Distinct layout
identity preserves defaults/depth/greedy behavior. CLI wiring regression
extended;all55 tests pass0.428s,README updated,no public API change.
Three beam15 retries170962/170954/170964 remain unknown after15.060101/
15.014097/15.059872s. Codex owns next bounded shrinking scan on170880,
selected degree33 eight/six unchanged,goal active,all attempts inMariaDB,
reports ignored,older branches paused,no push/publication.

Runtime beam scan on170880:interior-shrink146 fully visited in4 bounded
slices,95 exact nonnegative,34 prunes,16 initial limits,1 duplicate not
revalidated,no empty/errors. Two lowest-degree gaps171042/171190 resolve
nonnegative on beam15 retries10.288116/5.830071s;14 new signs unknown.
Three older fewer-ban retries remain limited. No candidate improvement;
degree33 eight/six retained. Every geometry/permutation/profile/outcome inDB,
reports ignored,all55 tests pass,private source checkpoint2570dc6,no live job.
Codex retains ownership/active goal;next shape/content-removal mutations
retaining width or paired20/210. Full bounds/timing inmutation handoff.

## Verified bounded ordering retry option — 2026-09-09

Private mutation retry adds --relabel-beam:32-state natural-order search,
ranked by peak/total live frontier,with original/depth/greedy fallbacks.
Full coordinate equivalence is mandatory;maintained exact engine unchanged.
Distinct beam32-peak-total-frontier-v1 identity,stored profiles/permutation/
layout time;old identities unchanged,inherited evidence cleared. Heuristic,
not optimal-width/sign proof. Exhaustive small mixed-face and CLI identity/
permutation tests pass;all55 tests0.424s. README updated,no public API change.
Paired170880 resumed[12,20):8 limits40.887671s,no exact result;next20/210.
Selected eight/six degree33 unchanged. Codex owns bounded runtime checks,
goal active,all records inMariaDB,reports ignored,no push/publication.

Runtime verification:known-negative170883 ->beam retry170983 exactly
reproduces full rational polynomial,1.993462s counting/0.015267s layout;
peak/total frontier improves6/116 to5/70 relative to greedy. Stalled170910
still beam15 limited15.063187s. Three older gaps170904/170958/170960
resolve exact nonnegative in1.897163/2.031394/8.545474s (DB170987/170989/
170991). Every geometry/permutation/profile/outcome persisted.8 new paired
signs still unknown,next20/210. No candidate improvement;selected degree33
eight/six unchanged. All55 tests pass,private source checkpoint eff3b53,
no maintained engine change,reports ignored,no live job. Codex retains
ownership and active goal;next verified beam retries/scans,no push/publication.

## Eight/six removal bottleneck checkpoint — 2026-09-09

Selected170883 unchanged:10x15,degree33,8 additional opposite/6 internal,
linear `-3944550620261/1444035528936`;not a KTT witness. Four5s batches28
slots:1 exact nonnegative,8 prunes,19 unknown limits,no duplicate/empty/errors.
Internal170880 all6 visited;paired next12/210. Degree40 internal child170910
also times out on separate depth15 retry15.038141s. Its lower-flag queue
all10 visited:1 nonnegative,9 unresolved;no intermediate sign assumed.
All19 new signs unknown,skipped signs unclaimed. All52 tests pass0.393s,
source unchanged,reports ignored,no live job. Codex owns continuation,
goal active,older branches paused. Full bounds/IDs/timing inMariaDB and
mutation handoff;next paired12 or verified layout work,no push/publication.

## Certified restarted eight/six degree33 — 2026-09-09

New170883:10x15,degree33,8 additional opposite constraints/6 internal bans,
linear `-3944550620261/1444035528936`,shape `(16,1^10)/(1)`,weight `(1^25)`.
Lineage170321 ->prune170666 ->lower/remove170880(row6,lower10,remove130)
->certificate170883. Full coordinate/fresh endpoints agree8.781897s.
Opposite count improves9 to8 and degree36 to33. Retain all alternatives;
not a minimum theorem or KTT witness. Two batches19 slots:11 exact(1 negative),
8 initial limits,no other outcomes. Degree31 flag child170902 resolves
nonnegative atgreedy10;degree32 child170904 still limited10.032332s.
7 new signs unknown. Parent lower/remove next92/162;new lowerflag all10
visited. All52 tests pass0.410s,source unchanged,reports ignored,no live job.
Codex owns continuation,goal active,older branches paused. Exact identities,
bounds,timing inMariaDB/mutation handoff;next170880 removals,no push/publication.

## Nine/six five-second continuation checkpoint — 2026-09-09

Selected10x15 degree36 nine/six geometry unchanged;not a KTT witness.
Three5s batches34 slots:2 exact nonnegative,15 prunes,17 unknown timeouts,
no duplicate/empty/errors. Lower/remove170666 next83/162;earlier identities
preserved. Targeted degree43 internal-removal170324 greedy15 still limited
15.058607s. Its lower-flag neighborhood all10 visited:8 above40,2 limited;
smallest degree39 descendant170854 also greedy15 limited15.059735s.
No intermediate sign or certified opposite count inferred;skipped signs
unclaimed. All records inMariaDB,full bounds/timing inmutation handoff.
All52 tests pass0.413s,source unchanged,reports ignored,no live job. Codex
owns continuation,goal active,older branches paused,no push/publication.

## Bounded per-case screen budgets — 2026-09-09

Private mutation scanner adds `--candidate-seconds`1--30,default2 unchanged.
Version4/default DB identity preserved;larger budget has distinct identity.
Loop reserves candidate budget+1 before45s cutoff;60s outer guard retained.
Tests cover identity,actual timeout argument,cutoff and invalid input beforeDB;
README updated,all52 tests pass2.454s. No public Ehrcalc API change.
Reason:negative parent takes about4s;2s screens leave nearby cases unknown.
Pruned170666 lower/remove40 [0,30)/162:2 exact nonnegative,9 prunes,19 limits,
43.943812s. No count improvement;selected10x15 degree36 nine/six unchanged.
Codex owns next bounded5s continuation from30;all records inMariaDB,
reports ignored,goal active,older branches paused,no push/publication.

Runtime follow-up:5s [30,44) gives3 exact nonnegative/4 prunes/7 limits in
41.221918s;[44,59) gives7 exact nonnegative/4 prunes/4 limits in39.714623s.
Both guards exit normally;DB config readback verifies5s/45s/distinct identity.
Across3 slices59 slots:12 exact nonnegative,17 prunes,30 unknown timeouts,
no duplicates/empty/errors. Next59/162 at5s;earlier2s evidence retained.
No count improvement. All52 tests pass,source checkpoint committed privately,
no public engine change,reports ignored,no live job. Codex retains ownership
and active goal;full bounds/timing inmutation handoff/MariaDB.

## Nine/six presentation and shrinking checkpoint — 2026-09-09

Selected geometry unchanged:10x15,degree36,9 additional opposite/6 internal,
linear `-2689218931133/1146059943600`. Four batches152 slots:78 exact
nonnegative,37 prunes,37 unknown limits,no duplicate/empty/errors.
Internal170321 all6 and shrinking146 fully visited;three separate degree31
shrinking greedy5 retries remain limited. Skipped signs unclaimed.
Pruning170321 ->170666 removes13/37 tested redundant equations with full
coordinate/fresh endpoint verification;raw18 horizontal,6 internal remain.
Fresh certificate170667 confirms unchanged9/6 and polynomial8.179561s.
Presentation improvement only,not a smaller polytope/fewer nonflag constraints
or KTT witness. Use pruned170666 for fewer redundant mutation proposals.
All50 tests pass0.397s,source unchanged,reports ignored,no live job. Codex
owns continuation,goal active,older branches paused. Full identities,bounds,
pruning,timing inMariaDB/mutation handoff;no push/publication.

## Certified restarted nine/six degree36 — 2026-09-09

New170321:10x15,degree36,9 additional opposite constraints/6 internal bans,
linear `-2689218931133/1146059943600`,shape `(16,1^10)/(1)`,weight `(1^25)`.
Lineage169735 ->internal169964 ->retry170283 ->lowerflag170306 ->retry170319
->certificate170321. Full coordinate/fresh endpoints agree8.103040s.
Degree37 nine/six ancestor170285 also certified9.208752s. One fewer internal
ban;retain degree29 nine/seven and all siblings,not a KTT witness/minimum claim.
Four batches169 slots:123 exact nonnegative,15 prunes,31 initial limits,
no duplicates/empty/errors. Nine distinct retries resolve eight source signs:
two negative/six nonnegative;23 new signs unknown. Lowerflag170283 all10
now exactly resolved;paired169735 next152/300,internal7 fully visited.
All50 tests pass1.695s,source unchanged,reports ignored,no live job.
Codex owns continuation,goal active,older branches paused. Full bounds,
IDs,timing inMariaDB/mutation handoff;next170321 mutations,no push/publication.

## Restarted nine/seven degree29 siblings — 2026-09-09

Fresh169733/169735:10x15,degree29,9 additional opposite constraints/7
internal bans,shape `(16,1^10)/(1)`,weight `(1^25)`. Source169265 lower/remove
169688/169692;linear `-367784497/185732820` and
`-89252138329/38818159380`. Full coordinate/fresh endpoints agree0.475763s
and0.356850s. Degree improves31 to29;counts unchanged,not a KTT witness
or minimum claim. Preserve both and all ancestors. Five batches299 slots:
218 exact(53 negative),40 prunes,40 unknown timeouts,1 duplicate not
revalidated,no empty/errors. Lower/remove169265 complete245;paired next54/280.
No lower opposite count/degree found among stored negatives. Significant
unrelated Lean host load observed;subsequent scans serialized,other workers
untouched. All50 tests pass1.774s,source unchanged,reports ignored,no live job.
Codex owns continuation,goal active,older branches paused. Bounds/IDs/timing
inMariaDB/mutation handoff;next169735 removal neighborhoods,no push/publication.

## Smaller restarted nine/seven degree31 — 2026-09-09

New169265:10x15,degree31,9 additional opposite constraints/7 internal bans,
linear `-168866117347/45842397744`,shape `(16,1^10)/(1)`,weight `(1^25)`.
Lineage168334 ->interior shrink169187(row11,column14) ->fresh certificate169265.
Full coordinate/fresh endpoints agree0.322230s. Smaller/lower degree at
unchanged counts;retain all alternatives,not a KTT witness or minimum claim.
Six batches354 slots:243 exact(15 negative),64 prunes,46 new limits,
1 duplicate not revalidated,no empty/errors. Shrinking168334 complete172;
content/removal next228/1047. New internal7/single38 complete,no further
count/degree improvement. Prior shrinking gap168530 resolves nonnegative
atgreedy5;new internal169268 degree39 still limited5.007764s. All46 new
signs unknown;skipped signs unclaimed. All50 tests pass0.384s,source unchanged,
reports ignored,no live job. Codex owns continuation,goal active,older branches
paused. Bounds,IDs,pruning,timing inMariaDB/mutation handoff. Next169265
combined removal neighborhoods;no push/publication.

## Nine/seven bounded neighborhood checkpoint — 2026-09-09

Selected168334 unchanged:11x16,degree36,9 additional opposite/7 internal,
linear `-4300739861017/1504203675975`;not a KTT witness. Seven batches152
slots:54 exact nonnegative,31 prunes,63 initial limits,3 duplicates not
revalidated,1 empty,no errors. Lower11/internal7/single43 fully visited;
content/removal next26/1047,interior shrinking next65/172. Two degree35
boundary greedy5 retries exact nonnegative;content168464 greedy5 still
limited5.010766s.61 new signs unknown;skipped signs unclaimed. All records
inMariaDB,bounds/timing inmutation handoff,reports ignored. All50 tests
pass0.377s,source unchanged,no live job. Codex owns continuation,goal active,
older branches paused. Retain cheaper nine/eight and all alternatives;
next resume bounded shrinking/content removal. No push/publication.

## Certified restarted nine/seven degree36 — 2026-09-09

New168334:11x16,degree36,9 additional opposite constraints/7 internal bans,
linear `-4300739861017/1504203675975`,shape `(17,1^11)/(1)`,weight `(1^27)`.
Lineage164524 ->lower/remove167691 ->certificate167894 ->internal168147
->retry168305 ->lowerflag168325 ->retry168332 ->certificate168334.
Full coordinate and both fresh greedy endpoint counts agree4.756786s.
Degree38 nine/seven sibling168307 also certified6.661300s. Keep degree29
nine/eight and all siblings;one fewer internal ban,not minimal-distance
theorem or KTT witness. Eight batches1059 slots:749 exact(24 negative),
188 prunes,59 initial limits,1 duplicate not revalidated,62 empty,no errors.
Three separate greedy5 retries resolve two negative/one nonnegative;56 new
signs unknown. Content/removal164524 complete1022 across turns;lower/remove
complete300;other current queues complete. All50 tests pass0.379s,source
unchanged,reports ignored,no live job. Codex owns continuation,goal active,
older branches paused. Bounds,IDs,pruning,timing inMariaDB/mutation handoff.
Next168334 mutation neighborhoods;no push/publication.

## Nine/eight neighborhood checkpoint — 2026-09-09

Selected164524 remains11x16,degree29,9 additional opposite/8 internal bans.
Fresh alternate certificate165753 from164772:degree30,same9/8 and shape
`(17,1^11)/(1)`,weight `(1^27)`,linear `-68708816183/22181805360`.
Full geometry/fresh endpoints agree0.896459s. Keep both;no new count
reduction or KTT witness. Nine batches660 slots:450 exact nonnegative,
117 prunes,67 initial limits,25 duplicates not revalidated,1 empty,no errors.
Paired164524 complete310;content/removal next323/1022;others complete.
Internal165948 degree39 still limited atgreedy5;flag descendant166162
degree34 resolves nonnegative at4.540208s.66 new signs unknown;skipped signs
unclaimed. All50 tests pass0.399s,source unchanged,reports ignored,no live job.
Codex owns continuation,goal active,older branches paused. Full bounds,
IDs,pruning,timing inMariaDB/mutation handoff. No push/publication.

## Certified restarted nine/eight degree29 — 2026-09-09

New164524:11x16,degree29,9 additional opposite constraints/8 internal bans,
linear `-1210368967/2823138864`,shape `(17,1^11)/(1)`,weight `(1^27)`.
Lineage163745 ->paired164256(add boundary22,remove155) ->certificate164524.
Full coordinate and both fresh greedy endpoint counts agree0.244234s.
Opposite count improves from10 to9,degree unchanged. Not a minimal-distance
theorem or KTT witness;preserve all alternatives. Following single45 queue
complete41:35 exact(20 negative),3 prunes,3 limits,no further improvement.
Internal removal164179 resolves nonnegative at3.246625s;lower flags on that
intermediate have6 exact nonnegative,4 limits,1 empty. Paired163745 complete
320 slots:260 exact(1 negative),30 prunes,30 limits. Across current queues:
380 slots,301 exact(21 negative),39 prunes,39 initial limits,1 empty,no
duplicates/errors. One retry resolves nonnegative;38 new signs unknown.
All records inMariaDB,reports ignored. All50 tests pass0.723s,source unchanged.
No live job at checkpoint;Codex owns continuation,goal active. Next164524
removal neighborhoods. Bounds,timing,gaps in mutation handoff;no push/publication.

## Smaller certified ten/eight restart — 2026-09-09

Further fresh certificate163745 lowers degree to29,still11x16 and10/8,
linear `-1210368967/2823138864`,same shape/weight. Source163433 is a
lower-flag/removal child of163351;fresh endpoints/geometry agree0.249575s.
Its parent's lower-remove45 queue complete302 slots:217 exact(32 negative),
30 prunes,32 timeouts,23 empty,no duplicates/errors. No opposite-count
improvement found among those negatives. Next joint mutations explicitly
delete opposite+internal constraints together;selected first24 above cap45.
Joint follow-up[96,188) has72 prunes/20 timeouts,no exact result;next188/2560
with explicit earlier gaps in mutation handoff. Targeted degree41 simultaneous
removal164153 still limited on separate greedy10 retry10.018476s.
52 new signs unknown across these scans;no inferred sign for skipped cases.
All50 tests pass0.373s,no source change,no live job at checkpoint. Codex owns
continuation;goal remains active,all examples persisted,older branches paused.

Certificate163351:11x16,degree30,10 additional opposite constraints/8 internal
bans,linear `-68708816183/22181805360`,shape `(17,1^11)/(1)`,weight `(1^27)`.
Lineage161618 ->162326 ->compression162636 ->prune162820 ->163003
->flag163329 ->163351. Full coordinate/fresh endpoint checks agree0.392713s.
Smaller at unchanged counts;retain degree24 ten/nine and all siblings.
Not a minimal-distance theorem or KTT witness. Eight bounded batches:
796 slots,550 exact(99 negative),149 prunes,46 initial limits,51 empty,
no duplicates/errors. Five separate5s retries resolve two nonnegative;
44 initial signs unknown. Content/removal161618 next346/1143;others complete.
Exact IDs,bounds,pruning,timing in MariaDB and mutation handoff. All50 tests
pass,source unchanged,reports ignored. No live scan at checkpoint;Codex
owns continuation,prioritizes fewer constraints,older branches paused.
Goal active,no push/publication;next combined removals on163351.

## Certified restarted ten/eight tradeoff and greedy endpoints — 2026-09-09

New161620:13x16,degree31,10 additional opposite constraints/8 internal
bans,linear `-26354632895/21879326196`,shape `(17,1^13)/(1)`,weight `(1^29)`.
Lineage159634 ->159690 ->160342 ->160775 ->161308 ->greedy certificate161620.
Fresh coordinate/endpoint checks agree0.432258s. Separate depth failure161412
preserved (5.015183s). Smaller ten/nine branch161618:12x16,degree24,linear
`-17095754261/5354228880`,shape `(17,1^12)/(1)`,weight `(1^28)`;compression,
pruning and fresh certificate verified. Keep both and all siblings;not a
minimal-distance theorem or KTT witness.
Private lower-flag certificate helper adds verified greedy endpoint option,
distinct identity preserving default depth behavior. Both fresh counts and
full geometry checks remain mandatory. README/regression updated;all50 tests
pass,no public Ehrcalc API change. Four queues:484 slots,366 exact(122
negative),101 prunes,3 duplicates,14 limits,no scan errors/empty cases.
Content/removal159634 next349/1185;others complete. All14 scan signs unknown,
skipped signs unclaimed. Bounds,IDs,timing in MariaDB/mutation handoff.
Caps36--40,2s cases,45s batches/60s guards. Reports ignored,no live scan;
Codex owns continuation,goal active,older branches paused,no push/publication.

## Genuine opposite reduction: restarted ten/nine degree24 — 2026-09-09

New159634:13x16,degree24,10 additional opposite constraints/9 internal
bans,linear `-17095754261/5354228880`,shape `(17,1^13)/(1)`,weight `(1^29)`.
Lineage158898 ->lower flag/removal159180 ->certificate159634. Fresh
coordinate/endpoint checks agree (0.088652s). Actual additional-opposite
reduction from eleven;degree25 sibling159655 also certified10/9. Keep all
alternatives. Not global-frontier improvement,minimal-distance theorem or
KTT witness. Internal-removal159690/flag160342 are exactly negative with8
internal bans,but separate degree33 certificate160343 times out5.015482s;
its inherited per-cut opposite count is NOT certified. Selected159634 retained.
Eight batches:773 slots,522 exact(76 negative),113 prunes,40 limits,
98 empty,no duplicates/scan engine errors. All queues fully visited;
40 new signs unknown,skipped signs unclaimed. Certificate failure separately
recorded. IDs,bounds,timing in MariaDB/mutation handoff. Caps36--45,2s
cases,45s batches/60s guards. All49 tests pass,source unchanged,reports
ignored,no live scan. Codex owns continuation,goal active,older branches
paused,no push/publication. Continue removal-first prioritization.

## Removal-first restarted eleven/nine degree25 candidate — 2026-09-09

User reaffirmed fewer bad edges primary,search judgment delegated. New158898:
13x16,degree25,11 additional opposite constraints/9 internal bans,linear
`-15368048617/1070845776`,shape `(17,1^13)/(1)`,weight `(1^29)`.
Lineage157041 ->content/removal158096 ->compression158500 ->prune158719
->lower flag158737 ->flags158800/158874 ->certificate158898. Fresh
coordinate/endpoint checks agree (0.502563s). Prune183/212 in2.429952s.
Additional opposite count unchanged; one internal ban removed and size
reduced at one-degree cost. Preserve degree24 parent and all siblings.
Not global-frontier improvement,minimal-distance theorem or KTT witness.
Eight queues:588 slots,329 exact(98 negative),188 prunes,57 duplicates,
12 limits,2 empty,no scan engine errors. Two certificate depth5 failures
preserved separately; later lower-degree descendants freshly certified.
Two direct opposite-removal depth5 retries remain limited;all12 scan signs
unknown,skipped signs unclaimed. Content/removal157041 next400/1112;
other queues complete. Bounds,IDs,timing in MariaDB/mutation handoff.
Caps32--45,2s cases,45s batches/60s guards. All49 tests pass,source
unchanged,reports ignored,no live scan. Codex owns continuation,goal active,
older branches paused,no push/publication. Prioritize fewer nonflag constraints.

## Smaller restarted degree24 eleven/ten candidate — 2026-09-09

New157041:14x16,degree24,11 additional opposite constraints/10 internal
bans,linear `-1968472027/205931880`,shape `(17,1^14)/(1)`,weight `(1^30)`.
Lineage155516 ->shrink156110[15,15] ->certificate157041. Fresh coordinate
and endpoint checks agree (0.091477s). New-restart improvement,not global
frontier,minimal-distance theorem or KTT witness. Degree29 15x17 eleven/ten
sibling156941 and all alternatives retained. Next shrinking no new negative.
Seven queues:987 slots,555 exact(88 negative),376 prunes,30 duplicates,
24 limits,2 empty,no errors. Two separate depth5 retries remain limited;
all24 new signs unknown,skipped signs unclaimed. Full-content/removal155516
next400/1220;other queues complete. IDs,bounds,timing in MariaDB/mutation
handoff. Caps32/34/36/40,2s cases,45s batches/60s guards. All49 tests pass,
source unchanged,reports ignored,no live scan. Codex owns continuation,
goal active,older branches paused,no push/publication.

## Smaller restarted eleven/eleven degree24 candidate — 2026-09-09

New155516:15x17,degree24,11 additional opposite constraints/11 internal
bans,linear `-1968472027/205931880`,shape `(18,1^15)/(1)`,weight `(1^32)`.
Lineage153513 ->content/removal154528 ->compression154917 ->prune155288
->certificate155381 ->lower flag155402 ->flag155476 ->certificate155516.
Fresh full-coordinate/endpoint checks agree (final0.091049s);prune240/272
deletions in3.195189s. Retain degree23 16x17 12/11 sibling155383 and all
negative alternatives. New-restart improvement,not global-frontier
improvement,minimal-distance theorem or KTT witness.
Ten queues:891 slots,471 exact(146 negative),369 prunes,24 duplicates,
27 initial limits. One greedy5 retry resolves internal limit negative;
26 new signs unknown. No errors/empty cases,skipped signs unclaimed.
Full-content/removal153513 next356/1100;other queues complete. All bounds,
IDs,timing in MariaDB/mutation handoff. Caps32/34/36/40,2s cases,45s
batches/60s guards. All49 tests pass,source unchanged,reports ignored,
no live scan. Codex owns continuation,goal active,older branches paused,
no push/publication.

## Smaller degree23 restarted twelve/twelve candidate — 2026-09-09

New153513:16x17,degree23,12 additional opposite constraints/12 internal
bans,linear `-191262307/41186376`,shape `(18,1^16)/(1)`,weight `(1^33)`.
Lineage151502 ->shrink151632/certificate152260 ->internal152500/152971
->zero label34 result153150 ->compression153196 ->prune153512 ->153513.
Coordinate-map certificates and fresh endpoint counts agree;final certificate
0.069070s. Redundancy pruning277/311 deletions,3.572589s. Keep degree22
16x18 13/13 alternate152258. New-restart improvement,not global-frontier
improvement,minimal-distance theorem or KTT witness. All siblings retained.
Nine queues:823 slots,394 exact(95 negative),338 prunes,75 duplicates,
16 initial limits,no errors/empty cases. Three separate greedy5 retries
resolve cases nonnegative;13 new signs remain unknown. All queues visited,
skipped signs unclaimed. Bounds,IDs,timing in MariaDB/mutation handoff.
Caps32/36/40,2s cases,45s batches/60s guards. All49 tests pass,source
unchanged,reports ignored,no live scan. Codex owns continuation,goal active,
older branches paused,no push/publication.

## Restarted degree26 branch with13/14 constraints — 2026-09-09

New151502:17x19,degree26,13 additional opposite constraints/14 internal
bans,cubic `-10159311283213/49423651200`,shape `(20,1^17)/(1)`,weight
`(1^36)`. Raw opposite24,11 lower-flag forced. Fresh coordinate/endpoint
certificate agrees (0.145764s). Lineage143802 ->internal146487/certificate
148716(degree23,13/15) ->internal149389(degree37) ->lower flag151247
->certificate151502. Keep cheaper degree22/23 options and all siblings.
Not global-frontier improvement,minimal-distance certificate or KTT witness.
Thirteen bounded batches:2539 slots,801 exact(31 negative),1701 prunes,
37 duplicates,no limits/errors/empty cases. All queues fully visited;
skipped signs unclaimed. IDs,bounds,timing in MariaDB/mutation handoff.
Caps30/32/40,2s cases,45s batches/60s guards. All49 tests pass,source
unchanged,reports ignored,no live scan. Codex owns continuation,goal active,
older branches paused,no push/publication.

## Restarted degree22 branch improves to13/16 — 2026-09-09

New143802:17x19,degree22,shape `(20,1^17)/(1)`,weight `(1^36)`,linear
`-14376701/2238390`,13 additional opposite constraints/16 internal bans.
Lineage133713 ->slide138372 ->internal removal141994 ->flag143073
->certificate143802. Fresh endpoint counts and full coordinate checks
agree (0.051307s). New-restart improvement,not global-frontier improvement,
minimal-distance certificate or flagged-Kostka witness. Siblings retained.
Eleven bounded batches:4699 slots,367 exact(7 negative),4277 prunes,
55 duplicate skips,no limits/errors/empty cases. Joint133713 all3757
visited in disjoint slices;68 exact nonnegative,3689 prunes. All other
queues complete;skipped signs unclaimed. Prior four unknown signs remain.
All IDs,bounds,timing,lineages in MariaDB/mutation handoff. Cap30,2s cases,
45s batches/60s guards. All49 tests pass,source unchanged,reports ignored,
no live scan. Codex owns continuation,goal active,older branches paused,
no push/publication.

## Restarted combined queues complete; twelve gaps resolved — 2026-09-09

133713 full-content/remove1067 and lower-remove221 queues fully visited.
Six new bounded batches:1048 slots,380 fresh exact nonnegative,652 prunes,
16 initial limits,no errors/duplicates/empty cases. Twenty separate retries
resolve12 limits nonnegative. Remaining signs unknown:136635 degree37,
136661 degree38,136687 degree39,136031 degree40. Smallest also exceeds
depth10 (10.024551s); case-specific gaps,not a global blocker. Skipped signs
unclaimed. Full-content queue aggregate322 exact/745 prunes; lower-remove
after retries68 exact/149 prunes/4 unresolved. No negative/frontier change.
All identities,bounds,timing,layouts in MariaDB/mutation handoff. Caps28/40,
2s cases,45s batches/60s guards;5s retries and one10s retry. All49 tests
pass,source unchanged,reports ignored,no live scan. Codex owns continuation,
goal active,older branches paused,no push/publication or KTT witness.

## Smaller degree22 restarted branch,13/17 constraints — 2026-09-09

New133713:17x19,degree22,shape `(20,1^17)/(1)`,weight `(1^36)`,linear
`-32641909/2238390`,13 additional opposite constraints/17 internal bans.
Lineage131665 ->132483 internal removal ->132591 flag ->133403 shrinking
->133477 internal removal ->133613 flag ->certificate133713. Full coordinate
checks and fresh endpoint counts agree (0.049211s). New-restart improvement,
not global-frontier improvement,minimal-distance certificate or KTT witness.
Fourteen bounded queues:1234 slots,457 fresh exact(34 negative),738 prunes,
39 duplicate skips,no limits/errors/empty cases. Skipped signs unclaimed.
First13 queues complete;133713 full-content/remove next300/1067. All exact
IDs,lineages,bounds,timing in MariaDB/mutation handoff. Caps26/28,2s cases,
45s batches/60s guards. All49 tests pass,source unchanged,reports ignored,
no live scan. Codex owns continuation,goal active,older branches paused,
no push/publication.

## Renewed direct original degree21 restart — 2026-09-09

User steering pauses current branches; original DB213 is the active root
again. Fresh audit129268--129270 verifies degree21,linear `-168011/330`,
independent t=0..25 values and maintained Rust coefficients/h* (0.012503s).
New direct lineage213 ->129600 ->compression130471 ->prune130983 gives
19x21,degree21,linear `-101918449/1492260`,shape `(22,1^19)/(1)`,weight
`(1^40)`,16 additional opposite/22 internal bans,certificate131588.
Shrinking child131665/certificate132409:18x20,degree22,14 additional
opposite/20 internal,linear `-39799937/8953560`. Full coordinate checks
and fresh endpoint counts agree. Not a global-frontier improvement or
flagged-Kostka witness. Prior later branches saved but paused.
Five bounded queues:1329 slots,491 exact(114 negative),817 prunes,21
duplicate skips,no limits/errors/empty cases; all skipped signs unclaimed.
Original full-content/remove next1020/8815, joint next300/126700. Other
three queues fully visited. Exact bounds,timing,IDs and prior pending batch
terminal results in MariaDB/mutation handoff. All49 tests pass,source
unchanged,reports ignored,no live scan. Codex owns continuation,goal active,
no push/publication. Protected local MariaDB adapter; no DB MCP registered.

## Degree29 nine/nine negative candidate — 2026-09-09

New128214:degree29,source11x14,shape `(15,1^11)/(1)`,weight `(1^25)`,
linear `-884916547/750109360`,9 additional opposite/9 internal bans.
Lineage127402 ->127786/certificate128012(degree30) ->128035 ->128214.
Version2 lower-flag certificate includes all weaker implied zero masks;
fixes conservative version1 overcount without changing geometry/polynomial.
Full coordinate checks and two fresh endpoint counts agree. Not a minimal-
distance theorem or KTT witness. Sibling128023/earlier tradeoffs retained.
Nine bounded scans:377 slots,193 exact(3 negative),137 prunes,17 duplicates,
30 limits,no errors. Shrinking150 visited with19 unknowns;lower-remove11
unknowns. Separate degree41 direct-removal127455 depth5 attempt times out.
All bounds,IDs,timing in MariaDB/mutation handoff;skipped signs unclaimed.
Helper/README/regression updated,all49 tests pass,no public Ehrcalc API change.
Reports ignored,no live scan. Codex owns continuation,goal active,original213
restart only,older branches paused,no push/publication.

## Degree31,nine/nine,smaller11x14 candidate — 2026-09-09

New127402:degree31,source11x14,shape `(15,1^11)/(1)`,weight `(1^25)`,
linear `-10129665704623/8022419605200`,9 additional opposite constraints
and9 internal bans. Six explicit lower-flag forcing equations account for
6 of15 canonical opposite equations. Coordinate certificates and fresh
endpoint counts verified;not a minimal-distance theorem or KTT witness.
Lineage121891 ->123203/retry126847 ->compression126869 ->prune127036
->certificate127044 ->lower flag127057/certificate127110(degree33)
->zero label23 result127231 ->compression127238 ->prune127401 ->127402.
Prior tradeoffs retained;next tested zero-weight cuts nonnegative.
Twenty old-gap depth5 retries resolve16 cases(2 negative);four old signs
remain unknown. Six fresh bounded queues:120 slots,105 exact(11 negative),
7 prunes,6 duplicates,2 limits. No engine errors. All IDs,ranges,bounds,
timing in MariaDB/mutation handoff. All49 tests pass,source unchanged,
reports ignored,no live scan. Codex owns continuation,goal active,original213
restart only,older branches paused,no push/publication.

## Twenty-three content/removal gaps resolved — 2026-09-09

29 separately recorded retries resolve23 initial121891 full-content/removal
limits exactly nonnegative:all degree34/36/37 cases. Six earlier retry limits
preserved. Remaining20 signs atdimensions38--41 unknown;pruned signs still
unclaimed. Most depth5 retries suffice;degree34 required a justified depth30
attempt(finished7.080377s),then depth10 siblings(7.108956/7.072282s).
All outer60s guards,reduced priority,maintained exact engine;no engine errors.
Full IDs,timing,layouts,bounds in MariaDB/mutation handoff. Negative frontier
unchanged,all49 tests pass,source unchanged,reports ignored,no live scan.
Codex owns continuation,goal active,original213 restart only,older branches
paused,no push/publication or flagged-Kostka witness.

## Shrinking gaps resolved and dual presentation checks — 2026-09-09

Seven remaining shrinking121891 time limits resolve exactly nonnegative
with separate depth5 retries. Full176 queue now96 exact nonnegative,
79 prunes,1 duplicate,no remaining time limits;skipped signs unclaimed.
Private transpose-complement helper/tests/README/scanner dedup added.
Full coordinate reversal/mark/slant certificate and exhaustive small mixed
mask tests pass,including fresh endpoint-layout wiring;all49 tests pass.
Large15x12 dual of121891 passes coordinate certificate and source count,
but both depth/greedy target5s recounts time out. DB126773/126776 remain
verification_failed with no inherited negative evidence. No new verified
negative/frontier improvement. Bounds,timing,IDs in mutation handoff/MariaDB.
No public Ehrcalc API change,reports ignored,no live scan. Codex owns
continuation,goal active,original213 restart only,older branches paused,
no push/publication or flagged-Kostka witness.

## Higher-degree joint-removal bounds recorded — 2026-09-09

All1440 joint and20 slide-internal proposals from121891 exceed cap43:
dimensions46--93 and47--91 respectively. Proposal coverage only,no sign
certificate. Five degree46 joint cases remain unknown after greedy5/depth5
retries;degree47 internal123794 remains unknown after greedy5. One monitored
depth30 retry of123838 also timed out;at8s CPU99.5%,RSS81020KiB (not peak).
All12 new retries persisted separately,prior evidence preserved,no engine
errors. Negative frontier unchanged. Local counting obstacle,not global
blocker;other neighborhoods/lineages remain. Exact bounds,IDs,timing in
mutation handoff/MariaDB. All47 tests pass,source unchanged,reports ignored,
no live scan. Codex owns continuation,goal active,original213 restart only,
older branches paused,no push/publication or flagged-Kostka witness.

## Complete content/removal and shrinking proposal coverage — 2026-09-09

121891 full-content/removal582 and shrinking176 queues now fully visited.
Six new batches:523 slots,182 exact nonnegative,304 prunes,1 duplicate,
36 initial limits. Fifteen separate depth5 retries resolve shrinking cases
nonnegative;seven shrinking signs and43 full-content/removal signs remain
unknown. Skipped signs unclaimed. Smallest internal removal hasdegree47;
separate five-second depth attempt123794 timed out. Other nine internal
removals dimension56--92 remain pruned. No negative frontier improvement.
All attempt IDs,bounds,timing in MariaDB/mutation handoff. Caps41/43,
two-second cases,45-second batches/60-second guards;no engine errors.
All47 tests pass,source unchanged,reports ignored,no live scan. Best121891
retained:degree37,12x15,9 additional opposite/10 internal bans. Codex owns
continuation,goal active,original213 restart only,older branches paused,
no push/publication or flagged-Kostka witness.

## Nine-additional branch coverage and twenty resolved gaps — 2026-09-09

Six bounded121891 batches:396 slots,99 fresh exact nonnegative,233 prunes,
64 initial limits. Twenty smaller lower-flag/removal cases resolve exactly
nonnegative using separately verified greedy five-second retries;44 new
signs remain unknown(15 lower-remove,29 full-content/remove). Initial failed
attempts preserved. Lower-remove124 fully visited;full-content/remove next245
of582. No frontier improvement,KTT witness,engine errors,or skipped-sign claim.
Best121891 remains degree37,12x15,9 additional opposite/10 internal bans.
All ranges,IDs,bounds,timing in MariaDB/mutation handoff. Cap41,two-second
cases,45-second batches/60-second guards;all47 tests pass,source unchanged,
reports ignored,no live scan. Codex owns continuation;goal active,original213
restart only,older branches paused,no push/publication.

## Nine additional opposite constraints,degree37 — 2026-09-09

New121891:source12x15,degree37,shape `(16,1^12)/(1)`,weight `(1^27)`,
linear `-108994039977551/534293145706320`,10 internal bans and9 additional
opposite constraints. Lower flags augmented row13>=15,row12>=12 force3
of12 canonical opposite equations. Full coordinate-order certificate and
two fresh endpoint counts verified. This improves the previous10-additional
branch's size/count tradeoff,not a minimal-distance theorem or KTT witness.
Lineage120214 ->120513 ->compression121351 ->internal-first prune121866
->certificate121891. Private certificate helper,README,scanner dedup and
two regression tests added;all47 tests pass,no public Ehrcalc API change.
Five bounded scans:627 slots,60 exact(3 negative),518 prunes,36 duplicates,
1 empty,12 initial limits. Three resolve exactly nonnegative under distinct
depth5 retries;nine new signs remain unknown. All IDs,bounds,timing,skips,
lineages in MariaDB/mutation handoff. Cap41,two-second cases,45-second
batches/60-second guards;reports ignored,no live scan. Codex owns continuation,
goal active,original213 restart only,older branches paused,no push/publication.

## Smaller degree37 ten-additional-constraint chain — 2026-09-09

New119907 ->120028 ->120089 ->lower flags120154/120188/120214:
finaldegree37,source13x15,shape `(16,1^13)/(1)`,weight `(1^28)`,linear
`-108994039977551/534293145706320`. Ten internal bans;union of ALL three
ancestor forcing masks accounts for4 of14 raw opposite equations,leaving
10 additional opposite constraints. Same degree/count tradeoff as older
14x15 chain in smaller instance. No minimal-distance or flagged-Kostka claim.
Fresh exact depth5 recount agrees in1.417226s;next degree36 flag cut is
exact nonnegative. Prior negative alternatives retained.
Ten complete bounded queues:146 slots,26 exact(7 negative),44 prunes,
54 duplicates,22 initial limits. Four distinct depth5 retries resolve limits
nonnegative;18 new signs remain unknown. No engine errors/empty cases.
Mutation handoff/MariaDB record every range,bound,timing,mask and retry.
Caps42--44,two-second cases,45-second batches/60-second guards. All45 tests
pass,source unchanged,reports ignored,no live scan. Codex owns continuation,
goal active,original213 restart only,older branches paused,no push/publication.

## Smaller degree37 negative face and depth endpoint support — 2026-09-09

107400 ->119153 ->compression119623 ->prune119830 reduces source14x15
to13x15 with exact coordinate-order and fresh endpoint polynomial agreement.
Degree37,linear `-108994039977551/534293145706320`,shape `(16,1^13)/(1)`,
weight `(1^28)`,12 raw opposite/12 internal bans. Further opposite removal
119834 ->exact retry119907 givesdegree38,11/12,linear
`-405244004991473/89048857617720`. Smaller tradeoffs,not certified minimal
bad-edge counts or flagged-Kostka witnesses. Prior candidates retained.

Private compression/pruning helpers now support existing verified depth
layouts with distinct DB identities; README and two wiring tests added,
all45 tests pass. No public Ehrcalc code change. Five bounded scans:286 slots,
78 fresh exact(6 negative),158 prunes,24 duplicate skips,26 initial limits.
Two separate depth5 retries resolve one negative/one nonnegative. Remaining
24 new full-content/removal signs unknown;107400 next235 of682. Mutation
handoff/MariaDB record IDs,ranges,bounds,timing,failed greedy compression.
Cap41,two-second cases,45-second batches/60-second guards;reports ignored,
no live scan. Codex owns continuation;goal active,original213 restart only,
older branches paused,no push/publication.

## Sibling coverage and retained computational gaps — 2026-09-09

Six bounded114125 batches visit1586 slots:269 fresh exact nonnegative,
1291 dimension prunes,25 duplicate skips,1 empty,no engine errors/scan
limits. Full-content/removal queue1163 fully visited;skipped signs unclaimed.
Separate greedy5 and depth10 retries of115778/115802/115824 all time out;
these three paired-removal signs remain unknown. Original evidence preserved.
All identities,bounds,timing in MariaDB/mutation handoff. Cap26,two-second
cases,45-second batches/60-second guards,maintained Ehrcalc,verified layouts.
Negative frontier unchanged,no flagged-Kostka witness. Source unchanged,
43-test baseline,reports ignored,no live scan. Codex owns continuation;
goal active,original213 restart only,older branches paused,no push/publication.

## Degree22 flag descendant and paired-removal coverage — 2026-09-09

Eleven bounded queues fully visited:1141 slots,233 fresh exact,21 negative,
860 dimension prunes,38 duplicate skips,4 empty,6 initial limits. Three
limits resolve exactly nonnegative under distinct five-second depth retries;
115778/115802/115824 (dimension37) remain unknown. All evidence in MariaDB
and mutation handoff;no engine errors,skipped signs unclaimed.
New112882 ->114023 internal removal ->114121 boundary flag:degree22,
15 raw opposite/16 internal bans,linear `-42068777/1279080`,18x20,
shape `(21,1^18)/(1)`,weight `(1^38)`. Separate fresh depth count agrees.
Not a global frontier improvement or flagged-Kostka witness. Lower-count
and lower-degree saved alternatives retained. Paired-removal107400 all456
visited;ancestor flag certificates not automatically inherited on deletion.
Caps26/28/41,two-second cases,45-second batches/60-second guards. All43
mutation tests pass,source unchanged,reports ignored,no live scan. Codex
owns handoff continuation,original213 restart only,older branches paused,
goal active,no push/publication.

## Direct degree21 restart diversification — 2026-09-09

Eleven bounded DB-backed batches visit2463 slots:782 fresh exact results,
39 negative,1562 dimension prunes,119 duplicate skips. No time limits or
engine errors. Cap24,two-second cases,45-second batches/60-second guards,
verified depth/greedy layouts and maintained Ehrcalc. Original213 lower-
flag/removal queue676 fully visited; full-content/removal next720 of8815.
Skipped signs unclaimed. Mutation handoff records every range/timing/lineage.

New original213 ->109661 ->compression110184 ->prune110698 ->shrink111556:
degree21,18x20,15 raw opposite/18 internal bans,linear `-2365057/87780`.
Internal removal112884 givesdegree22,15 opposite/17 internal,linear
`-19596637/175560`,confirmed by separate fresh depth-layout count in0.024830s.
Shape `(21,1^18)/(1)`,weight `(1^38)`. Distinct saved negative lineages,
not global frontier improvements or flagged-Kostka witnesses;raw counts
are not certified minimal bad-edge distances. All prior tradeoffs retained.
All43 mutation tests pass,source unchanged,reports ignored,no live scan.
Codex owns handoff continuation;goal active,older branches paused,no push.

## Four additional exact shrinking resolutions — 2026-09-09

Depth-layout five-second retries resolve107836/107908 (dimension34) and
107906/107929 (dimension35) exactly nonnegative. Shrinking107400 now has35
unresolved initial limits. Eight other shrinking retries and six lower-flag
removal retries remain limited under separately recorded layouts/bounds.
All18 attempts/timing/IDs in MariaDB and mutation handoff;prior evidence
preserved,no engine errors. Negative frontier unchanged,no flagged-Kostka
witness. Source unchanged,43-test baseline,reports ignored,no live scan.
Codex owns handoff continuation;goal active,original-restart scope,old
branches paused,no push/publication.


## Twelve additional exact gap resolutions — 2026-09-09

Four dimension32 and eight dimension33 shrinking cases resolve exactly
nonnegative under separate five-second depth-layout retries. Shrinking107400
now has39 unresolved initial limits; six dimension33 retries remain limited.
New lower-flag/removal queue107400:all106 visited,1 exact nonnegative,
75 prunes,22 duplicates,2 empty,6 limits,19.697739s. Cap41,two-second cases,
45-second batch/60-second guard. Mutation handoff/MariaDB record all IDs,
bounds/layouts/timing;old attempts preserved,skipped signs unclaimed.
Negative frontier unchanged,no flagged-Kostka witness. All43 tests pass,
source unchanged,reports ignored,no live scan. Codex owns continuation;
goal active,original-restart scope,old branches paused,no push/publication.


## Shrinking coverage complete; eleven gaps resolved — 2026-09-09

All206 shrinking proposals from degree37 lower-flag107400 now visited in
disjoint slices. Initial results:32 exact nonnegative,95 prunes,16 duplicates,
1 empty,62 time limits. Eleven separate depth-layout retries resolve nine
degree24 and two degree32 cases as nonnegative;51 shrinking signs remain
unknown. Four greedy five-second retries were limited before two of those
cases resolved with depth. All attempts preserved in MariaDB;mutation handoff
records IDs,layouts,bounds and timing. No frontier improvement or flagged-
Kostka witness. Cap41,two-second scans,45-second batches/60-second guards;
five-second retry bounds,no engine errors. Codex owns handoff continuation,
source unchanged,43-test baseline,reports ignored,no live scan,goal active.
Original-restart scope,old branches paused,no push/publication.


## Explicit lower-flag degree37 checkpoint — 2026-09-09

New107400:degree37,linear `-108994039977551/534293145706320`,source `14 x 15`,
shape `(16,1^14)/(1)`,weight `(1^29)`. Strongest lower flags row14>=14,
row15>=17;union of ALL ancestor forcing masks contains4 of14 raw opposite
equations,leaving10 additional constraints and10 internal bans. Fresh
depth-layout retry succeeds in1.402080s. Next degree36 lower cut is exactly
nonnegative (3.687296s). No flagged-Kostka witness/minimal-distance claim.
Six bounded scans and separate retries are in mutation handoff/MariaDB;
shrinking next97 of206,with39 unresolved shrinking cases in visited slices
plus two internal and one weight case. Original failed attempts retained.
Cap41,two-second cases,45-second batches/60-second guards;reports ignored,
source unchanged,43-test baseline,no live scan. Codex owns continuation,
goal active;next target degree24 unknowns or remaining disjoint tail.
Original-restart scope,old branches paused,no push/publication.


## Explicit lower flags lower ten-extra-opposite branch to38 — 2026-09-09

87727 ->106637 ->107370 (exact retry107371) reaches degree38,source `14 x 15`,
linear `-307759680405013/381637961218800`,shape `(16,1^14)/(1)`,weight `(1^29)`.
Flags augmented row14>=14,row15>=16 force3 of13 raw opposite equations:
10 additional opposite constraints and10 internal bans remain. Union of both
stored forcing masks checked;scanner's per-current-cut field alone does not
certify accumulated flags. No minimal-distance or flagged-Kostka witness claim.
Depth-layout exact recount succeeds in1.081490s;degree39 parent separately
verified. Six bounded neighborhoods,all outcomes/empty/prunes/timing and six
new unresolved shrinking cases are in mutation handoff/MariaDB. Four prior
cases remain unknown after distinct five-second retries. Cap45/41,two-second
cases,45-second batches/60-second guards;no engine errors. Source unchanged,
43-test baseline,reports ignored,no live scan. Codex owns handoff continuation;
goal active,original-restart scope,older branches paused,no push/publication.


## Combined queue coverage checkpoint — 2026-09-09

No new negative; ten/ten87727 and other saved tradeoffs unchanged. Eight
bounded batches add329 fresh exact nonnegative results and four new unknowns
(104672:36,104724:37,105522:37,105828:37). All871 combined-label slots for
101281 and all1300 joint slots for87727 now visited in disjoint slices.
The latter has1291 prunes and9 earlier limits,NO exact signs; completed
proposal coverage must not be read as a nonnegativity certificate.
Mutation handoff/MariaDB record all bounds,skips,timing and resume evidence.
Maintained Ehrcalc,greedy layouts,cap37/45,two-second cases,45-second batches,
60-second guards;no engine errors. All43 tests pass,source unchanged,reports
ignored,no live scan. Codex owns handoff continuation,goal active,no global
blocker or flagged-Kostka witness. Original-restart scope,older branches
paused,no push/publication.


## Distinct thirteen/thirteen restart lineage — 2026-09-09

Flag additions lower100861 from degree36 to33;shrinking yields101281:
`16 x 18`,degree33,13 internal/13 opposite,linear
`-2092314867403/1604483921040`,shape `(19,1^16)/(1)`,weight `(1^34)`.
Fresh depth-layout count agrees in0.130059s;redundancy pruning accepts no
deletion. Full polynomial equals earlier82998,but coordinate-identified
faces differ. Preserve this distinct lineage without claiming a new global
best or flagged-Kostka witness. Mutation handoff/DB record nine bounded
neighborhoods,all skips/timing/lineages and certificate checks. Cap37,
two-second cases,45-second batches/60-second guards,no new limits/errors.
Codex owns handoff continuation,source unchanged,43-test baseline,reports
ignored,no live scan. Goal active,original-restart scope,old branches paused.
No push/publication.


## Low-degree coverage and new alternative — 2026-09-09

Seven bounded neighborhoods of95922/95858 completed without new negative;
all1029 combined-content slots visited in disjoint slices. Pruned/duplicate
signs unclaimed. Targeted retry of95922's smallest opposite deletion yields
degree37 fourteen/fourteen100830;flag child100861 lowers degree to36 with
negative cubic/quartic coefficients. Source `17 x 19`,shape `(20,1^17)/(1)`,
weight `(1^36)`. Separate depth-layout count agrees in0.175774s. Retain
ten/ten and cheaper low-degree alternatives;no global frontier improvement
or flagged-Kostka witness claimed. Mutation handoff/MariaDB record all
bounds,skips,timing,lineages and exact coefficients. Cap28/37 as documented,
two-second cases,45-second batches/60-second guards;no new limits/errors.
Codex owns handoff continuation,source unchanged,43-test baseline,reports
ignored,no live scan,goal active. Old branches paused,no push/publication.


## Smaller degree22 alternative — 2026-09-09

New original-restart descendant95922: `17 x 19`,degree22,14 internal/15
opposite,linear `-14376701/2238390`,shape `(20,1^17)/(1)`,weight `(1^36)`.
Lineage94094 ->94787 (shrink) ->95922 (flag). Separate depth-layout exact
recount agrees in0.027914s. Weight-compressed alternative95858 retains
degree22,15 internal/16 opposite; full coordinate-map/pruning endpoint
checks agree. Preserve all prior low-degree/ten-ten tradeoffs; no global
metric improvement or flagged-Kostka witness claimed. Mutation handoff/DB
record10 finite neighborhoods,all skips/timing/certificates. Cap28,two-second
cases,45-second batches/60-second guards; no new limits/errors. Source
unchanged,43-test baseline,reports ignored,no live scan. Codex owns handoff
continuation; goal active,old branches paused,no push/publication.


## Low-degree diversification descendants — 2026-09-09

Original-restart lineage91866 ->92144 ->92636 ->94094 gives `18 x 20`,
degree22,16 internal/16 opposite,linear `-40022579/2984520`. Joint sibling
94348 has degree23,15 internal/16 opposite,negative linear and quadratic.
Shape `(21,1^18)/(1)`,weight `(1^38)`. Separate depth-layout counts confirm
both (0.074960/0.062247s). Retain all previous lower-degree/ten-ten tradeoffs;
no claim of global metric improvement or flagged-Kostka witness.
Mutation handoff/MariaDB record11 bounded neighborhoods,disjoint shrinking
completion,all exact lineages/skips/timing. Joint92144 next361 of4896; original
combined next420. Cap26,two-second cases,45-second batches/60-second guards;
no new limits/errors. Source unchanged,43-test baseline,reports ignored.
Codex owns handoff continuation,no live scan,goal active,old branches paused.
No push/publication.


## Low-degree diversification checkpoint — 2026-09-09

Ten/ten87727 frontier unchanged. Sibling weight/slide and joint-prefix scans
add14 unresolved two-second cases; three old lower-degree cases remain unknown
after distinct ten-second retries. All bounds/timing/IDs in mutation handoff/DB.
Direct original213 combined-content offsets120--419 give198 fresh exact
results (23 negative),102 prunes,no errors/limits. Next420 of8815. New
degree21 diversification seed91866 comes from90936 ->91349 certified
compulsory-label compression ->redundancy pruning(476/512 equations):
source `19 x 21`,18 internal/18 opposite,linear `-161741/660`,shape
`(22,1^19)/(1)`,weight `(1^40)`. Fresh maintained-engine endpoints agree.
Not a metric-frontier improvement or flagged-Kostka witness; retain all
lower-degree/ten-ten alternatives. Codex owns handoff continuation; source
unchanged,43-test baseline,reports ignored,no live scan,goal active.
Pre-restart branches paused,no push/publication.


## Ten/ten bounded neighborhoods completed — 2026-09-09

No new negative this checkpoint; retain87727 and lower-degree alternatives.
Nine DB-backed batches visit974 proposals:253 fresh exact nonnegative,
668 dimension prunes,11 duplicates not revalidated,2 empty,40 time limits.
All206 shrinking and570 combined-content slots completed in disjoint slices;
skipped signs unclaimed. Three separate alternate-layout retries remain
time-limited. Mutation handoff records exact IDs, bounds, timing and next
options. Maintained Ehrcalc, verified greedy layouts, cap45, two-second cases,
45-second batches/60-second outer guards. All43 tests pass, source unchanged,
reports ignored, no live scan. Codex owns handoff continuation; goal active,
not globally blocked. No flagged-Kostka witness; original-restart scope and
paused older branches unchanged. No push/publication.


## Smaller ten/ten restarted candidate — 2026-09-09

New87727: source `14 x 15`, degree41,10 internal bans/10 opposite equations,
linear `-454598497536619/128105374116720`, shape `(16,1^14)/(1)`, weight `(1^29)`.
Original-restart lineage86601 ->87268 (zero weight) ->87283 (certified
alphabet compression) ->87586 (226 redundant equations removed) ->87727
(shrink). Fresh depth-layout recount agrees in2.161713s. Keep degree32
twelve/twelve and degree21/22 alternatives. Still mixed faces, not flagged-
Kostka witnesses or minimal-distance certificates. Mutation handoff/MariaDB
record10 bounded neighborhoods, disjoint shrink completion, all timing/skips,
13 new unresolved two-second cases and exact lineages. Cap45, two-second
cases,45-second batches/60-second outer guards; no engine errors. Reports
ignored, source unchanged,43-test baseline. Codex owns handoff continuation;
no live scan, goal active, old branches paused. Next mutate87727.
No push/publication.


## Eleven-opposite higher-degree restart alternative — 2026-09-09

Targeted exact retry of84358's smallest opposite deletion yields86601:
source `15 x 17`, degree45,12 internal/11 opposite equations, shape
`(18,1^15)/(1)`, weight `(1^32)`. Negative linear coefficient
`-102494679063787327/1682069314071861`, also negative quadratic. Fresh greedy
count (0.509278s) and depth-layout repeat (1.316601s) agree. Retain degree32
twelve/twelve84358 and lower-degree alternatives. Still mixed-face data,
not a flagged-Kostka witness or certified minimal nonflag distance.
Mutation handoff/MariaDB record nine bounded neighborhoods, completed disjoint
combined-content queue, all skips/timing and eight unresolved cases after
separate two-second depth retries. Cap38, two-second cases,45-second batches,
60-second outer guards; no engine errors. Reports ignored; source unchanged,
43-test baseline. Codex owns handoff continuation, no live scan, goal active.
Next try degree-lowering flags on86601; old pre-restart branches paused.
No push/publication.


## Smaller twelve/twelve restarted negative — 2026-09-09

Original-restart lineage81911 ->82317 (internal slide) ->82998 (shrink)
->84358 (shrink) reaches `15 x 17`, degree32, 12 internal bans /12 opposite
equations, linear `-281272388967/534827973680`. Shape `(18,1^15)/(1)`,
weight `(1^32)`. Maintained-engine greedy result repeated with verified
depth layout in0.125882s. Keep lower-degree21/22 alternatives. Still mixed
faces, not a flagged-Kostka counterexample or certified minimal distance.
Mutation handoff/MariaDB record12 bounded neighborhoods, disjoint shrinking
resume, all prunes/duplicates/empty cases/timing and exact lineages. One new
dimension38 case82400 remains unknown after separate two-/five-second attempts.
Cap38, two-second cases,45-second batches/60-second outer guards; reports
ignored. Codex owns handoff continuation; source unchanged,43-test baseline.
No live scan; next mutate84358. Goal active; pre-restart branches paused,
no push/publication.


## Restarted thirteen-opposite alternative — 2026-09-09

Targeted exact retry of the smallest pruned opposite deletion from76956
finds a degree38 negative with13 opposite/15 internal. Five successive flag
additions lower degree to33: selected81911, source `17 x 19`, linear
`-3477883001903/534827973680`, shape `(20,1^17)/(1)`, weight `(1^36)`.
Verified depth-layout recount agrees (0.432690s). Its next single pass finds
no degree32 negative. Keep lower-degree76956/74571 alternatives; still mixed
faces, not flagged-Kostka witnesses or minimal-distance certificates.
Mutation handoff/MariaDB record all1030 combined-label slots from76956
(107 exact nonnegative,923 prunes), joint prefix351/3570 entirely pruned,
six flag neighborhoods, exact lineages/bounds/timing and separate retry IDs.
Cap26/38 as documented, two-second cases,45-second batches,60-second outer
guards; no new limits/errors, reports ignored. Codex owns handoff continuation;
source unchanged,43-test baseline, no live scan. Goal active, old branches
paused, no push/publication.


## Smaller restarted mixed face: degree22 — 2026-09-09

Codex owns handoff continuation. Original213 restart now yields76956 through
shrinking74503: `17 x 19`, degree22, 15 internal bans / 14 opposite equations,
linear `-14228681/746130`, shape `(20,1^17)/(1)`, weight `(1^36)`.
Maintained-engine greedy result repeated with verified depth layout (0.027199s).
Retain degree21 candidate74571. Still mixed-face data, not a flagged-Kostka
counterexample or minimal-distance certificate. A separate zero-weight lineage
75499 ->76214 ->77431 has certified alphabet compression and redundant-edge
pruning, but is larger. Mutation handoff/MariaDB record15 bounded neighborhoods,
all prunes/duplicates/timing and lineage. Cap26, two-second cases, 45-second
batches, 60-second outer guard; no new timeouts/errors. All43 tests pass;
source unchanged, reports ignored, no live scan. Next consider combined-content
or joint moves on76956; all earlier pre-restart branches remain paused.
Goal active; no push/publication.


## Original-restart descendants: fifteen/fourteen opposite equations — 2026-09-09

Codex owns KTT handoff continuation. Original213 lineage now reaches74571:
source `18 x 20`, degree21, 18 internal bans / 15 opposite equations, linear
`-1394849/248710`. Sibling74503 has degree24, 18 internal / 14 opposite,
negative quadratic `-4127447/277200`. Shape `(21,1^18)/(1)`, weight `(1^38)`.
Both fresh greedy-layout results repeated exactly with verified depth layouts
(0.026845/0.037284 s). These are still mixed faces, not flagged-Kostka
witnesses or certified minimal nonflag distances. Later pre-restart branches
remain paused. Mutation handoff/MariaDB record14 bounded neighborhoods,
all prunes/duplicates/timing, saturation/pruning checks and exact lineages.
Cap24, two-second cases, 45-second batches, 60-second outer bounds; no new
limits/errors. Joint71487 next500 of5472; original combined next120 of8815.
Reports ignored; source unchanged, 43-test baseline. No live scan at checkpoint;
goal active, no push/publication. Next prioritize74571/74503 tradeoffs.


## User-directed original degree-21 restart — 2026-09-09

New direct-original lineage 213 -> 69153 -> 70540 -> 71249 gives a smaller
`19 x 21` mixed face, still degree 21, linear `-8737469/87780`, 21 internal
bans / 17 opposite equations. Shape `(22,1^19)/(1)`, weight `(1^40)`.
Compulsory-label compression checks the full coordinate order and fresh
maintained-engine endpoint polynomials. Pruning removes 472 redundant
equations in 510 structural comparisons (11.014399 seconds). Still NOT a
flagged-Kostka witness or minimal-distance certificate. The mutation handoff
records 120 combined-content proposals (13 negative), all 700 paired slots,
all prunes/duplicates/timing, four new unknowns and one unresolved shrinking
retry. Bounds: cap24, two-second cases, 45-second batches, 60-second outer
guard. Next original combined offset120 of8815. All 43 tests pass, reports
ignored, no live scan. Later branches remain paused; search goal active.

Codex owns KTT handoff continuation. Latest user steering pauses later mutation
branches and selects original MariaDB observation 213 again. Fresh labeled
audit verifies degree 21, linear `-168011/330`, all coefficients/h* and
independent counts at dilations 0--25; maintained Rust took 0.234277 seconds.
Mutation work remains in the authorized mutation-search repository, using
the existing protected DB adapter (no MariaDB MCP registered). No live scan
at adoption; worktrees clean. Earlier negatives remain saved but are not
active parents. No flagged-Kostka witness is claimed.


## Certified height compression; eight opposite equations — 2026-09-09

Combined compulsory-label/equation-deletion mutations yield negative faces.
New exact height-compression map preserves all free coordinates and the full
quotient order; fresh maintained-engine endpoint polynomials agree. After
redundancy pruning, 67453 has source `14 x 14`, degree 37, 9 internal bans /
8 opposite equations, linear `-27713322577159507/2671465728531600`, shape
`(15,1^14)/(1)`, weight `(1^28)`. Still mixed-face data, not a flagged-Kostka
witness. All 513 combined proposals, 25 limits, 188 structural comparisons,
exact bounds/timing/lineage and the no-improvement next single pass are in
the mutation handoff/MariaDB. Scanner/compression/pruning now share verified
optional greedy counting layouts. 43 tests pass; no live scan, reports
ignored. Codex owns private adapter/docs continuation; goal active.

## Compulsory-label mutation adapter — 2026-09-09

Added a private frontier mode requiring one label in every original tableau
column. Complement/transposition maps this to exact zero-content equalities;
direct mixed-tableau/GT checks pass, 41 tests total. This is an original-
rectangle content condition, not a claimed change of augmented Kostka weight.
Initial 70 proposals from original seed 213 and current 64864 give 66 exact
nonnegative results, three empty faces and one duplicate. No negative
frontier change. Additional zero/shape scans and their disjoint tail are
recorded in the mutation handoff/MariaDB; eleven new cases remain time-limited.
All skips/timing are explicit, reports ignored, no live scan. Codex owns
adapter/tests/docs continuation; goal active, no flagged-Kostka witness.

## Degree-37 nine/nine negative — 2026-09-09

Pure internal-ban deletion yields 64864 (`15 x 14`, degree 37), with
9 internal bans / 9 opposite equations and linear
`-27713322577159507/2671465728531600`. Shape `(15,1^15)/(1)`, weight
`(1^29)`; retain the degree-36 and lower-degree alternatives. Still mixed
faces, not a certified flagged-Kostka witness. All 1,350 combined proposals
were visited in disjoint batches; 1,345 are dimension-pruned, not certified
nonnegative. Ten of twelve alternate-layout retries resolve nonnegative;
four new scan cases remain unknown. Precise bounds, timing, skipped cases
and lineages are in the mutation handoff/MariaDB. Reports ignored; no source
change or live scan, baseline 39 tests. Codex owns handoff; goal active.

## Greedy-layout retries and completed bounded tails — 2026-09-09

Six fresh scan passes on 61480/61478 gave 228 exact nonnegative results;
prunes, duplicates and timeouts are separately recorded in the mutation
handoff/MariaDB. Disjoint tails complete both shape queues. No new negative;
retain the degree-36 nine-opposite candidate and lower-degree alternatives.
Retry-only `--relabel-greedy` adds another fully verified natural coordinate
permutation (minimum next live frontier, deterministic ties). It resolves
six timeouts, including old 60604/60692, as nonnegative under two seconds.
Independent ideal DP confirms all six polynomials and h* vectors; all prior
attempts are preserved. Nineteen new scan timeouts remain explicitly unknown.
39 tests pass; source/README/handoff changes owned by Codex, reports ignored,
no live scan. No flagged-Kostka witness or global blocker; goal active.

## Depth-layout scanner and nine-opposite descendant — 2026-09-09

The mutation scanner now accepts the same verified `--relabel-depth` option
as retries, sharing one coordinate-signature/layout module. It counts in the
new natural order but preserves original-order DB identities and explicitly
records the permutation. All 38 tests pass, including counting/persistence
wiring. Maintained Ehrcalc remains the exact computation engine.
Shrinking 60789 gives `15 x 14` negatives; flag addition yields 61480:
degree 36, 10 internal bans / 9 opposite equations, linear
`-203940654205061/72201776446800`, shape `(15,1^15)/(1)`, weight `(1^29)`.
Still mixed-face data, not a flagged-Kostka witness. Seven finite scan passes,
disjoint shrinking resume, all skips/timing and nine unresolved two-second
cases are in the mutation handoff/MariaDB. Reports ignored; no live scan.
Codex owns scanner/layout/docs continuation; goal active, no publication.

## Verified layout retries and ten opposite equations — 2026-09-09

New seed 60789: `16 x 15`, degree 38, 11 internal bans / 10 opposite
equations, linear `-16214767175092789/2671465728531600`. Retain degree-29
11/11 and lower-degree alternatives; no flagged-Kostka witness yet.
The mutation retry adapter now offers a fully verified depth-based coordinate
relabeling before calling the SAME maintained Ehrcalc order engine. This
resolves a degree-25 case in 0.048 s after its original ordering timed out
at 15 s; full coordinate/order signatures agree. Seventeen of 19 two-second
relabeled retries resolve (15 negative, two nonnegative); degree-36 inputs
60604/60692 remain unknown. Independent ideal DP confirms every resolved
polynomial and h* in original labeling; all attempts/checks are in MariaDB.
Bounds, skips, timing, 37 passing tests and exact lineage are in the mutation
handoff. Reports ignored; no live scan. Codex owns adapter/docs continuation;
goal active, no engine implementation replaced or publication authorized.

## Degree-29 eleven/eleven checkpoint — 2026-09-09

Fresh original-fan descendant 59240 is `16 x 15`, degree 29, with 11
internal bans / 11 opposite equations and linear `-152045488051/155272637520`.
Shape `(16,1^16)/(1)`, weight `(1^31)`. Weight mutation, certified unused-label
compression, redundant-equation pruning and a freshly computed shape cut
produce this smaller/fewer-restriction seed. Retain degree-20/21 alternatives.
Ten bounded cap-34 scans, exact lineages, all skips/timing and 309 structural
checks are recorded in the mutation handoff/MariaDB; no new time limits.
Immediate singles from 59240 find no new negative. Still a mixed face, not
a flagged-Kostka witness. Reports ignored; no source change or live scan.
Codex owns handoff continuation; goal remains active.

## Thirteen-opposite alternative — 2026-09-09

Opening the smallest dimension-pruned opposite deletion from 55806 gives
57430: degree 34, 14 internal bans / 13 opposite equations, `18 x 18`.
Exact flag additions lower degree to 33; selected child 57687 has linear
`-81860962697061/382019981200`, shape `(19,1^18)/(1)`, weight `(1^36)`.
Its immediate single-edge pass finds no new negative. Retain the degree-21
14/14 and degree-20 alternatives; these are still mixed faces, not a
flagged-Kostka witness. Nine bounded passes, all prunes/duplicates, precise
coverage and timing are recorded in the mutation handoff/MariaDB. No new
time limits; 35 tests pass. Reports ignored, no live scan, goal active.

## Fourteen/fourteen original-seed descendant — 2026-09-09

Fresh zero-content compression and redundant-equation pruning give 55806:
source `18 x 18`, degree 21, 14 internal bans / 14 opposite equations,
linear `-2716195/596904`, shape `(19,1^18)/(1)`, weight `(1^36)`.
Full coordinate-order equivalence and fresh maintained-engine endpoint
computations agree. Retain degree-20 54009 as a lower-degree tradeoff.
Ten finite cap-24 mutation neighborhoods, all outcomes, 400 structural
comparisons and timing are in the mutation handoff/MariaDB; no new limits.
55806's immediate singles/slides found no fresh negative. Counts remain
presentation metrics, not a flagged-Kostka witness. Reports ignored, no
source changes, no live scan; Codex owns handoff continuation. Goal active.

## Renewed degree-21 seed branch — 2026-09-09

Current freshly restarted low-degree selection: 54009, source `18 x 19`,
degree 20, 18 internal bans / 16 opposite equations, linear `-457039/19380`.
Shape `(20,1^18)/(1)`, weight `(1^37)`. Saved degree-21 siblings 53975/53977
trade one more internal ban for one fewer opposite equation. Explicit lower
flag/deletion mutations also preserve negative degree-20 quadratic terms
(53937/53939). None is a flagged-Kostka counterexample. All new lineages,
eight further bounded neighborhoods, structural flag/pruning certificates
and skipped cases are recorded in the mutation handoff and MariaDB.
The three original-shrink time limits remain unknown after separately
persisted five-second retries. No scan is live; the search goal remains active.

Latest user direction pauses later branches and restarts from MariaDB 213.
Codex owns KTT handoff updates; mutation source/DB work stays in the existing
authorized mutation-search repository. Fresh labeled audit verifies degree 21,
linear `-168011/330`, all coefficients/h*, and independent counts at 0--25.
Direct shrinking finds degree-20 child 50464 (`19 x 20`), linear
`-192203/1596`. Direct zero-weight child 50420 compresses to `20 x 20`
without changing the original polynomial; pruning gives 51775, 18 internal
bans / 18 opposite equations, shape `(21,1^20)/(1)`, weight `(1^40)`.
These are mixed-face negatives, not flagged-Kostka counterexamples.
The mutation handoff records 20 lower-flag, 41 weight and 416 shrinking
proposals, all prunes/duplicates, three unresolved two-second cases and
exact lineages. Thirty-five tests pass; labeled audits now preserve prior
evidence and skip same-label terminal attempts. No DB MCP is registered;
the existing local MariaDB adapter remains available. No push/publication.

## KTT mutation checkpoint — 2026-09-09

New children 49687/49689 retain negativity at degree 31 with 9 internal bans
and 9 opposite equations in `15 x 14`; 49689 has linear
`-1396680158221/2674139868400`. Two timed slide cases (49747/49749) resolved
exact positive under distinct 15-second bounds, taking 4.97/8.23 s.
Other limited cases remain explicitly unresolved. Retry bounds are now
configurable (1--30 seconds), and frontier deduplication recognizes exact
retry/compression/presentation records without claiming fresh sign validation.
Thirty-four tests pass; all results are in MariaDB, reports ignored, no live
worker. These remain mixed faces rather than flagged-Kostka witnesses.

## Original fan restart — 2026-09-08

General single-zero alphabet compression is now proved by a full coordinate
order check and tested on small mixed faces. Interior-zero child 49152 gives
an equivalent `15 x 14` source; opposite-first redundancy pruning produces
49539, degree 31, 10 internal bans, 9 opposite equations, linear
`-87887067115/16889304432`, shape `(15,1^15)/(1)`, weight `(1^29)`.
A local shift gives 49605: degree 32, 9 internal bans, 9 opposite equations,
linear `-765183135727/382019981200`. Eight-opposite degree-32 siblings remain
saved. Several new two-second limits are explicitly unresolved in the mutation
handoff; no scan is live. Thirty-three tests pass. No flagged-Kostka witness.

New fewer-opposite seed 49178: `15 x 15`, degree 33, 12 internal bans,
8 opposite equations, negative linear and quadratic coefficients. Keep the
degree-31/9-opposite siblings 49095/49097/49099. Zero-content child 49152
also has degree 31 but its missing label is interior, so the existing
last-label compression is not applicable without a new coordinate proof.
Two dimension-36 slide cases (49045/49047) remain unresolved after separate
five-second retries; all statuses are in MariaDB and no scan is live.

New `15 x 15` tradeoff seeds: 48420 has degree 28, 14 internal bans,
10 opposite equations, linear `-390146849/104984880`; 48482 has degree 33,
12 internal bans, 9 opposite equations, negative linear and quadratic terms.
Their lifted shape is `(16,1^15)/(1)`, weight `(1^30)`. Both were freshly
computed with maintained Ehrcalc; 48482 also passed saturation revalidation.
The new dimension-36 timeout 48556 resolved exact positive in a five-second
retry (2.15 s); older 47109 remains unresolved. All ranges/outcomes are in
the mutation handoff and MariaDB. No live scan or flagged-Kostka witness.

The eleven-opposite branch now reaches degree 31: observation 46739,
`17 x 17`, 17 internal bans, 11 opposite equations, negative linear
`-4626783419267/534827973680`. Retain it alongside smaller source 46257
(`17 x 16`, degree 31, 15 internal bans, 12 opposite equations).
One new shrinking case, 47109 at dimension 35, remains unresolved after
two- and five-second limits. An accidental duplicate five-second retry is
documented; the new retry helper now skips all same-bound terminal outcomes,
and its regression passes. Thirty-two tests pass; no worker remains live.

Local internal-ban shifts produce a smaller selected branch: two shifts reduce
internal bans 17 -> 16 -> 15, then a zero-content mutation lowers degree
32 -> 31. Certified deletion of the unused last symbol gives observation
46257: `17 x 16`, degree 31, 15 internal bans, 12 opposite equations,
linear `-12924577634203/4813451763120`, shape `(17,1^17)/(1)`, weight `(1^33)`.
Keep alternative 45668 with 11 opposite equations at degree 35 as well.
All outcomes/lineages are in MariaDB; 31 tests pass and no live scan remains.
These are still mixed faces, not unconstrained flagged-Kostka witnesses.

New smaller mixed-face seed 44009: `17 x 17`, degree 32, 17 internal bans,
12 opposite equations, negative linear `-311101414951/123421840080`.
It improves size, degree and both presentation counts over the selected
thirteen-opposite parent. Saturation/pruning accepts no further change.
Its next shrinking neighborhood found no fresh negative; the sole two-second
timeout was resolved positive in a separately recorded five-second retry
(actual 2.36 s). Dimension-pruned/duplicate cases remain unclaimed. Exact
lineages, bounds, retry identity and sibling negatives are in the mutation
handoff. No flagged-Kostka counterexample or live scan at this checkpoint.

Cap-36 reopening found a new thirteen-opposite negative (43014, degree 36).
Successive exact flag additions reduce its degree 36 -> 35 -> 34 -> 33,
while retaining 20 internal bans and 13 opposite equations in `18 x 18`.
Selected degree-33 seed 43672 and sibling 43730 remain in MariaDB; the latter
has negative linear coefficient `-225869578496153/1604483921040`.
No degree-32 negative appeared in that seed's immediate single-edge pass.
All runs kept two-second exact-count limits, and all prunes/duplicates/empty
cases are separately recorded in the mutation handoff. No live scan remains.

Reopened selected dimension-pruned neighborhoods at cap 28 with the same
two-second per-candidate bound. Direct boundary additions yield four new
degree-22 negative parents (41461/41465/41467/41469), still with 20 internal
bans and 14 opposite equations. Other newly exact reopened cases were positive;
pruned/duplicate signs remain unclaimed. Exact ranges/timing are in the mutation
handoff. Corrected a reporting distinction: zero-weight 18274 had one empty
face, not a dimension prune; the DB row was correct, and summaries now count
empty faces separately. No frontier-size or constraint-count improvement yet.

Explicit lower-flag cuts are now encoded and tested independently against
small tableau enumeration. Their imposed horizontal equations are separated
from additional opposite-slant constraints. Five bounded neighborhoods from
33306/21718 cover 1,713 proposal outcomes: 109 fresh exact positives, 1,593
dimension prunes, 11 duplicates not revalidated, no timeouts. No improvement
to the negative frontier; signs of pruned cases remain unknown. Exact ranges,
timing and DB-backed resume are in the mutation handoff. Thirty tests pass.

Observation 33306 is an exactly equivalent `18 x 18` representation of the
last-zero-weight negative: delete its fixed first GT column and relabel
coordinates. Full coordinate/order signatures and maintained Ehrcalc outputs
agree. It has degree 22, linear `-6330697/298452`, lifted shape
`(19,1^18)/(1)` and all-one weight of length 36. Follow-up single-zero content,
paired and interior-shrinking passes found no new negative among their fresh
exact computations; skips and bounds are in the mutation handoff. General
rectangle support and coordinate-map regressions now pass 28 tests.

First non-neutral weight mutations now tested exactly while retaining mixed
equalities: zero one content entry and shorten the complement row by one.
From degree-23 parent 21718, observations 33301/33303/33305 retain negativity
at degree 22, with linear `-6330697/298452`. Their lifted shape is
`(19,1^18)/(1)` and weight has 37 entries, all one except label 35/36/37.
Source 33305 is the next promising smaller-weight lineage. The new adapter's
diagonal-zero condition is independently tested against hook-content counts.
The mutation handoff distinguishes weight-forced from additional equations;
these mixed faces are not yet unconstrained flagged-Kostka counterexamples.

Complete bounded joint neighborhood from 17018: 5,670 distinct genomes,
54 exact results (36 negative), 5,616 dimension prunes, no limits, 97.66 s
across four segments shorter than 45 s. Negative children have degree 23,
20 internal bans and 14 opposite equations. Observation 21718 has negative
linear and quadratic coefficients; retain it with the lower-degree parents.
No unconstrained flagged-Kostka witness is claimed. The mutation handoff
contains exact ranges, coefficients, alternate-parent results, and tests.

Further mutation yields observation 18274: `18 x 19`, degree 22, 21 internal
bans and 14 opposite equations, negative linear `-14812659/497420`. Keep it
alongside the degree-21/15-opposite parent 17018. Their interior row/column
shrinking passes found no negative among exactly computed cases, but many
dimension-pruned cases remain uncomputed. The mutation handoff records exact
proposal bounds and distinguishes duplicate masks from distinct examples.
Database-aware duplicate suppression and same-run resume now avoid repeating
exact counts without overwriting existing evidence; a 238-slot resume was
verified to do no new computations and preserve all original result records.

Implied-flag saturation plus exact redundancy pruning reduced the current
degree-21 presentation to 25 internal bans and 18 opposite equations.
Subsequent shape shrinking reached observation 17018: `GT(1^18,0^19)` mixed
face, degree 21, 21 internal bans, 15 opposite equations, and negative linear
coefficient `-4494071/87780`. This is a smaller source than the original
`20 x 21`; no flagged-Kostka counterexample is claimed. All computations use
maintained Ehrcalc; coordinate identity certifies the neutral presentation
changes. Exact bounds, skips, DB lineage, and pending weight adapter are in
the mutation-search handoff. No live scan remains at this checkpoint.

Presentation audit removes 131 redundant internal bans using label-independent
coordinate equality/order signatures; observation 14499 is the same degree-21
polytope, now presented with 50 internal bans and 33 opposite equations.
Fresh maintained Ehrcalc checks both endpoints. Its next paired neighborhood
has three negative degree-21 children with 50 internal bans and 32 opposite
equations. These are still mixed faces, not flagged-Kostka counterexamples.
The mutation handoff records all 214 structural comparisons, 714 subsequent
proposals, skips, timing, and the superseded raw-numbering signature audit.

Fresh continuation reaches MariaDB observation 13019: degree 21, negative
linear coefficient `-168011/330`, and 33 opposite-slant equations (down from
35). This is NOT 33 certified flagged-Kostka holes: 181 presentation-internal
vertical bans also remain. Separate degree-22 children reduce internal bans
to 163; internal-ban deletion also preserves degree 21 in 130 cases. All exact
results use the maintained Rust order engine. The mutation handoff records
finite ranges, dimension prunes, four timed-out cases, and negative lineages.
Four smaller corner transports yielded three exact positives and one dimension
prune. General mixed-slant weight mutations remain pending, not simulated by
the vertical-only Kostka adapter.

User paused the old mutation sequence and requested the original degree-21
example. Recovered MariaDB observation 213 in the mutation-search repository:
the fan quotient in `GT(1^20,0^21)`, with 35 opposite-slant equations.
Fresh maintained Ehrcalc computation and independent power-sum checks at
dilations 0--25 agree: degree 21, linear coefficient `-168011/330`.
The 35 opposite-slant equations are NOT a certified flagged-Kostka hole count.
First 55 unique single-edge proposals: five negative deletion children at
degrees 22--23 with 34 opposite equations; one negative boundary-addition
child at degree 21; 30 dimension prunes and 19 nonreduced prunes. All stored
in MariaDB. One proposal was recomputed during an explicitly recorded crash
recovery. Shape and weight mutations remain pending. Source, bounds, and
ownership are in the mutation-search handoff. The historical offset sequence
below remains paused and its coverage claims are invalidated as follows.

## Coverage correction — 2026-09-08

The historical hole-swap offset coverage claims below are superseded. An audit
of the 183 available `turn-holeswap-20261003-offset*.jsonl` reports found 1,830
rows but only 12 distinct masks. The generator sorted each growing capped prefix
before applying its offset, so successive slices were not disjoint. Moreover,
the script loaded historical Ehrhart cache rows without independent validation;
its `exact` count did not distinguish fresh computations from cache hits.
These batches do not establish the claimed scanned ranges. No new mathematical
frontier result follows from them. Codex owns correction of the search script
and fresh bounded verification in the reduced-kogan-mutation-search repository.

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
- Triple-swap two-hole continuation (proposal offsets 590–599, target holes 2,
  max move distance 10, exact DP-state cap 2,000,000, 8-second candidate cap)
  completed with 10/10 exact rows and zero negative polynomials. The batch did
  reach smaller positive dimensions (minimum 73), but produced no certified
  negative descendant and did not change the degree-84, two-hole frontier.
- Triple-swap continuation at proposal offsets 600–609 completed with 10/10
  exact rows, no limited rows, and zero negative polynomials. MariaDB persistence
  was enabled; the certified frontier remains degree 84 with two nonflag holes.
- Triple-swap continuation at offsets 610–619 likewise completed 10/10 exact,
  with no limited rows and no negative polynomial; MariaDB persistence remains
  enabled and the degree-84 two-hole frontier is unchanged.
- Triple-swap continuation at offsets 620–629 completed 10/10 exact rows,
  with no limited rows and no negative polynomial. The persisted frontier is
  unchanged at degree 84 with two nonflag holes.
- Triple-swap continuation at offsets 630–639 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; the degree-84 two-hole
  frontier remains unchanged.
- Triple-swap continuation at offsets 640–649 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 650–659 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 660–669 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 670–679 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 680–689 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 690–699 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 700–709 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 710–719 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 720–729 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 730–739 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- A bounded four-swap branch from the current two-hole seed was attempted
  (target 2, 10-proposal cap). It generated zero proposals because the seed
  has only three selected pairs; no computation was skipped silently and no
  frontier claim changed.
- Shape/size genetic mutation (population 16, two generations, elite 6,
  side range 2–12, exact DP cap 2,000,000, 8-second candidate cap) completed
  under the 60-second guard. It independently rediscovered the canonical
  negative `GT(1^9,0^10)` certificate (degree 84, two nonflag holes) and did
  not produce a smaller or lower-hole negative.
- A second fresh-seed shape/weight genetic batch (population 16, two
  generations, size mutation rate 0.45, side range 2–12) completed under the
  bounded guard and again returned the canonical degree-84, two-hole negative;
  no smaller instance or fewer bad edges appeared.
- A widened fresh-seed genetic batch (population 24, two generations, elite 8,
  candidate cap 6 seconds) completed within the guard. All reported generation
  bests were the canonical degree-84, two-hole negative; no improvement was
  found.
- A bounded reservoir mutation probe (32 candidates, exact limit 16, 8-second
  timeout, deduplicated samples) scanned no nonempty reservoir rows, so it
  yielded no exact evaluations or negatives; this is recorded as a zero-input
  diagnostic rather than evidence against the frontier.
- A small-size-biased shape/weight genetic batch (population 20, two
  generations, side range 1–10, size mutation rate 0.55) completed under the
  guard. Every generation best remained the hook-shaped degree-84 negative
  with two nonflag holes; no smaller negative was found.
- A further fresh-seed small-size genetic batch (population 20, two
  generations, side range 1–11, mutation rate 0.60, max dimension 90) again
  returned degree 84 with two nonflag holes in every generation best; no
  smaller negative appeared.
- Triple-swap continuation at offsets 740–749 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; the degree-84 two-hole
  frontier remains unchanged.
- Triple-swap continuation at offsets 750–759 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 760–769 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 770–779 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 780–789 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 790–799 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 800–809 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 810–819 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 820–829 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 830–839 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 840–849 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 850–859 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 860–869 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 870–879 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 880–889 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 890–899 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 900–909 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 910–919 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 920–929 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 930–939 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 940–949 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 950–959 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 960–969 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 970–979 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 980–989 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 990–999 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1000–1009 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1010–1019 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1020–1029 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1030–1039 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1040–1049 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1050–1059 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1060–1069 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1070–1079 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1080–1089 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1090–1099 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1100–1109 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1110–1119 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1120–1129 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1130–1139 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1140–1149 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1150–1159 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1160–1169 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1170–1179 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1180–1189 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1190–1199 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1200–1209 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1210–1219 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1220–1229 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1230–1239 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1240–1249 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1250–1259 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1260–1269 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1270–1279 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1280–1289 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1290–1299 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1300–1309 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1310–1319 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1320–1329 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1330–1339 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1340–1349 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- A dimension-capped genetic run (`max-dimension=83`, population 20, two
  generations, size mutation rate 0.65) found a certified negative at degree
  83, but with three nonflag holes. Shape is still the hook
  `[11,1,1,1,1,1,1,1,1,1]`; this is a smaller negative lineage, not a
  two-hole improvement, and is persisted in MariaDB.
- A stricter subdegree run (`max-dimension=82`, population 20, two
  generations) found a certified negative at degree 78 with seven nonflag
  holes. Other generation bests included degree 82 negatives with four holes;
  this expands the smaller-instance lineage but does not beat the two-hole
  frontier.
- A further cap at `max-dimension=77` found a certified negative at degree 73
  with four nonflag holes (`a=b=9`, outer hook `[10,1^9]`, mask
  `0x1208000000100000100`). This is the current smallest certified negative,
  but it has more holes than the degree-84 frontier.
- A larger population search capped at dimension 73 (population 24, two
  generations) found no negative in its final best; its best reported positive
  candidate had dimension 72 and five nonflag holes. Thus the degree-73,
  four-hole negative remains the smallest certified negative currently.
- A larger population-32 search capped at dimension 73 produced no negative;
  generation bests were positive degree-73 candidates with 3–4 holes. The
  existing degree-73/four-hole negative remains the certified subdegree best.
- A fresh population-32 search capped at dimension 77 produced no negative;
  its generation bests were positive degree-76 candidates with one nonflag
  equality. This is useful positive near-frontier data but does not replace the
  degree-73/four-hole negative.
- An intermediate cap at dimension 75 (population 24, two generations) found a
  new certified negative at degree 72 with four nonflag holes, mask
  `0x200000002010000021000`, and negatives in coefficients 1 and 2. This is
  the current smallest certified negative lineage; hole count is unchanged at
  four.
- A cap at dimension 71 produced no negative, but did find positive degree-67
  candidates with only two nonflag holes. This establishes that low dimension
  and low hole count coexist on the positive side, while the smallest negative
  remains degree 72 with four holes.
- A cap at dimension 70 likewise produced no negative; its best positive was
  degree 68 with a single nonflag equality. This sharpens the positive
  low-hole boundary but leaves degree 72/four-hole as the smallest negative.
- A cap at dimension 69 likewise produced no negative; its best positive was
  degree 69 with two nonflag holes. The smallest certified negative remains
  degree 72 with four holes.
- A larger population-32 search capped at dimension 72 produced no negative;
  its best reported candidate was positive degree 68 with one nonflag hole.
- A low-size-mutation degree-72-capped batch (population 24, mutation rate
  0.30) also produced no negative; its best reported candidate was positive
  degree 70 with six nonflag holes.
- A high-size-mutation degree-72-capped batch (population 24, mutation rate
  0.85) produced no negative; its best reported candidate was positive degree
  70 with two nonflag holes. No change to the negative frontier.
- An independent degree-72-capped batch (population 16, side range 1–10,
  mutation rate 0.50) produced no negative; its best positive was degree 70
  with two nonflag holes. The certified negative frontier remains degree 72
  with four holes.
- A fresh degree-72-capped batch (population 20, mutation rate 0.45) produced
  no negative; its best reported candidate was positive degree 72 with five
  nonflag holes. The certified negative frontier remains unchanged.
- A fresh degree-72-capped search (population 20, two generations, size
  mutation rate 0.75, seed 20260929) produced no negative. Its generation-0
  best was an unevaluated degree-74 candidate with three holes; the later
  exact best was positive degree 68 with one nonflag hole (`mask=0x200000000000080`,
  `a=8,b=9`, hook outer shape `[10,1^9]`). No change to the certified
  negative frontier (degree 72 with four holes); all rows were persisted in
  MariaDB and the report remains ignored/untracked.
- A three-generation degree-72-capped search produced no negative; its best
- A fresh high-size-mutation degree-72-capped batch (population 24, three
  generations, mutation rate 0.90, seed 20260930) produced no negative. It
  reached a certified positive degree-71 candidate with one nonflag equality
  (`a=8,b=9`, hook outer shape `[10,1^8]`, mask `0x800`). Every evaluated row
  was persisted in MariaDB; the degree-72/four-hole negative remains the
  smallest certified negative.
- A fresh degree-71-capped batch (population 32, three generations, mutation
  rate 0.80, seed 20260931) produced no negative. Its exact generation best
  was positive degree 71 with one nonflag equality (`a=8,b=9`, hook outer
  shape `[10,1^8]`, mask `0x1000000`); all evaluated rows were persisted.
  The certified negative frontier is unchanged at degree 72 with four holes.
- A diverse-seed degree-71-capped batch (population 24, three generations,
  mutation rate 0.70, seed 20261001, with three additional historical masks)
  produced no negative. It reached a positive degree-67 candidate with zero
  nonflag equalities (`a=7,b=10`, hook outer shape `[11,1^7]`, mask
  `0x100000000000000000`). All evaluated rows were persisted; no negative
  frontier change.
- A small-seed degree-70-capped batch (population 28, three generations,
  mutation rate 0.80, seed 20261002, including the degree-67 zero-hole seed)
  produced no negative. Its exact best was positive degree 67 with three
  nonflag equalities (`a=8,b=10`, hook outer shape `[11,1^8]`, mask
  `0x40000000000000200000010000001000`). All evaluated rows were persisted;
  the degree-72/four-hole negative remains the certified frontier.
- A degree-66-capped mixed-seed batch (population 32, three generations,
  mutation rate 0.85, seed 20261003) produced no negative. It reached a
  positive degree-66 candidate with zero nonflag equalities (`a=7,b=10`, hook
  outer shape `[11,1^7]`, mask `0x200000000000000000000`). MariaDB persistence
  was enabled for all evaluated rows; the negative frontier is unchanged.
- A degree-65-capped mixed-seed batch (population 32, three generations,
  mutation rate 0.90, seed 20261004) produced no negative. Its exact best was
  positive degree 65 with three nonflag equalities (`a=9,b=8`, outer hook
  `[9,1^9]`, mask `0x400000100000108`). All rows were persisted in MariaDB;
  the degree-72/four-hole negative remains certified frontier.
- A degree-64-capped mixed-seed batch (population 36, three generations,
  mutation rate 0.85, seed 20261005) produced no negative. Its exact best
  was positive degree 64 with three nonflag equalities (`a=7,b=10`, outer
  hook `[11,1^7]`, mask `0x300004000000000100`). MariaDB persistence was
  enabled; no change to the degree-72/four-hole negative frontier.
- A degree-63-capped mixed-seed batch (population 36, three generations,
  mutation rate 0.90, seed 20261006) produced no negative. Its exact best
  was positive degree 62 with one nonflag equality (`a=6,b=11`, outer hook
  `[12,1^6]`, mask `0x100000000000000000002`). All evaluated rows were
  persisted in MariaDB; the degree-72/four-hole negative remains certified.
- A degree-60-capped mixed-seed batch (population 36, three generations,
  mutation rate 0.90, seed 20261007) produced no negative. Its exact best was
  positive degree 57 with three nonflag equalities (`a=7,b=9`, outer hook
  `[10,1^7]`, mask `0x200000001008010`). All rows were persisted in MariaDB;
  no change to the degree-72/four-hole negative frontier.
- A degree-55-capped mixed-seed batch (population 40, three generations,
  mutation rate 0.90, seed 20261008) produced no negative. Its exact best
  was positive degree 55 with zero nonflag equalities (`a=7,b=8`, outer hook
  `[9,1^7]`, mask `0x10000000`). All evaluated rows were persisted in
  MariaDB; the negative frontier remains degree 72 with four holes.
- A degree-50-capped mixed-seed batch (population 40, three generations,
  mutation rate 0.95, seed 20261009) produced no negative. Its exact best was
  positive degree 50 with one nonflag equality (`a=6,b=9`, outer hook
  `[10,1^6]`, mask `0x200000200000000`). All evaluated rows were persisted;
  the certified negative frontier remains degree 72 with four holes.
- A degree-45-capped mixed-seed batch (population 40, three generations,
  mutation rate 0.95, seed 20261010) produced no negative. Its exact best was
  positive degree 45 with one nonflag equality (`a=6,b=8`, outer hook
  `[9,1^6]`, mask `0x2004000000`). All evaluated rows were persisted; no
  change to the degree-72/four-hole negative frontier.
- A degree-35-capped mixed-seed batch (population 40, three generations,
  mutation rate 0.95, seed 20261011) produced no negative. Its exact best was
  positive degree 34 with one nonflag equality (`a=5,b=7`, outer hook
  `[8,1^5]`). All evaluated rows were persisted; the certified negative
  frontier remains degree 72 with four holes.
- A degree-25-capped mixed-seed batch (population 40, three generations,
  mutation rate 0.95, seed 20261012) produced no negative. Its exact best was
  positive degree 24 with one nonflag equality (`a=5,b=5`, outer hook
  `[6,1^5]`). All evaluated rows were persisted; no change to the certified
  degree-72/four-hole negative frontier.
- A degree-15-capped mixed-seed batch (population 40, three generations,
  mutation rate 0.95, seed 20261013) produced no negative. Its exact best was
  positive degree 15 with zero nonflag equalities (`a=5,b=3`, outer hook
  `[4,1^5]`, empty mask). All evaluated rows were persisted; no change to the
  certified degree-72/four-hole negative frontier.
- A degree-10-capped mixed-seed batch (population 40, three generations,
  mutation rate 0.95, seed 20261014) produced no negative. Its exact best was
  positive degree 10 with zero nonflag equalities (`a=4,b=3`, outer hook
  `[4,1^4]`). All evaluated rows were persisted; the certified negative
  frontier remains degree 72 with four holes.
- A broad low-hole run (population 24, two generations, seed 20261016,
  dimension cap 100, exact DP cap 1,000,000) certified a new two-hole
  negative at degree 88. It uses hook outer shape `[11,1^8]`, mask
  `0x10000001000`, with negative coefficients in degrees 1 and 2. This
  matches the best bad-edge count (two) but is larger than the degree-84
  two-hole certificate; all rows were persisted in MariaDB. The preceding
  population-48 attempt timed out before producing a report and is not treated
  as evidence.
- A two-hole-focused run (population 24, two generations, seed 20261017,
  dimension cap 84) seeded both the degree-84 and degree-88 two-hole masks.
  It independently revalidated the degree-84 certificate in every generation
  and found no smaller two-hole negative. All rows were persisted in MariaDB.
- A broader sub-73 run (population 28, three generations, seed 20261018,
  dimension cap 73) independently revalidated a degree-73 negative with four
  nonflag equalities (`a=b=9`, hook `[10,1^9]`) and a negative coefficient in
  degree 1. It found no negative below degree 73; all rows were persisted in
  MariaDB, and the degree-84/two-hole certificate remains the best bad-edge
  frontier.
- A low-mutation two-hole lineage run (population 24, three generations,
  mutation rate 0.25, seed 20261019, cap 84) revalidated the degree-84
  two-hole negative in every generation. No smaller two-hole descendant was
  found; all rows were persisted in MariaDB.
- A mixed high-mutation run (population 28, three generations, seed 20261020,
  dimension cap 84) seeded both two-hole negatives and the degree-73 lineage.
  Every generation independently selected the degree-84 two-hole certificate;
  no smaller two-hole or sub-72 negative appeared. MariaDB persistence was
  enabled for all evaluated rows.
- A fresh escape run (population 20, three generations, side range 1–16,
  seed 20261021, dimension cap 100) selected the degree-88 two-hole lineage
  in every generation. No smaller two-hole negative or lower-dimensional
  negative was found; all evaluated rows were persisted in MariaDB.
- A sub-80 lineage run (population 32, three generations, seed 20261022,
  dimension cap 80) repeatedly selected the degree-73 four-hole negative
  (`a=b=9`, mask `0x1208000000100000100`). No smaller negative or reduced-hole
  descendant appeared; all evaluated rows were persisted in MariaDB.
- A fresh high-mutation degree-72-capped run (population 32, three
  generations, seed 20261023) produced no negative. Generation bests included
  positive degree 68 and degree 71 candidates with one nonflag equality; all
  rows were persisted and the degree-72/four-hole negative remains the smallest
  certified negative.
- An alternate degree-72-capped batch (population 24, three generations,
  side range 2–16, mutation rate 0.55, seed 20261024) produced no negative.
  It reached positive degree-71 candidates with one nonflag equality; all
  evaluated rows were persisted in MariaDB and the negative frontier is
  unchanged.
- Triple-swap two-hole continuation at proposal offsets 1370–1379 completed
  with 10/10 exact rows, zero limited rows, and zero negative polynomials.
  MariaDB persistence was enabled; the degree-84/two-hole frontier is
  unchanged.
- Triple-swap two-hole continuation at offsets 1380–1389 likewise completed
  10/10 exact rows, with no limited rows and no negative polynomial; MariaDB
  persistence was enabled and the frontier is unchanged.
- Triple-swap two-hole continuation at offsets 1390–1399 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1400–1409 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1410–1419 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1420–1429 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1430–1439 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1440–1449 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1450–1459 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1460–1469 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1470–1479 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1480–1489 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1490–1499 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1500–1509 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1510–1519 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1520–1529 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1530–1539 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1540–1549 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1550–1559 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1560–1569 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1570–1579 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1580–1589 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1590–1599 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1600–1609 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1610–1619 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1620–1629 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1630–1639 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1640–1649 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1650–1659 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1660–1669 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1670–1679 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1680–1689 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1690–1699 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1700–1709 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1710–1719 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1720–1729 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1730–1739 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1740–1749 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1750–1759 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1760–1769 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1770–1779 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1780–1789 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1790–1799 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1800–1809 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1810–1819 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1820–1829 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1830–1839 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1840–1849 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1850–1859 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1860–1869 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1870–1879 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1880–1889 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1890–1899 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1900–1909 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1910–1919 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1920–1929 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1930–1939 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1940–1949 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1950–1959 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1960–1969 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1970–1979 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1980–1989 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 1990–1999 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2000–2009 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2010–2019 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2020–2029 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2030–2039 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2040–2049 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2050–2059 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2060–2069 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2070–2079 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2080–2089 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2090–2099 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- Triple-swap two-hole continuation at offsets 2100–2109 completed 10/10
  exact rows, with zero limited rows and zero negative polynomials. MariaDB
  persistence was enabled; no frontier change.
- A three-generation degree-72-capped search produced no negative; its best
  candidate was positive degree 70 with zero nonflag equalities. This further
  separates the positive and negative frontiers but yields no counterexample
  improvement.
- Triple-swap continuation at offsets 1350–1359 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
- Triple-swap continuation at offsets 1360–1369 completed 10/10 exact rows,
  with no limited rows and no negative polynomial; no frontier improvement.
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
- The offset-310 batch (total cap 320) evaluated proposals 310–319 exactly; all
  10 were positive.
- The offset-320 batch (total cap 330) evaluated proposals 320–329 exactly; all
  10 were positive.
- The offset-330 batch (total cap 340) evaluated proposals 330–339 exactly; all
  10 were positive.
- The offset-340 batch (total cap 350) evaluated proposals 340–349 exactly; all
  10 were positive.
- The offset-350 batch (total cap 360) evaluated proposals 350–359 exactly; all
  10 were positive.
- The offset-360 batch (total cap 370) evaluated proposals 360–369 exactly; all
  10 were positive.
- The offset-370 batch (total cap 380) evaluated proposals 370–379 exactly; all
  10 were positive.
- The offset-380 batch (total cap 390) evaluated proposals 380–389 exactly; all
  10 were positive.
- The offset-390 batch (total cap 400) evaluated proposals 390–399 exactly; all
  10 were positive.
- The offset-400 batch (total cap 410) evaluated proposals 400–409 exactly; all
  10 were positive.
- The offset-410 batch (total cap 420) evaluated proposals 410–419 exactly; all
  10 were positive.
- The offset-420 batch (total cap 430) evaluated proposals 420–429 exactly; all
  10 were positive.
- The offset-430 batch (total cap 440) evaluated proposals 430–439 exactly; all
  10 were positive.
- The offset-440 batch (total cap 450) evaluated proposals 440–449 exactly; all
  10 were positive.
- The offset-450 batch (total cap 460) evaluated proposals 450–459 exactly; all
  10 were positive.
- The offset-460 batch (total cap 470) evaluated proposals 460–469 exactly; all
  10 were positive.
- The offset-470 batch (total cap 480) evaluated proposals 470–479 exactly; all
  10 were positive.
- The offset-480 batch (total cap 490) evaluated proposals 480–489 exactly; all
  10 were positive.
- The offset-490 batch (total cap 500) evaluated proposals 490–499 exactly; all
  10 were positive.
- The offset-500 batch (total cap 510) evaluated proposals 500–509 exactly; all
  10 were positive.
- The offset-510 batch (total cap 520) evaluated proposals 510–519 exactly; all
  10 were positive.
- The offset-520 batch (total cap 530) evaluated proposals 520–529 exactly; all
  10 were positive.
- The offset-530 batch (total cap 540) evaluated proposals 530–539 exactly; all
  10 were positive.
- The offset-540 batch (total cap 550) evaluated proposals 540–549 exactly; all
  10 were positive.
- The offset-550 batch (total cap 560) evaluated proposals 550–559 exactly; all
  10 were positive.
- The offset-560 batch (total cap 570) evaluated proposals 560–569 exactly; all
  10 were positive.
- The offset-570 batch (total cap 580) evaluated proposals 570–579 exactly; all
  10 were positive.
- The offset-580 batch (total cap 590) evaluated proposals 580–589 exactly; all
  10 were positive.
- Resumed the bounded three-swap/two-hole frontier through proposal offsets
  590–2119 (10-proposal batches, exact arithmetic, 2,000,000-state and
  8-second candidate caps). Every completed batch was recorded in the ignored
  search-repo reports and MariaDB; no negative coefficient or improvement over
  the degree-84 two-hole frontier appeared. The latest offset-2110 batch was
  10/10 exact, 0 limited, 0 negative.
- Offset 2120–2129 likewise completed 10/10 exactly (0 limited, 0 negative,
  14 negative seeds retained); no smaller-degree or fewer-hole candidate arose.
- Offset 2130–2139 completed 10/10 exactly (0 limited, 0 negative); MariaDB
  persistence and the ignored resumable report are both updated, with no
  frontier improvement.
- Offset 2140–2149 completed 10/10 exactly (0 limited, 0 negative); no
  smaller-degree or fewer-hole candidate appeared.
- Offset 2150–2159 completed 10/10 exactly (0 limited, 0 negative); the
  two-hole degree-84 frontier is unchanged.
- Offset 2160–2169 completed 10/10 exactly (0 limited, 0 negative); no
  lower-degree or fewer-hole descendant was found.
- Offset 2170–2179 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2180–2189 completed 10/10 exactly (0 limited, 0 negative); the
  degree-84 two-hole frontier remains unchanged.
- Offset 2190–2199 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or lower-hole negative descendant was found.
- Offset 2200–2209 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2210–2219 completed 10/10 exactly (0 limited, 0 negative); no
  smaller instance or fewer-hole negative candidate appeared.
- Offset 2220–2229 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2230–2239 completed 10/10 exactly (0 limited, 0 negative); no
  smaller instance or fewer-hole candidate appeared.
- Offset 2240–2249 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2250–2259 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2260–2269 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2270–2279 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole negative candidate appeared.
- Offset 2280–2289 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2290–2299 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole negative candidate appeared.
- Offset 2300–2309 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2310–2319 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2320–2329 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2330–2339 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2340–2349 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2350–2359 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2360–2369 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2370–2379 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2380–2389 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2390–2399 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2400–2409 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2410–2419 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2420–2429 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2430–2439 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2440–2449 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2450–2459 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2460–2469 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2470–2479 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2480–2489 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2490–2499 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2500–2509 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2510–2519 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2520–2529 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2530–2539 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2540–2549 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2550–2559 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2560–2569 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2570–2579 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2580–2589 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2590–2599 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2600–2609 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2610–2619 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2620–2629 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2630–2639 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2640–2649 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2650–2659 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2660–2669 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2670–2679 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2680–2689 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2690–2699 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2700–2709 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2710–2719 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2720–2729 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2730–2739 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2740–2749 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2750–2759 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2760–2769 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2770–2779 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2780–2789 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2790–2799 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2800–2809 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2810–2819 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2820–2829 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2830–2839 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2840–2849 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2850–2859 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2860–2869 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2870–2879 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2880–2889 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2890–2899 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2900–2909 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2910–2919 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2920–2929 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2930–2939 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2940–2949 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2950–2959 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2960–2969 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2970–2979 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 2980–2989 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 2990–2999 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3000–3009 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3010–3019 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3020–3029 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3030–3039 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3040–3049 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3050–3059 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3060–3069 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3070–3079 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3080–3089 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3090–3099 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3100–3109 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3110–3119 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3120–3129 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3130–3139 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3140–3149 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3150–3159 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3160–3169 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3170–3179 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
- Offset 3180–3189 completed 10/10 exactly (0 limited, 0 negative); no
  frontier improvement was found.
- Offset 3190–3199 completed 10/10 exactly (0 limited, 0 negative); no
  smaller or fewer-hole candidate appeared.
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
