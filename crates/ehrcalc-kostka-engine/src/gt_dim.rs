/// Fast computation of the dimension of GT polytopes via the chain model.
///
/// The GT polytope for shape λ/μ with weight w is parameterized by a chain:
///   μ = α⁰ ⊂ α¹ ⊂ … ⊂ αᵏ = λ
/// where each α^{i-1} ⊂ α^i is a horizontal strip of size w_i, and k = len(w).
///
/// Interior levels: α¹, …, α^{k-1}  (k-1 levels, each a partition with ≤ n parts).
/// Total interior entries = (k-1) × n, where n = len(λ).
///
/// For each interior entry α^i_j, the tightest bounds from the boundary values are:
///   lb = max(μ_j,  λ_{j+k-i})     [vertical from bottom, diagonal from top]
///   ub = min(λ_j,  μ_{j-i})       [vertical from top, diagonal from bottom]
/// where out-of-range indices give lb contribution 0 and ub contribution +∞.
///
/// An entry is frozen (lb == ub).  Weight constraints can force additional entries
/// (when a level has exactly 1 free entry), and forced values propagate via
/// interlacing to tighten bounds at neighboring levels.
///
/// Flags (optional, length k = w.len()):
///   upper_flags[ℓ] = f  →  label ℓ+1 appears only in rows 1..=f  (1-indexed)
///                         ⟺  α^{ℓ+1}_j = α^ℓ_j for j (0-indexed) ≥ f
///   lower_flags[ℓ] = g  →  label ℓ+1 appears only in rows g..=n
///                         ⟺  α^{ℓ+1}_j = α^ℓ_j for j (0-indexed) < g-1
/// These are enforced in Pass 3 as bidirectional bound-coupling between adjacent levels.
///
/// The dimension is:
///   dim = Σ_ℓ max(free_ℓ − 1, 0)
/// i.e., free entries minus one weight constraint per active level.

/// Dimension of the GT polytope GT(λ/μ, w), optionally with row flags.
/// Returns `None` if the polytope is empty (infeasible constraints),
/// or `Some(d)` where d = 0 means a single lattice point.
/// Core implementation shared by the public functions below.
fn gt_polytope_dim_impl(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
) -> Option<(usize, Vec<Vec<u32>>, Vec<Vec<u32>>)> {
    let n = lambda.len();
    let k = w.len();

    if n == 0 || k <= 1 {
        return Some((0, vec![], vec![]));
    }

    let n_int = k - 1; // number of interior levels

    // Pad μ to length n with zeros.
    let mut mu_pad = mu.to_vec();
    mu_pad.resize(n, 0);

    // Weight-sum targets: |α^{ℓ+1}| = |μ| + w[0] + … + w[ℓ].
    let mu_sum: u32 = mu_pad.iter().sum();
    let w_prefix: Vec<u32> = {
        let mut v = vec![0u32; k];
        let mut acc = 0u32;
        for (i, &wi) in w.iter().enumerate() {
            acc += wi;
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
    let mut changed = true;
    while changed {
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
            let mut forced_sum = 0u32;
            let mut min_sum = 0u32;
            let mut max_sum = 0u32;
            let mut free_entries: Vec<usize> = Vec::new();

            for j in 0..n {
                if lb[ell][j] >= ub[ell][j] {
                    forced_sum += lb[ell][j];
                    min_sum += lb[ell][j];
                    max_sum += lb[ell][j];
                } else {
                    free_entries.push(j);
                    min_sum += lb[ell][j];
                    max_sum += ub[ell][j];
                }
            }

            if free_entries.is_empty() {
                continue;
            }

            // Infeasible: target outside feasible range → empty polytope.
            if target < min_sum || target > max_sum {
                return None;
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
                let val = target - forced_sum;
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

        // Pass 3: flag constraints.
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
        // The macro deduplicates the coupling logic for both flag types.
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

        // Infeasibility check after all passes.
        for ell in 0..n_int {
            for j in 0..n {
                if lb[ell][j] > ub[ell][j] {
                    return None;
                }
            }
        }
    }

    // Dimension = Σ max(free_count - 1, 0) over all levels.
    let mut dim = 0usize;
    for ell in 0..n_int {
        let free_count = (0..n).filter(|&j| lb[ell][j] < ub[ell][j]).count();
        if free_count > 0 {
            dim += free_count - 1;
        }
    }
    Some((dim, lb, ub))
}

/// Dimension without flags (fast path, no flag overhead).
/// Returns `None` if the polytope is empty, `Some(d)` otherwise.
pub fn gt_polytope_dim(lambda: &[u32], mu: &[u32], w: &[u32]) -> Option<usize> {
    gt_polytope_dim_impl(lambda, mu, w, None, None).map(|(d, _, _)| d)
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
    gt_polytope_dim_impl(lambda, mu, w, upper_flags, lower_flags).map(|(d, _, _)| d)
}

/// Dimension and propagated lb/ub bounds for each interior level.
///
/// Returns `None` if the polytope is empty, or
/// `Some((dim, lb, ub))` where:
/// - `dim` is the polytope dimension
/// - `lb[ell][j]`, `ub[ell][j]` are the tight bounds for `α^{ell+1}[j]`
///   (interior levels `ell = 0..k-2`, columns `j = 0..n-1`).
///
/// Used by `strict_skew_kostka` to identify globally-tight constraints.
pub fn gt_polytope_bounds(
    lambda: &[u32],
    mu: &[u32],
    w: &[u32],
) -> Option<(usize, Vec<Vec<u32>>, Vec<Vec<u32>>)> {
    gt_polytope_dim_impl(lambda, mu, w, None, None)
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
                let max_mu_size = if lam_size >= 2 { lam_size - 2 } else { 0 };
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
}
