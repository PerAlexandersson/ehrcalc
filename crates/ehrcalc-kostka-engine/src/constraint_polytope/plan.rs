use super::count::{pilot, state_estimate, IntegerProblem, PilotProblem};
use super::{ConstraintError, PreparedConstraintPolytope, Result};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::VecDeque;

#[path = "orders.rs"]
mod orders;

/// Maximum number of extra interpolation checks per request.
pub const MAX_EXTRA_SAMPLES: usize = 64;

/// Candidate ordering algorithm. Exact subset minimax is available through 22 free variables.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OrderHeuristic {
    #[default]
    Auto,
    Greedy,
    SumContiguous,
    ReverseCuthillMcKee,
    ExactSubset,
    MinimumDegree,
    MinimumFill,
    CuthillMcKee,
    GtLevel,
    GtRow,
    LocalImprovement,
}

/// Core engine and Ehrhart-service controls. Orders name original input variables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstraintOptions {
    pub positive_only: bool,
    pub extra_samples: usize,
    pub order: Option<Vec<usize>>,
    pub order_heuristic: OrderHeuristic,
    pub max_states: Option<usize>,
    pub exact: bool,
    pub plan_only: bool,
}

impl Default for ConstraintOptions {
    fn default() -> Self {
        Self {
            positive_only: false,
            extra_samples: 1,
            order: None,
            order_heuristic: OrderHeuristic::Auto,
            max_states: Some(1_000_000),
            exact: false,
            plan_only: false,
        }
    }
}

/// A candidate order and its exact box-product frontier estimate.
/// Estimate-only selection minimizes peak then work; auto uses recorded pilot predictions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderCandidate {
    pub heuristic: OrderHeuristic,
    pub order: Vec<usize>,
    pub estimated_peak: BigUint,
    pub estimated_work: BigUint,
    pub peak_frontier: usize,
    /// Seed/direction or local-search source, stable across runs.
    pub variant: String,
    pub pilots: Vec<OrderPilot>,
    /// Index of an identical order whose pilot was reused.
    pub duplicate_of: Option<usize>,
    /// Heuristic prediction, never a certified bound or a count.
    pub predicted_peak: Option<String>,
    pub predicted_runtime_micros: Option<String>,
    pub prediction_basis: Option<String>,
}

/// Bounded enumeration evidence. Incomplete peaks are lower bounds only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderPilot {
    pub dilation: u64,
    pub strict: bool,
    pub status: String,
    pub peak_states: usize,
    pub layer_states: Vec<usize>,
    pub transitions: u64,
    pub elapsed_micros: u64,
    pub state_budget: usize,
    pub transition_budget: u64,
    pub time_budget_micros: u64,
}

/// Chosen order at a specified dilation, plus all evaluated candidates.
/// The estimate is an upper bound on layer states, not a prediction of transitions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CountingPlan {
    pub dilation: u64,
    pub strict: bool,
    pub order: Vec<usize>,
    pub heuristic: OrderHeuristic,
    pub user_order: bool,
    pub estimated_peak: BigUint,
    pub estimated_work: BigUint,
    pub peak_frontier: usize,
    pub candidates: Vec<OrderCandidate>,
    /// `pilot`, `estimate-only`, `explicit-heuristic`, or `user-order`.
    pub selection: String,
}

/// Interpolation schedule: zero, `1..=positive`, and `-1..=-reciprocal`.
/// Extra checks use the next positive dilations and are excluded from interpolation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SamplePlan {
    pub positive: usize,
    pub reciprocal: usize,
    pub extra_dilations: Vec<u64>,
    pub estimated_work: BigUint,
    /// Ordinary and strict orders at the largest scheduled dilation.
    pub ordinary_plans: Vec<CountingPlan>,
    pub strict_plans: Vec<CountingPlan>,
}

pub(super) fn validate_order(n: usize, options: &ConstraintOptions) -> Result<()> {
    if options.extra_samples > MAX_EXTRA_SAMPLES {
        return Err(ConstraintError::InvalidInput(format!(
            "extra_samples must be at most {MAX_EXTRA_SAMPLES}; got {}",
            options.extra_samples
        )));
    }
    if let Some(order) = &options.order {
        let mut sorted = order.clone();
        sorted.sort_unstable();
        if sorted != (0..n).collect::<Vec<_>>() {
            return Err(ConstraintError::InvalidInput(
                "order must be a permutation of all original variable indices".into(),
            ));
        }
    }
    Ok(())
}

fn evaluate(
    problem: &IntegerProblem,
    order: Vec<usize>,
    heuristic: OrderHeuristic,
) -> OrderCandidate {
    let mut done = vec![false; problem.widths.len()];
    let mut peak = if problem.empty {
        BigUint::zero()
    } else {
        BigUint::one()
    };
    let mut work = BigUint::zero();
    let mut frontier = 0;
    for &v in &order {
        done[v] = true;
        let (states, coordinates) = state_estimate(problem, &done);
        work += &states;
        peak = peak.max(states);
        frontier = frontier.max(coordinates);
    }
    OrderCandidate {
        heuristic,
        order,
        estimated_peak: peak,
        estimated_work: work,
        peak_frontier: frontier,
        variant: String::new(),
        pilots: Vec::new(),
        duplicate_of: None,
        predicted_peak: None,
        predicted_runtime_micros: None,
        prediction_basis: None,
    }
}

fn greedy(problem: &IntegerProblem) -> Vec<usize> {
    let n = problem.widths.len();
    let mut done = vec![false; n];
    let mut order = Vec::new();
    for _ in 0..n {
        let next = (0..n)
            .filter(|&v| !done[v])
            .map(|v| {
                let mut after = done.clone();
                after[v] = true;
                let now = state_estimate(problem, &after);
                let lookahead = (0..n)
                    .filter(|&w| !after[w])
                    .map(|w| {
                        let mut lookahead_done = after.clone();
                        lookahead_done[w] = true;
                        state_estimate(problem, &lookahead_done).0
                    })
                    .min()
                    .unwrap_or_else(|| now.0.clone());
                let closes = problem
                    .sums
                    .iter()
                    .chain(&problem.differences)
                    .filter(|r| {
                        r.terms.iter().any(|&(w, _)| w == v)
                            && r.terms.iter().all(|&(w, _)| after[w])
                    })
                    .count();
                (
                    (
                        now.0.clone().max(lookahead),
                        now.0,
                        usize::MAX - closes,
                        now.1,
                        v,
                    ),
                    v,
                )
            })
            .min_by(|a, b| a.0.cmp(&b.0))
            .expect("an unprocessed variable")
            .1;
        done[next] = true;
        order.push(next);
    }
    order
}

fn sum_contiguous(problem: &IntegerProblem) -> Vec<usize> {
    let mut order = Vec::new();
    let fallback = greedy(problem);
    let mut remaining = (0..problem.sums.len()).collect::<Vec<_>>();
    while !remaining.is_empty() {
        let (position, &sum) = remaining
            .iter()
            .enumerate()
            .max_by_key(|&(_, &s)| {
                let members = &problem.sums[s].terms;
                (
                    members.iter().filter(|&&(v, _)| order.contains(&v)).count(),
                    usize::MAX - members.len(),
                    usize::MAX - s,
                )
            })
            .expect("remaining sum");
        remaining.remove(position);
        for &v in &fallback {
            if !order.contains(&v) && problem.sums[sum].terms.iter().any(|&(w, _)| w == v) {
                order.push(v);
            }
        }
    }
    for v in fallback {
        if !order.contains(&v) {
            order.push(v);
        }
    }
    order
}

fn reverse_cuthill_mckee(problem: &IntegerProblem) -> Vec<usize> {
    let n = problem.widths.len();
    let mut adjacent = vec![Vec::new(); n];
    for range in problem.sums.iter().chain(&problem.differences) {
        for &(a, _) in &range.terms {
            for &(b, _) in &range.terms {
                if a != b && !adjacent[a].contains(&b) {
                    adjacent[a].push(b);
                }
            }
        }
    }
    let bfs = |root: usize, blocked: &[bool]| {
        let mut seen = blocked.to_vec();
        seen[root] = true;
        let mut queue = VecDeque::from([(root, 0_usize)]);
        let mut traversal = Vec::new();
        while let Some((v, depth)) = queue.pop_front() {
            traversal.push((v, depth));
            let mut neighbors = adjacent[v]
                .iter()
                .copied()
                .filter(|&w| !seen[w])
                .collect::<Vec<_>>();
            neighbors.sort_by_key(|&w| (adjacent[w].len(), w));
            for w in neighbors {
                if !seen[w] {
                    seen[w] = true;
                    queue.push_back((w, depth + 1));
                }
            }
        }
        traversal
    };
    let mut done = vec![false; n];
    let mut order = Vec::new();
    while let Some(mut root) = (0..n)
        .filter(|&v| !done[v])
        .min_by_key(|&v| (adjacent[v].len(), v))
    {
        let mut depth = 0;
        loop {
            let traversal = bfs(root, &done);
            let height = traversal.last().map_or(0, |&(_, d)| d);
            let candidate = traversal
                .iter()
                .filter(|&&(_, d)| d == height)
                .min_by_key(|&&(v, _)| (adjacent[v].len(), v))
                .expect("BFS root")
                .0;
            if height <= depth {
                break;
            }
            root = candidate;
            depth = height;
        }
        let traversal = bfs(root, &done);
        for &(v, _) in traversal.iter().rev() {
            done[v] = true;
            order.push(v);
        }
    }
    order
}

fn exact_subset(problem: &IntegerProblem) -> Vec<usize> {
    let n = problem.widths.len();
    let count = 1_usize << n;
    // Minimax recurrence: opt(S) = max(weight(S), min_v opt(S-v)).
    // We retain one predecessor per subset, with exact BigUint comparisons.
    let mut optimum = vec![BigUint::zero(); count];
    let mut predecessor = vec![0_u8; count];
    let mut done = vec![false; n];
    for mask in 1..count {
        for (v, processed) in done.iter_mut().enumerate() {
            *processed = mask & (1 << v) != 0;
        }
        let weight = state_estimate(problem, &done).0;
        let mut members = mask;
        let mut best = None;
        while members != 0 {
            let v = members.trailing_zeros() as usize;
            members &= members - 1;
            let cost = &optimum[mask ^ (1 << v)];
            if best.is_none_or(|(previous, _)| cost < previous) {
                best = Some((cost, v));
            }
        }
        let (cost, v) = best.expect("nonempty subset");
        let cost = cost.clone().max(weight);
        optimum[mask] = cost;
        predecessor[mask] = v as u8;
    }
    let mut mask = count - 1;
    let mut order = Vec::new();
    while mask != 0 {
        let v = usize::from(predecessor[mask]);
        order.push(v);
        mask ^= 1 << v;
    }
    order.reverse();
    order
}

fn log_size(value: &BigUint) -> f64 {
    use num_traits::ToPrimitive;
    if value.is_zero() {
        return 0.0;
    }
    let shift = value.bits().saturating_sub(52);
    (value >> shift).to_f64().expect("52-bit mantissa").ln() + shift as f64 * std::f64::consts::LN_2
}

fn prediction(
    problem: &IntegerProblem,
    pilots: &[PilotProblem],
    candidate: &OrderCandidate,
) -> (f64, f64, &'static str) {
    if problem.empty {
        return (f64::NEG_INFINITY, f64::NEG_INFINITY, "empty-target");
    }
    let usable = candidate
        .pilots
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            p.status == "complete"
                && p.layer_states.len() == candidate.order.len()
                && p.layer_states.last().is_some_and(|&n| n != 0)
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let Some(&last) = usable.last() else {
        // Empty small dilations and censored runs give no full-layer density.
        // Retain the certified range estimate rather than treating them as zero cost.
        return (
            log_size(&candidate.estimated_peak),
            log_size(&candidate.estimated_work),
            "range-fallback",
        );
    };
    let measured = &candidate.pilots[last];
    let mut done = vec![false; problem.widths.len()];
    let mut peak = 0.0_f64;
    let mut work = 0.0_f64;
    for (step, &v) in candidate.order.iter().enumerate() {
        done[v] = true;
        let target = log_size(&state_estimate(problem, &done).0);
        let small = log_size(&state_estimate(&pilots[last].problem, &done).0);
        let actual = (measured.layer_states[step].max(1) as f64).ln();
        let mut power = 1.0;
        if usable.len() >= 2 {
            let previous = usable[usable.len() - 2];
            let first = &candidate.pilots[previous];
            let first_model = log_size(&state_estimate(&pilots[previous].problem, &done).0);
            if small > first_model + 1e-9 {
                power = ((actual - (first.layer_states[step].max(1) as f64).ln())
                    / (small - first_model))
                    .clamp(0.0, 1.0);
            }
        }
        let projected = actual + (target - small).max(0.0) * power;
        peak = peak.max(projected);
        work += projected.min(690.0).exp();
    }
    // A censored second pilot may have unseen layers; increase the score rather
    // than presenting its observed lower bound as a completed peak.
    if usable.len() < 2 && candidate.pilots.iter().any(|p| p.status.ends_with("limit")) {
        peak += 2.0_f64.ln();
    }
    let runtime = (measured.elapsed_micros.max(1) as f64).ln()
        + (work.max(1.0).ln() - (measured.layer_states.iter().sum::<usize>().max(1) as f64).ln())
            .max(0.0);
    let adaptive = measured.dilation > 2;
    let basis = if usable.len() >= 2 {
        if adaptive {
            "adaptive-two-pilot-growth"
        } else {
            "two-pilot-growth"
        }
    } else if candidate.pilots.iter().any(|p| p.status.ends_with("limit")) {
        if adaptive {
            "adaptive-one-pilot-censored"
        } else {
            "one-pilot-censored"
        }
    } else if adaptive {
        "adaptive-one-pilot-density"
    } else {
        "one-pilot-density"
    };
    (peak, runtime, basis)
}

pub(super) fn choose_order(
    problem: &IntegerProblem,
    original: &[usize],
    names: &[String],
    pilots: &[PilotProblem],
    dilation: u64,
    strict: bool,
    options: &ConstraintOptions,
) -> Result<CountingPlan> {
    let mut candidates = Vec::new();
    if let Some(order) = &options.order {
        let local = order
            .iter()
            .filter_map(|v| original.iter().position(|w| w == v))
            .collect();
        candidates.push(evaluate(problem, local, OrderHeuristic::Auto));
    } else {
        for heuristic in [
            OrderHeuristic::Greedy,
            OrderHeuristic::SumContiguous,
            OrderHeuristic::ReverseCuthillMcKee,
            OrderHeuristic::ExactSubset,
            OrderHeuristic::MinimumDegree,
            OrderHeuristic::MinimumFill,
            OrderHeuristic::CuthillMcKee,
            OrderHeuristic::GtLevel,
            OrderHeuristic::GtRow,
        ] {
            if options.order_heuristic != OrderHeuristic::Auto
                && options.order_heuristic != OrderHeuristic::LocalImprovement
                && options.order_heuristic != heuristic
            {
                continue;
            }
            if heuristic == OrderHeuristic::ExactSubset {
                let limit = if options.order_heuristic == heuristic {
                    22
                } else {
                    16
                };
                if original.len() > limit {
                    if options.order_heuristic == heuristic {
                        return Err(ConstraintError::InvalidInput(
                            "exact subset ordering requires at most 22 free variables per component".into()));
                    }
                    continue;
                }
            }
            let generated = match heuristic {
                OrderHeuristic::Greedy => vec![(String::new(), greedy(problem))],
                OrderHeuristic::SumContiguous => vec![(String::new(), sum_contiguous(problem))],
                OrderHeuristic::ReverseCuthillMcKee => {
                    vec![(String::new(), reverse_cuthill_mckee(problem))]
                }
                OrderHeuristic::ExactSubset => vec![(String::new(), exact_subset(problem))],
                OrderHeuristic::MinimumDegree => {
                    vec![(String::new(), orders::elimination(problem, false))]
                }
                OrderHeuristic::MinimumFill => {
                    vec![(String::new(), orders::elimination(problem, true))]
                }
                OrderHeuristic::CuthillMcKee => orders::cuthill_mckee(problem),
                OrderHeuristic::GtLevel => orders::gt_orders(names, original, false),
                OrderHeuristic::GtRow => orders::gt_orders(names, original, true),
                OrderHeuristic::Auto | OrderHeuristic::LocalImprovement => {
                    unreachable!("enumerated candidate")
                }
            };
            for (variant, order) in generated {
                let mut candidate = evaluate(problem, order, heuristic);
                candidate.variant = variant;
                candidates.push(candidate);
            }
        }
        if candidates.is_empty() {
            return Err(ConstraintError::InvalidInput(
                "GT ordering requires alpha_LEVEL_ROW names for every free variable".into(),
            ));
        }
        if matches!(
            options.order_heuristic,
            OrderHeuristic::Auto | OrderHeuristic::LocalImprovement
        ) {
            let mut sources = candidates.clone();
            sources.sort_by(|a, b| {
                (&a.estimated_peak, &a.estimated_work).cmp(&(&b.estimated_peak, &b.estimated_work))
            });
            sources.dedup_by(|a, b| a.order == b.order);
            for source in sources.iter().take(3) {
                candidates.push(orders::improve(problem, source));
            }
            if options.order_heuristic == OrderHeuristic::LocalImprovement {
                candidates.retain(|c| c.heuristic == OrderHeuristic::LocalImprovement);
            }
        }
    }
    let run_pilots = options.order.is_none()
        && options.order_heuristic == OrderHeuristic::Auto
        && !options.plan_only;
    let mut scores = Vec::new();
    for i in 0..candidates.len() {
        let duplicate = (0..i).find(|&j| candidates[j].order == candidates[i].order);
        candidates[i].duplicate_of = duplicate;
        if run_pilots {
            candidates[i].pilots = if let Some(j) = duplicate {
                candidates[j].pilots.clone()
            } else {
                let mut observations = Vec::new();
                for (j, p) in pilots.iter().enumerate() {
                    let informative = observations
                        .iter()
                        .filter(|p: &&OrderPilot| {
                            p.status == "complete"
                                && p.layer_states.len() == original.len()
                                && p.layer_states.last().is_some_and(|&n| n > 0)
                        })
                        .count();
                    if j >= 2 && informative >= 2 {
                        break;
                    }
                    observations.push(pilot(
                        &p.problem,
                        &candidates[i],
                        p.dilation,
                        strict,
                        options.max_states,
                    ));
                }
                observations
            };
            let score = prediction(problem, pilots, &candidates[i]);
            candidates[i].predicted_peak = Some(format!("{:.6e}", score.0.min(690.0).exp()));
            candidates[i].predicted_runtime_micros =
                (score.2 != "range-fallback").then(|| format!("{:.6e}", score.1.min(690.0).exp()));
            candidates[i].prediction_basis = Some(score.2.into());
            scores.push(score);
        }
    }
    let best_index = (0..candidates.len())
        .min_by(|&a, &b| {
            if run_pilots {
                scores[a]
                    .0
                    .total_cmp(&scores[b].0)
                    .then(scores[a].1.total_cmp(&scores[b].1))
                    .then(a.cmp(&b))
            } else {
                (&candidates[a].estimated_peak, &candidates[a].estimated_work)
                    .cmp(&(&candidates[b].estimated_peak, &candidates[b].estimated_work))
            }
        })
        .expect("at least one candidate");
    for candidate in &mut candidates {
        candidate.order = candidate.order.iter().map(|&v| original[v]).collect();
    }
    let best = &candidates[best_index];
    Ok(CountingPlan {
        dilation,
        strict,
        order: best.order.clone(),
        heuristic: best.heuristic,
        user_order: options.order.is_some(),
        estimated_peak: best.estimated_peak.clone(),
        estimated_work: best.estimated_work.clone(),
        peak_frontier: best.peak_frontier,
        selection: if options.order.is_some() {
            "user-order"
        } else if run_pilots {
            "pilot"
        } else if options.order_heuristic == OrderHeuristic::Auto {
            "estimate-only"
        } else {
            "explicit-heuristic"
        }
        .into(),
        candidates,
    })
}

impl PreparedConstraintPolytope {
    /// Choose a split `p+q=d` by estimating ordinary and strict frontier work.
    /// Split selection reuses one ordinary greedy order at every trial dilation.
    /// Positive-only requests never estimate interior counts. Final candidates
    /// are evaluated once per required mode at the largest scheduled dilation.
    pub fn sample_plan(&self, options: &ConstraintOptions) -> Result<SamplePlan> {
        validate_order(self.input.variables, options)?;
        let d = self.dimension().unwrap_or(0);
        let mut estimation = options.clone();
        estimation.plan_only = true;
        if estimation.order.is_none() {
            estimation.order_heuristic = OrderHeuristic::Greedy;
            let seeds = self.plan((d as u64).max(1), false, &estimation)?;
            let mut order = seeds.into_iter().flat_map(|p| p.order).collect::<Vec<_>>();
            for v in 0..self.input.variables {
                if !order.contains(&v) {
                    order.push(v);
                }
            }
            estimation.order = Some(order);
        }
        let mut ordinary_prefix = vec![BigUint::zero()];
        let mut strict_prefix = vec![BigUint::zero()];
        for dilation in 1..=d {
            let ordinary = self
                .plan(dilation as u64, false, &estimation)?
                .iter()
                .map(|p| p.estimated_work.clone())
                .sum::<BigUint>();
            let strict = if options.positive_only {
                BigUint::zero()
            } else {
                self.plan(dilation as u64, true, &estimation)?
                    .iter()
                    .map(|p| p.estimated_work.clone())
                    .sum::<BigUint>()
            };
            ordinary_prefix.push(&ordinary_prefix[dilation - 1] + ordinary);
            strict_prefix.push(&strict_prefix[dilation - 1] + strict);
        }
        let positive = (0..=d)
            .filter(|&p| !options.positive_only || p == d)
            .min_by_key(|&p| &ordinary_prefix[p] + &strict_prefix[d - p])
            .expect("split exists");
        let reciprocal = d - positive;
        let mut extra_dilations = Vec::new();
        for extra in 1..=options.extra_samples {
            let dilation = positive
                .checked_add(extra)
                .and_then(|n| u64::try_from(n).ok())
                .ok_or_else(|| ConstraintError::InvalidInput("too many extra samples".into()))?;
            extra_dilations.push(dilation);
        }
        let largest = extra_dilations
            .last()
            .copied()
            .unwrap_or(positive as u64)
            .max(reciprocal as u64)
            .max(1);
        Ok(SamplePlan {
            positive,
            reciprocal,
            extra_dilations,
            estimated_work: &ordinary_prefix[positive] + &strict_prefix[reciprocal],
            ordinary_plans: self.plan(largest, false, options)?,
            strict_plans: if reciprocal == 0 {
                Vec::new()
            } else {
                self.plan(largest, true, options)?
            },
        })
    }
}
