use crate::Partition;

use crate::kostka_dp::{kostka, skew_kostka};
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
/// Visit (beta, new_d) pairs, stopping immediately on a visitor error.
fn yamanouchi_extensions<F>(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
    d: &[u32],
    n: usize,
    mut visit: F,
) -> Result<(), String>
where
    F: FnMut(DpState) -> Result<(), String>,
{
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
        &mut visit,
    )
}

#[allow(clippy::too_many_arguments)]
fn yamanouchi_enumerate<F>(
    alpha: &Partition,
    lambda: &Partition,
    remaining: u32,
    d: &[u32],
    row: usize,
    n: usize,
    cumsum: u32,
    increments: &mut Vec<u32>,
    visit: &mut F,
) -> Result<(), String>
where
    F: FnMut(DpState) -> Result<(), String>,
{
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
            visit((Partition::from_sorted(parts), new_d))?;
        }
        return Ok(());
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
            visit,
        )?;
    }
    increments[row] = 0;
    Ok(())
}

// Existing states may merge at capacity; a new key must never exceed it.
fn add_lr_capped(
    map: &mut HashMap<DpState, BigUint>,
    state: DpState,
    count: &BigUint,
    limit: Option<usize>,
) -> Result<(), String> {
    if let Some(value) = map.get_mut(&state) {
        *value += count;
    } else if limit.is_some_and(|limit| map.len() >= limit) {
        return Err(format!(
            "LR DP state count exceeds --max-states {}: a level would exceed the limit",
            limit.expect("checked limit")
        ));
    } else {
        map.insert(state, count.clone());
    }
    Ok(())
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
    let skew_size = lambda.size_wide().saturating_sub(mu.size_wide());
    let w: Vec<u32> = nu.parts().to_vec();
    let w_size: u128 = w.iter().map(|&x| u128::from(x)).sum();

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
    // The first strip has no Yamanouchi restriction: these bounds are
    // redundant because every prefix uses at most the whole strip size.
    yamanouchi_extensions(mu, lambda, w[0], &vec![w[0]; n + 1], n, |state| {
        add_lr_capped(&mut dp, state, &BigUint::one(), max_states)
    })?;

    for &strip_size in w.iter().skip(1) {
        let mut new_dp = HashMap::new();
        for ((alpha, d), count) in &dp {
            yamanouchi_extensions(alpha, lambda, strip_size, d, n, |state| {
                add_lr_capped(&mut new_dp, state, count, max_states)
            })?;
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
    let n = lambda.size_wide().saturating_sub(mu.size_wide());
    let nu_size = nu.size_wide();
    if n != nu_size || !mu.partition_less_equal(lambda) {
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
    let parts = partitions_of(u32::try_from(n).expect("Kostka inversion size exceeds u32"));
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
mod budget_tests {
    use super::*;

    #[test]
    fn initial_level_and_duplicate_keys_respect_capacity() {
        let one = Partition::new(vec![1]);
        assert!(try_lr_dp(&one, &Partition::new(vec![]), &one, Some(0)).is_err());
        assert_eq!(
            try_lr_dp(&one, &Partition::new(vec![]), &one, Some(1)).unwrap(),
            BigUint::one()
        );
        let mut map = HashMap::new();
        let key = (one.clone(), vec![0, 1]);
        add_lr_capped(&mut map, key.clone(), &BigUint::one(), Some(1)).unwrap();
        add_lr_capped(&mut map, key.clone(), &BigUint::one(), Some(1)).unwrap();
        assert_eq!(map[&key], BigUint::from(2_u32));
        assert!(add_lr_capped(&mut map, (one, vec![0, 2]), &BigUint::one(), Some(1)).is_err());
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn streaming_strips_stop_on_budget_error_and_match_kostka_inversion() {
        let lambda = Partition::new((1..=20).rev().collect());
        let mu = Partition::new((0..20).rev().collect());
        let mut visited = 0;
        let result = yamanouchi_extensions(&mu, &lambda, 10, &[10; 21], 20, |_| {
            visited += 1;
            if visited == 2 {
                Err("stop".into())
            } else {
                Ok(())
            }
        });
        assert!(result.is_err());
        assert_eq!(visited, 2);
        for parts in [vec![3, 2, 1], vec![4, 2], vec![3, 3]] {
            let outer = Partition::new(parts);
            let inner = Partition::new(vec![1]);
            for content in partitions_of(5) {
                let weight = Partition::new(content);
                let expected = lr_kostka_inverse(&outer, &inner, &weight, None);
                let actual = try_lr_dp(&outer, &inner, &weight, Some(1000)).unwrap();
                assert_eq!(BigInt::from(actual), expected);
            }
        }
    }
}
