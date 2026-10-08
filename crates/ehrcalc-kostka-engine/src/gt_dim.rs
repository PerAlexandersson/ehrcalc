//! Exact dimension and affine structure of GT polytopes via the chain model.
//!
//! The GT polytope for shape λ/μ with weight w is parameterized by a chain:
//!   μ = α⁰ ⊂ α¹ ⊂ … ⊂ αᵏ = λ
//! where each α^{i-1} ⊂ α^i is a horizontal strip of size w_i, and k = len(w).
//!
//! Interior levels: α¹, …, α^{k-1}  (k-1 levels, each a partition with ≤ n parts).
//! Total interior entries = (k-1) × n, where n = len(λ).
//!
//! Presolve.  For each interior entry α^i_j, the boundary values give the bounds:
//!   lb = max(μ_j,  λ_{j+k-i})     [vertical from bottom, diagonal from top]
//!   ub = min(λ_j,  μ_{j-i})       [vertical from top, diagonal from bottom]
//! where out-of-range indices give lb contribution 0 and ub contribution +∞.
//!
//! An entry is frozen (lb == ub).  Weight constraints can force additional entries
//! (when a level has exactly 1 free entry), and forced values propagate via
//! interlacing to tighten bounds at neighboring levels.
//!
//! Flags and forbidden-row masks (optional, length k = w.len()):
//!   `upper_flags[ℓ] = f`  →  label ℓ+1 appears only in rows 1..=f  (1-indexed)
//!                         ⟺  α^{ℓ+1}_j = α^ℓ_j for j (0-indexed) ≥ f
//!   `lower_flags[ℓ] = g`  →  label ℓ+1 appears only in rows g..=n
//!                         ⟺  α^{ℓ+1}_j = α^ℓ_j for j (0-indexed) < g-1
//! The presolve enforces them as bidirectional bound coupling between adjacent levels.
//! Bit j of `forbidden_row_masks[ℓ]` imposes the same equality for row j at
//! label ℓ+1.  This is the constraint form produced by complement-row lifts
//! of individual Kogan faces.
//!
//! Interval propagation is sound but incomplete: it never derives a false
//! equality, but several interlacing inequalities can be forced to equality
//! jointly by a level-weight equation while every coordinate interval stays
//! open, and a polytope can be empty although every interval is nonempty.
//! Propagation is therefore only a presolve.  After contracting its fixed
//! coordinates and merged components, [`crate::affine_hull`] decides
//! emptiness and every remaining implicit equality with a certified exact
//! linear program.  The dimension and the tight interlacing masks come from
//! that exact affine hull; lattice points at one dilation are never used.
//! Counting recursions that only need pruning bounds should call
//! [`gt_polytope_propagated_bounds_masked`], which skips the linear program.
//!
//! Both propagation loops stop after a bounded number of sweeps.  Rational
//! interval propagation need not stabilize after finitely many steps in
//! general, and integer propagation can take a number of sweeps proportional
//! to the coordinate sizes.  Stopping early only leaves valid but looser
//! intervals, so exactness is unaffected: the exact affine hull decides.
//!
//! The dimension routines accept any number of rows.  Forbidden-row masks
//! and the `u32` tight-inequality masks are bitmasks, so they address rows
//! `0..32` only; see [`gt_polytope_tight_inequalities_masked`] for tightness
//! without a row limit.
//!
//! The public functions return `None` if the polytope is empty, or `Some(d)`
//! for its affine dimension `d`.  Dimension zero means a single rational
//! point; this module does not claim that the point is integral, and the
//! Ehrhart interpolation checks that separately.

use crate::affine_hull::{AffineHull, RationalPolyhedron};
use num_traits::Zero;

type GtBounds = (usize, Vec<Vec<u32>>, Vec<Vec<u32>>, TightRows, TightRows);

/// `tight[step][row]` for every step and row; see [`GtTightInequalities`].
type TightRows = Vec<Vec<bool>>;

/// Maximum number of sweeps of either presolve propagation loop.
///
/// Propagation is only a presolve, so this bound trades pruning strength for
/// guaranteed termination; it never affects the exact results.
fn presolve_sweep_limit(variables: usize) -> usize {
    #[cfg(test)]
    if let Some(limit) = PRESOLVE_SWEEP_OVERRIDE.with(std::cell::Cell::get) {
        return limit;
    }
    2 * variables + 64
}

#[cfg(test)]
thread_local! {
    /// Test-only override showing that the presolve cap cannot change results.
    static PRESOLVE_SWEEP_OVERRIDE: std::cell::Cell<Option<usize>> =
        const { std::cell::Cell::new(None) };
}

/// Whether bit `row` of a forbidden-row bitmask is set.  Bitmasks address
/// rows `0..32`; later rows are never masked.
fn mask_has_row(mask: u32, row: usize) -> bool {
    row < u32::BITS as usize && mask & (1_u32 << row) != 0
}

#[derive(Debug)]
struct EqualityComponents {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl EqualityComponents {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, value: usize) -> usize {
        if self.parent[value] != value {
            self.parent[value] = self.find(self.parent[value]);
        }
        self.parent[value]
    }

    fn union(&mut self, left: usize, right: usize) {
        let mut left_root = self.find(left);
        let mut right_root = self.find(right);
        if left_root == right_root {
            return;
        }
        if self.rank[left_root] < self.rank[right_root] {
            std::mem::swap(&mut left_root, &mut right_root);
        }
        self.parent[right_root] = left_root;
        if self.rank[left_root] == self.rank[right_root] {
            self.rank[left_root] += 1;
        }
    }
}

fn finish_order(node: usize, graph: &[Vec<usize>], visited: &mut [bool], order: &mut Vec<usize>) {
    if visited[node] {
        return;
    }
    visited[node] = true;
    for &next in &graph[node] {
        finish_order(next, graph, visited, order);
    }
    order.push(node);
}

fn assign_component(
    node: usize,
    reverse_graph: &[Vec<usize>],
    component: usize,
    components: &mut [usize],
) {
    if components[node] != usize::MAX {
        return;
    }
    components[node] = component;
    for &next in &reverse_graph[node] {
        assign_component(next, reverse_graph, component, components);
    }
}

#[allow(clippy::too_many_arguments)]
fn affine_structure_from_equalities(
    lambda: &[u32],
    mu: &[u32],
    weight: &[u32],
    lower_bounds: &[Vec<u32>],
    upper_bounds: &[Vec<u32>],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
) -> Option<(usize, TightRows, TightRows)> {
    let rows = lambda.len();
    let levels = weight.len();
    if rows == 0 || levels == 0 {
        return Some((
            0,
            vec![vec![true; rows]; levels],
            vec![vec![true; rows]; levels],
        ));
    }

    let interior_levels = levels - 1;
    let node_count = (levels + 1) * rows;
    let node = |level: usize, row: usize| level * rows + row;
    let mut components = EqualityComponents::new(node_count);
    let mut fixed_representatives = std::collections::BTreeMap::new();

    let mut record_fixed = |node_index: usize, value: u32, components: &mut EqualityComponents| {
        if let Some(&representative) = fixed_representatives.get(&value) {
            components.union(node_index, representative);
        } else {
            fixed_representatives.insert(value, node_index);
        }
    };

    for row in 0..rows {
        record_fixed(node(0, row), mu[row], &mut components);
        record_fixed(node(levels, row), lambda[row], &mut components);
    }

    for level in 1..levels {
        for row in 0..rows {
            if lower_bounds[level - 1][row] == upper_bounds[level - 1][row] {
                record_fixed(
                    node(level, row),
                    lower_bounds[level - 1][row],
                    &mut components,
                );
            }
        }
    }

    for step in 0..levels {
        let lower_row = lower_flags
            .map(|flags| (flags[step] as usize).saturating_sub(1).min(rows))
            .unwrap_or(0);
        let upper_row = upper_flags
            .map(|flags| (flags[step] as usize).min(rows))
            .unwrap_or(rows);
        let forbidden = forbidden_row_masks.map_or(0, |masks| masks[step]);
        for row in 0..rows {
            if weight[step] != 0
                && row >= lower_row
                && row < upper_row
                && !mask_has_row(forbidden, row)
            {
                continue;
            }
            components.union(node(step, row), node(step + 1, row));
        }
    }

    // Contract direct equalities, then close them under the order inequalities.
    // Contracting comparable endpoints can create a directed cycle; every edge
    // in such a cycle is necessarily tight on the entire face.
    let roots = (0..node_count)
        .map(|value| components.find(value))
        .collect::<Vec<_>>();
    let mut graph = vec![Vec::new(); node_count];
    let mut reverse_graph = vec![Vec::new(); node_count];
    let mut add_edge = |source: usize, destination: usize| {
        let source_root = roots[source];
        let destination_root = roots[destination];
        if source_root != destination_root {
            graph[source_root].push(destination_root);
            reverse_graph[destination_root].push(source_root);
        }
    };
    for step in 0..levels {
        for row in 0..rows {
            add_edge(node(step, row), node(step + 1, row));
            if row > 0 {
                add_edge(node(step + 1, row), node(step, row - 1));
            }
        }
    }

    let mut visited = vec![false; node_count];
    let mut order = Vec::with_capacity(node_count);
    for &root in &roots {
        finish_order(root, &graph, &mut visited, &mut order);
    }
    let mut strongly_connected = vec![usize::MAX; node_count];
    let mut component_count = 0;
    for &root in order.iter().rev() {
        if strongly_connected[root] == usize::MAX {
            assign_component(
                root,
                &reverse_graph,
                component_count,
                &mut strongly_connected,
            );
            component_count += 1;
        }
    }
    let mut representatives = vec![None; component_count];
    for &root in &roots {
        let component = strongly_connected[root];
        if let Some(representative) = representatives[component] {
            components.union(root, representative);
        } else {
            representatives[component] = Some(root);
        }
    }

    let final_roots = (0..node_count)
        .map(|value| components.find(value))
        .collect::<Vec<_>>();
    let rational =
        |value: u32| num_rational::BigRational::from_integer(num_bigint::BigInt::from(value));
    let mut root_lower = vec![None; node_count];
    let mut root_upper = vec![None; node_count];
    let mut record_bounds =
        |node_index: usize, lower: num_rational::BigRational, upper: num_rational::BigRational| {
            let root = final_roots[node_index];
            if root_lower[root].as_ref().is_none_or(|prior| prior < &lower) {
                root_lower[root] = Some(lower);
            }
            if root_upper[root].as_ref().is_none_or(|prior| prior > &upper) {
                root_upper[root] = Some(upper);
            }
        };
    for row in 0..rows {
        record_bounds(node(0, row), rational(mu[row]), rational(mu[row]));
        record_bounds(
            node(levels, row),
            rational(lambda[row]),
            rational(lambda[row]),
        );
    }
    for level in 1..levels {
        for row in 0..rows {
            record_bounds(
                node(level, row),
                rational(lower_bounds[level - 1][row]),
                rational(upper_bounds[level - 1][row]),
            );
        }
    }

    let mut order_edges = Vec::new();
    for step in 0..levels {
        for row in 0..rows {
            order_edges.push((
                final_roots[node(step, row)],
                final_roots[node(step + 1, row)],
            ));
            if row > 0 {
                order_edges.push((
                    final_roots[node(step + 1, row)],
                    final_roots[node(step, row - 1)],
                ));
            }
        }
    }

    let mut level_coefficients = vec![vec![0_u32; node_count]; interior_levels];
    for level in 1..levels {
        for row in 0..rows {
            level_coefficients[level - 1][final_roots[node(level, row)]] += 1;
        }
    }
    let mu_sum = mu.iter().map(|&part| u64::from(part)).sum::<u64>();
    let mut prefix = 0_u64;
    let level_targets = weight
        .iter()
        .take(interior_levels)
        .map(|&part| {
            prefix += u64::from(part);
            num_rational::BigRational::from_integer(num_bigint::BigInt::from(mu_sum + prefix))
        })
        .collect::<Vec<_>>();

    // Propagate exact component intervals through the order edges and level
    // sums. This finds many equalities created jointly by a face equality, a
    // boundary inequality, and a weight equation.  The sweep count is capped
    // because rational propagation can converge without stabilizing.
    for _ in 0..presolve_sweep_limit(node_count) {
        let mut changed = false;
        for &(source, destination) in &order_edges {
            if source == destination {
                continue;
            }
            let source_lower = root_lower[source].as_ref()?.clone();
            let source_upper = root_upper[source].as_ref()?.clone();
            let destination_lower = root_lower[destination].as_ref()?.clone();
            let destination_upper = root_upper[destination].as_ref()?.clone();
            if destination_lower < source_lower {
                root_lower[destination] = Some(source_lower);
                changed = true;
            }
            if source_upper > destination_upper {
                root_upper[source] = Some(destination_upper);
                changed = true;
            }
        }
        for root in 0..node_count {
            if root_lower[root]
                .as_ref()
                .zip(root_upper[root].as_ref())
                .is_some_and(|(lower, upper)| lower > upper)
            {
                return None;
            }
        }

        for (coefficients, target) in level_coefficients.iter().zip(&level_targets) {
            for (root, &coefficient) in coefficients.iter().enumerate() {
                if coefficient == 0 {
                    continue;
                }
                let mut minimum = num_rational::BigRational::from_integer(0.into());
                let mut maximum = num_rational::BigRational::from_integer(0.into());
                for (other_root, &other_coefficient) in coefficients.iter().enumerate() {
                    if other_coefficient == 0 {
                        continue;
                    }
                    let other_coefficient =
                        num_rational::BigRational::from_integer(other_coefficient.into());
                    minimum += &other_coefficient * root_lower[other_root].as_ref()?;
                    maximum += &other_coefficient * root_upper[other_root].as_ref()?;
                }
                if target < &minimum || target > &maximum {
                    return None;
                }
                let coefficient = num_rational::BigRational::from_integer(coefficient.into());
                let old_lower = root_lower[root].as_ref()?.clone();
                let old_upper = root_upper[root].as_ref()?.clone();
                let rest_minimum = &minimum - &coefficient * &old_lower;
                let rest_maximum = &maximum - &coefficient * &old_upper;
                let new_lower = ((target - rest_maximum) / &coefficient).max(old_lower);
                let new_upper = ((target - rest_minimum) / &coefficient).min(old_upper);
                if new_lower > new_upper {
                    return None;
                }
                if root_lower[root].as_ref() != Some(&new_lower) {
                    root_lower[root] = Some(new_lower);
                    changed = true;
                }
                if root_upper[root].as_ref() != Some(&new_upper) {
                    root_upper[root] = Some(new_upper);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Interval propagation only derives valid consequences, so its fixed
    // components and merged equalities are true equalities of the polytope.
    // They are used to shrink the exact problem.  Propagation is not complete:
    // several inequalities can be forced jointly by a level equation without
    // any single interval collapsing.  The exact affine hull below decides
    // every remaining inequality with a certified linear program.
    let mut component_columns = vec![usize::MAX; node_count];
    let mut free_components = 0;
    for level in 1..levels {
        for row in 0..rows {
            let root = final_roots[node(level, row)];
            if root_lower[root] != root_upper[root] && component_columns[root] == usize::MAX {
                component_columns[root] = free_components;
                free_components += 1;
            }
        }
    }
    let fixed_value = |root: usize| -> Option<&num_rational::BigRational> {
        (component_columns[root] == usize::MAX).then(|| root_lower[root].as_ref().expect("bound"))
    };

    let mut system = RationalPolyhedron::new(free_components);
    for (coefficients, target) in level_coefficients.iter().zip(&level_targets) {
        let mut rhs = target.clone();
        let mut terms = Vec::new();
        for (root, &coefficient) in coefficients.iter().enumerate() {
            if coefficient == 0 {
                continue;
            }
            let coefficient = num_rational::BigRational::from_integer(coefficient.into());
            match fixed_value(root) {
                Some(value) => rhs -= &coefficient * value,
                None => terms.push((component_columns[root], coefficient)),
            }
        }
        if terms.is_empty() {
            if !rhs.is_zero() {
                return None;
            }
            continue;
        }
        system.add_equality(terms, rhs);
    }

    // Order edge `(source, destination)` means value(source) <= value(destination).
    let mut edge_inequality = std::collections::HashMap::new();
    for &(source, destination) in &order_edges {
        if source == destination || edge_inequality.contains_key(&(source, destination)) {
            continue;
        }
        let one = num_rational::BigRational::from_integer(1.into());
        let index = match (fixed_value(source), fixed_value(destination)) {
            (Some(left), Some(right)) => {
                if left > right {
                    return None;
                }
                None
            }
            (Some(left), None) => {
                Some(system.add_inequality([(component_columns[destination], -one)], -left.clone()))
            }
            (None, Some(right)) => {
                Some(system.add_inequality([(component_columns[source], one)], right.clone()))
            }
            (None, None) => Some(system.add_inequality(
                [
                    (component_columns[source], one.clone()),
                    (component_columns[destination], -one),
                ],
                num_rational::BigRational::zero(),
            )),
        };
        edge_inequality.insert((source, destination), index);
    }

    let AffineHull::Nonempty(hull) = system.affine_hull() else {
        return None;
    };
    let dimension = hull.dimension;
    let inequality_is_tight = |source: usize, destination: usize| {
        if source == destination {
            return true;
        }
        match edge_inequality[&(source, destination)] {
            Some(index) => hull.implicit_equalities[index],
            None => fixed_value(source) == fixed_value(destination),
        }
    };

    let tight_lower = (0..levels)
        .map(|step| {
            (0..rows)
                .map(|row| {
                    inequality_is_tight(
                        final_roots[node(step, row)],
                        final_roots[node(step + 1, row)],
                    )
                })
                .collect()
        })
        .collect();
    let tight_diagonal = (0..levels)
        .map(|step| {
            (0..rows)
                .map(|row| {
                    row == 0
                        || inequality_is_tight(
                            final_roots[node(step + 1, row)],
                            final_roots[node(step, row - 1)],
                        )
                })
                .collect()
        })
        .collect();
    Some((dimension, tight_lower, tight_diagonal))
}

type PropagatedGtBounds = (Vec<u32>, Vec<Vec<u32>>, Vec<Vec<u32>>);

fn gt_polytope_dim_impl(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
) -> Option<GtBounds> {
    let (mu_pad, lb, ub) =
        propagate_gt_bounds(lambda, mu, w, upper_flags, lower_flags, forbidden_row_masks)?;
    let (dim, tight_lower, tight_diagonal) = affine_structure_from_equalities(
        lambda,
        &mu_pad,
        w,
        &lb,
        &ub,
        upper_flags,
        lower_flags,
        forbidden_row_masks,
    )?;
    Some((dim, lb, ub, tight_lower, tight_diagonal))
}

/// Validate the input and propagate sound coordinate intervals.
///
/// `None` proves emptiness.  `Some` does not prove nonemptiness: interval
/// propagation is incomplete, and only the exact affine-hull step decides it.
fn propagate_gt_bounds(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
) -> Option<PropagatedGtBounds> {
    let n = lambda.len();
    let k = w.len();

    // Pad μ to length n with zeros.
    if mu
        .get(n..)
        .is_some_and(|tail| tail.iter().any(|&part| part != 0))
    {
        return None;
    }
    let mut mu_pad = mu.to_vec();
    mu_pad.resize(n, 0);
    if mu_pad
        .iter()
        .zip(lambda)
        .any(|(&mu_part, &lambda_part)| mu_part > lambda_part)
    {
        return None;
    }
    let skew_size = lambda.iter().map(|&part| u64::from(part)).sum::<u64>()
        - mu_pad.iter().map(|&part| u64::from(part)).sum::<u64>();
    if skew_size != w.iter().map(|&part| u64::from(part)).sum::<u64>() {
        return None;
    }
    if upper_flags.is_some_and(|flags| flags.len() != k)
        || lower_flags.is_some_and(|flags| flags.len() != k)
        || forbidden_row_masks.is_some_and(|masks| masks.len() != k)
    {
        return None;
    }
    if let Some(masks) = forbidden_row_masks {
        // Bits must name existing rows; with 32 or more rows every bit does.
        if n < u32::BITS as usize && masks.iter().any(|&mask| mask >> n != 0) {
            return None;
        }
    }

    if k == 0 {
        return (skew_size == 0).then(|| (mu_pad, vec![], vec![]));
    }
    if k == 1 {
        let horizontal_strip = (1..n).all(|j| lambda[j] <= mu_pad[j - 1]);
        let flags_allow_strip = (0..n).all(|j| {
            if lambda[j] == mu_pad[j] {
                return true;
            }
            upper_flags.is_none_or(|flags| j < flags[0] as usize)
                && lower_flags.is_none_or(|flags| j >= flags[0].saturating_sub(1) as usize)
                && forbidden_row_masks.is_none_or(|masks| !mask_has_row(masks[0], j))
        });
        if !horizontal_strip || !flags_allow_strip {
            return None;
        }
        return Some((mu_pad, vec![], vec![]));
    }

    let n_int = k - 1; // number of interior levels

    // Weight-sum targets: |α^{ℓ+1}| = |μ| + w[0] + … + w[ℓ].
    let mu_sum = mu_pad.iter().map(|&part| u64::from(part)).sum::<u64>();
    let w_prefix: Vec<u64> = {
        let mut v = vec![0u64; k];
        let mut acc = 0u64;
        for (i, &wi) in w.iter().enumerate() {
            acc += u64::from(wi);
            v[i] = acc;
        }
        v
    };

    // lb[ℓ][j], ub[ℓ][j] for each interior entry.
    let mut lb = vec![vec![0u32; n]; n_int];
    let mut ub = vec![vec![0u32; n]; n_int];

    // Initialize from boundary bounds.
    for ell in 0..n_int {
        let i = ell + 1; // chain level
        for j in 0..n {
            // lb: max(μ_j, λ_{j+k-i} if valid)
            lb[ell][j] = mu_pad[j];
            let diag_top = j + k - i;
            if diag_top < n {
                lb[ell][j] = lb[ell][j].max(lambda[diag_top]);
            }
            // ub: min(λ_j, μ_{j-i} if valid)
            ub[ell][j] = lambda[j];
            if j >= i {
                ub[ell][j] = ub[ell][j].min(mu_pad[j - i]);
            }
        }
    }

    // Check for individual infeasibility: lb > ub means empty polytope.
    for ell in 0..n_int {
        for j in 0..n {
            if lb[ell][j] > ub[ell][j] {
                return None; // empty
            }
        }
    }

    // Propagation: iterate until no bounds change.
    // Integer propagation terminates, but possibly only after a number of
    // sweeps proportional to the coordinates; cap it as a presolve.
    let mut changed = true;
    let mut sweeps = 0;
    while changed && sweeps < presolve_sweep_limit(n_int * n) {
        sweeps += 1;
        changed = false;

        // Propagate forced entries to neighbors via interlacing.
        for ell in 0..n_int {
            for j in 0..n {
                if lb[ell][j] < ub[ell][j] {
                    continue; // not forced
                }
                let v = lb[ell][j];

                // To level above (ell+1): α^{i+1}_j ≥ v, α^{i+1}_{j+1} ≤ v
                if ell + 1 < n_int {
                    if lb[ell + 1][j] < v {
                        lb[ell + 1][j] = v;
                        changed = true;
                    }
                    if j + 1 < n && ub[ell + 1][j + 1] > v {
                        ub[ell + 1][j + 1] = v;
                        changed = true;
                    }
                }

                // To level below (ell-1): α^{i-1}_j ≤ v, α^{i-1}_{j-1} ≥ v
                if ell > 0 {
                    if ub[ell - 1][j] > v {
                        ub[ell - 1][j] = v;
                        changed = true;
                    }
                    if j >= 1 && lb[ell - 1][j - 1] < v {
                        lb[ell - 1][j - 1] = v;
                        changed = true;
                    }
                }
            }
        }

        // Check for infeasibility created by propagation.
        for ell in 0..n_int {
            for j in 0..n {
                if lb[ell][j] > ub[ell][j] {
                    return None;
                }
            }
        }

        // Weight forcing: check each level's feasible sum range.
        for ell in 0..n_int {
            let target = mu_sum + w_prefix[ell];
            let mut forced_sum = 0u64;
            let mut min_sum = 0u64;
            let mut max_sum = 0u64;
            let mut free_entries: Vec<usize> = Vec::new();

            for j in 0..n {
                if lb[ell][j] >= ub[ell][j] {
                    forced_sum += u64::from(lb[ell][j]);
                    min_sum += u64::from(lb[ell][j]);
                    max_sum += u64::from(lb[ell][j]);
                } else {
                    free_entries.push(j);
                    min_sum += u64::from(lb[ell][j]);
                    max_sum += u64::from(ub[ell][j]);
                }
            }

            // Infeasible: target outside feasible range → empty polytope.
            if target < min_sum || target > max_sum {
                return None;
            }

            if free_entries.is_empty() {
                continue;
            }

            // Target at minimum: all free entries forced to their lower bound.
            if target == min_sum {
                for &j in &free_entries {
                    ub[ell][j] = lb[ell][j];
                    changed = true;
                }
            }
            // Target at maximum: all free entries forced to their upper bound.
            else if target == max_sum {
                for &j in &free_entries {
                    lb[ell][j] = ub[ell][j];
                    changed = true;
                }
            }
            // Exactly 1 free entry: determined by weight.
            else if free_entries.len() == 1 {
                let j = free_entries[0];
                let val = u32::try_from(target - forced_sum)
                    .expect("a single feasible GT coordinate must fit u32");
                if val > lb[ell][j] {
                    lb[ell][j] = val;
                    changed = true;
                }
                if val < ub[ell][j] {
                    ub[ell][j] = val;
                    changed = true;
                }
            }
        }

        // Pass 3: row-forcing constraints.
        //
        // upper_flags[ell] = f: label ell+1 only in rows 1..=f (1-indexed)
        //   ⟺  α^{ell+1}_j = α^ell_j  for j (0-indexed) ≥ f.
        // lower_flags[ell] = g: label ell+1 only in rows g..=n (1-indexed)
        //   ⟺  α^{ell+1}_j = α^ell_j  for j (0-indexed) < g-1.
        //
        // α^{ell+1} lives in lb/ub[ell]  (for ell < n_int)
        // α^{ell}   lives in lb/ub[ell-1] (ell ≥ 1), mu_pad (ell = 0),
        //                    or lambda     (ell = n_int).
        //
        // Equality is enforced by bidirectional tightening of the bound intervals.
        // forbidden_row_masks[ell] bit j imposes the same equality directly.
        // The macro deduplicates the coupling logic for all three sources.
        macro_rules! apply_flag {
            ($ell:expr, $j:expr) => {{
                let ell = $ell;
                let j = $j;
                if ell == 0 {
                    let val = mu_pad[j];
                    if lb[0][j] < val {
                        lb[0][j] = val;
                        changed = true;
                    }
                    if ub[0][j] > val {
                        ub[0][j] = val;
                        changed = true;
                    }
                } else if ell < n_int {
                    let new_lb = lb[ell][j].max(lb[ell - 1][j]);
                    let new_ub = ub[ell][j].min(ub[ell - 1][j]);
                    if lb[ell][j] != new_lb {
                        lb[ell][j] = new_lb;
                        changed = true;
                    }
                    if ub[ell][j] != new_ub {
                        ub[ell][j] = new_ub;
                        changed = true;
                    }
                    if lb[ell - 1][j] != new_lb {
                        lb[ell - 1][j] = new_lb;
                        changed = true;
                    }
                    if ub[ell - 1][j] != new_ub {
                        ub[ell - 1][j] = new_ub;
                        changed = true;
                    }
                } else {
                    // ell == n_int: last weight part, α^{n_int}_j must equal λ_j.
                    let val = lambda[j];
                    if lb[n_int - 1][j] < val {
                        lb[n_int - 1][j] = val;
                        changed = true;
                    }
                    if ub[n_int - 1][j] > val {
                        ub[n_int - 1][j] = val;
                        changed = true;
                    }
                }
            }};
        }

        if let Some(uf) = upper_flags {
            for (ell, &f) in uf.iter().enumerate() {
                for j in (f as usize)..n {
                    apply_flag!(ell, j);
                }
            }
        }
        if let Some(lf) = lower_flags {
            for (ell, &g) in lf.iter().enumerate() {
                let g = g as usize;
                for j in 0..g.saturating_sub(1).min(n) {
                    apply_flag!(ell, j);
                }
            }
        }
        if let Some(masks) = forbidden_row_masks {
            for (ell, &mask) in masks.iter().enumerate() {
                for j in 0..n {
                    if mask_has_row(mask, j) {
                        apply_flag!(ell, j);
                    }
                }
            }
        }

        // Infeasibility check after all passes.
        for ell in 0..n_int {
            for j in 0..n {
                if lb[ell][j] > ub[ell][j] {
                    return None;
                }
            }
        }
    }

    Some((mu_pad, lb, ub))
}

/// Dimension without flags (fast path, no flag overhead).
/// Returns `None` if the polytope is empty, `Some(d)` otherwise.
pub fn gt_polytope_dim(lambda: &[u32], mu: &[u32], w: &[u32]) -> Option<usize> {
    gt_polytope_dim_impl(lambda, mu, w, None, None, None).map(|(d, _, _, _, _)| d)
}

/// Dimension with optional row flags.
/// Returns `None` if the polytope is empty, `Some(d)` otherwise.
pub fn gt_polytope_dim_full(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
) -> Option<usize> {
    gt_polytope_dim_impl(lambda, mu, w, upper_flags, lower_flags, None).map(|(d, _, _, _, _)| d)
}

/// Exact dimension and propagated lb/ub bounds for each interior level.
///
/// Returns `None` if the polytope is empty, or
/// `Some((dim, lb, ub))` where:
/// - `dim` is the exact affine dimension
/// - `lb[ell][j]`, `ub[ell][j]` are sound propagated bounds for `α^{ell+1}[j]`
///   (interior levels `ell = 0..k-2`, columns `j = 0..n-1`).
///
/// Equal bounds prove a fixed coordinate, but unequal bounds do not prove
/// that an interlacing inequality is non-implicit; use
/// [`gt_polytope_affine_masks_masked`] for relative-interior masks.
#[allow(clippy::type_complexity)]
pub fn gt_polytope_bounds(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
) -> Option<(usize, Vec<Vec<u32>>, Vec<Vec<u32>>)> {
    gt_polytope_dim_impl(lambda, mu, w, None, None, None)
        .map(|(dimension, lower, upper, _, _)| (dimension, lower, upper))
}

/// Dimension and propagated bounds with optional row flags.
///
/// This is the flagged counterpart of [`gt_polytope_bounds`].
#[allow(clippy::type_complexity)]
pub fn gt_polytope_bounds_full(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
) -> Option<(usize, Vec<Vec<u32>>, Vec<Vec<u32>>)> {
    gt_polytope_dim_impl(lambda, mu, w, upper_flags, lower_flags, None)
        .map(|(dimension, lower, upper, _, _)| (dimension, lower, upper))
}

/// Dimension and propagated bounds for an individual masked tableau face.
///
/// Bit `j` of `forbidden_row_masks[ell]` forces label `ell + 1` to occur zero
/// times in zero-indexed row `j`.  The equalities are propagated together with
/// the optional interval flags.  This representation covers complement-row
/// lifts of individual Kogan faces; it does not perform inclusion-exclusion or
/// deduplication for a union of faces.
#[allow(clippy::type_complexity)]
pub fn gt_polytope_bounds_masked(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
) -> Option<(usize, Vec<Vec<u32>>, Vec<Vec<u32>>)> {
    gt_polytope_dim_impl(lambda, mu, w, upper_flags, lower_flags, forbidden_row_masks)
        .map(|(dimension, lower, upper, _, _)| (dimension, lower, upper))
}

/// Sound propagated coordinate bounds for pruning a counting recursion.
///
/// Every lattice point of the masked face satisfies
/// `lower[ell][j] <= alpha^{ell+1}_j <= upper[ell][j]`.  `None` proves that the
/// face is empty.  Unlike [`gt_polytope_bounds_masked`], this does not compute
/// the affine hull and therefore costs no linear program; a `Some` result does
/// not prove that the face is nonempty.
#[allow(clippy::type_complexity)]
pub fn gt_polytope_propagated_bounds_masked(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
) -> Option<(Vec<Vec<u32>>, Vec<Vec<u32>>)> {
    propagate_gt_bounds(lambda, mu, w, upper_flags, lower_flags, forbidden_row_masks)
        .map(|(_, lower, upper)| (lower, upper))
}

/// Exact affine dimension and implicit interlacing equalities of a face.
///
/// `lower[step][row]` is true when `alpha^{step+1}_row >= alpha^step_row` is
/// an equality on the whole face, and `diagonal[step][row]` when
/// `alpha^{step+1}_row <= alpha^step_{row-1}` is.  There is no diagonal
/// inequality in row zero, so `diagonal[step][0]` is always true.  Here
/// `alpha^0 = mu` and `alpha^k = lambda`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GtTightInequalities {
    pub dimension: usize,
    pub lower: Vec<Vec<bool>>,
    pub diagonal: Vec<Vec<bool>>,
}

/// Exact affine dimension and implicit interlacing equalities of an
/// individual masked tableau face, for any number of rows.
///
/// Forbidden-row masks address rows `0..32`.  Returns `None` if the face is
/// empty or the masks name rows that do not exist.
pub fn gt_polytope_tight_inequalities_masked(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
) -> Option<GtTightInequalities> {
    gt_polytope_dim_impl(lambda, mu, w, upper_flags, lower_flags, forbidden_row_masks).map(
        |(dimension, _, _, lower, diagonal)| GtTightInequalities {
            dimension,
            lower,
            diagonal,
        },
    )
}

/// Affine dimension and globally tight interlacing masks for an individual
/// masked tableau face.
///
/// A set bit marks an inequality that is an equality throughout the face.
/// The masks are the implicit equalities of the exact affine hull, including
/// those forced jointly by forbidden Kogan cells and level-weight equations.
/// Bit `row` of a diagonal mask is never set for row zero.
///
/// # Panics
///
/// Panics if `lambda` has more than 32 rows, since the masks could not
/// represent every row.  Use [`gt_polytope_tight_inequalities_masked`] or
/// [`gt_polytope_dim_full`] for larger shapes.
pub fn gt_polytope_affine_masks_masked(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
) -> Option<(usize, Vec<u32>, Vec<u32>)> {
    assert!(
        lambda.len() <= u32::BITS as usize,
        "GT tight-inequality bitmasks support at most 32 rows, got {}",
        lambda.len()
    );
    let to_mask = |rows: &[bool], skip_first: bool| {
        rows.iter()
            .enumerate()
            .filter(|&(row, &tight)| tight && !(skip_first && row == 0))
            .fold(0_u32, |mask, (row, _)| mask | (1_u32 << row))
    };
    gt_polytope_tight_inequalities_masked(
        lambda,
        mu,
        w,
        upper_flags,
        lower_flags,
        forbidden_row_masks,
    )
    .map(|tight| {
        (
            tight.dimension,
            tight
                .lower
                .iter()
                .map(|rows| to_mask(rows, false))
                .collect(),
            tight
                .diagonal
                .iter()
                .map(|rows| to_mask(rows, true))
                .collect(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Partition;

    // Helper: compute the empirical Ehrhart degree by sampling K(nλ, nw).
    fn empirical_degree(lambda: &[u32], mu: &[u32], w: &[u32], max_n: u64) -> usize {
        use crate::kostka_dp::skew_kostka;
        use num_bigint::ToBigInt;
        use num_rational::BigRational;
        use num_traits::Zero;

        // Compute K(n*λ / n*μ, n*w) for n = 1, …, max_n.
        // (Start at 1 to avoid the degenerate n=0 case.)
        let values: Vec<BigRational> = (1..=max_n)
            .map(|n| {
                let nl = Partition::from_sorted(lambda.iter().map(|&x| x * n as u32).collect());
                let nm = Partition::from_sorted(mu.iter().map(|&x| x * n as u32).collect());
                let nw: Vec<u32> = w.iter().map(|&x| x * n as u32).collect();
                let k = skew_kostka(&nl, &nm, &nw, None, true);
                BigRational::from(k.to_bigint().unwrap())
            })
            .collect();

        // If all values are zero, the polytope is empty → degree 0.
        if values.iter().all(|v| v.is_zero()) {
            return 0;
        }

        // Compute degree via finite differences.
        let mut diffs = values;
        for step in 0..max_n as usize {
            let new_diffs: Vec<BigRational> = diffs.windows(2).map(|w| &w[1] - &w[0]).collect();
            if new_diffs.iter().all(|v| v.is_zero()) {
                return step;
            }
            diffs = new_diffs;
        }
        max_n as usize
    }

    /// Verify against empirical degrees for all non-skew partitions with w=(1,…,1).
    #[test]
    fn chain_model_matches_empirical_unit_weight() {
        for size in 1..=7u32 {
            let partitions = Partition::all_of_size(size);
            let w: Vec<u32> = vec![1; size as usize];
            for p in &partitions {
                let lambda = p.parts();
                let n = lambda.len();
                let k = w.len();
                let fast = gt_polytope_dim(lambda, &[], &w);
                let fast_val = fast.unwrap_or(0);

                // Empirical check: need enough sample points.
                let max_n = (fast_val + 2) as u64;
                let emp = empirical_degree(lambda, &[], &w, max_n);
                assert_eq!(
                    fast_val, emp,
                    "mismatch for lambda={:?}, w={:?} (n={}, k={}): fast={:?} empirical={}",
                    lambda, w, n, k, fast, emp
                );
            }
        }
    }

    /// Test with non-unit weights.
    #[test]
    fn chain_model_various_weights() {
        let cases: Vec<(&[u32], &[u32], Vec<u32>)> = vec![
            (&[4, 3, 2, 1], &[], vec![2, 2, 2, 2, 2]),
            (&[4, 3, 2, 1], &[], vec![3, 3, 2, 2]),
            (&[3, 2, 1], &[], vec![1, 1, 1, 1, 1, 1]),
            (&[3, 2, 1], &[], vec![2, 2, 2]),
            (&[3, 2, 1], &[], vec![3, 3]),
            (&[5, 3], &[], vec![1, 1, 1, 1, 1, 1, 1, 1]),
            (&[5, 3], &[], vec![4, 4]),
            (&[4, 3, 2], &[], vec![3, 3, 3]),
            (&[4, 2], &[], vec![2, 2, 2]),
        ];
        for (lambda, mu, w) in &cases {
            let fast = gt_polytope_dim(lambda, mu, w);
            let fast_val = fast.unwrap_or(0);
            let max_n = (fast_val + 3) as u64;
            let emp = empirical_degree(lambda, mu, w, max_n);
            assert_eq!(
                fast_val, emp,
                "mismatch for lambda={:?}, mu={:?}, w={:?}: fast={:?} empirical={}",
                lambda, mu, w, fast, emp
            );
        }
    }

    /// Test skew cases.
    #[test]
    fn chain_model_skew() {
        let cases: Vec<(&[u32], &[u32], Vec<u32>)> = vec![
            (&[4, 3, 1], &[2, 1], vec![2, 2, 1]),
            (&[3, 3, 3], &[1, 1, 1], vec![2, 2, 2]),
            (&[6, 4], &[2], vec![2, 2, 2, 2]),
            (&[5, 3, 2], &[2, 1], vec![1, 1, 1, 1, 1, 1, 1]),
            (&[4, 4], &[2, 2], vec![1, 1, 1, 1]),
        ];
        for (lambda, mu, w) in &cases {
            let fast = gt_polytope_dim(lambda, mu, w);
            let fast_val = fast.unwrap_or(0);
            let max_n = (fast_val + 3) as u64;
            let emp = empirical_degree(lambda, mu, w, max_n);
            assert_eq!(
                fast_val, emp,
                "mismatch for lambda={:?}, mu={:?}, w={:?}: fast={:?} empirical={}",
                lambda, mu, w, fast, emp
            );
        }
    }

    #[test]
    fn dimension_formula_examples() {
        // λ=(2,1), w=(1,1,1): dim should be 1
        assert_eq!(gt_polytope_dim(&[2, 1], &[], &[1, 1, 1]), Some(1));

        // λ=(3,1), w=(1,1,1,1): dim should be 2
        assert_eq!(gt_polytope_dim(&[3, 1], &[], &[1, 1, 1, 1]), Some(2));

        // λ=(2,2), w=(1,1,1,1): dim should be 1
        assert_eq!(gt_polytope_dim(&[2, 2], &[], &[1, 1, 1, 1]), Some(1));

        // λ=(3,2,1), w=(1,1,1,1,1,1): dim should be 7
        assert_eq!(
            gt_polytope_dim(&[3, 2, 1], &[], &[1, 1, 1, 1, 1, 1]),
            Some(7)
        );
    }

    #[test]
    fn flagged_fixed_content_dimension_can_exceed_scale_one_lattice_span() {
        // This fixed-content flagged polytope has only a 24-dimensional span
        // among its scale-one lattice points, but its affine dimension is 25.
        // Inferring dimension from the n=1 tableaux would therefore break
        // Ehrhart--Macdonald reciprocity by shifting the terminal h* entries.
        let lambda = [6, 5, 5, 5, 5, 5, 5, 4, 4, 3, 2, 1];
        let weight = [6, 5, 5, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1];
        let upper = [8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 12];
        let lower = [1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 8, 8, 8, 9, 11, 11];
        assert_eq!(
            gt_polytope_dim_full(&lambda, &[], &weight, Some(&upper), Some(&lower)),
            Some(25)
        );
    }

    #[test]
    fn masked_face_equalities_propagate_into_dimension_and_bounds() {
        // Of the two tableaux of shape (2,1) with standard content, forbidding
        // label 2 in the first row leaves a single tableau at every dilation.
        let forbidden = [0, 1, 0];
        let (dimension, lower, upper) =
            gt_polytope_bounds_masked(&[2, 1], &[], &[1, 1, 1], None, None, Some(&forbidden))
                .expect("the masked face is nonempty");

        assert_eq!(dimension, 0);
        assert_eq!(lower, upper);
    }

    #[test]
    fn masked_bounds_use_wide_level_sums() {
        // Every coordinate fits u32, but the boundary and intermediate level
        // totals exceed u32.  Wrapping those totals used to report this
        // one-dimensional interval as empty.
        let lambda = [1_073_741_825, 1_073_741_825, 1_073_741_825, 1_073_741_823];
        let mu = [1_073_741_825, 1_073_741_825, 1_073_741_823, 1_073_741_822];
        let (dimension, lower, upper) =
            gt_polytope_bounds_masked(&lambda, &mu, &[1, 2], None, None, None)
                .expect("wide-total interval is nonempty");

        assert_eq!(dimension, 1);
        assert_eq!(lower.len(), 1);
        assert_eq!(upper.len(), 1);
        assert_ne!(lower, upper);
    }

    #[test]
    fn degenerate_chains_still_validate_feasibility() {
        assert_eq!(gt_polytope_dim(&[2], &[], &[1]), None);
        assert_eq!(gt_polytope_dim(&[2, 2], &[], &[4]), None);
        assert_eq!(gt_polytope_dim(&[2, 1], &[1], &[2]), Some(0));
        assert_eq!(gt_polytope_dim(&[2], &[], &[]), None);
    }

    /// Exhaustive test: all skew shapes with |λ| ≤ 5, all partition weights.
    /// For each valid (λ, μ), tries every composition w that is a partition
    /// (weakly decreasing, all parts > 0) with |w| = |λ/μ|.
    #[test]
    fn chain_model_skew_all_partition_weights() {
        let mut count = 0u32;
        for lam_size in 1..=7u32 {
            let lambdas = Partition::all_of_size(lam_size);
            for lam in &lambdas {
                let lambda = lam.parts();
                let max_mu_size = lam_size.saturating_sub(2);
                for mu_size in 0..=max_mu_size {
                    let mus = if mu_size == 0 {
                        vec![Partition::empty()]
                    } else {
                        Partition::all_of_size_bounded(mu_size, lambda.len(), lambda[0])
                    };
                    for mu_p in &mus {
                        let mu = mu_p.parts();
                        if !mu_p.partition_less_equal(lam) {
                            continue;
                        }
                        let s = lam_size - mu_size;
                        if s == 0 {
                            continue;
                        }

                        // All partitions of s (= all weights with weakly
                        // decreasing positive parts summing to s).
                        let weights = Partition::all_of_size(s);
                        for wp in &weights {
                            let w = wp.parts();

                            let fast = gt_polytope_dim(lambda, mu, w);
                            let fast_val = fast.unwrap_or(0);
                            let max_n = (fast_val + 3).max(3) as u64;
                            let emp = empirical_degree(lambda, mu, w, max_n);
                            assert_eq!(
                                fast_val, emp,
                                "mismatch for lambda={:?}, mu={:?}, w={:?}: fast={:?} empirical={}",
                                lambda, mu, w, fast, emp
                            );
                            count += 1;
                        }
                    }
                }
            }
        }
        eprintln!(
            "chain_model_skew_all_partition_weights: tested {} cases",
            count
        );
    }

    /// 30 larger cases with |λ| in 8..15, random μ ⊂ λ, various partition weights.
    /// Weights are kept compact (few large parts) so Kostka DP stays tractable.
    #[test]
    fn chain_model_random_larger() {
        // Hand-picked larger cases covering a variety of shapes and weights.
        // Short weights (small k) keep the empirical verification fast.
        let cases: Vec<(&[u32], &[u32], &[u32])> = vec![
            // Non-skew, compact weights (k ≤ 5)
            (&[5, 4, 3, 2, 1], &[], &[5, 4, 3, 2, 1]), // 1
            (&[5, 4, 3, 2, 1], &[], &[3, 3, 3, 3, 3]), // 2
            (&[6, 5, 4], &[], &[5, 5, 5]),             // 3
            (&[4, 4, 4, 4], &[], &[4, 4, 4, 4]),       // 4
            (&[8, 5, 2], &[], &[5, 5, 5]),             // 5
            (&[7, 3, 2, 1], &[], &[4, 3, 3, 3]),       // 6
            (&[5, 5, 5], &[], &[5, 5, 5]),             // 7
            (&[6, 3, 3], &[], &[4, 4, 4]),             // 8
            (&[4, 3, 3, 2], &[], &[6, 6]),             // 9
            (&[4, 3, 3, 2], &[], &[4, 4, 4]),          // 10
            (&[6, 6, 3], &[], &[5, 5, 5]),             // 11
            (&[7, 7], &[], &[7, 7]),                   // 12
            (&[8, 4, 2, 1], &[], &[5, 5, 5]),          // 13
            // Skew shapes, compact weights
            (&[6, 5, 4], &[3, 2], &[5, 5]),          // 14
            (&[5, 4, 3, 2, 1], &[2, 1], &[4, 4, 4]), // 15
            (&[7, 5, 3], &[2, 1], &[4, 4, 4]),       // 16
            (&[8, 6, 4], &[3, 2, 1], &[4, 4, 4]),    // 17
            (&[8, 6, 4], &[3, 2, 1], &[6, 6]),       // 18
            (&[6, 4, 4, 2], &[2, 2], &[4, 4, 4]),    // 19
            (&[5, 5, 5], &[2, 2, 2], &[3, 3, 3]),    // 20
            (&[7, 4, 3, 1], &[3, 1], &[4, 3, 2, 2]), // 21
            (&[6, 6, 3], &[2, 1], &[4, 4, 4]),       // 22
            (&[8, 4, 2, 1], &[3], &[4, 3, 3, 2]),    // 23
            (&[6, 5, 3, 1], &[2, 1], &[4, 4, 4]),    // 24
            (&[7, 5, 3, 1], &[3, 2, 1], &[5, 5]),    // 25
            (&[5, 4, 3], &[2, 1], &[3, 3, 3]),       // 26
            (&[9, 5, 1], &[3], &[4, 4, 4]),          // 27
            (&[6, 4, 2], &[1, 1], &[5, 5]),          // 28
            (&[8, 8, 4], &[3, 3], &[7, 7]),          // 29
            (&[10, 5], &[3], &[4, 4, 4]),            // 30
        ];

        for (i, &(lambda, mu, w)) in cases.iter().enumerate() {
            let fast = gt_polytope_dim(lambda, mu, w);
            let fast_val = fast.unwrap_or(0);
            let max_n = (fast_val + 3).max(4) as u64;
            let emp = empirical_degree(lambda, mu, w, max_n);
            assert_eq!(
                fast_val,
                emp,
                "case {}: lambda={:?}, mu={:?}, w={:?}: fast={:?} empirical={}",
                i + 1,
                lambda,
                mu,
                w,
                fast,
                emp
            );
        }
        eprintln!("chain_model_random_larger: tested {} cases", cases.len());
    }

    /// Verify the T(n-1) formula for non-skew with standard GL(n) weight (k=n).
    #[test]
    fn triangular_formula_gln() {
        // For k = n, the chain model should give:
        //   dim = T(n-1) - Σ T(mᵢ-1) - active_weight
        // which equals the triangular GT model result.
        let cases: Vec<(&[u32], Vec<u32>, usize)> = vec![
            (&[3, 2, 1], vec![1, 2, 3], 0), // k=n=3, distinct parts — weight at min forces all entries
            (&[4, 4, 2], vec![2, 4, 4], 0), // k=n=3, run of 2 — all entries forced
            (&[3, 3, 3], vec![3, 3, 3], 0), // all equal
        ];
        for (lambda, w, expected) in &cases {
            let dim = gt_polytope_dim(lambda, &[], w);
            assert_eq!(
                dim,
                Some(*expected),
                "lambda={:?}, w={:?}: got {:?} expected Some({})",
                lambda,
                w,
                dim,
                expected
            );
        }
    }

    #[test]
    fn stopping_presolve_early_does_not_change_exact_results() {
        fn compositions(total: u32, parts: usize) -> Vec<Vec<u32>> {
            if total == 0 {
                return vec![Vec::new()];
            }
            if parts == 0 {
                return Vec::new();
            }
            (1..=total)
                .flat_map(|first| {
                    compositions(total - first, parts - 1)
                        .into_iter()
                        .map(move |mut rest| {
                            rest.insert(0, first);
                            rest
                        })
                })
                .collect()
        }
        let structure =
            |lambda: &[u32], w: &[u32], lower: Option<&[u32]>, masks: Option<&[u32]>| {
                gt_polytope_dim_impl(lambda, &[], w, None, lower, masks).map(
                    |(dimension, _, _, tight_lower, tight_diagonal)| {
                        (dimension, tight_lower, tight_diagonal)
                    },
                )
            };
        let mut checked = 0;
        for size in 1..=6_u32 {
            for lambda in Partition::all_of_size_bounded(size, 4, size) {
                for w in compositions(size, 5) {
                    let lower = (0..w.len())
                        .map(|label| (label as u32 % lambda.num_parts() as u32) + 1)
                        .collect::<Vec<_>>();
                    let masks = (0..w.len())
                        .map(|label| (label as u32) & 2)
                        .collect::<Vec<_>>();
                    for (lower, masks) in [
                        (None, None),
                        (Some(&lower[..]), None),
                        (None, Some(&masks[..])),
                    ] {
                        let full = structure(lambda.parts(), &w, lower, masks);
                        PRESOLVE_SWEEP_OVERRIDE.with(|limit| limit.set(Some(0)));
                        let unpropagated = structure(lambda.parts(), &w, lower, masks);
                        PRESOLVE_SWEEP_OVERRIDE.with(|limit| limit.set(None));
                        assert_eq!(full, unpropagated, "{lambda:?} {w:?} {lower:?} {masks:?}");
                        checked += 1;
                    }
                }
            }
        }
        assert!(checked > 1000);
    }
}
