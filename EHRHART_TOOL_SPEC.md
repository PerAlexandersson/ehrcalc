# Ehrcalc Specification

Date: 2026-07-19

Status: v0.1 implementation and continuing requirements.

Owner: unassigned.  This spec is intended for a dedicated Rust tooling worker
or the human maintainer.  It should not be treated as part of any single
research-paper project.

## Purpose

Build a `polytool`-style unified tool for Ehrhart computations across the
workspace.  The tool should consolidate the current scattered code for:

- Ehrhart polynomials;
- h*-vectors;
- binomial-basis transforms;
- finite differences and discrete summation;
- Gelfand-Tsetlin and Kostka computations;
- key-polynomial Ehrhart computations;
- order-polytope Ehrhart computations;
- research diagnostics that feed real-rootedness and interlacing checks.

The immediate client project is:

```text
/workspace/projects/Key-HStar-Bruhat-Interlacing
```

but the tool should be general enough to serve future Ehrhart, h*, GT,
Kostka, key-polynomial, order-polytope, and lattice-point projects.

## Design Philosophy

Follow the `polytool` model:

- one exact library implementation for each core operation;
- thin CLI wrappers around library APIs;
- exact arithmetic as the default;
- JSON output for machine use;
- human-readable output for exploration;
- small reproducible fixtures;
- clear failure reporting with first counterexamples;
- no floating-only mathematical conclusions.

The tool should be useful both as:

```text
cargo run -p ehrcalc -- ...
```

and as a library crate used by experiment binaries.

## Non-Goals

Do not begin by rewriting all existing Ehrhart-related code.

Do not move every experiment into the new tool immediately.  The first version
should expose stable core transformations and reproduce a small number of
important fixture suites.

Do not use this tool as a replacement for `polytool` real-rootedness and
interlacing checks.  Instead, make it easy to pipe h*, B, D, and packet
polynomials into `polytool`.

Do not make every API fully generic over coefficient types unless the generic
version clearly improves reuse.  Prefer one exact primary implementation plus
checked ergonomic wrappers.

## Existing Code To Inventory

Start by reading and inventorying:

```text
/workspace/rust/kostka/src/ehrhart.rs
/workspace/rust/kostka/src/table.rs
/workspace/rust/kostka/src/kostka_dp.rs
/workspace/rust/combinatoric-core/src/key_polynomial.rs
/workspace/rust/combinatoric-core/src/poset.rs
/workspace/rust/polytool/src/real_rootedness.rs
/workspace/rust/experiments/src/bin/key_hstar_interlacing.rs
/workspace/rust/experiments/src/bin/key_hstar_staircase.rs
/workspace/rust/experiments/src/bin/key_hstar_bruhat_d_route.rs
```

Relevant existing APIs include:

```text
kostka::ehrhart::compute_ehrhart
kostka::ehrhart::compute_hstar
combinatoric_core::key_polynomial::key_ehrhart_polynomial
combinatoric_core::poset::Poset::order_polytope_ehrhart
combinatoric_core::poset::Poset::order_polytope_hstar
polytool::real_rootedness::ehrhart_to_hstar
polytool::real_rootedness::check_weak_interlacing
```

The inventory should answer:

- which implementation is fastest for each family;
- which implementation is most general;
- which implementation is easiest to expose as a stable API;
- which duplicate conversions should be consolidated;
- which experiment binaries should become fixtures rather than permanent code.

## Proposed Workspace Shape

Current workspace shape:

```text
/workspace/rust/ehrcalc
```

with:

```text
ehrcalc/src/lib.rs
ehrcalc/src/main.rs
ehrcalc/src/cli.rs
ehrcalc/src/exact.rs
ehrcalc/src/families.rs
ehrcalc/src/render.rs
ehrcalc/mcp/Cargo.toml
ehrcalc/mcp/src/lib.rs
ehrcalc/mcp/src/main.rs
ehrcalc/docs/
```

Ehrcalc is a standalone public package.  It uses `polytool` for future
real-rootedness and interlacing checks rather than absorbing those algorithms.
The GT/Kostka/LR/flow and key/order implementations have been migrated into
the internal `ehrcalc-kostka-engine` and `ehrcalc-foundations` crates.  The
legacy `kostka` repository is now a frozen compatibility/reference source,
while the broader `combinatoric-core` project remains separate.

## Core Data Types

The library should have a canonical exact representation for an Ehrhart
polynomial.

Suggested types:

```rust
pub struct EhrhartPolynomial {
    pub power_coeffs: Vec<BigRational>,
}

pub struct HStarPolynomial {
    pub coeffs: Vec<BigInt>,
}

pub struct BinomialBasisPolynomial {
    pub coeffs: Vec<BigInt>,
}
```

Important operations:

```text
EhrhartPolynomial::degree
EhrhartPolynomial::evaluate(k)
EhrhartPolynomial::to_hstar(dimension)
EhrhartPolynomial::to_binomial_basis()
EhrhartPolynomial::from_values(values)
EhrhartPolynomial::from_binomial_basis(coeffs)
BinomialBasisPolynomial::to_ehrhart()
BinomialBasisPolynomial::finite_difference()
BinomialBasisPolynomial::discrete_sum()
HStarPolynomial::to_ehrhart(dimension)
```

The tool should distinguish:

- actual polynomial degree;
- ambient or declared Ehrhart dimension;
- trimmed h*-degree;
- roots at infinity caused by degree drops.

This distinction matters for the Key-HStar-Bruhat `B` and `D` transforms.

## Core Transform Requirements

Implement and test exact transformations:

1. Ehrhart power basis to h*:

```text
P(k) -> H(t)
```

2. h* to Ehrhart:

```text
H(t), dimension d -> P(k)
```

3. Ehrhart power basis to binomial basis:

```text
P(k)=sum_j b_j binom(k,j)
```

4. Binomial basis to Ehrhart power basis.

5. Finite difference:

```text
Delta P(k)=P(k+1)-P(k)
```

6. Discrete summation:

```text
S(k)=sum_{r=0}^{k-1}P(r)
```

7. Dilation and shift:

```text
P(ak+b)
```

8. Product and scalar operations on exact polynomials.

9. Parsing and formatting:

```text
power basis
binomial basis
h* vector
polytool coefficient vector
JSON
```

## CLI Requirements

The CLI should be friendly to shell pipelines and other agents.

Suggested subcommands:

```text
ehrcalc interpolate
ehrcalc hstar
ehrcalc binomial
ehrcalc delta
ehrcalc sum
ehrcalc eval
ehrcalc key
ehrcalc gt
ehrcalc order
ehrcalc verify
```

Every subcommand should support:

```text
--format text|json
--quiet
--first-failure
```

Commands that may be expensive should support:

```text
--max-n
--limit
--start
--stop-after
--summary-only
```

Do not rely on terminal progress lines as the only output.  For reproducible
work, print structured summaries.

## Key-Polynomial Requirements

The key-polynomial family must support the staircase/gap-one specialization:

```text
rho_n=(n-1,n-2,...,1,0)
```

and more general positive gap vectors:

```text
a=(a_1,...,a_{n-1})
lambda=(a_1+...+a_{n-1},...,a_{n-1},0).
```

Required key commands:

```text
ehrcalc key row --n 5 --sigma 24531 --staircase
ehrcalc key scan --max-n 5 --staircase --format json
ehrcalc key covers --max-n 5 --staircase --packets d-route
```

The key row output should include:

```text
sigma
Bruhat length
Ehrhart polynomial
h* vector
B_sigma
D_sigma
lower covers with labels and weights
```

The `--packets d-route` mode should include:

```text
G_sigma
W_sigma
L_{tau,sigma}
K_{tau,sigma}
bar K_{tau,sigma}
```

where:

```text
B_sigma(u)=sum_j b_j(sigma)u^j
D_id(u)=1
D_sigma(u)=B_sigma(u)/(1+u) for sigma != id
W_sigma(u)=sum_{tau <dot sigma}m(tau,sigma)D_tau(u)
D_sigma(u)=G_sigma(u)+uW_sigma(u)
L_{tau,sigma}(u)=(G_sigma(u)-D_tau(u))/u
bar K_{tau,sigma}(u)
  =(D_sigma(u)-(1+m(tau,sigma)u)D_tau(u))/u
```

## Key-HStar-Bruhat Fixture Suite

The tool must reproduce the current project evidence through `S_5`.

Reference diagnostic:

```bash
cd /workspace/rust/experiments
timeout 60s nice -n 10 cargo run --release --quiet --bin key_hstar_bruhat_d_route 5
```

Required fixture counts:

```text
S_3:
  H coefficientwise nonnegative: 6/6
  D_sigma real-rooted: 6/6
  D_tau << D_sigma for Bruhat covers: 8/8
  W_sigma << G_sigma: 3/3
  W_sigma-mD_tau << G_sigma: 6/6
  L_{tau,sigma} << D_tau, all covers: 6/6
  each upper has a good L lower cover: 3/3
  max-weight L_{tau,sigma} << D_tau: 4/4
  max-weight L_{tau,sigma} << G_sigma: 4/4
  max-weight L_{tau,sigma} << W_sigma: 4/4
  bar K coefficientwise nonnegative: 6/6
  bar K_{tau,sigma} << D_tau: 6/6
  bar K_{tau,sigma} << D_alpha: 12/12
  bar K_{tau,sigma} << G_sigma: 6/6

S_4:
  H coefficientwise nonnegative: 24/24
  D_sigma real-rooted: 24/24
  D_tau << D_sigma for Bruhat covers: 58/58
  W_sigma << G_sigma: 20/20
  W_sigma-mD_tau << G_sigma: 55/55
  L_{tau,sigma} << D_tau, all covers: 55/55
  each upper has a good L lower cover: 20/20
  max-weight L_{tau,sigma} << D_tau: 27/27
  max-weight L_{tau,sigma} << G_sigma: 27/27
  max-weight L_{tau,sigma} << W_sigma: 27/27
  bar K coefficientwise nonnegative: 55/55
  bar K_{tau,sigma} << D_tau: 55/55
  bar K_{tau,sigma} << D_alpha: 159/159
  bar K_{tau,sigma} << G_sigma: 55/55

S_5:
  H coefficientwise nonnegative: 120/120
  D_sigma real-rooted: 120/120
  D_tau << D_sigma for Bruhat covers: 444/444
  W_sigma << G_sigma: 115/115
  W_sigma-mD_tau << G_sigma: 440/440
  L_{tau,sigma} << D_tau, all covers: 424/440
  each upper has a good L lower cover: 115/115
  max-weight L_{tau,sigma} << D_tau: 154/154
  max-weight L_{tau,sigma} << G_sigma: 154/154
  max-weight L_{tau,sigma} << W_sigma: 154/154
  bar K coefficientwise nonnegative: 440/440
  bar K_{tau,sigma} << D_tau: 440/440
  bar K_{tau,sigma} << D_alpha: 1798/1798
  bar K_{tau,sigma} << G_sigma: 440/440
```

Required first failure:

```text
24513 <dot 24531, label (4,5), m=1:
  L=50+558u+1318u^2+839u^3
  D_tau=1+89u+649u^2+1295u^3+755u^4
  G=1+139u+1207u^2+2613u^3+1594u^4
```

The fixture suite should also reproduce:

```text
staircase S_3: Bruhat covers 8/8 interlace
staircase S_4: Bruhat covers 58/58 interlace
staircase S_5: Bruhat covers 444/444 interlace
```

and the non-staircase guardrail:

```text
lambda=(3,1,0), rank 3:
  312 covers 132
  h*(312)=1+6t+t^2
  h*(132)=1
  cover interlacing fails
```

## Formula-Search Requirements

The tool should expose the finite-difference objects used in the
Key-HStar-Bruhat notes.

For each cover `tau <dot sigma`, weight `m=m(tau,sigma)`, define:

```text
U_{tau,sigma}(u)=(B_sigma(u)-B_tau(u))/u
K_{tau,sigma}(u)=U_{tau,sigma}(u)-mB_tau(u)
```

At the Ehrhart level, if `Delta F(k)=F(k+1)-F(k)`, then `K` is the binomial
transform of:

```text
Q_{tau,sigma}(k)
=Delta(P_sigma(k rho_n)-P_tau(k rho_n))
 -m(tau,sigma)P_tau(k rho_n).
```

Equivalently:

```text
P_sigma(k rho_n)-P_tau(k rho_n)
=sum_{r=0}^{k-1}(mP_tau(r rho_n)+Q_{tau,sigma}(r)).
```

The tool should be able to print `Q_{tau,sigma}` in power and binomial bases.

For each upper `sigma`, define the primitive residual:

```text
B_sigma(u)=1+u sum_{tau <dot sigma}m(tau,sigma)B_tau(u)+E_sigma(u).
```

At the Ehrhart level this is:

```text
P^prim_sigma(k)
=P_sigma(k rho_n)-1
 -sum_{tau <dot sigma}m(tau,sigma)sum_{r=0}^{k-1}P_tau(r rho_n).
```

The tool should print `E_sigma` and `P^prim_sigma` in both power and binomial
bases.

## Interoperation With Polytool

The tool should not duplicate `polytool` real-rootedness internals.  Instead:

- expose coefficient vectors in `polytool`-friendly order;
- optionally call `polytool` library functions when in the same workspace;
- provide `--emit-polytool` or JSON fields that downstream scripts can feed
  to `polytool interlacing --json`.

For Key-HStar-Bruhat, the tool should be able to emit batches for:

```text
D_tau, D_sigma
bar K_{tau,sigma}, D_tau
bar K_{tau,sigma}, D_alpha
bar K_{tau,sigma}, G_sigma
W_sigma, G_sigma
L_{tau,sigma}, D_tau
```

Remember that the current `polytool interlacing --json` CLI reports sliding
consecutive pairs.  If batching intended pairs `p1,q1,p2,q2,...`, consumers
must select even-indexed output pairs unless the CLI is extended.

## Performance Requirements

Initial targets:

```text
key staircase S_5 d-route scan: under 10 seconds in release mode
key staircase S_6 partial/resumable scan: no lost progress at 60-second timeout
standard hstar/binomial transforms: negligible compared to family evaluator
```

Longer-term target:

```text
key staircase S_6 complete scan should be feasible with the best available
Ehrhart engine, or should produce a clear report explaining the bottleneck.
```

Every expensive scan should support resumability:

```text
--start-index
--limit
--checkpoint file.jsonl
--resume file.jsonl
```

## JSON Output Requirements

JSON rows should be stable enough for fixtures.  Suggested row shape:

```json
{
  "family": "key",
  "specialization": "staircase",
  "n": 5,
  "sigma": "24531",
  "length": 7,
  "ehrhart_power": ["1", "139/6", "..."],
  "hstar": [1, 138, "..."],
  "B": [1, 139, "..."],
  "D": [1, 138, "..."],
  "lower_covers": [
    {
      "tau": "24513",
      "label": [4, 5],
      "weight": 1,
      "barK": [50, 558, 1318, 839],
      "L": [50, 558, 1318, 839]
    }
  ]
}
```

Use strings for large rationals and integers if there is any risk of JSON
number overflow.

## Testing Requirements

Minimum test layers:

1. Unit tests for transformations:
   - power to h*;
   - h* to power;
   - power to binomial;
   - binomial to power;
   - finite difference;
   - discrete summation.

2. Family smoke tests:
   - order polytopes with known h* vectors;
   - GT/Kostka examples already tested in `kostka`;
   - key-polynomial examples from `verify_key_ehrhart`.

3. Key-HStar-Bruhat fixtures:
   - exact `S_3/S_4` rows from `notes/SMALL_RANK_TABLES.md`;
   - `S_5` summary counts from `key_hstar_bruhat_d_route`;
   - non-staircase rank-three cover failure.

4. CLI tests:
   - text output contains expected summaries;
   - JSON output parses and has stable field names;
   - first-failure output is deterministic.

## Migration Plan

Phase 1:

- create the crate or module;
- implement exact transformations;
- add CLI for h*, binomial basis, delta, and summation;
- add unit tests.

Phase 2:

- migrate the required `kostka` Ehrhart APIs into `ehrcalc-kostka-engine`;
- migrate the required `combinatoric-core` key and order APIs into
  `ehrcalc-foundations`;
- add key row and key scan commands;
- reproduce `key_hstar_interlacing` and `key_hstar_staircase` through `S_4`.

Phase 3:

- reproduce `key_hstar_bruhat_d_route 5`;
- replace the bespoke evaluator if the unified engine is faster and exact;
- add resumable `S_6` scans.

Phase 4:

- decide which experiment binaries become obsolete;
- move durable fixture logic into tests;
- keep paper-project notes pointing at the new tool commands.

## Open Design Questions

1. When, if ever, should neutral exact-transform code move into a separate
   `ehrcalc-core` crate shared with compatibility packages?

2. Which existing key Ehrhart engine is fastest for staircase `S_6`:
   `combinatoric_core::key_polynomial::key_ehrhart_polynomial`, the bespoke
   Demazure evaluator, or a `kostka`/GT route?

3. Should interlacing checks be library calls into `polytool`, or external CLI
   calls for looser coupling?

4. How much of the GT/Kostka API should be re-exported versus wrapped?

5. What is the right long-term representation for integer-valued polynomials:
   always power basis plus on-demand binomial basis, or a basis-tagged enum?

## Immediate Next Action

Before implementing broad infrastructure, run a small benchmark comparing the
three available ways to compute key h*-data:

```text
1. bespoke Demazure evaluator from key_hstar_staircase.rs;
2. combinatoric_core::key_polynomial::key_ehrhart_polynomial;
3. any applicable kostka/GT route.
```

Benchmark fixture:

```text
staircase S_4 all rows
staircase S_5 all rows
selected hard S_6 rows near and after index 340 in lexicographic permutation order
```

Record:

```text
correctness against known h* vectors,
time per row,
maximum memory if easy to measure,
whether the method gives Ehrhart polynomial directly,
whether it gives enough data for B/D/G/W/barK packets.
```

Only after this benchmark should the implementation choose the default key
Ehrhart backend.
