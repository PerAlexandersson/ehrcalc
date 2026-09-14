use crate::Partition;
use num_bigint::BigUint;
use num_traits::{One, Zero};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KostkaDpStats {
    pub value: BigUint,
    pub peak_states: usize,
    pub level_states: Vec<usize>,
}

/// Level-by-level DP for computing skew Kostka coefficients K(lambda/mu, w).
///
/// A SSYT of shape lambda/mu and content w = (w_1,...,w_k) corresponds bijectively to a chain
///   mu = alpha^0 ⊂ alpha^1 ⊂ ... ⊂ alpha^k = lambda
/// where each alpha^i / alpha^{i-1} is a horizontal strip of size w_i.
///
/// DP state at level i: HashMap<Partition, BigUint> — number of paths from mu to each alpha.
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

type PartitionBuildHasher = BuildHasherDefault<PartitionKeyHasher>;
type PartitionCountMap = HashMap<Partition, BigUint, PartitionBuildHasher>;

#[derive(Clone, Copy, Debug)]
struct PartitionKeyHasher {
    state: u64,
}

impl Default for PartitionKeyHasher {
    fn default() -> Self {
        Self {
            state: 0x9e37_79b9_7f4a_7c15,
        }
    }
}

impl Hasher for PartitionKeyHasher {
    fn finish(&self) -> u64 {
        avalanche(self.state)
    }

    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut value = 0u64;
            for (shift, byte) in chunk.iter().enumerate() {
                value |= u64::from(*byte) << (8 * shift);
            }
            self.mix(value);
        }
    }

    fn write_u8(&mut self, value: u8) {
        self.mix(u64::from(value));
    }

    fn write_u16(&mut self, value: u16) {
        self.mix(u64::from(value));
    }

    fn write_u32(&mut self, value: u32) {
        self.mix(u64::from(value));
    }

    fn write_u64(&mut self, value: u64) {
        self.mix(value);
    }

    fn write_u128(&mut self, value: u128) {
        self.mix(value as u64);
        self.mix((value >> 64) as u64);
    }

    fn write_usize(&mut self, value: usize) {
        self.mix(value as u64);
    }

    fn write_i8(&mut self, value: i8) {
        self.mix(value as u64);
    }

    fn write_i16(&mut self, value: i16) {
        self.mix(value as u64);
    }

    fn write_i32(&mut self, value: i32) {
        self.mix(value as u64);
    }

    fn write_i64(&mut self, value: i64) {
        self.mix(value as u64);
    }

    fn write_i128(&mut self, value: i128) {
        self.write_u128(value as u128);
    }

    fn write_isize(&mut self, value: isize) {
        self.mix(value as u64);
    }
}

impl PartitionKeyHasher {
    fn mix(&mut self, value: u64) {
        self.state = self
            .state
            .wrapping_mul(0x9e37_79b9_7f4a_7c15)
            .wrapping_add(value.rotate_left(27));
    }
}

fn avalanche(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn partition_count_map() -> PartitionCountMap {
    HashMap::with_hasher(PartitionBuildHasher::default())
}

fn partition_size_u64(partition: &Partition) -> u64 {
    partition.parts().iter().map(|&part| u64::from(part)).sum()
}

fn weight_size_u64(weight: &[u32]) -> u64 {
    weight.iter().map(|&part| u64::from(part)).sum()
}

fn horizontal_strip_row_capacity(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
    row: usize,
    row_lo: usize,
    row_hi: usize,
) -> u32 {
    if row < row_lo || row >= row_hi {
        return 0;
    }

    let base = alpha.part(row);
    let shape_capacity = lambda.part(row).saturating_sub(base);
    let strip_capacity = if row == 0 {
        strip_size
    } else {
        alpha.part(row - 1).saturating_sub(base)
    };
    shape_capacity.min(strip_capacity)
}

fn horizontal_strip_suffix_capacities(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
    n_rows: usize,
    row_lo: usize,
    row_hi: usize,
) -> Vec<u32> {
    let mut capacities = vec![0_u32; n_rows + 1];
    for row in (0..n_rows).rev() {
        capacities[row] = capacities[row + 1]
            .saturating_add(horizontal_strip_row_capacity(
                alpha, lambda, strip_size, row, row_lo, row_hi,
            ))
            .min(strip_size);
    }
    capacities
}

/// Legacy vector-building enumerator kept for correctness checks and benchmarking.
pub fn horizontal_strip_extensions_legacy(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
) -> Vec<Partition> {
    if strip_size == 0 {
        return vec![alpha.clone()];
    }
    let n_rows = lambda.num_parts();
    if n_rows == 0 {
        return vec![];
    }

    let mut results = Vec::new();
    let mut increments = vec![0u32; n_rows];
    enumerate_strips(
        alpha,
        lambda,
        strip_size,
        0,
        n_rows,
        &mut increments,
        &mut results,
    );
    results
}

/// Enumerate all partitions beta such that beta/alpha is a horizontal strip of size `strip_size`,
/// with alpha ⊆ beta ⊆ lambda (containment constraint).
///
/// This public helper keeps the legacy `Vec`-returning interface used elsewhere in the crate.
pub fn horizontal_strip_extensions(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
) -> Vec<Partition> {
    horizontal_strip_extensions_legacy(alpha, lambda, strip_size)
}

fn enumerate_strips(
    alpha: &Partition,
    lambda: &Partition,
    remaining: u32,
    row: usize,
    n_rows: usize,
    increments: &mut Vec<u32>,
    results: &mut Vec<Partition>,
) {
    if row == n_rows {
        if remaining == 0 {
            // Build beta from alpha + increments.
            let mut parts = vec![0u32; n_rows];
            for r in 0..n_rows {
                parts[r] = alpha.part(r) + increments[r];
            }
            results.push(Partition::from_sorted(parts));
        }
        return;
    }

    // Max increment at this row:
    //   - can't exceed lambda[row] - alpha[row]  (stay in lambda)
    //   - horizontal strip: c[row] ≤ alpha[row-1] - alpha[row]  (for row > 0)
    //   - can't exceed remaining
    let max_from_lambda = lambda.part(row).saturating_sub(alpha.part(row));
    let max_from_strip = if row == 0 {
        remaining // no upper-row constraint for the first row
    } else {
        alpha.part(row - 1).saturating_sub(alpha.part(row))
    };
    let max_c = remaining.min(max_from_lambda).min(max_from_strip);

    for c in 0..=max_c {
        increments[row] = c;
        enumerate_strips(
            alpha,
            lambda,
            remaining - c,
            row + 1,
            n_rows,
            increments,
            results,
        );
    }
    increments[row] = 0;
}

fn for_each_horizontal_strip_extension<F>(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
    mut visit: F,
) where
    F: FnMut(Partition),
{
    if strip_size == 0 {
        visit(alpha.clone());
        return;
    }

    let n_rows = lambda.num_parts();
    if n_rows == 0 {
        return;
    }

    let mut parts = alpha.parts().to_vec();
    parts.resize(n_rows, 0);
    let suffix_capacity =
        horizontal_strip_suffix_capacities(alpha, lambda, strip_size, n_rows, 0, n_rows);
    enumerate_strips_streaming(
        alpha,
        lambda,
        strip_size,
        0,
        n_rows,
        &suffix_capacity,
        &mut parts,
        &mut visit,
    );
}

#[allow(clippy::too_many_arguments)]
fn enumerate_strips_streaming<F>(
    alpha: &Partition,
    lambda: &Partition,
    remaining: u32,
    row: usize,
    n_rows: usize,
    suffix_capacity: &[u32],
    parts: &mut Vec<u32>,
    visit: &mut F,
) where
    F: FnMut(Partition),
{
    if row == n_rows {
        if remaining == 0 {
            visit(Partition::from_sorted(parts.clone()));
        }
        return;
    }

    if remaining > suffix_capacity[row] {
        return;
    }

    let base = alpha.part(row);
    let max_c = remaining.min(horizontal_strip_row_capacity(
        alpha, lambda, remaining, row, 0, n_rows,
    ));
    let min_c = remaining.saturating_sub(suffix_capacity[row + 1]);

    for c in min_c..=max_c {
        parts[row] = base + c;
        enumerate_strips_streaming(
            alpha,
            lambda,
            remaining - c,
            row + 1,
            n_rows,
            suffix_capacity,
            parts,
            visit,
        );
    }
    parts[row] = base;
}

/// Compute K(lambda/mu, w) using the level-by-level DP.
/// Returns 0 if lambda/mu/w are incompatible (sizes don't match, mu not contained in lambda, etc.).
///
/// `max_states`: if Some(limit), abort with an error message if any DP level exceeds `limit` states.
/// `sort_weight`: if true, sort w in decreasing order before the DP to minimise peak state count.
///   Only pass true when no flag bounds are active (sorting is always valid for K, but callers
///   that use w-ordering for degree/flag bookkeeping should pass false).
pub fn skew_kostka_legacy(
    lambda: &Partition,
    mu: &Partition, // inner shape; use Partition::empty() for non-skew
    w: &[u32],      // weight composition
    max_states: Option<usize>,
    sort_weight: bool,
) -> BigUint {
    // Validate.
    let skew_size: u32 = lambda.size().saturating_sub(mu.size());
    let w_size: u32 = w.iter().sum();
    if skew_size != w_size {
        return BigUint::zero();
    }
    if !mu.partition_less_equal(lambda) {
        return BigUint::zero();
    }

    // Optionally sort weight descending to reduce peak intermediate state count.
    let mut w_sorted;
    let w_eff: &[u32] = if sort_weight {
        w_sorted = w.to_vec();
        w_sorted.sort_unstable_by(|a, b| b.cmp(a));
        &w_sorted
    } else {
        w
    };

    // Initial DP state: the single partition mu with count 1.
    let mut dp = partition_count_map();
    dp.insert(mu.clone(), BigUint::one());

    for &strip_size in w_eff {
        if strip_size == 0 {
            // No boxes added; state is unchanged.
            continue;
        }

        // Build new DP map by merging extensions directly, avoiding an intermediate Vec.
        let mut new_dp = partition_count_map();
        for (alpha, count) in &dp {
            for beta in horizontal_strip_extensions_legacy(alpha, lambda, strip_size) {
                *new_dp.entry(beta).or_insert_with(BigUint::zero) += count;
            }
        }

        if let Some(limit) = max_states {
            if new_dp.len() > limit {
                panic!(
                    "DP state count {} exceeds --max-states {}. \
                     Use a smaller input or raise the limit.",
                    new_dp.len(),
                    limit
                );
            }
        }

        dp = new_dp;
    }

    // The answer is the count at lambda.
    dp.remove(lambda).unwrap_or_else(BigUint::zero)
}

pub fn skew_kostka(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    max_states: Option<usize>,
    sort_weight: bool,
) -> BigUint {
    skew_kostka_stats(lambda, mu, w, max_states, sort_weight).value
}

/// Fallible variant of [`skew_kostka`] that reports a state-limit breach.
pub fn try_skew_kostka(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    max_states: Option<usize>,
    sort_weight: bool,
) -> Result<BigUint, String> {
    Ok(try_skew_kostka_stats(lambda, mu, w, max_states, sort_weight)?.value)
}

pub fn skew_kostka_stats(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    max_states: Option<usize>,
    sort_weight: bool,
) -> KostkaDpStats {
    try_skew_kostka_stats(lambda, mu, w, max_states, sort_weight)
        .expect("Kostka DP state limit exceeded")
}

/// Fallible variant of [`skew_kostka_stats`] that reports a state-limit breach.
pub fn try_skew_kostka_stats(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    max_states: Option<usize>,
    sort_weight: bool,
) -> Result<KostkaDpStats, String> {
    if !mu.partition_less_equal(lambda) {
        return Ok(KostkaDpStats {
            value: BigUint::zero(),
            peak_states: 0,
            level_states: Vec::new(),
        });
    }
    let skew_size = partition_size_u64(lambda) - partition_size_u64(mu);
    if skew_size != weight_size_u64(w) {
        return Ok(KostkaDpStats {
            value: BigUint::zero(),
            peak_states: 0,
            level_states: Vec::new(),
        });
    }

    let mut w_sorted;
    let w_eff: &[u32] = if sort_weight {
        w_sorted = w.to_vec();
        w_sorted.sort_unstable_by(|a, b| b.cmp(a));
        &w_sorted
    } else {
        w
    };

    let mut dp = partition_count_map();
    dp.insert(mu.clone(), BigUint::one());
    let mut peak_states = dp.len();
    let mut level_states = vec![dp.len()];

    for &strip_size in w_eff {
        if strip_size == 0 {
            level_states.push(dp.len());
            continue;
        }

        let mut new_dp = partition_count_map();
        for (alpha, count) in &dp {
            for_each_horizontal_strip_extension(alpha, lambda, strip_size, |beta| {
                *new_dp.entry(beta).or_insert_with(BigUint::zero) += count;
            });
        }

        if let Some(limit) = max_states {
            if new_dp.len() > limit {
                return Err(format!(
                    "DP state count {} exceeds --max-states {}. Use a smaller input or raise the limit.",
                    new_dp.len(), limit
                ));
            }
        }

        peak_states = peak_states.max(new_dp.len());
        level_states.push(new_dp.len());
        dp = new_dp;
    }

    Ok(KostkaDpStats {
        value: dp.remove(lambda).unwrap_or_else(BigUint::zero),
        peak_states,
        level_states,
    })
}

/// Convenience wrapper for non-skew K(lambda, w).
pub fn kostka(lambda: &Partition, w: &[u32], max_states: Option<usize>) -> BigUint {
    skew_kostka(lambda, &Partition::empty(), w, max_states, true)
}

// ── Flagged Kostka numbers ────────────────────────────────────────────────────
//
// A flagged SSYT restricts which rows each label may appear in:
//   upper_flags[i] = f  →  label i+1 only in rows 1..=f  (1-indexed)
//                      ⟺  strip i must have c[j] = 0 for j ≥ f  (0-indexed)
//   lower_flags[i] = g  →  label i+1 only in rows g..=n
//                      ⟺  strip i must have c[j] = 0 for j < g-1 (0-indexed)
//
// Both flags are per-step (length k = len(w)).  Missing entries mean no restriction.
// The weight cannot be sorted when flags are active.

/// Legacy vector-building restricted enumerator kept for checks and benchmarks.
#[allow(dead_code)]
fn horizontal_strip_extensions_restricted_legacy(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
    row_lo: usize,
    row_hi: usize,
) -> Vec<Partition> {
    if strip_size == 0 {
        return vec![alpha.clone()];
    }
    let n_rows = lambda.num_parts();
    if n_rows == 0 {
        return vec![];
    }
    let mut results = Vec::new();
    let mut increments = vec![0u32; n_rows];
    enumerate_strips_restricted(
        alpha,
        lambda,
        strip_size,
        0,
        n_rows,
        row_lo,
        row_hi,
        &mut increments,
        &mut results,
    );
    results
}

#[allow(dead_code, clippy::too_many_arguments)]
fn enumerate_strips_restricted(
    alpha: &Partition,
    lambda: &Partition,
    remaining: u32,
    row: usize,
    n_rows: usize,
    row_lo: usize,
    row_hi: usize,
    increments: &mut Vec<u32>,
    results: &mut Vec<Partition>,
) {
    if row == n_rows {
        if remaining == 0 {
            let mut parts = vec![0u32; n_rows];
            for r in 0..n_rows {
                parts[r] = alpha.part(r) + increments[r];
            }
            results.push(Partition::from_sorted(parts));
        }
        return;
    }

    // Rows outside the allowed range: forced zero.
    if row < row_lo || row >= row_hi {
        increments[row] = 0;
        enumerate_strips_restricted(
            alpha,
            lambda,
            remaining,
            row + 1,
            n_rows,
            row_lo,
            row_hi,
            increments,
            results,
        );
        return;
    }

    let max_from_lambda = lambda.part(row).saturating_sub(alpha.part(row));
    let max_from_strip = if row == 0 {
        remaining
    } else {
        alpha.part(row - 1).saturating_sub(alpha.part(row))
    };
    let max_c = remaining.min(max_from_lambda).min(max_from_strip);

    for c in 0..=max_c {
        increments[row] = c;
        enumerate_strips_restricted(
            alpha,
            lambda,
            remaining - c,
            row + 1,
            n_rows,
            row_lo,
            row_hi,
            increments,
            results,
        );
    }
    increments[row] = 0;
}

fn for_each_horizontal_strip_extension_restricted<F>(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
    row_lo: usize,
    row_hi: usize,
    mut visit: F,
) where
    F: FnMut(Partition),
{
    if strip_size == 0 {
        visit(alpha.clone());
        return;
    }

    let n_rows = lambda.num_parts();
    if n_rows == 0 {
        return;
    }

    let mut parts = alpha.parts().to_vec();
    parts.resize(n_rows, 0);
    let suffix_capacity =
        horizontal_strip_suffix_capacities(alpha, lambda, strip_size, n_rows, row_lo, row_hi);
    enumerate_strips_restricted_streaming(
        alpha,
        lambda,
        strip_size,
        0,
        n_rows,
        row_lo,
        row_hi,
        &suffix_capacity,
        &mut parts,
        &mut visit,
    );
}

#[allow(clippy::too_many_arguments)]
fn enumerate_strips_restricted_streaming<F>(
    alpha: &Partition,
    lambda: &Partition,
    remaining: u32,
    row: usize,
    n_rows: usize,
    row_lo: usize,
    row_hi: usize,
    suffix_capacity: &[u32],
    parts: &mut Vec<u32>,
    visit: &mut F,
) where
    F: FnMut(Partition),
{
    if row == n_rows {
        if remaining == 0 {
            visit(Partition::from_sorted(parts.clone()));
        }
        return;
    }

    if remaining > suffix_capacity[row] {
        return;
    }

    let base = alpha.part(row);
    let max_c = remaining.min(horizontal_strip_row_capacity(
        alpha, lambda, remaining, row, row_lo, row_hi,
    ));
    let min_c = remaining.saturating_sub(suffix_capacity[row + 1]);

    for c in min_c..=max_c {
        parts[row] = base + c;
        enumerate_strips_restricted_streaming(
            alpha,
            lambda,
            remaining - c,
            row + 1,
            n_rows,
            row_lo,
            row_hi,
            suffix_capacity,
            parts,
            visit,
        );
    }
    parts[row] = base;
}

/// Compute the flagged skew Kostka coefficient K_flags(lambda/mu, w).
///
/// `upper_flags[i] = f`: label i+1 restricted to rows 1..=f (1-indexed).
/// `lower_flags[i] = g`: label i+1 restricted to rows g..=n (1-indexed).
/// Flags are per-step; lengths should equal len(w).  Missing entries = no restriction.
#[allow(dead_code)]
pub fn flagged_skew_kostka_legacy(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    max_states: Option<usize>,
) -> BigUint {
    let skew_size: u32 = lambda.size().saturating_sub(mu.size());
    let w_size: u32 = w.iter().sum();
    if skew_size != w_size {
        return BigUint::zero();
    }
    if !mu.partition_less_equal(lambda) {
        return BigUint::zero();
    }

    let n = lambda.num_parts();
    let mut dp = partition_count_map();
    dp.insert(mu.clone(), BigUint::one());

    for (i, &strip_size) in w.iter().enumerate() {
        if strip_size == 0 {
            continue;
        }

        // Allowed row range for this step (0-indexed, exclusive upper bound).
        let row_lo = lower_flags
            .and_then(|lf| lf.get(i))
            .map(|&g| (g as usize).saturating_sub(1).min(n))
            .unwrap_or(0);
        let row_hi = upper_flags
            .and_then(|uf| uf.get(i))
            .map(|&f| (f as usize).min(n))
            .unwrap_or(n);

        let mut new_dp = partition_count_map();
        for (alpha, count) in &dp {
            for beta in horizontal_strip_extensions_restricted_legacy(
                alpha, lambda, strip_size, row_lo, row_hi,
            ) {
                *new_dp.entry(beta).or_insert_with(BigUint::zero) += count;
            }
        }

        if let Some(limit) = max_states {
            if new_dp.len() > limit {
                panic!(
                    "DP state count {} exceeds --max-states {}.",
                    new_dp.len(),
                    limit
                );
            }
        }

        dp = new_dp;
    }

    dp.remove(lambda).unwrap_or_else(BigUint::zero)
}

pub fn flagged_skew_kostka(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    max_states: Option<usize>,
) -> BigUint {
    try_flagged_skew_kostka(lambda, mu, w, upper_flags, lower_flags, max_states)
        .expect("flagged Kostka DP state limit exceeded")
}

/// Fallible variant of [`flagged_skew_kostka`] that reports a state-limit breach.
pub fn try_flagged_skew_kostka(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    max_states: Option<usize>,
) -> Result<BigUint, String> {
    if !mu.partition_less_equal(lambda) {
        return Ok(BigUint::zero());
    }
    let skew_size = partition_size_u64(lambda) - partition_size_u64(mu);
    if skew_size != weight_size_u64(w) {
        return Ok(BigUint::zero());
    }

    let n = lambda.num_parts();
    let mut dp = partition_count_map();
    dp.insert(mu.clone(), BigUint::one());

    for (i, &strip_size) in w.iter().enumerate() {
        if strip_size == 0 {
            continue;
        }

        let row_lo = lower_flags
            .and_then(|lf| lf.get(i))
            .map(|&g| (g as usize).saturating_sub(1).min(n))
            .unwrap_or(0);
        let row_hi = upper_flags
            .and_then(|uf| uf.get(i))
            .map(|&f| (f as usize).min(n))
            .unwrap_or(n);

        let mut new_dp = partition_count_map();
        for (alpha, count) in &dp {
            for_each_horizontal_strip_extension_restricted(
                alpha,
                lambda,
                strip_size,
                row_lo,
                row_hi,
                |beta| {
                    *new_dp.entry(beta).or_insert_with(BigUint::zero) += count;
                },
            );
        }

        if let Some(limit) = max_states {
            if new_dp.len() > limit {
                return Err(format!(
                    "DP state count {} exceeds --max-states {}.",
                    new_dp.len(),
                    limit
                ));
            }
        }

        dp = new_dp;
    }

    Ok(dp.remove(lambda).unwrap_or_else(BigUint::zero))
}

// ── Strict (interior) counting for Ehrhart-Macdonald reciprocity ─────────────
//
// A chain μ=α⁰ ⊂ α¹ ⊂ … ⊂ αᵏ=λ is in the *relative interior* of the
// GT-polytope iff every interlacing constraint that is NOT in the affine hull
// of the polytope holds strictly.
//
// A constraint (between adjacent chain levels) is in the affine hull iff it is
// always an equality over all feasible chains.  After full propagation by the
// dimension algorithm (gt_polytope_bounds), this is equivalent to both
// endpoints being frozen at the same value:
//   globally_tight = (lb_src == ub_src) && (lb_dst == ub_dst) && (lb_src == lb_dst)
//
// The two constraints per step i (0-indexed), row j:
//   lower[i][j]:   α^{i+1}[j] >= α^i[j]
//   diagonal[i][j] (j≥1): α^{i+1}[j] <= α^i[j-1]
//
// For non-globally-tight lower:   require c[j] = β[j] - α[j] >= 1
// For non-globally-tight diagonal: require α[j-1] - β[j] >= 1
//
// By Ehrhart-Macdonald reciprocity:
//   (-1)^d * strict_skew_kostka(t·λ, t·μ, t·w) = P_Ehrhart(-t)

/// Enumerate horizontal strip extensions β of α with per-row strictness flags.
///
/// `strict_lower[j]`: require β[j] > α[j] (c[j] ≥ 1).
/// `strict_diag[j]`  (j≥1): require α[j-1] > β[j] (c[j] ≤ gap - 1).
fn strict_horizontal_strip_extensions_legacy(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
    strict_lower: &[bool],
    strict_diag: &[bool],
) -> Vec<Partition> {
    let n_rows = lambda.num_parts();
    if n_rows == 0 {
        return vec![];
    }

    // Minimum boxes needed at each suffix for pruning.
    let mut min_needed_suffix = vec![0u32; n_rows + 1];
    for r in (0..n_rows).rev() {
        min_needed_suffix[r] = min_needed_suffix[r + 1]
            + if r < strict_lower.len() && strict_lower[r] {
                1
            } else {
                0
            };
    }

    if strip_size < min_needed_suffix[0] {
        return vec![];
    }

    let mut results = Vec::new();
    let mut increments = vec![0u32; n_rows];
    enumerate_strips_strict_new(
        alpha,
        lambda,
        strip_size,
        0,
        n_rows,
        strict_lower,
        strict_diag,
        &min_needed_suffix,
        &mut increments,
        &mut results,
    );
    results
}

#[allow(clippy::too_many_arguments)]
fn constrained_horizontal_strip_row_bounds(
    alpha: &Partition,
    lambda: &Partition,
    row: usize,
    strict_lower: &[bool],
    strict_diag: &[bool],
    row_lo: usize,
    row_hi: usize,
    forbidden_mask: u32,
) -> Option<(u32, u32)> {
    let forced_zero = row < row_lo || row >= row_hi || forbidden_mask & (1_u32 << row) != 0;
    let need_strict_lower = row < strict_lower.len() && strict_lower[row];
    let need_strict_diag = row > 0 && row < strict_diag.len() && strict_diag[row];

    let base = alpha.part(row);
    let gap = if row == 0 {
        u32::MAX
    } else {
        alpha.part(row - 1).saturating_sub(base)
    };
    if forced_zero {
        if need_strict_lower || (need_strict_diag && gap == 0) {
            return None;
        }
        return Some((0, 0));
    }

    let minimum = u32::from(need_strict_lower);
    let shape_capacity = lambda.part(row).saturating_sub(base);
    let diagonal_capacity = if need_strict_diag {
        gap.checked_sub(1)?
    } else {
        gap
    };
    let maximum = shape_capacity.min(diagonal_capacity);
    (minimum <= maximum).then_some((minimum, maximum))
}

#[allow(clippy::too_many_arguments)]
fn enumerate_strips_strict_new(
    alpha: &Partition,
    lambda: &Partition,
    remaining: u32,
    row: usize,
    n_rows: usize,
    strict_lower: &[bool],
    strict_diag: &[bool],
    min_needed_suffix: &[u32],
    increments: &mut Vec<u32>,
    results: &mut Vec<Partition>,
) {
    if row == n_rows {
        if remaining == 0 {
            let mut parts = vec![0u32; n_rows];
            for r in 0..n_rows {
                parts[r] = alpha.part(r) + increments[r];
            }
            results.push(Partition::from_sorted(parts));
        }
        return;
    }

    // Prune: not enough remaining to satisfy all strict-lower rows ahead.
    if remaining < min_needed_suffix[row] {
        return;
    }

    let need_strict_lower = row < strict_lower.len() && strict_lower[row];
    let need_strict_diag = row > 0 && row < strict_diag.len() && strict_diag[row];

    let min_c: u32 = if need_strict_lower { 1 } else { 0 };

    let max_from_lambda = lambda.part(row).saturating_sub(alpha.part(row));

    // Regular horizontal-strip diagonal bound.
    let gap = if row == 0 {
        u32::MAX
    } else {
        alpha.part(row - 1).saturating_sub(alpha.part(row))
    };
    // Apply strict diagonal: reduce gap by 1.
    let max_from_diag = if need_strict_diag {
        gap.saturating_sub(1)
    } else {
        gap
    };

    // Can't use more than remaining minus what future strict-lower rows still need.
    let max_for_row = remaining.saturating_sub(min_needed_suffix[row + 1]);

    let max_c = remaining
        .min(max_from_lambda)
        .min(max_from_diag)
        .min(max_for_row);

    if max_c < min_c {
        return;
    }

    for c in min_c..=max_c {
        increments[row] = c;
        enumerate_strips_strict_new(
            alpha,
            lambda,
            remaining - c,
            row + 1,
            n_rows,
            strict_lower,
            strict_diag,
            min_needed_suffix,
            increments,
            results,
        );
    }
    increments[row] = 0;
}

#[allow(clippy::too_many_arguments)]
fn for_each_strict_horizontal_strip_extension_restricted<F>(
    alpha: &Partition,
    lambda: &Partition,
    strip_size: u32,
    strict_lower: &[bool],
    strict_diag: &[bool],
    row_lo: usize,
    row_hi: usize,
    forbidden_mask: u32,
    mut visit: F,
) where
    F: FnMut(Partition),
{
    let n_rows = lambda.num_parts();
    if n_rows == 0 {
        return;
    }

    let mut row_minimum = vec![0_u32; n_rows];
    let mut row_maximum = vec![0_u32; n_rows];
    for row in 0..n_rows {
        let Some((minimum, maximum)) = constrained_horizontal_strip_row_bounds(
            alpha,
            lambda,
            row,
            strict_lower,
            strict_diag,
            row_lo,
            row_hi,
            forbidden_mask,
        ) else {
            return;
        };
        row_minimum[row] = minimum;
        row_maximum[row] = maximum;
    }

    let mut min_needed_suffix = vec![0_u32; n_rows + 1];
    let mut max_possible_suffix = vec![0_u32; n_rows + 1];
    for r in (0..n_rows).rev() {
        min_needed_suffix[r] = min_needed_suffix[r + 1].saturating_add(row_minimum[r]);
        max_possible_suffix[r] = max_possible_suffix[r + 1]
            .saturating_add(row_maximum[r])
            .min(strip_size);
    }

    if strip_size < min_needed_suffix[0] || strip_size > max_possible_suffix[0] {
        return;
    }

    let mut parts = alpha.parts().to_vec();
    parts.resize(n_rows, 0);
    enumerate_strips_strict_streaming(
        alpha,
        strip_size,
        0,
        n_rows,
        &row_minimum,
        &row_maximum,
        &min_needed_suffix,
        &max_possible_suffix,
        &mut parts,
        &mut visit,
    );
}

#[allow(clippy::too_many_arguments)]
fn enumerate_strips_strict_streaming<F>(
    alpha: &Partition,
    remaining: u32,
    row: usize,
    n_rows: usize,
    row_minimum: &[u32],
    row_maximum: &[u32],
    min_needed_suffix: &[u32],
    max_possible_suffix: &[u32],
    parts: &mut Vec<u32>,
    visit: &mut F,
) where
    F: FnMut(Partition),
{
    if row == n_rows {
        if remaining == 0 {
            visit(Partition::from_sorted(parts.clone()));
        }
        return;
    }

    if remaining < min_needed_suffix[row] || remaining > max_possible_suffix[row] {
        return;
    }

    let min_c = row_minimum[row].max(remaining.saturating_sub(max_possible_suffix[row + 1]));
    let base = alpha.part(row);
    let max_for_row = remaining.saturating_sub(min_needed_suffix[row + 1]);
    let max_c = remaining.min(row_maximum[row]).min(max_for_row);

    if max_c < min_c {
        return;
    }

    for c in min_c..=max_c {
        parts[row] = base + c;
        enumerate_strips_strict_streaming(
            alpha,
            remaining - c,
            row + 1,
            n_rows,
            row_minimum,
            row_maximum,
            min_needed_suffix,
            max_possible_suffix,
            parts,
            visit,
        );
    }
    parts[row] = base;
}

fn validate_row_masks(
    name: &str,
    masks: Option<&[u32]>,
    weight_len: usize,
    rows: usize,
) -> Result<(), String> {
    let Some(masks) = masks else {
        return Ok(());
    };
    if masks.len() != weight_len {
        return Err(format!(
            "{name} length {} does not match weight length {weight_len}",
            masks.len()
        ));
    }
    let allowed = if rows == u32::BITS as usize {
        u32::MAX
    } else {
        (1_u32 << rows) - 1
    };
    if let Some((label, mask)) = masks
        .iter()
        .enumerate()
        .find(|(_, mask)| **mask & !allowed != 0)
    {
        return Err(format!(
            "{name} for label {} contains a bit outside the {rows} shape rows: {mask:#x}",
            label + 1
        ));
    }
    Ok(())
}

/// Convert one-indexed forbidden `(row, label)` pairs from a tableau-face
/// presentation into the per-label bitmasks accepted by
/// [`try_masked_flagged_skew_kostka`].
pub fn forbidden_row_masks_from_pairs(
    rows: usize,
    labels: usize,
    forbidden_pairs: &[(usize, usize)],
) -> Result<Vec<u32>, String> {
    if rows > u32::BITS as usize {
        return Err("masked tableau faces support at most 32 rows".to_string());
    }
    let mut masks = vec![0_u32; labels];
    for &(row, label) in forbidden_pairs {
        if row == 0 || row > rows {
            return Err(format!("forbidden tableau row {row} is outside 1..={rows}"));
        }
        if label == 0 || label > labels {
            return Err(format!(
                "forbidden tableau label {label} is outside 1..={labels}"
            ));
        }
        masks[label - 1] |= 1_u32 << (row - 1);
    }
    Ok(masks)
}

/// Count a fixed-content tableau face with optional row flags, forbidden
/// row/label pairs, and explicit relative-interior masks.
///
/// Bit `r` of `forbidden_row_masks[i]` forces label `i + 1` to occur zero
/// times in zero-indexed row `r`. Bits in `strict_lower_masks[i]` require a
/// positive increment, while bits in `strict_diagonal_masks[i]` require the
/// corresponding horizontal-strip diagonal inequality to be strict. This is
/// the low-level constraint form used by complement-row lifts of individual
/// Kogan faces. Every supplied mask slice must have the same length as
/// `weight`; at most 32 rows are supported by this interface.
#[allow(clippy::too_many_arguments)]
pub fn try_masked_flagged_skew_kostka(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
    strict_lower_masks: Option<&[u32]>,
    strict_diagonal_masks: Option<&[u32]>,
    max_states: Option<usize>,
) -> Result<BigUint, String> {
    if !mu.partition_less_equal(lambda) {
        return Ok(BigUint::zero());
    }
    if partition_size_u64(lambda) - partition_size_u64(mu) != weight_size_u64(weight) {
        return Ok(BigUint::zero());
    }
    if upper_flags.is_some_and(|flags| flags.len() != weight.len())
        || lower_flags.is_some_and(|flags| flags.len() != weight.len())
    {
        return Err("flag lengths must match the weight length".to_string());
    }

    let rows = lambda.num_parts();
    if rows > u32::BITS as usize {
        return Err("masked tableau faces support at most 32 rows".to_string());
    }
    validate_row_masks(
        "forbidden-row mask",
        forbidden_row_masks,
        weight.len(),
        rows,
    )?;
    validate_row_masks("strict-lower mask", strict_lower_masks, weight.len(), rows)?;
    validate_row_masks(
        "strict-diagonal mask",
        strict_diagonal_masks,
        weight.len(),
        rows,
    )?;
    if strict_diagonal_masks.is_some_and(|masks| masks.iter().any(|mask| mask & 1 != 0)) {
        return Err("strict-diagonal masks cannot contain the first-row bit".to_string());
    }

    let mut states = partition_count_map();
    states.insert(mu.clone(), BigUint::one());
    for (label, &strip_size) in weight.iter().enumerate() {
        if strip_size == 0 {
            if strict_lower_masks.is_some_and(|masks| masks[label] != 0)
                || strict_diagonal_masks.is_some_and(|masks| masks[label] != 0)
            {
                return Err("zero-weight labels must have zero strictness masks".to_string());
            }
            continue;
        }

        let row_lo = lower_flags
            .map(|flags| (flags[label] as usize).saturating_sub(1).min(rows))
            .unwrap_or(0);
        let row_hi = upper_flags
            .map(|flags| (flags[label] as usize).min(rows))
            .unwrap_or(rows);
        let forbidden_mask = forbidden_row_masks.map_or(0, |masks| masks[label]);
        let strict_lower_mask = strict_lower_masks.map_or(0, |masks| masks[label]);
        let strict_diagonal_mask = strict_diagonal_masks.map_or(0, |masks| masks[label]);
        let strict_lower = (0..rows)
            .map(|row| strict_lower_mask & (1_u32 << row) != 0)
            .collect::<Vec<_>>();
        let strict_diagonal = (0..rows)
            .map(|row| strict_diagonal_mask & (1_u32 << row) != 0)
            .collect::<Vec<_>>();

        let mut next = partition_count_map();
        for (alpha, count) in &states {
            for_each_strict_horizontal_strip_extension_restricted(
                alpha,
                lambda,
                strip_size,
                &strict_lower,
                &strict_diagonal,
                row_lo,
                row_hi,
                forbidden_mask,
                |beta| {
                    *next.entry(beta).or_insert_with(BigUint::zero) += count;
                },
            );
        }
        if let Some(limit) = max_states {
            if next.len() > limit {
                return Err(format!(
                    "DP state count {} exceeds --max-states {}.",
                    next.len(),
                    limit
                ));
            }
        }
        states = next;
    }

    Ok(states.remove(lambda).unwrap_or_else(BigUint::zero))
}

/// Count interior lattice points of the GT-polytope GT(λ/μ, w).
///
/// Uses the lb/ub bounds from `gt_polytope_bounds` to identify which interlacing
/// constraints are globally tight (part of the affine hull) and only enforces
/// strictness for the remaining ones.  This correctly implements the relative
/// interior condition for all polytope dimensions.
///
/// `sort_weight` is ignored: sorting would break the lb/ub correspondence.
pub fn strict_skew_kostka_legacy(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    max_states: Option<usize>,
    _sort_weight: bool,
) -> BigUint {
    let skew_size: u32 = lambda.size().saturating_sub(mu.size());
    let w_size: u32 = w.iter().sum();
    if skew_size != w_size {
        return BigUint::zero();
    }
    if !mu.partition_less_equal(lambda) {
        return BigUint::zero();
    }

    let n = lambda.num_parts();
    let k = w.len();

    // Obtain propagated bounds; None means the polytope is empty.
    let (_, lb, ub) = match crate::gt_dim::gt_polytope_bounds(lambda.parts(), mu.parts(), w) {
        None => return BigUint::zero(),
        Some(data) => data,
    };

    // Pad μ to length n.
    let mut mu_pad = mu.parts().to_vec();
    mu_pad.resize(n, 0);
    let lambda_parts = lambda.parts();

    // Tight bounds for α^i[j]:
    //   i = 0      → boundary μ, both lb and ub equal μ_j
    //   i = 1..k-1 → interior level ell = i-1: lb[i-1][j], ub[i-1][j]
    //   i = k      → boundary λ, both equal λ_j
    let src_lb = |i: usize, j: usize| -> u32 {
        if i == 0 {
            mu_pad[j]
        } else {
            lb[i - 1][j]
        }
    };
    let src_ub = |i: usize, j: usize| -> u32 {
        if i == 0 {
            mu_pad[j]
        } else {
            ub[i - 1][j]
        }
    };
    let dst_lb = |i: usize, j: usize| -> u32 {
        if i + 1 == k {
            lambda_parts[j]
        } else {
            lb[i][j]
        }
    };
    let dst_ub = |i: usize, j: usize| -> u32 {
        if i + 1 == k {
            lambda_parts[j]
        } else {
            ub[i][j]
        }
    };

    // Precompute strictness flags per step i and row j.
    // A constraint is globally tight iff both sides are frozen at the same value.
    let mut strict_lower = vec![vec![false; n]; k];
    let mut strict_diag = vec![vec![false; n]; k];
    for i in 0..k {
        for j in 0..n {
            // Lower constraint: α^{i+1}[j] >= α^i[j]
            let (sl, su) = (src_lb(i, j), src_ub(i, j));
            let (dl, du) = (dst_lb(i, j), dst_ub(i, j));
            strict_lower[i][j] = !(sl == su && dl == du && sl == dl);

            // Diagonal constraint (j≥1): α^{i+1}[j] <= α^i[j-1]
            if j >= 1 {
                let (sl2, su2) = (src_lb(i, j - 1), src_ub(i, j - 1));
                // dst side is the same α^{i+1}[j]
                strict_diag[i][j] = !(sl2 == su2 && dl == du && sl2 == dl);
            }
        }
    }

    // Level-by-level DP with per-row strictness.
    let mut dp = partition_count_map();
    dp.insert(mu.clone(), BigUint::one());

    for (i, &strip_size) in w.iter().enumerate() {
        if strip_size == 0 {
            // Zero-weight step: all its constraints are globally tight (both sides
            // frozen at the same value after propagation).  No strictness needed;
            // just skip as in the regular DP.
            continue;
        }

        let mut new_dp = partition_count_map();
        for (alpha, count) in &dp {
            for beta in strict_horizontal_strip_extensions_legacy(
                alpha,
                lambda,
                strip_size,
                &strict_lower[i],
                &strict_diag[i],
            ) {
                *new_dp.entry(beta).or_insert_with(BigUint::zero) += count;
            }
        }

        if let Some(limit) = max_states {
            if new_dp.len() > limit {
                panic!(
                    "DP state count {} exceeds --max-states {}.",
                    new_dp.len(),
                    limit
                );
            }
        }

        dp = new_dp;
    }

    dp.remove(lambda).unwrap_or_else(BigUint::zero)
}

pub fn strict_skew_kostka(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    max_states: Option<usize>,
    sort_weight: bool,
) -> BigUint {
    try_strict_skew_kostka(lambda, mu, w, max_states, sort_weight)
        .expect("strict Kostka DP state limit exceeded")
}

/// Fallible variant of [`strict_skew_kostka`] that reports a state-limit breach.
///
/// `sort_weight` is ignored because reordering the weight would invalidate the
/// propagated bounds used to recognize relatively interior lattice points.
pub fn try_strict_skew_kostka(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    max_states: Option<usize>,
    _sort_weight: bool,
) -> Result<BigUint, String> {
    try_strict_flagged_skew_kostka(lambda, mu, w, None, None, max_states)
}

/// Per-label masks describing the relative interior of a masked tableau face.
///
/// `dimension` is the propagated chain-model dimension.  Bit `r` in a lower
/// or diagonal mask says that the corresponding inequality must be strict;
/// inequalities belonging to the affine hull are omitted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelativeInteriorMasks {
    pub dimension: usize,
    pub strict_lower_masks: Vec<u32>,
    pub strict_diagonal_masks: Vec<u32>,
}

/// Derive relative-interior masks for a fixed-content tableau face.
///
/// The forbidden masks and interval flags force selected lower inequalities
/// to equality. The exact affine-structure analysis closes further globally
/// tight inequalities under GT order relations and level-weight equations.
/// `Ok(None)` means that the face is empty.
#[allow(clippy::too_many_arguments)]
pub fn masked_relative_interior_masks(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
) -> Result<Option<RelativeInteriorMasks>, String> {
    if !mu.partition_less_equal(lambda) {
        return Ok(None);
    }
    if partition_size_u64(lambda) - partition_size_u64(mu) != weight_size_u64(weight) {
        return Ok(None);
    }
    if upper_flags.is_some_and(|flags| flags.len() != weight.len())
        || lower_flags.is_some_and(|flags| flags.len() != weight.len())
    {
        return Err("flag lengths must match the weight length".to_string());
    }

    let rows = lambda.num_parts();
    if rows > u32::BITS as usize {
        return Err("masked tableau faces support at most 32 rows".to_string());
    }
    validate_row_masks(
        "forbidden-row mask",
        forbidden_row_masks,
        weight.len(),
        rows,
    )?;

    let (dimension, tight_lower_masks, tight_diagonal_masks) =
        match crate::gt_dim::gt_polytope_affine_masks_masked(
            lambda.parts(),
            mu.parts(),
            weight,
            upper_flags,
            lower_flags,
            forbidden_row_masks,
        ) {
            None => return Ok(None),
            Some(data) => data,
        };

    let allowed_rows = if rows == u32::BITS as usize {
        u32::MAX
    } else {
        (1_u32 << rows) - 1
    };
    let mut strict_lower_masks = tight_lower_masks
        .iter()
        .map(|mask| !mask & allowed_rows)
        .collect::<Vec<_>>();
    let mut strict_diagonal_masks = tight_diagonal_masks
        .iter()
        .map(|mask| !mask & allowed_rows & !1_u32)
        .collect::<Vec<_>>();
    for (label, &multiplicity) in weight.iter().enumerate() {
        if multiplicity == 0 {
            strict_lower_masks[label] = 0;
            strict_diagonal_masks[label] = 0;
        }
    }

    Ok(Some(RelativeInteriorMasks {
        dimension,
        strict_lower_masks,
        strict_diagonal_masks,
    }))
}

/// Count relative-interior lattice points of an individual masked tableau
/// face.  The caller supplies only the weak face constraints; strict masks are
/// derived from the affine hull.
#[allow(clippy::too_many_arguments)]
pub fn try_strict_masked_flagged_skew_kostka(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
    max_states: Option<usize>,
) -> Result<BigUint, String> {
    let Some(masks) = masked_relative_interior_masks(
        lambda,
        mu,
        weight,
        upper_flags,
        lower_flags,
        forbidden_row_masks,
    )?
    else {
        return Ok(BigUint::zero());
    };

    try_masked_flagged_skew_kostka(
        lambda,
        mu,
        weight,
        upper_flags,
        lower_flags,
        forbidden_row_masks,
        Some(&masks.strict_lower_masks),
        Some(&masks.strict_diagonal_masks),
        max_states,
    )
}

/// Count relative-interior lattice points with optional row flags.
///
/// The structural flagged bounds determine which interlacing constraints lie
/// in the affine hull.  This remains valid when the scale-one lattice points
/// do not affinely span the fixed-content polytope.
pub fn try_strict_flagged_skew_kostka(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    max_states: Option<usize>,
) -> Result<BigUint, String> {
    try_strict_masked_flagged_skew_kostka(lambda, mu, w, upper_flags, lower_flags, None, max_states)
}

/// Convenience wrapper for non-skew strict K(lambda, w).
pub fn strict_kostka(lambda: &Partition, w: &[u32], max_states: Option<usize>) -> BigUint {
    strict_skew_kostka(lambda, &Partition::empty(), w, max_states, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::{BigInt, BigUint, ToBigInt};

    fn p(parts: &[u32]) -> Partition {
        Partition::new(parts.to_vec())
    }
    fn biguint(n: u64) -> BigUint {
        BigUint::from(n)
    }

    // ── Weak Kostka numbers ────────────────────────────────────────────────────

    #[test]
    fn kostka_321_222() {
        // K(3,2,1 | 2,2,2) = 2  (mentioned in session)
        assert_eq!(kostka(&p(&[3, 2, 1]), &[2, 2, 2], None), biguint(2));
    }

    #[test]
    fn kostka_311_11111_t1() {
        // K(3,1,1 | 1,1,1,1,1) = 6  [Ehrhart poly P(1)]
        assert_eq!(kostka(&p(&[3, 1, 1]), &[1, 1, 1, 1, 1], None), biguint(6));
    }

    #[test]
    fn kostka_311_11111_t2() {
        // K(6,2,2 | 2,2,2,2,2) = 20  [P(2)]
        assert_eq!(kostka(&p(&[6, 2, 2]), &[2, 2, 2, 2, 2], None), biguint(20));
    }

    #[test]
    fn kostka_311_11111_t3() {
        // K(9,3,3 | 3,3,3,3,3) = 50  [P(3)]
        assert_eq!(kostka(&p(&[9, 3, 3]), &[3, 3, 3, 3, 3], None), biguint(50));
    }

    #[test]
    fn kostka_311_11111_t4() {
        // K(12,4,4 | 4,4,4,4,4) = 105  [P(4)]
        assert_eq!(
            kostka(&p(&[12, 4, 4]), &[4, 4, 4, 4, 4], None),
            biguint(105)
        );
    }

    // ── Strict Kostka numbers (Ehrhart–Macdonald reciprocity) ─────────────────
    //
    // For shape (3,1,1) / weight (1,1,1,1,1), Ehrhart polynomial:
    //   P(n) = (2n^4 + 16n^3 + 46n^2 + 56n + 24) / 24
    // Macdonald reciprocity: (-1)^4 * K_strict(t*λ, t*w) = P(-t)
    //   P(-1) = 0, P(-2) = 0, P(-3) = 0, P(-4) = 1, P(-5) = 6, P(-6) = 20

    #[test]
    fn strict_311_11111_t1() {
        assert_eq!(
            strict_kostka(&p(&[3, 1, 1]), &[1, 1, 1, 1, 1], None),
            biguint(0)
        );
    }

    #[test]
    fn strict_311_11111_t2() {
        assert_eq!(
            strict_kostka(&p(&[6, 2, 2]), &[2, 2, 2, 2, 2], None),
            biguint(0)
        );
    }

    #[test]
    fn strict_311_11111_t3() {
        assert_eq!(
            strict_kostka(&p(&[9, 3, 3]), &[3, 3, 3, 3, 3], None),
            biguint(0)
        );
    }

    #[test]
    fn strict_311_11111_t4() {
        assert_eq!(
            strict_kostka(&p(&[12, 4, 4]), &[4, 4, 4, 4, 4], None),
            biguint(1)
        );
    }

    #[test]
    fn strict_311_11111_t5() {
        assert_eq!(
            strict_kostka(&p(&[15, 5, 5]), &[5, 5, 5, 5, 5], None),
            biguint(6)
        );
    }

    #[test]
    fn strict_311_11111_t6() {
        assert_eq!(
            strict_kostka(&p(&[18, 6, 6]), &[6, 6, 6, 6, 6], None),
            biguint(20)
        );
    }

    #[test]
    fn weak_dp_matches_legacy() {
        let lambda = p(&[5, 3, 1]);
        let mu = p(&[2, 1]);
        let w = [2, 1, 2, 1, 1];
        assert_eq!(
            skew_kostka(&lambda, &mu, &w, None, false),
            skew_kostka_legacy(&lambda, &mu, &w, None, false)
        );
    }

    #[test]
    fn production_dp_uses_wide_shape_totals() {
        let lambda = p(&[2_147_483_648, 2_147_483_648]);
        let mu = p(&[2_147_483_648, 2_147_483_647]);

        assert_eq!(
            try_skew_kostka(&lambda, &mu, &[1], None, false).unwrap(),
            biguint(1)
        );
        assert_eq!(
            try_flagged_skew_kostka(&lambda, &mu, &[1], None, None, None).unwrap(),
            biguint(1)
        );
        assert_eq!(
            try_strict_skew_kostka(&lambda, &mu, &[1], None, false).unwrap(),
            biguint(1)
        );
    }

    #[test]
    fn masked_face_supports_internal_forbidden_row_label_pairs() {
        let lambda = p(&[2, 1]);
        let mu = Partition::empty();
        let weight = [1, 1, 1];
        let forbidden = forbidden_row_masks_from_pairs(2, 3, &[(1, 2)]).unwrap();
        assert_eq!(forbidden, [0, 1, 0]);

        assert_eq!(
            try_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                None,
                None,
                Some(&forbidden),
                None,
                None,
                None,
            )
            .unwrap(),
            biguint(1)
        );
    }

    #[test]
    fn masked_face_derives_relative_interior_constraints() {
        let lambda = p(&[2, 1]);
        let mu = Partition::empty();
        let weight = [1, 1, 1];
        let forbidden = forbidden_row_masks_from_pairs(2, 3, &[(1, 2)]).unwrap();
        let masks =
            masked_relative_interior_masks(&lambda, &mu, &weight, None, None, Some(&forbidden))
                .unwrap()
                .expect("the face is nonempty");

        assert_eq!(masks.dimension, 0);
        assert_eq!(masks.strict_lower_masks[1] & 1, 0);
        assert_eq!(
            try_strict_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                None,
                None,
                Some(&forbidden),
                None,
            )
            .unwrap(),
            biguint(1)
        );
    }

    #[test]
    fn vacuous_forbidden_mask_is_allowed_for_zero_weight_label() {
        let lambda = p(&[1]);
        let mu = Partition::empty();
        let weight = [0, 1];
        let forbidden = [1, 0];

        assert_eq!(
            try_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                None,
                None,
                Some(&forbidden),
                None,
                None,
                None,
            )
            .unwrap(),
            biguint(1)
        );
    }

    #[test]
    fn derived_masked_interiors_satisfy_ehrhart_reciprocity() {
        let cases = [
            (vec![2, 1], vec![], vec![1, 1, 1], None, None, vec![(1, 2)]),
            (
                vec![3, 1],
                vec![],
                vec![1, 1, 1, 1],
                None,
                None,
                vec![(1, 2)],
            ),
            (
                vec![3, 2, 1],
                vec![],
                vec![1, 1, 1, 1, 1, 1],
                Some(vec![3, 3, 3, 3, 2, 2]),
                None,
                vec![(1, 3), (2, 4)],
            ),
            (
                vec![4, 3, 1],
                vec![1],
                vec![2, 2, 1, 1, 1],
                Some(vec![3, 3, 3, 2, 2]),
                Some(vec![1, 1, 1, 2, 2]),
                vec![(2, 2)],
            ),
        ];

        for (outer, inner, weight, upper, lower, pairs) in cases {
            let lambda = Partition::from_sorted(outer.clone());
            let mu = Partition::from_sorted(inner.clone());
            let forbidden =
                forbidden_row_masks_from_pairs(lambda.num_parts(), weight.len(), &pairs).unwrap();
            let upper_ref = upper.as_deref();
            let lower_ref = lower.as_deref();
            let masks = masked_relative_interior_masks(
                &lambda,
                &mu,
                &weight,
                upper_ref,
                lower_ref,
                Some(&forbidden),
            )
            .unwrap()
            .expect("reciprocity fixture must be nonempty");

            // L(0)=1 for every nonempty lattice polytope.  Successive first
            // differences give L(-1) = sum_j (-1)^j Delta^j L(0).
            let mut values = vec![BigInt::from(1)];
            for dilation in 1..=masks.dimension {
                let scaled_outer = Partition::from_sorted(
                    outer.iter().map(|part| part * dilation as u32).collect(),
                );
                let scaled_inner = Partition::from_sorted(
                    inner.iter().map(|part| part * dilation as u32).collect(),
                );
                let scaled_weight = weight
                    .iter()
                    .map(|part| part * dilation as u32)
                    .collect::<Vec<_>>();
                values.push(
                    try_masked_flagged_skew_kostka(
                        &scaled_outer,
                        &scaled_inner,
                        &scaled_weight,
                        upper_ref,
                        lower_ref,
                        Some(&forbidden),
                        None,
                        None,
                        None,
                    )
                    .unwrap()
                    .to_bigint()
                    .unwrap(),
                );
            }

            let mut degree_differences = values.clone();
            let mut empirical_degree = 0;
            for order in 0..=masks.dimension {
                if degree_differences
                    .iter()
                    .any(|value| value != &BigInt::from(0))
                {
                    empirical_degree = order;
                }
                degree_differences = degree_differences
                    .windows(2)
                    .map(|window| &window[1] - &window[0])
                    .collect();
            }
            assert_eq!(
                masks.dimension, empirical_degree,
                "dimension mismatch for outer={outer:?}, inner={inner:?}, weight={weight:?}, \
                 forbidden={forbidden:?}"
            );

            let mut differences = values;
            let mut value_at_minus_one = BigInt::from(0);
            let mut sign = BigInt::from(1);
            while !differences.is_empty() {
                value_at_minus_one += &sign * &differences[0];
                sign = -sign;
                differences = differences
                    .windows(2)
                    .map(|window| &window[1] - &window[0])
                    .collect();
            }
            if masks.dimension % 2 == 1 {
                value_at_minus_one = -value_at_minus_one;
            }

            let strict = try_strict_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                upper_ref,
                lower_ref,
                Some(&forbidden),
                None,
            )
            .unwrap()
            .to_bigint()
            .unwrap();
            assert_eq!(
                strict, value_at_minus_one,
                "reciprocity mismatch for outer={outer:?}, inner={inner:?}, weight={weight:?}, \
                 forbidden={forbidden:?}"
            );
        }
    }

    #[test]
    fn all_small_masked_faces_have_correct_dimension_and_interior() {
        let outer = [3, 1];
        let weight = [1, 1, 1, 1];
        let lambda = p(&outer);
        let mu = Partition::empty();
        let rows = lambda.num_parts();

        for encoded in 0_u32..(1_u32 << (rows * weight.len())) {
            let forbidden = (0..weight.len())
                .map(|label| (encoded >> (label * rows)) & ((1_u32 << rows) - 1))
                .collect::<Vec<_>>();
            let derived =
                masked_relative_interior_masks(&lambda, &mu, &weight, None, None, Some(&forbidden))
                    .unwrap();
            let weak_at_one = try_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                None,
                None,
                Some(&forbidden),
                None,
                None,
                None,
            )
            .unwrap();
            let Some(masks) = derived else {
                assert!(weak_at_one.is_zero(), "encoded mask {encoded:#x}");
                continue;
            };

            let mut values = vec![BigInt::from(1)];
            for dilation in 1..=masks.dimension {
                let scaled_outer = p(&outer.map(|part| part * u32::try_from(dilation).unwrap()));
                let scaled_weight = weight.map(|part| part * u32::try_from(dilation).unwrap());
                values.push(
                    try_masked_flagged_skew_kostka(
                        &scaled_outer,
                        &mu,
                        &scaled_weight,
                        None,
                        None,
                        Some(&forbidden),
                        None,
                        None,
                        None,
                    )
                    .unwrap()
                    .to_bigint()
                    .unwrap(),
                );
            }

            let mut differences = values;
            let mut empirical_degree = 0;
            let mut value_at_minus_one = BigInt::from(0);
            let mut sign = BigInt::from(1);
            for order in 0..=masks.dimension {
                if differences.iter().any(|value| !value.is_zero()) {
                    empirical_degree = order;
                }
                value_at_minus_one += &sign * &differences[0];
                sign = -sign;
                differences = differences
                    .windows(2)
                    .map(|window| &window[1] - &window[0])
                    .collect();
            }
            assert_eq!(
                masks.dimension, empirical_degree,
                "dimension mismatch for encoded mask {encoded:#x}"
            );
            if masks.dimension % 2 == 1 {
                value_at_minus_one = -value_at_minus_one;
            }
            let strict = try_strict_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                None,
                None,
                Some(&forbidden),
                None,
            )
            .unwrap()
            .to_bigint()
            .unwrap();
            assert_eq!(
                strict, value_at_minus_one,
                "reciprocity mismatch for encoded mask {encoded:#x}"
            );
        }
    }

    #[test]
    fn masked_face_enforces_strict_constraints_and_conflicts() {
        let lambda = p(&[2, 1]);
        let mu = Partition::empty();
        let weight = [1, 1, 1];
        let strict_lower = [0, 1, 0];
        let strict_diagonal = [0, 2, 0];

        assert_eq!(
            try_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                None,
                None,
                None,
                Some(&strict_lower),
                None,
                None,
            )
            .unwrap(),
            biguint(1)
        );
        assert_eq!(
            try_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                None,
                None,
                None,
                None,
                Some(&strict_diagonal),
                None,
            )
            .unwrap(),
            biguint(1)
        );

        let forbidden = [0, 1, 0];
        assert_eq!(
            try_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                None,
                None,
                Some(&forbidden),
                Some(&strict_lower),
                None,
                None,
            )
            .unwrap(),
            biguint(0)
        );
    }

    #[test]
    fn strict_diagonal_is_enforced_on_a_flag_forced_zero_increment() {
        let lambda = p(&[2, 1]);
        let mu = Partition::empty();
        let weight = [1, 1, 1];
        let upper = [2, 2, 1];
        let strict_diagonal = [0, 0, 2];

        assert_eq!(
            try_masked_flagged_skew_kostka(
                &lambda,
                &mu,
                &weight,
                Some(&upper),
                None,
                None,
                None,
                Some(&strict_diagonal),
                None,
            )
            .unwrap(),
            biguint(0)
        );
    }

    #[test]
    fn flagged_dp_matches_legacy() {
        let lambda = p(&[5, 4, 2]);
        let mu = p(&[2, 1]);
        let w = [2, 2, 1, 1, 1, 1];
        let upper = [3, 3, 2, 3, 2, 1];
        let lower = [1, 1, 2, 1, 2, 3];
        assert_eq!(
            flagged_skew_kostka(&lambda, &mu, &w, Some(&upper), Some(&lower), None),
            flagged_skew_kostka_legacy(&lambda, &mu, &w, Some(&upper), Some(&lower), None)
        );
    }

    #[test]
    fn strict_dp_matches_legacy() {
        let lambda = p(&[6, 4, 2]);
        let mu = p(&[2, 1]);
        let w = [2, 1, 2, 1, 2, 1, 1];
        assert_eq!(
            strict_skew_kostka(&lambda, &mu, &w, None, false),
            strict_skew_kostka_legacy(&lambda, &mu, &w, None, false)
        );
    }
}
