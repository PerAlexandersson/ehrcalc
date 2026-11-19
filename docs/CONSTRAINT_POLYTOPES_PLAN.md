# Constraint polytopes: implementation plan

Status: plan, 2026-11-19. Owner: real-rooted-tools coordinator. Requested by Per Alexandersson.

## 1. Goal

Exact Ehrhart data for rational polytopes given by three kinds of constraints on integer
variables `x_0, ..., x_{n-1}`:

1. **bounds** `a_i <= x_i <= b_i`;
2. **pairwise differences** `x_i - x_j <= c_ij` (the order relation `x_i <= x_j` is `c_ij = 0`);
3. **subset sums** `l_k <= sum_{i in S_k} x_i <= u_k` (an equality when `l_k = u_k`; either side
   may be absent).

All right-hand sides are rationals and scale with the dilation `N`. The polytope need not have
integral vertices. The user asserts that the Ehrhart function is a polynomial (for flagged Kostka
this is Per's theorem). We compute it from enough exact samples, using reciprocity, the exact
dimension and the exact relative interior.

The class contains, among others: order polytopes and marked order polytopes; Gelfand–Tsetlin
polytopes, skew and flagged, either as full GT patterns or reduced to their free entries;
hypersimplices and matroid base and independence polytopes (sums over subsets, bounded by ranks);
transportation and Birkhoff polytopes (row and column sums); and slices of all of these.
Flow polytopes need signed coefficients and are out of scope for now (see section 11).

## 2. Input

The stable contract is JSON (`ehrcalc.constraint-polytope.v1`):

```json
{
  "variables": 5,
  "names": ["x0", "x1", "x2", "x3", "x4"],
  "bounds": [[0, 3], [0, 3], [0, "5/2"], [1, 3], [0, 3]],
  "differences": [[0, 1, 0], [2, 1, 0], [3, 4, "-1/2"]],
  "sums": [
    {"vars": [0, 2], "lower": 2, "upper": 2},
    {"vars": [1, 3, 4], "upper": 6}
  ]
}
```

- `bounds[i]` is `[a_i, b_i]`. Every variable must be bounded through bounds, differences or
  sums. Unbounded systems are rejected after presolve with the offending variable named.
- `differences` entries `[i, j, c]` mean `x_i - x_j <= c`.
- Rationals are written as integers or `"p/q"` strings.
- A short text form is accepted by the CLI for hand-written cases:

```text
vars 5
bound 0 0 3          # 0 <= x0 <= 3
le 0 1               # x0 <= x1
le 3 4 -1/2          # x3 - x4 <= -1/2
sum 0 2 = 2
sum 1 3 4 <= 6
```

Validation rejects: indices out of range, empty sums, `a_i > b_i`, `l_k > u_k`, duplicate names.
Duplicate constraints are merged (keep the tightest).

## 3. Semantics

`L(N) = #(N P ∩ Z^n)`, where every right-hand side is multiplied by `N`. For `N >= 1` a constraint
`x_i <= N b_i` becomes `x_i <= floor(N b_i)`, and so on. `L(0) = 1` when `P` is nonempty.

The relative-interior count `L°(N)` counts lattice points of `N P` that satisfy every
**non-implicit** inequality strictly and every implicit one with equality. For integer data a strict
inequality `y < N t` becomes `y <= ceil(N t) - 1`.

Ehrhart–Macdonald reciprocity holds for rational polytopes and their Ehrhart quasi-polynomials, so
if `L` is a polynomial of degree `d = dim P`, then `L(-N) = (-1)^d L°(N)`.

Output always records `assumes_polynomial: true` and the sample points used. The option
`--extra-samples k` computes `k` further values and reports any disagreement, for testing; it is
off by default.

## 4. Pipeline

```text
parse -> normalize -> presolve -> components -> affine hull -> sample plan -> count -> interpolate -> EhrhartData
```

1. **Normalize.** Merge duplicates. Sort each sum's variables. Turn a sum with one variable into a
   bound.
2. **Presolve** (sound, used only for pruning and fixing): interval propagation through
   differences and sums (bounds-consistency) until a fixed point or a pass limit. Variables with
   `lo == hi` for every `N` (identical rational bounds after propagation) are substituted.
   Propagation never decides the dimension.
3. **Components.** Build the constraint hypergraph: variables are nodes; each difference is an
   edge; each sum is a hyperedge. If it has several components, `P` is a product and its Ehrhart
   polynomial is the product of the factors' polynomials. Dimensions add; check this.
4. **Affine hull.** Use `ehrcalc_kostka_engine::affine_hull::RationalPolyhedron`, whose
   implicit-equality answer is LP-based and certificate-checked. It gives emptiness, the exact
   dimension `d`, the implicit-equality mask and a relative-interior point. Never infer the
   dimension from lattice points.
5. **Sample plan.** We need `d + 1` values. Use `L(0) = 1`, positive values `L(1..p)` and
   reciprocal values `L(-1..-q)` with `p + q = d`. Pick `(p, q)` to minimise the estimated cost
   (section 6): interior counts are often cheaper because strict constraints shrink ranges.
   `--positive-only` forces `q = 0`.
6. **Count** each sample by the frontier DP (section 5), with modular residues and CRT
   (section 7).
7. **Interpolate** with the existing `EhrhartPolynomial::interpolate(dimension, points)` and build
   `EhrhartData` (h*, binomial basis, finite differences) with the existing constructors. A rational
   polytope may have negative h* entries; report them as they are.

## 5. Counting: the frontier DP

Process the variables in an order `v_1, ..., v_n` (section 6). After step `t`:

- a processed variable is **live** while some difference constraint joins it to an unprocessed
  variable;
- a sum is **open** while it has both processed and unprocessed variables.

The state is the tuple of live values together with the partial sums of open sums.

**Transition for `v = v_t`:**
- The range of `x_v` is the bound range, intersected with the difference constraints to live
  variables.
- It is further cut so that every open sum containing `v` can still reach its range, using the
  presolved bounds of the remaining unprocessed members (a sound min/max).
- If `v` is the last unprocessed member of an equality sum, its value is forced: no branching.
- On closing a sum, check `l_k <= sum <= u_k`.
- Live values that are no longer needed are dropped. States with equal keys merge, adding their
  counts.

**Keys.** Values are bounded by the presolved ranges, so pack each value in `ceil(log2(range+1))`
bits into `u128` where possible, else a `SmallVec<[u32; 8]>`. Use a hash map with a fast hasher.

**Strict counts** use the same code with shifted integer right-hand sides for the non-implicit
constraints (section 3).

**Budget.** `--max-states` caps the largest layer. Before running, the plan reports an estimated
peak (section 6). On overflow the run stops with the step, frontier and estimate, rather than
exhausting memory.

**Reference.** A brute-force box enumerator for tiny systems, used only in tests.

## 6. Frontier order

The cost of a step is about the product of the value ranges of the live variables and the ranges
of the open partial sums, at the dilation being counted. We minimise the total over all steps, which
is dominated by the peak.

- **Model.** A hypergraph with variable nodes and sum nodes. A sum behaves like one accumulator
  variable (its partial sum), not like a clique on its members. That is why a long equality row
  costs one state coordinate and not many.
- **Candidates:**
  1. greedy minimum-frontier: pick the next variable minimising the resulting weighted frontier;
     break ties by closing sums and difference edges; one-step lookahead;
  2. sum-contiguous: process each sum's members consecutively, ordering the sums by a greedy path
     through the sum-overlap graph (GT rows and content lines become one sweep);
  3. reverse Cuthill–McKee from a pseudo-peripheral node of the primal graph;
  4. exact minimum of the maximum weighted frontier by dynamic programming over subsets when
     `n <= 22` (vertex separation number, weighted).
- **Choice.** Evaluate each candidate with the cost model at the largest planned dilation and keep
  the cheapest. Report the order, the peak frontier and the estimated states in the JSON `plan`
  field.
- **Override.** `--order 3,0,1,...` overrides the choice. `--order-heuristic` selects one candidate.
- **Extensions.** `order` and `gt` keep their own counters. They may later call this planner on
  their constraint systems for comparison.

## 7. Modular counting and CRT

- Count with `m` word-sized prime moduli at once: each state carries `m` residues, so the
  transition enumeration is shared. Reuse the moduli tables and `crt_reconstruct_bounded` from
  `packed_modular`.
- **Certified bound.** The count is at most the number of lattice points of the presolved box,
  `prod_i (hi_i - lo_i + 1)`. For equality sums use the sharper bound that drops one variable per
  independent equality: the product over a basis complement. Choose `m` so that the product of the
  moduli exceeds the bound. Report the bound bits and `m`.
- **Exact path.** `--exact` counts with `BigUint`; used as the reference in tests and for tiny
  inputs. CI tests check exact and CRT agreement.
- **Parallelism.** Split a layer's states across threads with rayon, as in `packed_modular`. This
  comes later, once the sequential version is correct.

## 8. Adapters

Each adapter builds the constraint system and must agree with the existing family:

- `order`: a poset to `0 <= x <= 1` and `x_i <= x_j` per cover. Compare with `order` (both methods).
- Flagged GT, full form: all GT entries as variables; frozen entries from flags and the top row
  as fixed bounds; interlacing as differences; level sums as equalities. Compare with `gt` and
  `kostka`, including skew `--mu` and per-label flags, on random and corner cases. This includes
  the 2026-10-08 regression fixtures (`flag_relaxation_geometry`, broad corner cases).
- Flagged GT, reduced form: only free entries, with marks as bounds (Per's marked poset of free
  entries; dictionary in `notes/flagged-kostka/` of real-rooted-tools). Compare with the full form
  and with `kostka`.
- Matroids (tests only at first): uniform matroids and small graphic matroids, base and
  independence polytopes, from an explicit rank function. Compare with brute force; hypersimplex
  `Δ(k,n)` against its closed formula.
- Birkhoff `B_3`, `B_4` and small transportation polytopes against known Ehrhart polynomials.

## 9. Interface

- Library: `ehrcalc::families::constraint_polytope_ehrhart(&ConstraintPolytopeInput,
  &ConstraintOptions) -> ExactResult<ConstraintComputation>`. It returns the `EhrhartData`, the
  dimension, the implicit-equality mask, the plan, the samples and the modular data. The core
  counter lives in a new module `ehrcalc-kostka-engine/src/constraint_polytope.rs` next to
  `affine_hull` and the CRT code. Nothing in it depends on GT types.
- CLI: `ehrcalc polytope --input FILE.json | --text FILE | -` with `--positive-only`,
  `--extra-samples K`, `--order`, `--order-heuristic`, `--max-states`, `--exact`, `--plan-only`
  (prints dimension, implicit equalities, plan and estimates without counting) and `--format`.
- Converters: `ehrcalc kostka --emit-constraints` and `ehrcalc order --emit-constraints` print the
  constraint JSON, so any existing input can be run through the new engine.
- MCP: tool `constraint_polytope_ehrhart`, calling the same library function.
- Documentation: `docs/CONSTRAINT_POLYTOPES.md` (model, semantics, ordering, CRT bound, examples),
  README, `EHRHART_TOOL_SPEC.md` family list, and the regenerated `docs/cli.md`.

## 10. Phases and checks

| Phase | Content | Gate |
|---|---|---|
| A | data model, JSON/text parsing, validation, presolve, components, affine hull, brute-force reference | unit tests; affine hull vs brute force on random tiny systems |
| B | frontier DP (exact `BigUint`), strict counts, sample plan, interpolation, reciprocity | DP = brute force on random systems; `L(-N)` = interior counts; order adapter = `order` |
| C | ordering heuristics, cost model, `--plan-only`, budget | heuristics never change counts; peak estimates reported; the exact-subset DP agrees with the greedy on small cases |
| D | modular residues and CRT, certified bound, rayon layers | CRT = exact on all tests; bound check |
| E | flagged GT adapters (full and reduced), matroid, Birkhoff fixtures | agreement with `gt`/`kostka` on ≥ 1000 random flagged and skew cases plus the regression fixtures; timing table vs `kostka-engine` |
| F | CLI, MCP, docs, generated CLI reference | CLI/MCP integration tests; CLI drift test |

Every phase runs `cargo fmt`, `cargo clippy -D warnings` and the focused tests
(`timeout 60s nice -n 10 cargo test -p ...`, longer only for documented benchmarks), under the shared
memory guard. Commits go to `feat/constraint-polytopes`; merge to `main` after phase F with all
crate tests green.

## 11. Open questions for Per

1. Naming: "constraint polytope" and `ehrcalc polytope` are working names.
2. Signed coefficients in sums (`x_i - x_j + x_k = s`) would bring in flow polytopes. The frontier
   DP handles them unchanged; only the presolve bounds and the CRT bound need care. Phase G?
3. Pairwise constraints with coefficients other than ±1 (`x_i <= 2 x_j`)? Not planned.
4. Should a quasi-polynomial check (`--extra-samples`) become a periodicity detector that reports
   the period when a polynomial fit fails?
