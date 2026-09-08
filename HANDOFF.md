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
