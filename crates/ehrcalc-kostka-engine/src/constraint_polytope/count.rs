use super::plan::{
    choose_order, validate_order, ConstraintOptions, CountingPlan, OrderCandidate, OrderPilot,
};
use super::{ConstraintError, PreparedConstraintPolytope, Result};
use crate::packed_modular::{crt_reconstruct_bounded, DEFAULT_MODULI, MAX_CRT_MODULI};
use hashbrown::HashMap;
use num::Integer;
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};
use smallvec::SmallVec;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

/// CRT uniqueness evidence for one connected factor at one dilation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModularCertificate {
    pub upper_bound: BigUint,
    pub bound_bits: u64,
    pub moduli: Vec<u32>,
    pub residues: Vec<u32>,
    pub combined_modulus: BigUint,
}

/// Exact count with the chosen orders, actual layer sizes and optional CRT evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstraintCount {
    pub value: BigUint,
    pub peak_states: usize,
    /// Component layers concatenated in component order, excluding initial layers.
    pub layer_states: Vec<usize>,
    pub plans: Vec<CountingPlan>,
    pub modular: Vec<ModularCertificate>,
}

#[derive(Clone, Debug)]
pub(super) struct IntegerRange {
    pub terms: Vec<(usize, i8)>,
    pub lower: Option<BigInt>,
    pub upper: Option<BigInt>,
}

#[derive(Clone, Debug)]
pub(super) struct IntegerProblem {
    pub widths: Vec<BigInt>,
    pub differences: Vec<IntegerRange>,
    pub sums: Vec<IntegerRange>,
    pub empty: bool,
}

#[derive(Clone, Debug)]
pub(super) struct PilotProblem {
    pub dilation: u64,
    pub problem: IntegerProblem,
}

pub(super) fn floor(q: &BigRational) -> BigInt {
    q.numer().div_floor(q.denom())
}
pub(super) fn ceil(q: &BigRational) -> BigInt {
    q.numer().div_ceil(q.denom())
}

impl PreparedConstraintPolytope {
    pub(super) fn integer_problem(&self, dilation: u64, strict: bool) -> IntegerProblem {
        let n = self.free_variables.len();
        let mut problem = IntegerProblem {
            widths: vec![BigInt::zero(); n],
            differences: Vec::new(),
            sums: Vec::new(),
            empty: self.hull.is_none(),
        };
        if problem.empty {
            return problem;
        }
        let factor = BigInt::from(dilation);
        if self
            .fixed_values
            .iter()
            .flatten()
            .any(|q| !(q * &factor).is_integer())
        {
            problem.empty = true;
            return problem;
        }
        let mut lo = self
            .bounds
            .iter()
            .map(|b| ceil(&(b.lower.as_ref().expect("prepared lower bound") * &factor)))
            .collect::<Vec<_>>();
        let mut hi = self
            .bounds
            .iter()
            .map(|b| floor(&(b.upper.as_ref().expect("prepared upper bound") * &factor)))
            .collect::<Vec<_>>();
        let mut groups = BTreeMap::<Vec<(usize, i8)>, IntegerRange>::new();
        let hull = self.hull.as_ref().expect("nonempty hull");
        for (row, implicit) in self.rows.iter().zip(&hull.implicit_equalities) {
            let rhs = &row.rhs * &factor;
            if *implicit && !rhs.is_integer() {
                problem.empty = true;
                return problem;
            }
            let endpoint = if strict && !implicit {
                ceil(&rhs) - 1
            } else {
                floor(&rhs)
            };
            let Some(&(_, first_sign)) = row.terms.first() else {
                if endpoint < BigInt::zero() {
                    problem.empty = true;
                }
                continue;
            };
            let terms = row
                .terms
                .iter()
                .map(|&(v, sign)| (v, sign * first_sign))
                .collect::<Vec<_>>();
            let group = groups.entry(terms.clone()).or_insert(IntegerRange {
                terms,
                lower: None,
                upper: None,
            });
            if first_sign > 0 {
                if *implicit && group.lower.as_ref().is_none_or(|old| &endpoint > old) {
                    group.lower = Some(endpoint.clone());
                }
                if group.upper.as_ref().is_none_or(|old| &endpoint < old) {
                    group.upper = Some(endpoint);
                }
            } else {
                let endpoint = -endpoint;
                if *implicit && group.upper.as_ref().is_none_or(|old| &endpoint < old) {
                    group.upper = Some(endpoint.clone());
                }
                if group.lower.as_ref().is_none_or(|old| &endpoint > old) {
                    group.lower = Some(endpoint);
                }
            }
        }
        let mut ranges = groups.into_values().collect::<Vec<_>>();
        // Integer bounds consistency sharpens both strict and ordinary boxes.
        for _ in 0..n.saturating_mul(4).saturating_add(16) {
            let mut changed = false;
            for range in &ranges {
                for &(v, sign) in &range.terms {
                    let mut min_other = BigInt::zero();
                    let mut max_other = BigInt::zero();
                    for &(w, s) in &range.terms {
                        if w == v {
                            continue;
                        }
                        if s > 0 {
                            min_other += &lo[w];
                            max_other += &hi[w];
                        } else {
                            min_other -= &hi[w];
                            max_other -= &lo[w];
                        }
                    }
                    let mut a = lo[v].clone();
                    let mut b = hi[v].clone();
                    if let Some(upper) = &range.upper {
                        if sign > 0 {
                            b = b.min(upper - &min_other);
                        } else {
                            a = a.max(&min_other - upper);
                        }
                    }
                    if let Some(lower) = &range.lower {
                        if sign > 0 {
                            a = a.max(lower - &max_other);
                        } else {
                            b = b.min(&max_other - lower);
                        }
                    }
                    changed |= a != lo[v] || b != hi[v];
                    lo[v] = a;
                    hi[v] = b;
                }
            }
            if lo.iter().zip(&hi).any(|(a, b)| a > b) {
                problem.empty = true;
                return problem;
            }
            if !changed {
                break;
            }
        }
        if lo.iter().zip(&hi).any(|(a, b)| a > b) {
            problem.empty = true;
            return problem;
        }
        problem.widths = lo.iter().zip(&hi).map(|(a, b)| b - a).collect();
        for range in &mut ranges {
            let offset = range.terms.iter().fold(BigInt::zero(), |sum, &(v, s)| {
                sum + &lo[v] * BigInt::from(s)
            });
            if let Some(lower) = &mut range.lower {
                *lower -= &offset;
            }
            if let Some(upper) = &mut range.upper {
                *upper -= &offset;
            }
        }
        for range in ranges {
            if range.terms.len() <= 1 {
                continue;
            }
            if range.terms.iter().all(|&(_, s)| s > 0) {
                problem.sums.push(range);
            } else {
                problem.differences.push(range);
            }
        }
        problem
    }

    fn pilot_problems(
        &self,
        dilation: u64,
        strict: bool,
        empty: bool,
        options: &ConstraintOptions,
    ) -> Vec<PilotProblem> {
        if options.order.is_some()
            || options.order_heuristic != super::OrderHeuristic::Auto
            || options.plan_only
        {
            return Vec::new();
        }
        let mut probes = [1, 2]
            .into_iter()
            .map(|n| PilotProblem {
                dilation: n,
                problem: self.integer_problem(n, strict),
            })
            .collect::<Vec<_>>();
        if strict && !empty {
            // Interval feasibility is a cheap screen. Enumeration remains bounded
            // and candidates stop after at most two additional strict probes.
            // Cap screening too: public count/plan APIs accept arbitrary u64 scales.
            for n in 3..=dilation.min(256) {
                let problem = self.integer_problem(n, true);
                if !problem.empty {
                    probes.push(PilotProblem {
                        dilation: n,
                        problem,
                    });
                }
                if probes.len() == 4 {
                    break;
                }
            }
        }
        probes
    }

    /// Plan one ordinary or relative-interior count, including bounded auto pilots.
    /// `plan_only` disables pilots and uses estimates alone.
    /// Orders in options use original input indices, including fixed variables.
    pub fn plan(
        &self,
        dilation: u64,
        strict: bool,
        options: &ConstraintOptions,
    ) -> Result<Vec<CountingPlan>> {
        validate_order(self.input.variables, options)?;
        let problem = self.integer_problem(dilation, strict);
        let pilot_problems = self.pilot_problems(dilation, strict, problem.empty, options);
        self.components
            .iter()
            .map(|component| {
                let local = problem.component(component);
                let original = component
                    .iter()
                    .map(|&v| self.free_variables[v])
                    .collect::<Vec<_>>();
                let pilots = pilot_problems
                    .iter()
                    .map(|p| PilotProblem {
                        dilation: p.dilation,
                        problem: p.problem.component(component),
                    })
                    .collect::<Vec<_>>();
                choose_order(
                    &local,
                    &original,
                    &self.input.names,
                    &pilots,
                    dilation,
                    strict,
                    options,
                )
            })
            .collect()
    }

    /// Count a nonnegative dilation exactly, multiplying independent connected factors.
    /// `L(0)=1` for a nonempty polytope; strict counts require positive dilation.
    /// `plan_only` is honored by the Ehrhart service; this explicit counting API counts.
    pub fn count(
        &self,
        dilation: u64,
        strict: bool,
        options: &ConstraintOptions,
    ) -> Result<ConstraintCount> {
        validate_order(self.input.variables, options)?;
        if strict && dilation == 0 {
            return Err(ConstraintError::InvalidInput(
                "strict counts require N >= 1".into(),
            ));
        }
        let mut result = ConstraintCount {
            value: BigUint::one(),
            peak_states: 0,
            layer_states: Vec::new(),
            plans: Vec::new(),
            modular: Vec::new(),
        };
        if self.hull.is_none() {
            result.value = BigUint::zero();
            return Ok(result);
        }
        if dilation == 0 {
            return Ok(result);
        }
        let problem = self.integer_problem(dilation, strict);
        let pilot_problems = self.pilot_problems(dilation, strict, problem.empty, options);
        if problem.empty {
            result.value = BigUint::zero();
            return Ok(result);
        }
        for component in &self.components {
            let local = problem.component(component);
            let original = component
                .iter()
                .map(|&v| self.free_variables[v])
                .collect::<Vec<_>>();
            let pilots = pilot_problems
                .iter()
                .map(|p| PilotProblem {
                    dilation: p.dilation,
                    problem: p.problem.component(component),
                })
                .collect::<Vec<_>>();
            let plan = choose_order(
                &local,
                &original,
                &self.input.names,
                &pilots,
                dilation,
                strict,
                options,
            )?;
            let order = plan
                .order
                .iter()
                .map(|v| {
                    original
                        .iter()
                        .position(|w| w == v)
                        .expect("order is a permutation")
                })
                .collect::<Vec<_>>();
            let request = FrontierRequest {
                order: &order,
                reported_order: &plan.order,
                estimated_peak: &plan.estimated_peak,
                max_states: options.max_states,
            };
            let (value, layers, certificate) = if options.exact {
                let (value, layers) =
                    frontier::<BigUint>(&local, &request, &[], &mut FrontierMonitor::default())?;
                (value, layers, None)
            } else {
                let upper_bound = local.count_bound();
                let moduli = select_moduli(&upper_bound)?;
                let (residues, layers) = frontier::<Vec<u32>>(
                    &local,
                    &request,
                    &moduli,
                    &mut FrontierMonitor::default(),
                )?;
                let value = crt_reconstruct_bounded(&residues, &moduli, &upper_bound)
                    .map_err(ConstraintError::Arithmetic)?;
                let certificate = ModularCertificate {
                    bound_bits: upper_bound.bits(),
                    upper_bound,
                    combined_modulus: moduli.iter().fold(BigUint::one(), |p, &m| p * m),
                    moduli,
                    residues,
                };
                (value, layers, Some(certificate))
            };
            result.value *= value;
            result.peak_states = result
                .peak_states
                .max(layers.iter().copied().max().unwrap_or(1));
            result.layer_states.extend(layers);
            result.plans.push(plan);
            if let Some(certificate) = certificate {
                result.modular.push(certificate);
            }
        }
        Ok(result)
    }
}

impl IntegerProblem {
    pub(super) fn component(&self, component: &[usize]) -> Self {
        let remap = |ranges: &[IntegerRange]| {
            ranges
                .iter()
                .filter(|r| component.contains(&r.terms[0].0))
                .map(|r| IntegerRange {
                    terms: r
                        .terms
                        .iter()
                        .map(|&(v, s)| {
                            (
                                component
                                    .iter()
                                    .position(|&w| w == v)
                                    .expect("constraint stays in its component"),
                                s,
                            )
                        })
                        .collect(),
                    lower: r.lower.clone(),
                    upper: r.upper.clone(),
                })
                .collect()
        };
        Self {
            widths: component.iter().map(|&v| self.widths[v].clone()).collect(),
            differences: remap(&self.differences),
            sums: remap(&self.sums),
            empty: self.empty,
        }
    }

    fn count_bound(&self) -> BigUint {
        if self.empty {
            return BigUint::zero();
        }
        let n = self.widths.len();
        let mut equations = self
            .sums
            .iter()
            .chain(&self.differences)
            .filter(|r| r.lower.is_some() && r.lower == r.upper)
            .map(|r| {
                (0..n)
                    .map(|v| {
                        BigRational::from_integer(
                            r.terms
                                .iter()
                                .find(|&&(w, _)| w == v)
                                .map_or(0_i8, |&(_, s)| s)
                                .into(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut columns = (0..n).collect::<Vec<_>>();
        columns.sort_by(|&a, &b| self.widths[b].cmp(&self.widths[a]).then(a.cmp(&b)));
        let mut pivots = vec![false; n];
        let mut rank = 0;
        for col in columns {
            let Some(row) = (rank..equations.len()).find(|&r| !equations[r][col].is_zero()) else {
                continue;
            };
            equations.swap(rank, row);
            let pivot = equations[rank][col].clone();
            for entry in &mut equations[rank] {
                *entry /= &pivot;
            }
            let (pivot_rows, other_rows) = equations.split_at_mut(rank + 1);
            for other in other_rows {
                let factor = other[col].clone();
                for (entry, pivot_entry) in other.iter_mut().zip(&pivot_rows[rank]) {
                    *entry -= &factor * pivot_entry;
                }
            }
            pivots[col] = true;
            rank += 1;
        }
        self.widths
            .iter()
            .enumerate()
            .filter(|(v, _)| !pivots[*v])
            .fold(BigUint::one(), |p, (_, w)| {
                p * (w + BigInt::one())
                    .to_biguint()
                    .expect("nonnegative presolved width")
            })
    }
}

fn select_moduli(bound: &BigUint) -> Result<Vec<u32>> {
    let mut moduli = Vec::new();
    let mut product = BigUint::one();
    for &prime in &DEFAULT_MODULI {
        moduli.push(prime);
        product *= prime;
        if product > *bound {
            return Ok(moduli);
        }
    }
    // Extend the maintained table deterministically for larger bounds, respecting
    // the maintained CRT routine's lane limit. Trial division is exact here.
    let mut candidate = DEFAULT_MODULI[DEFAULT_MODULI.len() - 1] - 2;
    while moduli.len() < MAX_CRT_MODULI {
        let prime = (3..)
            .step_by(2)
            .take_while(|&d| u64::from(d) * u64::from(d) <= u64::from(candidate))
            .all(|d| !candidate.is_multiple_of(d));
        if prime {
            moduli.push(candidate);
            product *= candidate;
            if product > *bound {
                return Ok(moduli);
            }
        }
        candidate -= 2;
    }
    Err(ConstraintError::ModularCapacity {
        bound_bits: bound.bits(),
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Key {
    Packed(u128),
    Wide(SmallVec<[u32; 8]>),
    Big(Vec<BigInt>),
}

struct Codec {
    bits: Vec<u32>,
    packed: bool,
    word: bool,
}

impl Codec {
    fn new(maxima: &[BigInt]) -> Self {
        let bits = maxima
            .iter()
            .map(|m| u32::try_from(m.bits()).unwrap_or(u32::MAX))
            .collect::<Vec<_>>();
        let packed = bits.iter().map(|&b| u64::from(b)).sum::<u64>() <= 128;
        let word = bits.iter().all(|&b| b <= 32);
        Self { bits, packed, word }
    }

    fn encode(&self, values: &[BigInt]) -> Key {
        if self.packed {
            let mut key = 0_u128;
            let mut offset = 0;
            for (value, &bits) in values.iter().zip(&self.bits) {
                if bits != 0 {
                    key |= value.to_u128().expect("coordinate fits packed range") << offset;
                    offset += bits;
                }
            }
            Key::Packed(key)
        } else if self.word {
            Key::Wide(
                values
                    .iter()
                    .map(|v| v.to_u32().expect("word range"))
                    .collect(),
            )
        } else {
            Key::Big(values.to_vec())
        }
    }

    fn decode(&self, key: &Key) -> Vec<BigInt> {
        match key {
            Key::Packed(key) => {
                let mut offset = 0;
                self.bits
                    .iter()
                    .map(|&bits| {
                        let value = if bits == 0 {
                            0
                        } else if bits == 128 {
                            *key
                        } else {
                            (key >> offset) & ((1_u128 << bits) - 1)
                        };
                        offset += bits;
                        BigInt::from(value)
                    })
                    .collect()
            }
            Key::Wide(values) => values.iter().map(|&v| BigInt::from(v)).collect(),
            Key::Big(values) => values.clone(),
        }
    }
}

pub(super) fn boundary(problem: &IntegerProblem, done: &[bool]) -> (Vec<usize>, Vec<usize>) {
    let mut live = vec![false; done.len()];
    for range in &problem.differences {
        let (a, b) = (range.terms[0].0, range.terms[1].0);
        if done[a] != done[b] {
            live[if done[a] { a } else { b }] = true;
        }
    }
    let live = live
        .iter()
        .enumerate()
        .filter_map(|(v, &yes)| yes.then_some(v))
        .collect();
    let sums = problem
        .sums
        .iter()
        .enumerate()
        .filter_map(|(s, r)| {
            let count = r.terms.iter().filter(|&&(v, _)| done[v]).count();
            (count > 0 && count < r.terms.len()).then_some(s)
        })
        .collect();
    (live, sums)
}

fn maxima(problem: &IntegerProblem, done: &[bool], live: &[usize], sums: &[usize]) -> Vec<BigInt> {
    live.iter()
        .map(|&v| problem.widths[v].clone())
        .chain(sums.iter().map(|&s| {
            let range = &problem.sums[s];
            let sum = range
                .terms
                .iter()
                .filter(|&&(v, _)| done[v])
                .fold(BigInt::zero(), |sum, &(v, _)| sum + &problem.widths[v]);
            range
                .upper
                .as_ref()
                .map_or_else(|| sum.clone(), |u| sum.clone().min(u.clone()))
                .max(BigInt::zero())
        }))
        .collect()
}

pub(super) fn state_estimate(problem: &IntegerProblem, done: &[bool]) -> (BigUint, usize) {
    if problem.empty {
        return (BigUint::zero(), 0);
    }
    let (live, sums) = boundary(problem, done);
    let maxima = maxima(problem, done, &live, &sums);
    let states = maxima.iter().fold(BigUint::one(), |p, m| {
        p * (m + BigInt::one())
            .to_biguint()
            .expect("nonnegative estimate range")
    });
    (states, maxima.len())
}

trait Weight: Clone {
    fn additive_zero(moduli: &[u32]) -> Self;
    fn multiplicative_one(moduli: &[u32]) -> Self;
    fn add_scaled(&mut self, source: &Self, multiplicity: &BigUint, moduli: &[u32]);
}

impl Weight for BigUint {
    fn additive_zero(_: &[u32]) -> Self {
        Zero::zero()
    }
    fn multiplicative_one(_: &[u32]) -> Self {
        One::one()
    }
    fn add_scaled(&mut self, source: &Self, multiplicity: &BigUint, _: &[u32]) {
        *self += source * multiplicity;
    }
}

impl Weight for Vec<u32> {
    fn additive_zero(moduli: &[u32]) -> Self {
        vec![0; moduli.len()]
    }
    fn multiplicative_one(moduli: &[u32]) -> Self {
        vec![1; moduli.len()]
    }
    fn add_scaled(&mut self, source: &Self, multiplicity: &BigUint, moduli: &[u32]) {
        for ((target, &value), &modulus) in self.iter_mut().zip(source).zip(moduli) {
            let multiplier = (multiplicity % modulus).to_u64().expect("word remainder");
            *target =
                ((u64::from(*target) + u64::from(value) * multiplier) % u64::from(modulus)) as u32;
        }
    }
}

struct FrontierRequest<'a> {
    order: &'a [usize],
    reported_order: &'a [usize],
    estimated_peak: &'a BigUint,
    max_states: Option<usize>,
}

#[derive(Default)]
struct FrontierMonitor {
    deadline: Option<Instant>,
    transition_budget: Option<u64>,
    transitions: u64,
    ticks: u64,
    peak: usize,
    layers: Vec<usize>,
    stop: Option<&'static str>,
}

impl FrontierMonitor {
    fn check(&mut self) -> Result<()> {
        self.ticks += 1;
        if self
            .transition_budget
            .is_some_and(|b| self.transitions >= b)
        {
            self.stop = Some("transition-limit");
        } else if self.ticks.is_multiple_of(256)
            && self.deadline.is_some_and(|d| Instant::now() >= d)
        {
            self.stop = Some("time-limit");
        }
        if self.stop.is_some() {
            return Err(ConstraintError::Arithmetic(
                "internal pilot budget exhausted".into(),
            ));
        }
        Ok(())
    }
}

/// Pilots use the production frontier and one modular lane. Residue values do
/// not affect state presence: even a zero residue remains a reachable key.
pub(super) fn pilot(
    problem: &IntegerProblem,
    candidate: &OrderCandidate,
    dilation: u64,
    strict: bool,
    max_states: Option<usize>,
) -> OrderPilot {
    let state_budget = max_states.unwrap_or(4096).min(4096);
    let transition_budget = 100_000;
    let time_budget_micros = 50_000;
    let start = Instant::now();
    let mut monitor = FrontierMonitor {
        deadline: Some(start + Duration::from_micros(time_budget_micros)),
        transition_budget: Some(transition_budget),
        peak: usize::from(!problem.empty),
        ..FrontierMonitor::default()
    };
    let request = FrontierRequest {
        order: &candidate.order,
        reported_order: &candidate.order,
        estimated_peak: &candidate.estimated_peak,
        max_states: Some(state_budget),
    };
    let status = if problem.empty {
        "empty"
    } else {
        match frontier::<Vec<u32>>(problem, &request, &DEFAULT_MODULI[..1], &mut monitor) {
            Ok(_) => "complete",
            Err(ConstraintError::StateLimitExceeded { .. }) => "state-limit",
            Err(_) => monitor.stop.unwrap_or("error"),
        }
    };
    OrderPilot {
        dilation,
        strict,
        status: status.into(),
        peak_states: monitor.peak,
        layer_states: monitor.layers,
        transitions: monitor.transitions,
        elapsed_micros: u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX),
        state_budget,
        transition_budget,
        time_budget_micros,
    }
}

fn frontier<W: Weight>(
    problem: &IntegerProblem,
    request: &FrontierRequest<'_>,
    moduli: &[u32],
    monitor: &mut FrontierMonitor,
) -> Result<(W, Vec<usize>)> {
    let n = problem.widths.len();
    let mut done = vec![false; n];
    let mut before_live = Vec::new();
    let mut before_sums = Vec::new();
    let mut codec = Codec::new(&[]);
    let mut layer = HashMap::new();
    layer.insert(Key::Packed(0), W::multiplicative_one(moduli));
    let mut layers = Vec::new();
    for (step, &v) in request.order.iter().enumerate() {
        done[v] = true;
        let (after_live, after_sums) = boundary(problem, &done);
        let next_codec = Codec::new(&maxima(problem, &done, &after_live, &after_sums));
        let mut next = HashMap::<Key, W>::new();
        for (key, count) in &layer {
            monitor.check()?;
            let values = codec.decode(key);
            let value_of = |w: usize| {
                values[before_live
                    .iter()
                    .position(|&u| u == w)
                    .expect("processed difference endpoint is live")]
                .clone()
            };
            let partial = |s: usize| {
                before_sums
                    .iter()
                    .position(|&t| t == s)
                    .map_or_else(BigInt::zero, |p| values[before_live.len() + p].clone())
            };
            let mut lo = BigInt::zero();
            let mut hi = problem.widths[v].clone();
            for range in &problem.differences {
                let Some(&(_, sign)) = range.terms.iter().find(|&&(w, _)| w == v) else {
                    continue;
                };
                let &(w, s) = range
                    .terms
                    .iter()
                    .find(|&&(w, _)| w != v)
                    .expect("two difference endpoints");
                if !done[w] {
                    continue;
                }
                let other = value_of(w) * BigInt::from(s);
                if let Some(upper) = &range.upper {
                    if sign > 0 {
                        hi = hi.min(upper - &other);
                    } else {
                        lo = lo.max(&other - upper);
                    }
                }
                if let Some(lower) = &range.lower {
                    if sign > 0 {
                        lo = lo.max(lower - &other);
                    } else {
                        hi = hi.min(&other - lower);
                    }
                }
            }
            for (s, range) in problem.sums.iter().enumerate() {
                if !range.terms.iter().any(|&(w, _)| w == v) {
                    continue;
                }
                let p = partial(s);
                let remaining = range
                    .terms
                    .iter()
                    .filter(|&&(w, _)| !done[w])
                    .fold(BigInt::zero(), |sum, &(w, _)| sum + &problem.widths[w]);
                // All translated coordinates are nonnegative, so this also gives
                // the monotone upper fail. Equality sums force the last member.
                if let Some(upper) = &range.upper {
                    hi = hi.min(upper - &p);
                }
                if let Some(lower) = &range.lower {
                    lo = lo.max(lower - &p - remaining);
                }
            }
            if lo > hi {
                continue;
            }
            let aggregated = !after_live.contains(&v)
                && !after_sums
                    .iter()
                    .any(|&s| problem.sums[s].terms.iter().any(|&(w, _)| w == v));
            let multiplicity = if aggregated {
                (&hi - &lo + BigInt::one())
                    .to_biguint()
                    .expect("positive choices")
            } else {
                BigUint::one()
            };
            let mut value = lo;
            while value <= hi {
                monitor.check()?;
                monitor.transitions += 1;
                let coordinates = after_live
                    .iter()
                    .map(|&w| if w == v { value.clone() } else { value_of(w) })
                    .chain(after_sums.iter().map(|&s| {
                        let mut p = partial(s);
                        if problem.sums[s].terms.iter().any(|&(w, _)| w == v) {
                            p += &value;
                        }
                        p
                    }))
                    .collect::<Vec<_>>();
                let key = next_codec.encode(&coordinates);
                if !next.contains_key(&key) && request.max_states.is_some_and(|m| next.len() >= m) {
                    return Err(ConstraintError::StateLimitExceeded {
                        step: step + 1,
                        variable: request.reported_order[step],
                        limit: request.max_states.expect("checked state budget"),
                        frontier: coordinates.len(),
                        estimated_peak: request.estimated_peak.to_string(),
                    });
                }
                next.entry(key)
                    .or_insert_with(|| W::additive_zero(moduli))
                    .add_scaled(count, &multiplicity, moduli);
                monitor.peak = monitor.peak.max(next.len());
                if aggregated {
                    break;
                }
                value += 1;
            }
        }
        layers.push(next.len());
        monitor.layers.push(next.len());
        layer = next;
        codec = next_codec;
        before_live = after_live;
        before_sums = after_sums;
        if layer.is_empty() {
            return Ok((W::additive_zero(moduli), layers));
        }
    }
    Ok((
        layer
            .into_values()
            .next()
            .unwrap_or_else(|| W::additive_zero(moduli)),
        layers,
    ))
}

#[cfg(test)]
mod key_tests {
    use super::*;

    #[test]
    fn pilot_time_and_transition_budgets_stop_cooperatively() {
        let mut time = FrontierMonitor {
            deadline: Some(Instant::now()),
            ticks: 255,
            ..FrontierMonitor::default()
        };
        assert!(time.check().is_err());
        assert_eq!(time.stop, Some("time-limit"));
        let mut transitions = FrontierMonitor {
            transition_budget: Some(2),
            transitions: 2,
            ..FrontierMonitor::default()
        };
        assert!(transitions.check().is_err());
        assert_eq!(transitions.stop, Some("transition-limit"));
    }

    #[test]
    fn packed_word_and_arbitrary_precision_keys_round_trip() {
        let full = BigInt::from(u128::MAX);
        let codec = Codec::new(&[BigInt::zero(), full.clone()]);
        let values = vec![BigInt::zero(), full];
        assert!(matches!(codec.encode(&values), Key::Packed(u128::MAX)));
        assert_eq!(codec.decode(&codec.encode(&values)), values);
        let values = vec![BigInt::from(u32::MAX); 8];
        let codec = Codec::new(&values);
        assert!(matches!(codec.encode(&values), Key::Wide(_)));
        assert_eq!(codec.decode(&codec.encode(&values)), values);
        let values = vec![BigInt::one() << 160_usize, BigInt::from(7)];
        let codec = Codec::new(&values);
        assert!(matches!(codec.encode(&values), Key::Big(_)));
        assert_eq!(codec.decode(&codec.encode(&values)), values);
    }

    #[test]
    fn dependent_equalities_do_not_over_sharpen_bound() {
        let input = super::super::ConstraintPolytopeInput::from_json(
            r#"{"variables":4,
            "bounds":[[0,4],[0,4],[0,4],[0,4]],"sums":[
            {"vars":[0,1],"lower":4,"upper":4},
            {"vars":[2,3],"lower":4,"upper":4},
            {"vars":[0,1,2,3],"lower":8,"upper":8}]}"#,
        )
        .unwrap();
        let prepared = PreparedConstraintPolytope::new(&input).unwrap();
        let result = prepared
            .count(1, false, &ConstraintOptions::default())
            .unwrap();
        assert_eq!(result.value, BigUint::from(25_u32));
        assert_eq!(result.modular[0].upper_bound, BigUint::from(25_u32));
    }

    #[test]
    fn excessive_crt_bound_reports_exact_alternative() {
        let bound = BigUint::one() << 2100_usize;
        assert!(matches!(
            select_moduli(&bound),
            Err(ConstraintError::ModularCapacity { .. })
        ));
    }
}
