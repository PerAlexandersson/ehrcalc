# Constraint polytopes

We compute exact lattice-point counts and sampled Ehrhart data for rational
polytopes defined by variable bounds, pairwise differences, and subset sums.
The same library service supplies `ehrcalc polytope` and the MCP tool
`constraint_polytope_ehrhart`.

## Model and input

Variables have zero-based indices. A difference `[i,j,c]` means
`x_i - x_j <= c`; a sum uses each member with coefficient one. All endpoints
are integers or rational strings such as `"-3/2"`. A missing or `null` endpoint
is absent. All variables must be bounded, possibly through other constraints.

```json
{
  "schema": "ehrcalc.constraint-polytope.v1",
  "variables": 4,
  "names": ["a", "b", "c", "d"],
  "bounds": [[0, 1], [0, 1], [0, 1], [0, 1]],
  "differences": [],
  "sums": [{"vars": [0, 1, 2, 3], "lower": 2, "upper": 2}]
}
```

The schema tag is optional on input. `names` may be omitted; otherwise it
contains one distinct name per variable. `bounds` may be omitted; otherwise
it contains one pair per variable. Duplicate constraints merge to the tightest
endpoints. We reject reversed endpoints, out-of-range indices, duplicate sum
members, empty sums, unknown fields, and floating-point endpoints. Valid but
mutually incompatible constraints describe an empty polytope, including
conflicting repeated text `bound` lines. Inputs are limited to **256 variables**;
this is checked before variable-array allocation in JSON, text and adapters.

The short text syntax supports comments and the relations `=`, `<=`, `>=`:

```text
vars 3
bound 0 0 2
bound 1 0 2
bound 2 0 2
le 0 1             # x0 <= x1
le 1 2 -1/2        # x1 - x2 <= -1/2
sum 0 1 2 = 3
```

Read JSON with `--input FILE.json`, text with `--text FILE`, and stdin with
`--input -`, `--text -`, or the positional `-` (JSON). Exactly one source is
required. `--format text|json|latex` selects the presentation.

## Dilations, dimension, and polynomiality

For a polytope \(P\), we count
\[
  L(N)=\#(NP\cap\mathbb Z^n).
\]
Every right-hand side, including lower bounds and marks, scales by \(N\).
For nonempty \(P\), we set \(L(0)=1\); an empty polytope has count zero,
`empty: true`, and `dimension: null`.

We determine the affine dimension and implicit equalities from a certified
exact rational affine-hull computation. Interval propagation only prunes the
search. In particular, lattice points at dilation one need not span \(P\).
The relative interior makes each non-implicit inequality strict and keeps
each implicit inequality as an equality. For an integer left-hand side,
`y < N*c` becomes `y <= ceil(N*c)-1`.

**The caller assumes that the Ehrhart function is a polynomial.** Rational
polytopes generally have quasi-polynomial Ehrhart functions. We interpolate
using \(d+1\) exact values, where \(d=\dim P\), and always record
`assumes_polynomial: true`. Under this assumption, reciprocity gives
\[
  L(-N)=(-1)^d L^\circ(N).
\]
The planner may combine positive ordinary samples with negative reciprocal
samples; `--positive-only` uses zero and positive samples exclusively.

`--extra-samples K` defaults to **1** and is limited to **64**. These further positive counts are
excluded from interpolation. A mismatch produces a visible stderr warning,
warnings in every output format, and exact `mismatches` entries in JSON.
A mismatch indicates failure of polynomiality or an error in counting or
dimension. Finite agreement does not prove polynomiality. Passing
`--extra-samples 0` disables this check.
Necessary checks still run with zero extra samples: a nonintegral fixed mark
or implicit-equality right-hand side proves that some dilations have no
lattice points, so the count function cannot be one polynomial. We also warn
if a nonempty fit has degree different from the affine dimension or a
nonpositive leading coefficient. JSON sets `polynomiality_checks_failed` and
retains the fitted coefficients as diagnostic data.

`--fail-on-mismatch` makes a failed extra sample or necessary check return a
nonzero CLI exit (with a diagnostic instead of the normal report). The MCP
field `fail_on_mismatch: true` returns a tool error under the same conditions.
Without it, warnings and exact fitted data remain available in every format.


For example, `[0,1/2]` has \(L(N)=\lfloor N/2\rfloor+1\), and
`echo '{"variables":1,"bounds":[[0,"1/2"]]}' | ehrcalc polytope -`
reports an extra-sample mismatch. We retain the exact fitted coefficients
even when the fit has no integral h* transform; unavailable transforms are
`null` and explained in `warnings`.

## Counting and ordering

We presolve bounds, substitute fixed coordinates, and split disconnected
constraint components. The frontier dynamic program stores live coordinate
values and partial sums of open subset constraints. Each variable is
translated by its integral lower endpoint at the current dilation, so partial
subset sums are nonnegative. Equality sums force their last variable.

`--order-heuristic` accepts these candidates:

| Name | Construction |
| --- | --- |
| `greedy` | Weighted frontier with one-step lookahead. |
| `sum-contiguous` | Sweep subset groups using the greedy variable order. |
| `rcm` | Original pseudo-peripheral reverse Cuthill–McKee on the primal graph. |
| `exact-subset` | Exact subset minimax of the range estimate, through 22 free variables. |
| `min-degree`, `min-fill` | Reverse elimination orders on the incidence graph. |
| `cm` | Forward/reverse Cuthill–McKee from up to six low-degree/peripheral seeds. |
| `gt-level` | GT level sweeps, both level and within-level directions. |
| `gt-row` | Tableau-row sweeps, both row and within-row directions. |
| `local` | Up to three adjacent-swap passes from the three best estimated candidates. |
| `auto` | All applicable candidates, with bounded frontier pilots. |

The incidence graph has one node per subset accumulator; a sum is not expanded
to a clique for the new graph heuristics. GT candidates recognize the adapters'
`alpha_LEVEL_ROW` names, including reduced systems. Level sweeps keep each
level-sum group contiguous; row sweeps provide an alternative traversal across
these groups. Explicit GT requests without these names are errors. Local
search minimizes estimated peak, then work, and skips swaps above 128 free
variables. Auto includes exact subset DP through **16** free variables to keep
planning affordable; explicit requests support 22 and fail above that limit.

Auto evaluates each distinct order with the production frontier at dilations
1 and 2, separately for ordinary and strict constraints. Each pilot has at most
4,096 states per layer (or the caller's smaller cap), 100,000 transitions, and
a cooperative 50 ms time budget, checked every 256 work units. Pilots use one
modular lane; zero residues retain their keys, so state presence is exact.
Duplicate orders share pilot results. Explicit orders and explicit heuristics
bypass pilots.

For strict counts, when the first two probes are empty or censored, we screen
integer boxes for the first possible scale from 3 up to the requested dilation,
with screening capped at scale 256.
Each order may try up to two additional bounded probes, stopping once two
complete nonempty observations are available. There are at most four probes
per distinct order; every probe keeps the same budgets. Box feasibility is
only a screen: a larger probe can still be empty or capped. The actual
scales and `adaptive-*` prediction bases are recorded in JSON.

For completed pilots with nonempty final layers, we extrapolate each measured
layer using its range growth. Two informative pilots calibrate an effective
growth exponent between zero and one relative to the range-product model;
one informative pilot uses exponent one. The predicted peak chooses the order,
with extrapolated runtime breaking ties. When more than two probes are
informative, the last two calibrate growth. A forecast based on only one completed pilot receives a factor-two peak
penalty if another probe is budget-limited. If no probe reaches a nonempty final layer,
selection falls back to the range estimate. Empty small dilations do not imply
that larger dilations are empty. These predictions use floating point only for
cost selection; feasibility, counts and certificates remain exact.

JSON plans contain the `selection` policy and the complete `candidates` table:
variant, original variable order, exact range estimates, duplicate index,
`prediction_basis`, predicted peak/runtime (runtime is null on range fallback), and each pilot's actual peak, completed layer sizes,
elapsed microseconds, transitions, status and budgets. An incomplete pilot's
peak is an observed lower bound. Wall-clock observations, and sometimes the
choice between tied orders, can vary between runs. The service freezes the
chosen orders for its scheduled samples; sample plans use `selection: scheduled`.
`--order 3,0,1,2` supplies a permutation of **all original input variables**,
including entries removed by presolve. `--order` and an explicitly supplied
`--order-heuristic` are mutually exclusive (also in MCP). Fixed entries are skipped when
constructing component orders. Plan orders use original variable indices.

`--plan-only` computes the affine hull, sample schedule and frontier estimates
without pilots or sample counting (`selection: estimate-only` for auto).
Estimates bound layer states; they do not estimate the
number of attempted transitions. The sampling split is scored with one fixed
ordinary greedy order rather than rerunning greedy at every dilation.
`--positive-only` skips all interior estimates. A single connected component
reuses the parent count, fit and plan; multiple factors retain their separate
product cross-check. The single-factor product match is an identity.

`--max-states` defaults to 1,000,000 and
stops a count when a layer exceeds that limit, with the step and frontier in
the error. This bounded default intentionally differs from `gt`/`kostka`,
whose default state budget is unlimited. Scheduled ordinary and interior
plans can use different orders.

The default counter carries residues for several word-sized primes through
one set of transitions. The number of points is bounded by the product of
the presolved integer box sizes, `prod_i (hi_i - lo_i + 1)`.
Independent equality rows allow us to drop
pivot variables from this product: an assignment to the remaining coordinates
determines at most one assignment to the pivots. We choose enough primes so
that their product **exceeds** this bound and reconstruct the unique count by
CRT. Each sample records `bound_bits`, `upper_bound`, `moduli_count`, the
moduli and residues, and their combined modulus, per connected component.
The current table extends deterministically to at most 64 moduli; capacity
failure is an error. `--exact` uses `BigUint` counts instead and omits CRT data.
Counting is currently serial.

## Bounded ordering benchmarks

`examples/frontier_order_benchmark.rs` is a JSONL adapter for comparing `auto`,
the original four-candidate `legacy-auto` policy, and `gt-level`. Fit mode
counts the full interpolation grid with `extra_samples=0`, using frozen orders;
it performs no interpolation or coefficient search. Count mode evaluates one
specified dilation and interior flag. Plans are flushed before counting, and
cap failures retain their observed state lower bound.

`examples/run_frontier_order_benchmark.py` requires only Python 3.12 and that
release executable on Linux x86-64. It isolates each case/policy in a process,
caps address space at 8 GiB, applies CPU and wall limits, records peak child RSS
with `wait4`, and stops each whole job within six hours. Results are append-only
JSONL, resumable with matching input and binary SHA-256 hashes. Each invocation
selects at most 100 inputs; `--start` chooses the next batch. Use an external
target directory and generic `-C target-cpu=x86-64` when building the binary.
On the coordinator, run the Python runner through the prescribed heavy guard
with `JOB_MB=9000` passed as an argument. A completed grid is counting evidence,
not a new validated polynomial. No queue submission is performed by the runner.

## Worked examples

### Hypersimplex

The JSON above describes \(\Delta(2,4)\). Run:

```sh
ehrcalc polytope --input examples/constraints/hypersimplex.json --format json
```

We obtain dimension 3,
\[
  L(N)=1+\tfrac73N+2N^2+\tfrac23N^3,
  \qquad h^*(t)=1+2t+t^2.
\]
The JSON h* vector is `["1","2","1","0"]`, retaining the trailing zero.
In general, inclusion-exclusion gives
\[
  L_{\Delta(k,n)}(N)=\sum_{j=0}^n(-1)^j\binom nj
    \binom{kN-j(N+1)+n-1}{n-1},
\]
where the last binomial coefficient is zero when its upper argument is less
than \(n-1\). The fixtures include \(k=0,n\), which are points.

### Birkhoff and transportation polytopes

For \(B_3\), we take nine nonnegative variables, one per matrix entry,
bounded above by one. Three row sums and three column sums equal one:

```sh
ehrcalc polytope --input examples/constraints/birkhoff3.json --format json
```

We obtain dimension 4 and
\[
  L(N)=1+\tfrac94N+\tfrac{15}8N^2+\tfrac34N^3+\tfrac18N^4,
  \qquad h^*(t)=1+t+t^2.
\]
The regression suite also checks \(B_4\), of dimension 9, against every
coefficient of its published Ehrhart polynomial. We use
[M. Beck and D. Pixton's coefficient table](https://matthbeck.github.io/birkhoff/polynomials.html),
accompanying their paper *The Ehrhart polynomial of the Birkhoff polytope*,
Discrete & Computational Geometry 30 (2003), 623–637.
Changing row and column totals gives transportation polytopes; the small
fixtures compare with enumeration of the original integer box.

### A flagged Kostka fibre in both forms

We take \(\lambda=(4,3,1)\), empty inner shape, weight \((2,2,2,2)\),
per-label upper row flags \((2,3,3,3)\), and lower row flags \((1,1,1,2)\).
Thus label 4 appears only in rows 2 and 3. Produce both constraint systems
and compute their Ehrhart data:

```sh
ehrcalc kostka --lambda 4,3,1 --weight 2,2,2,2 \
  --upper-flags 2,3,3,3 --lower-flags 1,1,1,2 \
  --emit-constraints > full.json
ehrcalc kostka --lambda 4,3,1 --weight 2,2,2,2 \
  --upper-flags 2,3,3,3 --lower-flags 1,1,1,2 \
  --emit-constraints --reduced > reduced.json
ehrcalc polytope --input full.json --format json
ehrcalc polytope --input reduced.json --format json
ehrcalc gt --lambda 4,3,1 --weight 2,2,2,2 \
  --upper-flags 2,3,3,3 --lower-flags 1,1,1,2 --format json
```

Both forms have dimension 2, \(L(N)=(N+1)^2\), and h* vector
`["1","1","0"]`. The checked-in examples are
[`flagged-full.json`](../examples/constraints/flagged-full.json) and
[`flagged-reduced.json`](../examples/constraints/flagged-reduced.json).

The full form includes the boundary levels and uses the rectangular chain
\(\mu=\alpha^0\subseteq\alpha^1\subseteq\dotsb\subseteq\alpha^k=\lambda\).
Entry \(\alpha^i_r\) is the length of row \(r\) after labels through \(i\).
Interlacing gives \(\alpha^i_r\leq\alpha^{i+1}_r\) and
\(\alpha^{i+1}_{r+1}\leq\alpha^i_r\); level sums are
\(|\mu|+w_1+\dotsb+w_i\). If label \(i+1\) is forbidden in row \(r\),
we also impose \(\alpha^{i+1}_r=\alpha^i_r\).

We propagate these order relations to expose frozen entries. The reduced
form deletes those entries, turns incident inequalities into bounds, and
subtracts their values from level sums. We repeat when a singleton sum fixes
another entry. Equalities between nonfixed entries
remain as two differences. Deleting fixed integral entries is an affine
lattice bijection compatible with dilation and relative interior, so both
forms have the same counts, affine dimension, and Ehrhart data.

Both converters use the existing partition normalization and one-based
per-label flag conventions, including zero-weight labels and skew `--mu`.
`ehrcalc order --chain 4 --emit-constraints` similarly emits unit bounds and
cover differences. Converter output is always constraint JSON.

## Library and MCP contracts

```text
families::gt_constraint_polytope(&GtInput) -> ExactResult<ConstraintPolytopeInput>
families::gt_free_constraint_polytope(&GtInput) -> ExactResult<ConstraintPolytopeInput>
families::order_constraint_polytope(&OrderInput) -> ExactResult<ConstraintPolytopeInput>
families::constraint_polytope_ehrhart(&ConstraintPolytopeInput, &ConstraintOptions)
    -> ExactResult<ConstraintComputation>
render::constraint_polytope_json(&ConstraintComputation) -> serde_json::Value
```

`ehrcalc.constraint-polytope-result.v1` records `dimension` separately from
polynomial and h* degrees; `implicit_equalities`; `free_variables` and
`fixed_values`; component dimensions; `samples`; `plan`; CRT data per sample;
`ehrhart_power`, `hstar`, `binomial_basis`; `mismatches`; and `warnings`.
Exact counts, coefficients and estimates are strings. Sample `dilation` is
the signed interpolation argument; `kind` is `ordinary` or `interior`;
`count` is the unsigned lattice count and `value` includes the reciprocity
sign. `interpolation: false` marks an extra check. The plan includes chosen
orders, candidate estimates and the maximum actual peak across samples
(`null` before counting). Each sample also contains its actual plans and
layer sizes.

The implicit-equality mask indexes normalized inequalities **after fixed
substitution**, in `PreparedConstraintPolytope::rows()` order: explicit lower
then upper bounds, differences, and lower then upper subset sums. Constant
rows remain. The `free_variables` mapping identifies the original coordinates
of reduced row terms. Plan-only results have no samples or polynomial data.

The MCP tool takes `input` as the constraint object plus optional
`positive_only`, `extra_samples`, `order` (an index array), `order_heuristic`,
`max_states`, `exact`, and `plan_only`. Defaults match the CLI. For example:

```json
{"input":{"variables":1,"bounds":[[0,2]]},"extra_samples":1}
```

Related Rust: `src/families/gt_constraints.rs`,
`families::constraint_polytope_ehrhart`,
`ehrcalc-kostka-engine::constraint_polytope`, and
`tests/constraint_{gt,fixtures,cli}.rs`.

Signed-coefficient sums for flow models are reserved for phase G.
