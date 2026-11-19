use super::count::{state_estimate, IntegerProblem};
use super::{ConstraintError, PreparedConstraintPolytope, Result};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::VecDeque;

/// Candidate ordering algorithm. Exact subset minimax is available through 22 free variables.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OrderHeuristic {
    #[default]
    Auto,
    Greedy,
    SumContiguous,
    ReverseCuthillMcKee,
    ExactSubset,
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
/// The optimization objective is `estimated_peak`; work breaks ties when choosing candidates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderCandidate {
    pub heuristic: OrderHeuristic,
    pub order: Vec<usize>,
    pub estimated_peak: BigUint,
    pub estimated_work: BigUint,
    pub peak_frontier: usize,
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

pub(super) fn choose_order(
    problem: &IntegerProblem,
    original: &[usize],
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
        ] {
            if options.order_heuristic != OrderHeuristic::Auto
                && options.order_heuristic != heuristic
            {
                continue;
            }
            if heuristic == OrderHeuristic::ExactSubset && original.len() > 22 {
                if options.order_heuristic == heuristic {
                    return Err(ConstraintError::InvalidInput(
                        "exact subset ordering requires at most 22 free variables per component"
                            .into(),
                    ));
                }
                continue;
            }
            let order = match heuristic {
                OrderHeuristic::Greedy => greedy(problem),
                OrderHeuristic::SumContiguous => sum_contiguous(problem),
                OrderHeuristic::ReverseCuthillMcKee => reverse_cuthill_mckee(problem),
                OrderHeuristic::ExactSubset => exact_subset(problem),
                OrderHeuristic::Auto => unreachable!("enumerated candidate"),
            };
            candidates.push(evaluate(problem, order, heuristic));
        }
    }
    for candidate in &mut candidates {
        candidate.order = candidate.order.iter().map(|&v| original[v]).collect();
    }
    let best = candidates
        .iter()
        .min_by(|a, b| {
            a.estimated_peak
                .cmp(&b.estimated_peak)
                .then(a.estimated_work.cmp(&b.estimated_work))
        })
        .expect("at least one candidate");
    Ok(CountingPlan {
        dilation,
        strict,
        order: best.order.clone(),
        heuristic: best.heuristic,
        user_order: options.order.is_some(),
        estimated_peak: best.estimated_peak.clone(),
        estimated_work: best.estimated_work.clone(),
        peak_frontier: best.peak_frontier,
        candidates,
    })
}

impl PreparedConstraintPolytope {
    /// Choose a split `p+q=d` by estimating ordinary and strict frontier work.
    /// Split selection uses greedy orders to avoid repeating exponential subset
    /// optimization for each trial; final candidates are evaluated at the largest N.
    pub fn sample_plan(&self, options: &ConstraintOptions) -> Result<SamplePlan> {
        validate_order(self.input.variables, options)?;
        let d = self.dimension().unwrap_or(0);
        let mut estimation = options.clone();
        if estimation.order.is_none() {
            estimation.order_heuristic = OrderHeuristic::Greedy;
        }
        let mut ordinary_prefix = vec![BigUint::zero()];
        let mut strict_prefix = vec![BigUint::zero()];
        for dilation in 1..=d {
            let ordinary = self
                .plan(dilation as u64, false, &estimation)?
                .iter()
                .map(|p| p.estimated_work.clone())
                .sum::<BigUint>();
            let strict = self
                .plan(dilation as u64, true, &estimation)?
                .iter()
                .map(|p| p.estimated_work.clone())
                .sum::<BigUint>();
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
