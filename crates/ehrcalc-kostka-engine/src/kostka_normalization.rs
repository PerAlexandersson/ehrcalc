//! Dilation-compatible affine reductions of flagged tableau inputs.

use crate::Partition;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ForcedFirstRowReduction {
    pub(crate) lambda: Partition,
    pub(crate) weight: Vec<u32>,
    pub(crate) removed: u32,
}

/// Delete isolated forced ones after `max(mu_1, lambda_2)`.
///
/// At most `mu_1` ones lie below row one. Its prefix through column
/// `weight[0]` is therefore one, and columns beyond `lambda_2` are isolated.
/// Subtracting the deleted length from the first GT coordinate at every
/// positive level is an affine lattice bijection, including interiors.
/// The multirow guard preserves row indices and leaves trivial one-row
/// instances to the existing counter. See `docs/FORCED_PREFIX.md`.
pub(crate) fn forced_first_row_reduction(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper: Option<&[u32]>,
    lower: Option<&[u32]>,
) -> Option<ForcedFirstRowReduction> {
    if lambda.num_parts() < 2
        || weight.is_empty()
        || !mu.partition_less_equal(lambda)
        || upper.is_some_and(|flags| flags.len() != weight.len())
        || lower.is_some_and(|flags| flags.len() != weight.len())
    {
        return None;
    }
    if upper.is_some_and(|flags| flags[0] == 0)
        || lower.is_some_and(|flags| flags[0] > 1)
        || weight[0] > lambda.part(0)
        || lambda.size_wide() - mu.size_wide()
            != weight.iter().map(|&part| u128::from(part)).sum::<u128>()
    {
        return None;
    }
    let threshold = mu.part(0).max(lambda.part(1));
    let removed = weight[0].checked_sub(threshold)?;
    if removed == 0 {
        return None;
    }
    let mut parts = lambda.parts().to_vec();
    parts[0] -= removed;
    let mut reduced_weight = weight.to_vec();
    reduced_weight[0] = threshold;
    Some(ForcedFirstRowReduction {
        lambda: Partition::from_sorted(parts),
        weight: reduced_weight,
        removed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ehrhart::{
        compute_ehrhart_legacy_with_mode, try_compute_ehrhart_with_mode, EhrhartInterpolation,
    };
    use crate::gt_dim::gt_polytope_dim_full;
    use crate::kostka_dp::{
        flagged_skew_kostka_legacy, try_flagged_skew_kostka, try_skew_kostka_stats,
        try_strict_flagged_skew_kostka, try_strict_masked_flagged_skew_kostka,
    };
    use num_traits::Zero;

    fn p(parts: &[u32]) -> Partition {
        Partition::new(parts.to_vec())
    }

    #[test]
    fn prefix_reduction_round_trip_and_wide_parts() {
        for (lambda, mu, weight) in [
            (p(&[7, 3, 1]), p(&[1]), vec![5, 3, 2]),
            (p(&[8, 3, 2]), p(&[4, 1]), vec![6, 1, 1]),
            (p(&[u32::MAX, 1]), p(&[]), vec![u32::MAX - 1, 2]),
        ] {
            let reduced = forced_first_row_reduction(&lambda, &mu, &weight, None, None).unwrap();
            let mut restored = reduced.lambda.parts().to_vec();
            restored[0] += reduced.removed;
            assert_eq!(restored, lambda.parts());
            let mut restored_weight = reduced.weight;
            restored_weight[0] += reduced.removed;
            assert_eq!(restored_weight, weight);
            assert!(mu.partition_less_equal(&reduced.lambda));
        }
        let reduced =
            forced_first_row_reduction(&p(&[u32::MAX, 1]), &p(&[]), &[u32::MAX - 1, 2], None, None)
                .unwrap();
        assert_eq!(reduced.lambda, p(&[2, 1]));
        assert_eq!(reduced.weight, [1, 2]);
    }

    #[test]
    fn prefix_reduction_rejects_invalid_or_inapplicable_inputs() {
        let lambda = p(&[7, 3, 1]);
        let mu = p(&[1]);
        let weight = [5, 3, 2];
        assert!(
            forced_first_row_reduction(&lambda, &mu, &weight, Some(&[0, 3, 3]), None).is_none()
        );
        assert!(
            forced_first_row_reduction(&lambda, &mu, &weight, None, Some(&[2, 1, 1])).is_none()
        );
        assert!(forced_first_row_reduction(&lambda, &mu, &weight, Some(&[3]), None).is_none());
        assert!(forced_first_row_reduction(&lambda, &mu, &[5, 2, 2], None, None).is_none());
        assert!(forced_first_row_reduction(&lambda, &p(&[8]), &weight, None, None).is_none());
        assert!(forced_first_row_reduction(&p(&[3, 2]), &p(&[]), &[4, 1], None, None).is_none());
        assert!(forced_first_row_reduction(&p(&[4, 3]), &p(&[]), &[3, 2, 2], None, None).is_none());
        assert!(forced_first_row_reduction(&p(&[5]), &p(&[]), &[5], None, None).is_none());
        assert!(forced_first_row_reduction(&p(&[]), &p(&[]), &[], None, None).is_none());
    }

    #[test]
    fn prefix_reduction_matches_independent_ordinary_and_interior_counters() {
        let fixtures = [
            (p(&[7, 3, 1]), p(&[1]), vec![5, 3, 2]),
            (p(&[8, 3, 2]), p(&[4, 1]), vec![6, 1, 1]),
            (p(&[6, 2]), p(&[]), vec![4, 3, 1]),
            (p(&[5, 3]), p(&[2, 1]), vec![4, 1, 0]),
            (p(&[8, 2]), p(&[3]), vec![7]),
        ];
        let mut positive_dimensional_interior = false;
        for (lambda, mu, weight) in fixtures {
            for flagged in [false, true] {
                let upper = vec![lambda.num_parts() as u32; weight.len()];
                let mut lower = vec![1; weight.len()];
                if weight.len() > 1 {
                    *lower.last_mut().unwrap() = 2;
                }
                let (upper, lower) = if flagged {
                    (Some(upper.as_slice()), Some(lower.as_slice()))
                } else {
                    (None, None)
                };
                let reduced =
                    forced_first_row_reduction(&lambda, &mu, &weight, upper, lower).unwrap();
                let dimension =
                    gt_polytope_dim_full(lambda.parts(), mu.parts(), &weight, upper, lower);
                assert_eq!(
                    dimension,
                    gt_polytope_dim_full(
                        reduced.lambda.parts(),
                        mu.parts(),
                        &reduced.weight,
                        upper,
                        lower
                    )
                );
                for dilation in 1..=3 {
                    let scale =
                        |parts: &[u32]| p(&parts.iter().map(|&x| x * dilation).collect::<Vec<_>>());
                    let scaled_lambda = scale(lambda.parts());
                    let scaled_mu = scale(mu.parts());
                    let scaled_weight: Vec<_> = weight.iter().map(|&x| x * dilation).collect();
                    let expected = flagged_skew_kostka_legacy(
                        &scaled_lambda,
                        &scaled_mu,
                        &scaled_weight,
                        upper,
                        lower,
                        Some(50_000),
                    );
                    assert_eq!(
                        try_flagged_skew_kostka(
                            &scaled_lambda,
                            &scaled_mu,
                            &scaled_weight,
                            upper,
                            lower,
                            Some(50_000)
                        )
                        .unwrap(),
                        expected
                    );
                    if !flagged {
                        assert_eq!(
                            try_skew_kostka_stats(
                                &scaled_lambda,
                                &scaled_mu,
                                &scaled_weight,
                                Some(50_000),
                                false
                            )
                            .unwrap()
                            .value,
                            expected
                        );
                    }
                    let interior = try_strict_masked_flagged_skew_kostka(
                        &scaled_lambda,
                        &scaled_mu,
                        &scaled_weight,
                        upper,
                        lower,
                        None,
                        Some(50_000),
                    )
                    .unwrap();
                    if dimension.is_some_and(|d| d > 0) && !interior.is_zero() {
                        positive_dimensional_interior = true;
                    }
                    assert_eq!(
                        try_strict_flagged_skew_kostka(
                            &scaled_lambda,
                            &scaled_mu,
                            &scaled_weight,
                            upper,
                            lower,
                            Some(50_000)
                        )
                        .unwrap(),
                        interior
                    );
                }
            }
        }
        assert!(positive_dimensional_interior);
    }

    #[test]
    fn prefix_reduction_preserves_empty_flagged_inputs() {
        let lambda = p(&[7, 3, 1]);
        let mu = p(&[1]);
        let weight = [5, 3, 2];
        let upper = [1, 1, 1];
        assert!(forced_first_row_reduction(&lambda, &mu, &weight, Some(&upper), None).is_some());
        assert_eq!(
            gt_polytope_dim_full(lambda.parts(), mu.parts(), &weight, Some(&upper), None),
            None
        );
        assert!(
            try_flagged_skew_kostka(&lambda, &mu, &weight, Some(&upper), None, None)
                .unwrap()
                .is_zero()
        );
        assert!(
            try_strict_flagged_skew_kostka(&lambda, &mu, &weight, Some(&upper), None, None)
                .unwrap()
                .is_zero()
        );
    }

    #[test]
    fn prefix_reduction_preserves_the_entire_ehrhart_polynomial() {
        let lambda = p(&[6, 2]);
        let mu = p(&[]);
        let weight = [4, 3, 1];
        let expected = compute_ehrhart_legacy_with_mode(
            &lambda,
            &mu,
            &weight,
            None,
            None,
            false,
            Some(50_000),
            EhrhartInterpolation::PositiveOnly,
        );
        for mode in [
            EhrhartInterpolation::PositiveOnly,
            EhrhartInterpolation::AdaptiveReciprocity,
        ] {
            let actual = try_compute_ehrhart_with_mode(
                &lambda,
                &mu,
                &weight,
                None,
                None,
                false,
                Some(50_000),
                mode,
            )
            .unwrap();
            assert_eq!(actual.degree, expected.degree);
            assert_eq!(actual.coeffs, expected.coeffs);
        }
    }
}
