//! Exact counting for a two-step GT chain, after weak bounds are validated.
//!
//! Its only variable level is an integer box slice. Interlacing with both
//! fixed boundaries already implies the partition inequalities within that
//! level. Strict masks shift the appropriate box endpoints by one; their
//! derivation remains the responsibility of the exact affine-hull API.

use crate::Partition;
use num::integer::binomial;
use num_bigint::BigUint;
use num_traits::{One, Zero};

// At most 4096 terms, independent of dilation and lattice-point count.
const MAX_FREE_COORDINATES: usize = 12;

fn bit(masks: Option<&[u32]>, step: usize, row: usize) -> bool {
    row < u32::BITS as usize && masks.is_some_and(|m| m[step] & (1_u32 << row) != 0)
}

/// `None` requests the existing DP when the fixed term bound is exceeded.
/// The caller has checked positive two-part content, flags, masks and totals.
#[allow(clippy::too_many_arguments)]
pub(crate) fn count_chain(
    lambda: &Partition,
    mu: &Partition,
    first_weight: u32,
    lower: &[u32],
    upper: &[u32],
    strict_lower: Option<&[u32]>,
    strict_diagonal: Option<&[u32]>,
) -> Option<BigUint> {
    let rows = lambda.num_parts();
    let mut capacities = Vec::new();
    let mut lower_sum = 0_u64;
    for row in 0..rows {
        let mut lo = u64::from(lower[row]);
        let mut hi = u64::from(upper[row]);
        // First step: alpha_j > mu_j, alpha_j < mu_{j-1}.
        if bit(strict_lower, 0, row) {
            lo = lo.max(u64::from(mu.part(row)) + 1);
        }
        if bit(strict_diagonal, 0, row) {
            let Some(endpoint) = u64::from(mu.part(row - 1)).checked_sub(1) else {
                return Some(BigUint::zero());
            };
            hi = hi.min(endpoint);
        }
        // Second step: alpha_j < lambda_j, alpha_j > lambda_{j+1}.
        if bit(strict_lower, 1, row) {
            let Some(endpoint) = u64::from(lambda.part(row)).checked_sub(1) else {
                return Some(BigUint::zero());
            };
            hi = hi.min(endpoint);
        }
        if row + 1 < rows && bit(strict_diagonal, 1, row + 1) {
            lo = lo.max(u64::from(lambda.part(row + 1)) + 1);
        }
        if lo > hi {
            return Some(BigUint::zero());
        }
        lower_sum += lo;
        if hi > lo {
            capacities.push(hi - lo);
            if capacities.len() > MAX_FREE_COORDINATES {
                return None;
            }
        }
    }
    let level_sum = mu.parts().iter().map(|&x| u64::from(x)).sum::<u64>() + u64::from(first_weight);
    let Some(target) = level_sum.checked_sub(lower_sum) else {
        return Some(BigUint::zero());
    };
    Some(count_box_slice(&capacities, target))
}

fn count_box_slice(capacities: &[u64], target: u64) -> BigUint {
    let total = capacities.iter().sum::<u64>();
    if target > total {
        return BigUint::zero();
    }
    if capacities.is_empty() {
        return BigUint::one();
    }
    let target = target.min(total - target);
    let degree = capacities.len() as u64 - 1;
    let mut positive = BigUint::zero();
    let mut negative = BigUint::zero();
    for subset in 0..(1_usize << capacities.len()) {
        let shift = capacities
            .iter()
            .enumerate()
            .filter(|(row, _)| subset & (1 << row) != 0)
            .map(|(_, cap)| cap + 1)
            .sum::<u64>();
        if shift > target {
            continue;
        }
        let term = binomial(
            BigUint::from(target - shift + degree),
            BigUint::from(degree),
        );
        if subset.count_ones() % 2 == 0 {
            positive += term;
        } else {
            negative += term;
        }
    }
    positive - negative
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kostka_dp::{
        flagged_skew_kostka_legacy, try_flagged_skew_kostka, try_masked_flagged_skew_kostka,
        try_strict_flagged_skew_kostka,
    };

    fn enumerate(capacities: &[u64], target: u64) -> u64 {
        match capacities.split_first() {
            None => u64::from(target == 0),
            Some((&cap, tail)) => (0..=cap.min(target))
                .map(|x| enumerate(tail, target - x))
                .sum(),
        }
    }

    #[test]
    fn box_slices_match_exhaustive_integer_points() {
        for rows in 0..=5 {
            for code in 0..4_usize.pow(rows) {
                let caps = (0..rows)
                    .map(|j| ((code / 4_usize.pow(j)) % 4) as u64)
                    .filter(|&cap| cap > 0)
                    .collect::<Vec<_>>();
                for target in 0..=caps.iter().sum::<u64>() + 1 {
                    assert_eq!(
                        count_box_slice(&caps, target),
                        BigUint::from(enumerate(&caps, target))
                    );
                }
            }
        }
    }

    #[test]
    fn generic_counts_match_legacy_for_small_skew_shapes_and_flags() {
        let mut checked = 0;
        for size in 2..=7 {
            for lambda in Partition::all_of_size(size) {
                let rows = lambda.num_parts() as u32;
                for inner_size in 0..=size - 2 {
                    for mu in Partition::all_of_size(inner_size) {
                        if !mu.partition_less_equal(&lambda) {
                            continue;
                        }
                        for first in 1..size - inner_size {
                            let weight = [first, size - inner_size - first];
                            for (upper, lower) in [
                                ([rows, rows], [1, 1]),
                                ([1, rows], [1, 1]),
                                ([rows, rows], [1, rows]),
                            ] {
                                let expected = flagged_skew_kostka_legacy(
                                    &lambda,
                                    &mu,
                                    &weight,
                                    Some(&upper),
                                    Some(&lower),
                                    None,
                                );
                                assert_eq!(
                                    try_flagged_skew_kostka(
                                        &lambda,
                                        &mu,
                                        &weight,
                                        Some(&upper),
                                        Some(&lower),
                                        None,
                                    )
                                    .unwrap(),
                                    expected,
                                    "{lambda:?}/{mu:?}, {weight:?}, {upper:?}, {lower:?}",
                                );
                                checked += 1;
                            }
                        }
                    }
                }
            }
        }
        assert!(checked > 1000);
    }

    // Directly inspect every intermediate partition and both strip steps.
    // This oracle uses neither propagated bounds nor endpoint shifts.
    #[allow(clippy::too_many_arguments)]
    fn direct_chains(
        lambda: &Partition,
        mu: &Partition,
        first: u32,
        upper: &[u32; 2],
        lower: &[u32; 2],
        forbidden: &[u32; 2],
        strict_lower: &[u32; 2],
        strict_diagonal: &[u32; 2],
    ) -> BigUint {
        let size = mu.parts().iter().sum::<u32>() + first;
        let mut count = 0_u64;
        for alpha in Partition::all_of_size_bounded(size, lambda.num_parts(), lambda.part(0)) {
            let chain = [mu, &alpha, lambda];
            let mut valid = true;
            for step in 0..2 {
                for row in 0..lambda.num_parts() {
                    let a = chain[step].part(row);
                    let b = chain[step + 1].part(row);
                    if b < a || (row > 0 && b > chain[step].part(row - 1)) {
                        valid = false;
                    }
                    if b != a
                        && ((row as u32 + 1) < lower[step]
                            || (row as u32 + 1) > upper[step]
                            || forbidden[step] & (1 << row) != 0)
                    {
                        valid = false;
                    }
                    if strict_lower[step] & (1 << row) != 0 && b <= a {
                        valid = false;
                    }
                    if row > 0
                        && strict_diagonal[step] & (1 << row) != 0
                        && b >= chain[step].part(row - 1)
                    {
                        valid = false;
                    }
                }
            }
            count += u64::from(valid);
        }
        BigUint::from(count)
    }

    #[test]
    fn masks_at_both_steps_match_exhaustive_direct_chains() {
        let lambda = Partition::new(vec![4, 2]);
        let mu = Partition::new(vec![2]);
        for flags in [([2, 2], [1, 1]), ([1, 2], [1, 2])] {
            for forbidden_code in 0..16_u32 {
                let forbidden = [forbidden_code & 3, forbidden_code >> 2];
                for strict_code in 0..16_u32 {
                    let strict = [strict_code & 3, strict_code >> 2];
                    for diagonal_code in 0..4_u32 {
                        let diagonal = [(diagonal_code & 1) << 1, (diagonal_code >> 1) << 1];
                        let expected = direct_chains(
                            &lambda, &mu, 2, &flags.0, &flags.1, &forbidden, &strict, &diagonal,
                        );
                        assert_eq!(
                            try_masked_flagged_skew_kostka(
                                &lambda, &mu, &[2, 2], Some(&flags.0), Some(&flags.1),
                                Some(&forbidden), Some(&strict), Some(&diagonal), None,
                            ).unwrap(), expected,
                            "flags={flags:?},forbidden={forbidden:?},strict={strict:?},diag={diagonal:?}",
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn hypersimplex_relative_interior_and_flag_forced_point() {
        for (t, weak, interior) in [(1, 6, 0), (2, 19, 1), (3, 44, 6), (4, 85, 19)] {
            let lambda = Partition::new(vec![4 * t, 3 * t, 2 * t, t]);
            let mu = Partition::new(vec![3 * t, 2 * t, t]);
            let weight = [2 * t, 2 * t];
            assert_eq!(
                try_flagged_skew_kostka(&lambda, &mu, &weight, None, None, Some(1)).unwrap(),
                BigUint::from(weak as u64)
            );
            assert_eq!(
                try_strict_flagged_skew_kostka(&lambda, &mu, &weight, None, None, Some(1)).unwrap(),
                BigUint::from(interior as u64)
            );
            // The first two rows must be label one and the other rows label two.
            assert_eq!(
                try_strict_flagged_skew_kostka(
                    &lambda,
                    &mu,
                    &weight,
                    Some(&[2, 4]),
                    Some(&[1, 3]),
                    Some(1)
                )
                .unwrap(),
                BigUint::one()
            );
        }
    }

    #[test]
    fn unequal_six_row_regression_stays_within_one_dp_state() {
        // Disjoint rows of lengths 1,...,6; first content part is 10*t.
        // Values at 8 are independently determined by the degree-five
        // polynomial interpolated from completed legacy signed samples.
        for (t, weak, interior) in [
            (1, 573_u64, 0_u64),
            (2, 8353, 801),
            (4, 169173, 54715),
            (8, 4213425, 2407503),
        ] {
            let lambda = Partition::new([21, 20, 18, 15, 11, 6].map(|x| x * t).to_vec());
            let mu = Partition::new([20, 18, 15, 11, 6].map(|x| x * t).to_vec());
            let weight = [10 * t, 11 * t];
            assert_eq!(
                try_flagged_skew_kostka(&lambda, &mu, &weight, None, None, Some(1)).unwrap(),
                BigUint::from(weak),
            );
            assert_eq!(
                try_strict_flagged_skew_kostka(&lambda, &mu, &weight, None, None, Some(1)).unwrap(),
                BigUint::from(interior),
            );
        }
    }

    #[test]
    fn wide_totals_large_dilations_and_fallback_guards() {
        let m = u32::MAX;
        assert_eq!(
            try_flagged_skew_kostka(
                &Partition::new(vec![m, m - 1]),
                &Partition::new(vec![m - 1, m - 2]),
                &[1, 1],
                None,
                None,
                Some(1),
            )
            .unwrap(),
            BigUint::from(2_u32)
        );
        assert_eq!(
            try_flagged_skew_kostka(
                &Partition::new(vec![m, m / 2]),
                &Partition::new(vec![m / 2]),
                &[m / 2, m - m / 2],
                None,
                None,
                Some(1),
            )
            .unwrap(),
            BigUint::from(u64::from(m / 2) + 1)
        );
        // More than twelve free coordinates must retain the existing DP.
        let lambda = Partition::new((1..=13).rev().collect());
        let mu = Partition::new((1..13).rev().collect());
        assert!(count_chain(&lambda, &mu, 6, &[0; 13], lambda.parts(), None, None).is_none());
        let lambda = Partition::new(vec![2, 1]);
        let mu = Partition::new(vec![1]);
        assert!(try_flagged_skew_kostka(&lambda, &mu, &[1, 1], None, None, Some(0)).is_err());
        assert!(try_masked_flagged_skew_kostka(
            &lambda,
            &mu,
            &[0, 2],
            None,
            None,
            None,
            Some(&[1, 0]),
            None,
            None,
        )
        .is_err());
        assert_eq!(
            try_flagged_skew_kostka(&lambda, &mu, &[0, 2], None, None, None).unwrap(),
            BigUint::one()
        );
    }
}
