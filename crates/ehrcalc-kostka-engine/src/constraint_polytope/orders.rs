//! Incidence-graph heuristics: sums are accumulator nodes, never primal cliques.

use super::{evaluate, IntegerProblem, OrderCandidate, OrderHeuristic};
use std::collections::{BTreeSet, VecDeque};

fn graph(problem: &IntegerProblem) -> Vec<BTreeSet<usize>> {
    let n = problem.widths.len();
    let mut adjacent = vec![BTreeSet::new(); n + problem.sums.len()];
    for range in &problem.differences {
        let (a, b) = (range.terms[0].0, range.terms[1].0);
        adjacent[a].insert(b);
        adjacent[b].insert(a);
    }
    for (s, range) in problem.sums.iter().enumerate() {
        for &(v, _) in &range.terms {
            adjacent[v].insert(n + s);
            adjacent[n + s].insert(v);
        }
    }
    adjacent
}

pub(super) fn elimination(problem: &IntegerProblem, fill: bool) -> Vec<usize> {
    let mut adjacent = graph(problem);
    let mut remaining = (0..adjacent.len()).collect::<BTreeSet<_>>();
    let mut order = Vec::new();
    while !remaining.is_empty() {
        let v = *remaining
            .iter()
            .min_by_key(|&&v| {
                let neighbors = adjacent[v].iter().copied().collect::<Vec<_>>();
                let missing = if fill {
                    neighbors
                        .iter()
                        .enumerate()
                        .map(|(i, a)| {
                            neighbors[i + 1..]
                                .iter()
                                .filter(|b| !adjacent[*a].contains(b))
                                .count()
                        })
                        .sum()
                } else {
                    0
                };
                (missing, neighbors.len(), v)
            })
            .expect("nonempty remaining graph");
        let neighbors = adjacent[v].iter().copied().collect::<Vec<_>>();
        // Both are elimination heuristics; degree uses degree as its priority,
        // fill uses the number of new edges. Both complete the eliminated clique.
        for &a in &neighbors {
            adjacent[a].remove(&v);
            for &b in &neighbors {
                if a != b {
                    adjacent[a].insert(b);
                }
            }
        }
        remaining.remove(&v);
        if v < problem.widths.len() {
            order.push(v);
        }
    }
    order.reverse();
    order
}

fn bfs(adjacent: &[BTreeSet<usize>], root: usize, seen: &mut [bool]) -> Vec<(usize, usize)> {
    seen[root] = true;
    let mut queue = VecDeque::from([(root, 0)]);
    let mut result = Vec::new();
    while let Some((v, depth)) = queue.pop_front() {
        result.push((v, depth));
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
    result
}

pub(super) fn cuthill_mckee(problem: &IntegerProblem) -> Vec<(String, Vec<usize>)> {
    let adjacent = graph(problem);
    if adjacent.is_empty() {
        return vec![("empty".into(), Vec::new())];
    }
    let mut roots = (0..problem.widths.len()).collect::<Vec<_>>();
    roots.sort_by_key(|&v| (adjacent[v].len(), v));
    roots.truncate(3);
    // Add low-degree vertices on the last BFS level of each seed.
    for root in roots.clone() {
        let traversal = bfs(&adjacent, root, &mut vec![false; adjacent.len()]);
        let depth = traversal.last().map_or(0, |&(_, d)| d);
        if let Some(&(v, _)) = traversal
            .iter()
            .filter(|&&(_, d)| d == depth)
            .min_by_key(|&&(v, _)| (adjacent[v].len(), v))
        {
            if !roots.contains(&v) {
                roots.push(v);
            }
        }
    }
    let mut orders = Vec::new();
    for root in roots {
        let mut seen = vec![false; adjacent.len()];
        let mut traversal = bfs(&adjacent, root, &mut seen);
        while let Some(v) = (0..adjacent.len())
            .filter(|&v| !seen[v])
            .min_by_key(|&v| (adjacent[v].len(), v))
        {
            traversal.extend(bfs(&adjacent, v, &mut seen));
        }
        let order = traversal
            .into_iter()
            .map(|(v, _)| v)
            .filter(|&v| v < problem.widths.len())
            .collect::<Vec<_>>();
        orders.push((format!("seed-{root}-forward"), order.clone()));
        orders.push((
            format!("seed-{root}-reverse"),
            order.into_iter().rev().collect(),
        ));
    }
    orders
}

pub(super) fn gt_orders(
    names: &[String],
    original: &[usize],
    row_major: bool,
) -> Vec<(String, Vec<usize>)> {
    let positions = original
        .iter()
        .map(|&v| {
            let mut parts = names.get(v)?.strip_prefix("alpha_")?.split('_');
            let level = parts.next()?.parse::<usize>().ok()?;
            let row = parts.next()?.parse::<usize>().ok()?;
            parts.next().is_none().then_some((level, row))
        })
        .collect::<Option<Vec<_>>>();
    let Some(positions) = positions else {
        return Vec::new();
    };
    let mut result = Vec::new();
    for reverse_outer in [false, true] {
        for reverse_inner in [false, true] {
            let mut order = (0..original.len()).collect::<Vec<_>>();
            order.sort_by_key(|&v| {
                let (level, row) = positions[v];
                let (a, b) = if row_major {
                    (row, level)
                } else {
                    (level, row)
                };
                (
                    if reverse_outer { usize::MAX - a } else { a },
                    if reverse_inner { usize::MAX - b } else { b },
                    v,
                )
            });
            result.push((
                format!("outer-reverse-{reverse_outer}-inner-reverse-{reverse_inner}"),
                order,
            ));
        }
    }
    result
}

pub(super) fn improve(problem: &IntegerProblem, source: &OrderCandidate) -> OrderCandidate {
    let mut best = evaluate(
        problem,
        source.order.clone(),
        OrderHeuristic::LocalImprovement,
    );
    best.variant = format!("{:?}/{}", source.heuristic, source.variant);
    // At most three adjacent-swap sweeps; do not make planning quadratic in huge inputs.
    if best.order.len() > 128 {
        return best;
    }
    for _ in 0..3 {
        let mut changed = false;
        for i in 1..best.order.len() {
            let mut order = best.order.clone();
            order.swap(i - 1, i);
            let candidate = evaluate(problem, order, OrderHeuristic::LocalImprovement);
            if (&candidate.estimated_peak, &candidate.estimated_work)
                < (&best.estimated_peak, &best.estimated_work)
            {
                let variant = best.variant.clone();
                best = candidate;
                best.variant = variant;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    best
}
