//! Kostka products at tight column-capacity partial sums.
//!
//! A column of height h contains at most min(t,h) labels <= t. If the prefix
//! content saturates the sum of these capacities, the intermediate shape beta
//! is forced: beta'_j = min(lambda'_j, mu'_j+t). The two skew factors form a
//! lattice product, compatible with dilation, flags and relative interiors.
//! Row indices and flags are unchanged; only the second alphabet is relabeled.
//! For straight shapes this is the primitive factorization recalled in
//! Garber--Pak--Slonim, On stretched Kostka polynomials (2026), Section 6.

use crate::Partition;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SaturatedSplitFactor {
    pub(crate) lambda: Partition,
    pub(crate) mu: Partition,
    pub(crate) weight: Vec<u32>,
    pub(crate) upper_flags: Option<Vec<u32>>,
    pub(crate) lower_flags: Option<Vec<u32>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SaturatedRowLabelSplit {
    pub(crate) top: SaturatedSplitFactor,
    pub(crate) bottom: SaturatedSplitFactor,
    pub(crate) cut_row: Option<usize>,
    pub(crate) cut_label: usize,
}

pub(crate) fn column_capacity_split(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper: Option<&[u32]>,
    lower: Option<&[u32]>,
) -> Option<SaturatedRowLabelSplit> {
    if !mu.partition_less_equal(lambda)
        || upper.is_some_and(|flags| flags.len() != weight.len())
        || lower.is_some_and(|flags| flags.len() != weight.len())
    {
        return None;
    }
    let total_area: u64 = (0..lambda.num_parts())
        .map(|row| u64::from(lambda.part(row) - mu.part(row)))
        .sum();
    let mut content_sum = 0u64;
    for cut in 1..weight.len() {
        content_sum += u64::from(weight[cut - 1]);
        // Conjugating the column formula gives beta_i=lambda_i for i<=t,
        // and beta_i=min(lambda_i,mu_(i-t)) afterwards. Work by rows so huge
        // part sizes do not cause an allocation or loop over individual boxes.
        let beta = Partition::new(
            (0..lambda.num_parts())
                .map(|row| {
                    if row < cut {
                        lambda.part(row)
                    } else {
                        lambda.part(row).min(mu.part(row - cut))
                    }
                })
                .collect(),
        );
        let capacity: u64 = (0..lambda.num_parts())
            .map(|row| u64::from(beta.part(row) - mu.part(row)))
            .sum();
        if capacity == 0 || capacity == total_area || capacity != content_sum {
            continue;
        }
        return Some(SaturatedRowLabelSplit {
            top: SaturatedSplitFactor {
                lambda: beta.clone(),
                mu: mu.clone(),
                weight: weight[..cut].to_vec(),
                upper_flags: upper.map(|flags| flags[..cut].to_vec()),
                lower_flags: lower.map(|flags| flags[..cut].to_vec()),
            },
            bottom: SaturatedSplitFactor {
                lambda: lambda.clone(),
                mu: beta,
                weight: weight[cut..].to_vec(),
                upper_flags: upper.map(|flags| flags[cut..].to_vec()),
                lower_flags: lower.map(|flags| flags[cut..].to_vec()),
            },
            cut_row: None,
            cut_label: cut,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ehrhart::try_compute_ehrhart;
    use crate::kostka_dp::{
        flagged_skew_kostka_legacy, try_flagged_skew_kostka, try_strict_flagged_skew_kostka,
    };
    use num_bigint::{BigInt, BigUint};
    use num_rational::BigRational;

    #[test]
    fn dominance_product_has_two_nonconstant_factors() {
        let lambda = Partition::new(vec![8, 6, 5, 3, 1]);
        let weight = [7, 6, 6, 2, 1, 1];
        let split =
            column_capacity_split(&lambda, &Partition::empty(), &weight, None, None).unwrap();
        assert_eq!(split.cut_label, 3);
        let result = try_compute_ehrhart(
            &lambda,
            &Partition::empty(),
            &weight,
            None,
            None,
            false,
            Some(200_000),
            true,
        )
        .unwrap();
        assert_eq!(
            result.coeffs,
            [1, 2, 1].map(|n| BigRational::from_integer(BigInt::from(n)))
        );
        for t in 1..=3 {
            let outer = Partition::new(lambda.parts().iter().map(|&x| x * t).collect());
            let content: Vec<_> = weight.iter().map(|&x| x * t).collect();
            let ordinary = try_flagged_skew_kostka(
                &outer,
                &Partition::empty(),
                &content,
                None,
                None,
                Some(200_000),
            )
            .unwrap();
            assert_eq!(ordinary, BigUint::from((t + 1).pow(2)));
            let strict = try_strict_flagged_skew_kostka(
                &outer,
                &Partition::empty(),
                &content,
                None,
                None,
                Some(200_000),
            )
            .unwrap();
            assert_eq!(strict, BigUint::from((t - 1).pow(2)));
        }
    }

    #[test]
    fn dominance_flags_match_unsplit_legacy_counts() {
        let lambda = Partition::new(vec![8, 6, 5, 3, 1]);
        let weight = [7, 6, 6, 2, 1, 1];
        for upper in [[5, 5, 5, 5, 5, 5], [2, 3, 3, 4, 5, 5], [2, 2, 2, 3, 4, 5]] {
            for lower in [[1, 1, 1, 1, 1, 1], [1, 1, 1, 3, 4, 4], [1, 2, 3, 4, 5, 5]] {
                let new = try_flagged_skew_kostka(
                    &lambda,
                    &Partition::empty(),
                    &weight,
                    Some(&upper),
                    Some(&lower),
                    Some(200_000),
                )
                .unwrap();
                let old = flagged_skew_kostka_legacy(
                    &lambda,
                    &Partition::empty(),
                    &weight,
                    Some(&upper),
                    Some(&lower),
                    Some(200_000),
                );
                assert_eq!(new, old, "upper={upper:?}, lower={lower:?}");
            }
        }
    }

    #[test]
    fn dominance_handles_content_zeros_and_avoids_false_component_split() {
        let lambda = Partition::new(vec![4, 2]);
        let content = [4, 0, 1, 1];
        let value = try_flagged_skew_kostka(
            &lambda,
            &Partition::empty(),
            &content,
            None,
            None,
            Some(200_000),
        )
        .unwrap();
        let old = flagged_skew_kostka_legacy(
            &lambda,
            &Partition::empty(),
            &content,
            None,
            None,
            Some(200_000),
        );
        assert_eq!(value, old);
        assert!(column_capacity_split(
            &Partition::new(vec![2, 1]),
            &Partition::new(vec![1]),
            &[1, 1],
            None,
            None
        )
        .is_none());
        assert!(column_capacity_split(
            &Partition::new(vec![3, 1]),
            &Partition::empty(),
            &[2, 1, 1],
            None,
            None
        )
        .is_none());
    }

    #[test]
    fn dominance_small_partition_pairs_match_legacy_weak_and_strict() {
        use crate::kostka_dp::{
            skew_kostka_legacy, strict_skew_kostka_legacy, try_skew_kostka_stats,
        };
        let mut split_pairs = 0;
        for size in 1..=8 {
            for lambda in Partition::all_of_size(size) {
                for content in Partition::all_of_size(size) {
                    let weight = content.parts();
                    if column_capacity_split(&lambda, &Partition::empty(), weight, None, None)
                        .is_none()
                    {
                        continue;
                    }
                    split_pairs += 1;
                    for t in 1..=2 {
                        let outer = Partition::new(lambda.parts().iter().map(|&x| x * t).collect());
                        let scaled: Vec<_> = weight.iter().map(|&x| x * t).collect();
                        let stats = try_skew_kostka_stats(
                            &outer,
                            &Partition::empty(),
                            &scaled,
                            Some(200_000),
                            false,
                        )
                        .unwrap();
                        assert_eq!(stats.level_states.len(), scaled.len() + 1);
                        assert_eq!(
                            stats.value,
                            skew_kostka_legacy(
                                &outer,
                                &Partition::empty(),
                                &scaled,
                                Some(200_000),
                                false
                            )
                        );
                        let strict = try_strict_flagged_skew_kostka(
                            &outer,
                            &Partition::empty(),
                            &scaled,
                            None,
                            None,
                            Some(200_000),
                        )
                        .unwrap();
                        assert_eq!(
                            strict,
                            strict_skew_kostka_legacy(
                                &outer,
                                &Partition::empty(),
                                &scaled,
                                Some(200_000),
                                false
                            )
                        );
                    }
                }
            }
        }
        assert!(split_pairs > 100);
    }

    #[test]
    fn skew_column_capacity_reproduces_four_product_fixtures() {
        for (outer, inner, content, cut, expected_beta, expected_values) in [
            (
                vec![4, 3, 2],
                vec![2, 1],
                vec![4, 1, 1],
                1,
                vec![4, 2, 1],
                vec![2, 3, 4],
            ),
            (
                vec![5, 4, 3, 2],
                vec![2, 1],
                vec![5, 3, 2, 1],
                1,
                vec![5, 2, 1],
                vec![6, 20, 50],
            ),
            (
                vec![5, 4, 3, 2, 1],
                vec![1],
                vec![4, 4, 4, 1, 1],
                3,
                vec![5, 4, 3, 1],
                vec![4, 9, 16],
            ),
            (
                vec![6, 5, 4, 3, 2, 1],
                vec![2, 1],
                vec![5, 5, 5, 1, 1, 1],
                3,
                vec![6, 5, 4, 2, 1],
                vec![12, 63, 220],
            ),
        ] {
            let lambda = Partition::new(outer);
            let mu = Partition::new(inner);
            let split = column_capacity_split(&lambda, &mu, &content, None, None).unwrap();
            assert_eq!(split.cut_label, cut);
            assert_eq!(split.top.lambda.parts(), expected_beta);
            for t in 1..=3u32 {
                let outer = Partition::new(lambda.parts().iter().map(|&x| x * t).collect());
                let inner = Partition::new(mu.parts().iter().map(|&x| x * t).collect());
                let weight: Vec<_> = content.iter().map(|&x| x * t).collect();
                let value =
                    try_flagged_skew_kostka(&outer, &inner, &weight, None, None, Some(200_000))
                        .unwrap();
                assert_eq!(value, BigUint::from(expected_values[t as usize - 1] as u32));
                assert_eq!(
                    value,
                    flagged_skew_kostka_legacy(&outer, &inner, &weight, None, None, Some(200_000))
                );
            }
            let polynomial = try_compute_ehrhart(
                &lambda,
                &mu,
                &content,
                None,
                None,
                false,
                Some(200_000),
                true,
            )
            .unwrap();
            let legacy = crate::ehrhart::compute_ehrhart_legacy(
                &lambda,
                &mu,
                &content,
                None,
                None,
                false,
                Some(200_000),
                false,
            );
            assert_eq!(polynomial.coeffs, legacy.coeffs);
        }
    }

    #[test]
    fn skew_flagged_products_match_unsplit_weak_and_strict_counts() {
        use crate::kostka_dp::try_strict_masked_flagged_skew_kostka;
        let lambda = Partition::new(vec![5, 4, 3, 2, 1]);
        let mu = Partition::new(vec![1]);
        let weight = [4, 4, 4, 1, 1];
        for upper in [[5, 5, 5, 5, 5], [3, 4, 5, 5, 5], [2, 3, 4, 4, 5]] {
            for lower in [[1, 1, 1, 1, 1], [1, 1, 2, 2, 3], [1, 2, 3, 4, 5]] {
                let split =
                    column_capacity_split(&lambda, &mu, &weight, Some(&upper), Some(&lower))
                        .unwrap();
                assert_eq!(split.bottom.upper_flags.as_deref(), Some(&upper[3..]));
                assert_eq!(split.bottom.lower_flags.as_deref(), Some(&lower[3..]));
                for t in 1..=2 {
                    let outer = Partition::new(lambda.parts().iter().map(|&x| x * t).collect());
                    let inner = Partition::new(mu.parts().iter().map(|&x| x * t).collect());
                    let content: Vec<_> = weight.iter().map(|&x| x * t).collect();
                    let weak = try_flagged_skew_kostka(
                        &outer,
                        &inner,
                        &content,
                        Some(&upper),
                        Some(&lower),
                        Some(200_000),
                    )
                    .unwrap();
                    assert_eq!(
                        weak,
                        flagged_skew_kostka_legacy(
                            &outer,
                            &inner,
                            &content,
                            Some(&upper),
                            Some(&lower),
                            Some(200_000)
                        )
                    );
                    let strict = try_strict_flagged_skew_kostka(
                        &outer,
                        &inner,
                        &content,
                        Some(&upper),
                        Some(&lower),
                        Some(200_000),
                    )
                    .unwrap();
                    let unsplit = try_strict_masked_flagged_skew_kostka(
                        &outer,
                        &inner,
                        &content,
                        Some(&upper),
                        Some(&lower),
                        None,
                        Some(200_000),
                    )
                    .unwrap();
                    assert_eq!(strict, unsplit);
                }
            }
        }
    }
}
