use crate::Partition;

use crate::kostka_dp::{horizontal_strip_extensions, kostka, skew_kostka};
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};
/// Littlewood-Richardson coefficients c^λ_{μ,ν} via two methods:
///
/// 1. **Augmented GT DP**: extends the Kostka DP with Yamanouchi constraints
///    integrated into the horizontal-strip enumeration.
///
/// 2. **Kostka matrix inversion**: uses the identity
///    K(λ/μ, α) = Σ_ν c^λ_{μ,ν} K(ν, α)
///    solved by back-substitution in dominance order (K is upper unitriangular).
use std::collections::HashMap;
use std::time::Instant;

/// One exact Kostka count required by a Kostka-matrix inversion for an LR
/// coefficient.  These jobs are independent and can be evaluated by the CPU
/// implementation or by an exact CRT/GPU backend.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LrKostkaJob {
    pub id: String,
    pub outer: Partition,
    pub inner: Partition,
    pub weight: Vec<u32>,
    /// Certified upper bound obtained by forgetting all tableau inequalities.
    pub upper_bound: BigUint,
}

/// Reusable plan for computing one LR coefficient through Kostka-matrix
/// inversion.
#[derive(Clone, Debug)]
pub struct LrKostkaPlan {
    partitions: Vec<Partition>,
    target_index: Option<usize>,
    trivial_result: Option<BigUint>,
    pub jobs: Vec<LrKostkaJob>,
}

impl LrKostkaPlan {
    /// Return an answer that follows from size/containment checks without any
    /// Kostka jobs.
    pub fn trivial_result(&self) -> Option<&BigUint> {
        self.trivial_result.as_ref()
    }

    /// Reconstruct the requested coefficient from exact job results keyed by
    /// [`LrKostkaJob::id`].
    pub fn reconstruct(&self, counts: &HashMap<String, BigUint>) -> Result<BigUint, String> {
        if let Some(result) = &self.trivial_result {
            return Ok(result.clone());
        }
        let target_index = self
            .target_index
            .ok_or_else(|| "nontrivial LR plan has no target partition".to_string())?;
        let mut coefficients = Vec::<BigInt>::with_capacity(target_index + 1);
        for index in 0..=target_index {
            let skew_id = format!("S:{index}");
            let skew = counts
                .get(&skew_id)
                .ok_or_else(|| format!("missing exact count for job {skew_id}"))?
                .to_bigint()
                .ok_or_else(|| format!("could not convert job {skew_id} to BigInt"))?;
            let mut coefficient = skew;
            for (earlier, earlier_coefficient) in coefficients.iter().enumerate() {
                if earlier_coefficient.is_zero()
                    || !dominates(&self.partitions[earlier], &self.partitions[index])
                {
                    continue;
                }
                let ordinary_id = format!("K:{earlier}:{index}");
                let ordinary = counts
                    .get(&ordinary_id)
                    .ok_or_else(|| format!("missing exact count for job {ordinary_id}"))?
                    .to_bigint()
                    .ok_or_else(|| format!("could not convert job {ordinary_id} to BigInt"))?;
                coefficient -= earlier_coefficient * ordinary;
            }
            coefficients.push(coefficient);
        }
        coefficients[target_index]
            .to_biguint()
            .ok_or_else(|| "Kostka inversion produced a negative LR coefficient".to_string())
    }
}

/// Build the independent Kostka jobs needed to compute `c^lambda_(mu,nu)`.
///
/// Only partitions through `nu` in dominance-compatible reverse-lexicographic
/// order are included.  Ordinary Kostka jobs known to vanish by dominance are
/// omitted.  Each remaining count is bounded by the corresponding multinomial
/// coefficient, which makes modular GPU results exactly reconstructible.
pub fn lr_kostka_plan(
    lambda: &Partition,
    mu: &Partition,
    nu: &Partition,
) -> Result<LrKostkaPlan, String> {
    let skew_size = lambda.size().saturating_sub(mu.size());
    if !mu.partition_less_equal(lambda) || skew_size != nu.size() {
        return Ok(LrKostkaPlan {
            partitions: Vec::new(),
            target_index: None,
            trivial_result: Some(BigUint::zero()),
            jobs: Vec::new(),
        });
    }
    if skew_size == 0 {
        return Ok(LrKostkaPlan {
            partitions: Vec::new(),
            target_index: None,
            trivial_result: Some(if lambda == mu {
                BigUint::one()
            } else {
                BigUint::zero()
            }),
            jobs: Vec::new(),
        });
    }

    let all_partitions = Partition::all_of_size(skew_size);
    let target_index = all_partitions
        .iter()
        .position(|partition| partition == nu)
        .ok_or_else(|| "content is not a partition of the skew size".to_string())?;
    let partitions = all_partitions[..=target_index].to_vec();
    let mut jobs = Vec::new();
    for (index, weight_partition) in partitions.iter().enumerate() {
        let weight = weight_partition.parts().to_vec();
        let upper_bound = multinomial_bound(&weight);
        jobs.push(LrKostkaJob {
            id: format!("S:{index}"),
            outer: lambda.clone(),
            inner: mu.clone(),
            weight: weight.clone(),
            upper_bound: upper_bound.clone(),
        });
        for (earlier, earlier_partition) in partitions.iter().enumerate().take(index) {
            if dominates(earlier_partition, weight_partition) {
                jobs.push(LrKostkaJob {
                    id: format!("K:{earlier}:{index}"),
                    outer: earlier_partition.clone(),
                    inner: Partition::empty(),
                    weight: weight.clone(),
                    upper_bound: upper_bound.clone(),
                });
            }
        }
    }
    Ok(LrKostkaPlan {
        partitions,
        target_index: Some(target_index),
        trivial_result: None,
        jobs,
    })
}

fn dominates(left: &Partition, right: &Partition) -> bool {
    let length = left.num_parts().max(right.num_parts());
    let mut left_sum = 0_u64;
    let mut right_sum = 0_u64;
    for index in 0..length {
        left_sum += u64::from(left.part(index));
        right_sum += u64::from(right.part(index));
        if left_sum < right_sum {
            return false;
        }
    }
    left_sum == right_sum
}

fn multinomial_bound(weight: &[u32]) -> BigUint {
    let total = weight.iter().copied().sum::<u32>();
    let mut result = factorial(total);
    for &part in weight {
        result /= factorial(part);
    }
    result
}

fn factorial(value: u32) -> BigUint {
    (2..=value).fold(BigUint::one(), |product, factor| product * factor)
}

// ── Method 1: Augmented GT DP with Yamanouchi ───────────────────────────────

/// DP state: (current partition α^(k), d-vector).
///
/// d\[j\] = Σ_{l=0}^{j-1} (α^(k)_l − α^(k−1)_l)  for j = 0..=n.
/// d\[0\] = 0, d\[n\] = previous strip size.
///
/// The Yamanouchi condition at step k→k+1 requires:
///   for all j = 0..n-1:  cumulative_increment\[j+1\] ≤ d\[j+1\]
/// where cumulative_increment\[j\] = Σ_{l=0}^{j-1} c\[l\], c\[l\] = α^(k+1)_l − α^(k)_l.
type DpState = (Partition, Vec<u32>);

/// Enumerate horizontal-strip extensions of `alpha` within `lambda` of size `strip_size`,
/// subject to the Yamanouchi constraint parameterised by `d`.
///
/// Returns (beta, new_d) pairs.
fn yamanouchi_extensions(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
    d: &[u32],
    n: usize,
) -> Vec<(Partition, Vec<u32>)> {
    let mut results = Vec::new();
    let mut increments = vec![0u32; n];
    yamanouchi_enumerate(
        alpha,
        lambda,
        strip_size,
        d,
        0,
        n,
        0,
        &mut increments,
        &mut results,
    );
    results
}

#[allow(clippy::too_many_arguments)]
fn yamanouchi_enumerate(
    alpha: &Partition,
    lambda: &Partition,
    remaining: u32,
    d: &[u32],
    row: usize,
    n: usize,
    cumsum: u32,
    increments: &mut Vec<u32>,
    results: &mut Vec<(Partition, Vec<u32>)>,
) {
    if row == n {
        if remaining == 0 {
            let mut parts = vec![0u32; n];
            let mut new_d = vec![0u32; n + 1];
            let mut cs = 0u32;
            for r in 0..n {
                parts[r] = alpha.part(r) + increments[r];
                new_d[r] = cs;
                cs += increments[r];
            }
            new_d[n] = cs;
            results.push((Partition::from_sorted(parts), new_d));
        }
        return;
    }

    // Yamanouchi bound: cumsum_so_far + c[row] ≤ d[row].
    // (At step k→k+1, the cumulative letter-(k+1) count through row `row`
    //  must not exceed the cumulative letter-k count through row `row−1`.)
    let yam_bound = d[row].saturating_sub(cumsum);

    // Standard horizontal-strip bounds.
    let max_from_lambda = lambda.part(row).saturating_sub(alpha.part(row));
    let max_from_strip = if row == 0 {
        remaining
    } else {
        alpha.part(row - 1).saturating_sub(alpha.part(row))
    };
    let max_c = remaining
        .min(max_from_lambda)
        .min(max_from_strip)
        .min(yam_bound);

    for c in 0..=max_c {
        increments[row] = c;
        yamanouchi_enumerate(
            alpha,
            lambda,
            remaining - c,
            d,
            row + 1,
            n,
            cumsum + c,
            increments,
            results,
        );
    }
    increments[row] = 0;
}

/// Compute c^λ_{μ,ν} via the augmented GT DP (Yamanouchi constraints in the DP).
pub fn lr_dp(
    lambda: &Partition,
    mu: &Partition,
    nu: &Partition,
    max_states: Option<usize>,
) -> BigUint {
    try_lr_dp(lambda, mu, nu, max_states).expect("LR DP state limit exceeded")
}

/// Fallible variant of [`lr_dp`] that reports a state-limit breach.
pub fn try_lr_dp(
    lambda: &Partition,
    mu: &Partition,
    nu: &Partition,
    max_states: Option<usize>,
) -> Result<BigUint, String> {
    let n = lambda.num_parts();
    let skew_size = lambda.size().saturating_sub(mu.size());
    let w: Vec<u32> = nu.parts().to_vec();
    let w_size: u32 = w.iter().sum();

    if skew_size != w_size || !mu.partition_less_equal(lambda) {
        return Ok(BigUint::zero());
    }
    if w.is_empty() {
        return Ok(if lambda == mu {
            BigUint::one()
        } else {
            BigUint::zero()
        });
    }

    // Level 0 → 1: no Yamanouchi check (no previous letter to compare against).
    let mut dp: HashMap<DpState, BigUint> = HashMap::new();
    for beta in horizontal_strip_extensions(mu, lambda, w[0]) {
        let mut d = vec![0u32; n + 1];
        let mut cs = 0u32;
        for (j, entry) in d.iter_mut().enumerate().take(n) {
            *entry = cs;
            cs += beta.part(j).saturating_sub(mu.part(j));
        }
        d[n] = cs;
        *dp.entry((beta, d)).or_insert_with(BigUint::zero) += BigUint::one();
    }

    // Levels 1 → 2, 2 → 3, ... with Yamanouchi.
    for &strip_size in w.iter().skip(1) {
        let mut new_dp: HashMap<DpState, BigUint> = HashMap::new();

        for ((alpha, d), count) in &dp {
            for (beta, new_d) in yamanouchi_extensions(alpha, lambda, strip_size, d, n) {
                *new_dp.entry((beta, new_d)).or_insert_with(BigUint::zero) += count;
            }
        }

        if let Some(limit) = max_states {
            if new_dp.len() > limit {
                return Err(format!(
                    "LR DP state count {} exceeds --max-states {}.",
                    new_dp.len(),
                    limit
                ));
            }
        }

        dp = new_dp;
    }

    // Sum over all final states reaching λ.
    Ok(dp
        .into_iter()
        .filter(|((part, _), _)| part == lambda)
        .map(|(_, count)| count)
        .sum())
}

// ── Method 2: Kostka matrix back-substitution ───────────────────────────────
//
// Identity:  K(λ/μ, α) = Σ_ν c^λ_{μ,ν} K(ν, α)     for all partitions α ⊢ n.
//
// The Kostka matrix K[ν][α] is upper unitriangular in dominance order, so
// back-substitution from the top gives:
//   c_ν = K(λ/μ, ν) − Σ_{ν' ⊳ ν} c_{ν'} · K(ν', ν)

/// Generate all partitions of `n` in reverse lexicographic order (largest first).
fn partitions_of(n: u32) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    fn helper(n: u32, max_part: u32, cur: &mut Vec<u32>, out: &mut Vec<Vec<u32>>) {
        if n == 0 {
            out.push(cur.clone());
            return;
        }
        for i in (1..=n.min(max_part)).rev() {
            cur.push(i);
            helper(n - i, i, cur, out);
            cur.pop();
        }
    }
    helper(n, n, &mut Vec::new(), &mut result);
    result
}

/// Compute c^λ_{μ,ν} by inverting the Kostka matrix via back-substitution.
///
/// For each partition α ⊢ n, computes K(λ/μ, α) and K(ν', α) for all ν' ⊵ α.
/// Then solves c by back-substitution in dominance order (reverse lex).
pub fn lr_kostka_inverse(
    lambda: &Partition,
    mu: &Partition,
    nu: &Partition,
    max_states: Option<usize>,
) -> BigInt {
    let n = lambda.size().saturating_sub(mu.size());
    let nu_size = nu.size();
    if n != nu_size {
        return BigInt::zero();
    }
    if n == 0 {
        return if lambda == mu {
            BigInt::one()
        } else {
            BigInt::zero()
        };
    }

    // All partitions of n in reverse-lex order (dominance-compatible).
    let parts = partitions_of(n);
    let num_parts = parts.len();

    // Map partition → index.
    let mut idx_of: HashMap<Vec<u32>, usize> = HashMap::new();
    for (i, p) in parts.iter().enumerate() {
        idx_of.insert(p.clone(), i);
    }

    // Skew Kostka: K(λ/μ, α) for each partition α.
    let skew_k: Vec<BigInt> = parts
        .iter()
        .map(|alpha| {
            skew_kostka(lambda, mu, alpha, max_states, true)
                .to_bigint()
                .unwrap()
        })
        .collect();

    // Back-substitution.
    let mut c: Vec<BigInt> = vec![BigInt::zero(); num_parts];

    for i in 0..num_parts {
        let mut val = skew_k[i].clone();

        // Subtract contributions from partitions processed earlier (j < i ⟹ parts[j] ⊵ parts[i]).
        // K(parts[j], parts[i]) is automatically 0 when parts[j] ⊁ parts[i].
        for j in 0..i {
            if c[j].is_zero() {
                continue;
            }
            let nu_pp = Partition::from_sorted(parts[j].clone());
            let k = kostka(&nu_pp, &parts[i], max_states);
            if !k.is_zero() {
                val -= &c[j] * k.to_bigint().unwrap();
            }
        }

        c[i] = val;
    }

    // Look up ν in the partition list.
    let nu_key = nu.parts().to_vec();
    idx_of
        .get(&nu_key)
        .map(|&i| c[i].clone())
        .unwrap_or_else(BigInt::zero)
}

/// Compute and display c^λ_{μ,ν} via both methods, cross-checking the results.
pub fn run(
    lambda: &Partition,
    mu: &Partition,
    nu: &Partition,
    format: &str,
    max_states: Option<usize>,
) {
    let t0 = Instant::now();
    let c_dp = lr_dp(lambda, mu, nu, max_states);
    let dp_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let t1 = Instant::now();
    let c_inv = lr_kostka_inverse(lambda, mu, nu, max_states);
    let inv_ms = t1.elapsed().as_secs_f64() * 1000.0;

    let agree = c_dp.to_bigint().unwrap() == c_inv;

    match format {
        "json" => {
            println!(
                "{{\"lambda\":{},\"mu\":{},\"nu\":{},\"lr\":\"{}\",\"dp_ms\":{:.1},\"inv_ms\":{:.1},\"agree\":{}}}",
                json_parts(lambda), json_parts(mu), json_parts(nu),
                c_dp, dp_ms, inv_ms, agree
            );
        }
        _ => {
            println!("c^({})_({},{}) = {}", lambda, mu, nu, c_dp);
            println!("  GT DP (Yamanouchi):     {}  ({:.1}ms)", c_dp, dp_ms);
            println!("  Kostka matrix inverse:  {}  ({:.1}ms)", c_inv, inv_ms);
            if !agree {
                println!("  *** MISMATCH ***");
            }
        }
    }
}

fn json_parts(p: &Partition) -> String {
    format!(
        "[{}]",
        p.parts()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planned_kostka_inversion_matches_yamanouchi_dp() {
        let cases = [
            (vec![2, 1], vec![1], vec![2]),
            (vec![3, 2], vec![1], vec![2, 1, 1]),
            (vec![3, 2, 1], vec![1, 1], vec![2, 1, 1]),
        ];
        for (lambda, mu, nu) in cases {
            let lambda = Partition::new(lambda);
            let mu = Partition::new(mu);
            let nu = Partition::new(nu);
            let plan = lr_kostka_plan(&lambda, &mu, &nu).expect("valid LR plan");
            let mut counts = HashMap::new();
            for job in &plan.jobs {
                let count = skew_kostka(&job.outer, &job.inner, &job.weight, None, true);
                assert!(count <= job.upper_bound);
                counts.insert(job.id.clone(), count);
            }
            assert_eq!(
                plan.reconstruct(&counts).expect("planned reconstruction"),
                lr_dp(&lambda, &mu, &nu, None)
            );
        }
    }

    #[test]
    fn planned_inversion_handles_trivial_zero() {
        let plan = lr_kostka_plan(
            &Partition::new(vec![2]),
            &Partition::new(vec![1]),
            &Partition::new(vec![2]),
        )
        .expect("trivial plan");
        assert_eq!(plan.trivial_result(), Some(&BigUint::zero()));
        assert!(plan.jobs.is_empty());
    }
}
