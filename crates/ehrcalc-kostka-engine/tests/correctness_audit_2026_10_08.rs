//! Regressions from the 2026-10-08 correctness audit of the GT and flow
//! dimension, interior, dilation, and interpolation paths.
//!
//! The expected values are mathematical answers.  Polynomials are checked
//! against direct lattice-point counts at fresh dilations that were not used
//! for interpolation, so these tests do not merely compare two routines that
//! share the same dimension computation.

use ehrcalc_kostka_engine::{
    ehrhart::{scale_parts, try_compute_ehrhart, EhrhartPoly},
    flow::FlowPolytope,
    gt_dim::{
        gt_polytope_affine_masks_masked, gt_polytope_dim, gt_polytope_dim_full,
        gt_polytope_tight_inequalities_masked,
    },
    kostka_dp::{
        masked_relative_interior_masks, strict_skew_kostka_legacy, try_flagged_skew_kostka,
        try_strict_flagged_skew_kostka, try_strict_skew_kostka,
    },
    packed_modular::try_strict_masked_flagged_skew_kostka_packed_exact_stats,
    Partition,
};
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};

fn p(parts: &[u32]) -> Partition {
    Partition::new(parts.to_vec())
}

fn scaled(parts: &[u32], dilation: u32) -> Vec<u32> {
    parts.iter().map(|part| part * dilation).collect()
}

fn evaluate(polynomial: &EhrhartPoly, dilation: i64) -> BigRational {
    let x = BigRational::from_integer(BigInt::from(dilation));
    polynomial
        .coeffs
        .iter()
        .rev()
        .fold(BigRational::zero(), |value, coefficient| {
            value * &x + coefficient
        })
}

fn rational(value: BigUint) -> BigRational {
    BigRational::from_integer(BigInt::from(value))
}

fn full_count(lambda: &[u32], weight: &[u32], lower: Option<&[u32]>, dilation: u32) -> BigUint {
    try_flagged_skew_kostka(
        &p(&scaled(lambda, dilation)),
        &Partition::empty(),
        &scaled(weight, dilation),
        None,
        lower,
        None,
    )
    .unwrap()
}

fn interior_count(lambda: &[u32], weight: &[u32], lower: Option<&[u32]>, dilation: u32) -> BigUint {
    try_strict_flagged_skew_kostka(
        &p(&scaled(lambda, dilation)),
        &Partition::empty(),
        &scaled(weight, dilation),
        None,
        lower,
        None,
    )
    .unwrap()
}

fn compositions(total: u32, max_parts: usize) -> Vec<Vec<u32>> {
    fn extend(remaining: u32, max_parts: usize, current: &mut Vec<u32>, out: &mut Vec<Vec<u32>>) {
        if remaining == 0 {
            out.push(current.clone());
            return;
        }
        if current.len() == max_parts {
            return;
        }
        for part in 1..=remaining {
            current.push(part);
            extend(remaining - part, max_parts, current, out);
            current.pop();
        }
    }
    let mut out = Vec::new();
    extend(total, max_parts, &mut Vec::new(), &mut out);
    out
}

/// At dilation `N` the first two levels are `(N,0,0)` and `(a,2N-a,0)`; the
/// next level is `(3N, a, 2N-a)` because three upper bounds sum to the level
/// total.  Hence the polytope is the segment `N <= a <= 2N`.
#[test]
fn gt_dimension_includes_jointly_forced_equalities() {
    let lambda = [3, 2, 1];
    let weight = [1, 1, 3, 1];
    assert_eq!(
        gt_polytope_dim_full(&lambda, &[], &weight, None, None),
        Some(1)
    );
    for dilation in 1..=4 {
        assert_eq!(
            full_count(&lambda, &weight, None, dilation),
            BigUint::from(dilation + 1)
        );
        assert_eq!(
            interior_count(&lambda, &weight, None, dilation),
            BigUint::from(dilation - 1)
        );
        assert_eq!(
            gt_polytope_dim_full(
                &scaled(&lambda, dilation),
                &[],
                &scaled(&weight, dilation),
                None,
                None
            ),
            Some(1)
        );
    }
    for reciprocity in [false, true] {
        let polynomial = try_compute_ehrhart(
            &p(&lambda),
            &Partition::empty(),
            &weight,
            None,
            None,
            false,
            None,
            reciprocity,
        )
        .unwrap();
        assert_eq!(polynomial.degree, 1);
        assert_eq!(evaluate(&polynomial, 7), rational(BigUint::from(8_u32)));
    }
    let legacy = strict_skew_kostka_legacy(
        &p(&[9, 6, 3]),
        &Partition::empty(),
        &[3, 3, 9, 3],
        None,
        false,
    );
    assert_eq!(legacy, BigUint::from(2_u32));
}

#[test]
fn flagged_and_redundantly_flagged_faces_keep_the_exact_dimension() {
    let lambda = [3, 2, 1];
    let weight = [1, 1, 3, 1];
    // Label 4 is already confined to rows 2 and 3 on this segment, so the
    // original flagged example has the same polytope.
    for lower in [[1, 1, 1, 2], [1, 1, 1, 1]] {
        assert_eq!(
            gt_polytope_dim_full(&lambda, &[], &weight, None, Some(&lower)),
            Some(1)
        );
        assert_eq!(
            gt_polytope_dim_full(&lambda, &[], &weight, Some(&[3, 3, 3, 3]), Some(&lower)),
            Some(1)
        );
        for dilation in 1..=4 {
            assert_eq!(
                full_count(&lambda, &weight, Some(&lower), dilation),
                BigUint::from(dilation + 1)
            );
            assert_eq!(
                interior_count(&lambda, &weight, Some(&lower), dilation),
                BigUint::from(dilation - 1)
            );
        }
        for reciprocity in [false, true] {
            let polynomial = try_compute_ehrhart(
                &p(&lambda),
                &Partition::empty(),
                &weight,
                None,
                Some(&lower),
                false,
                None,
                reciprocity,
            )
            .unwrap();
            assert_eq!(polynomial.degree, 1);
            assert_eq!(evaluate(&polynomial, 6), rational(BigUint::from(7_u32)));
        }
    }
}

#[test]
fn packed_interior_masks_include_jointly_forced_equalities() {
    let lambda = p(&[9, 6, 3]);
    let weight = [3, 3, 9, 3];
    let masks =
        masked_relative_interior_masks(&lambda, &Partition::empty(), &weight, None, None, None)
            .unwrap()
            .unwrap();
    assert_eq!(masks.dimension, 1);
    let (dimension, stats) = try_strict_masked_flagged_skew_kostka_packed_exact_stats(
        &lambda,
        &Partition::empty(),
        &weight,
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .unwrap();
    assert_eq!(dimension, 1);
    assert_eq!(stats.value, BigUint::from(2_u32));
}

/// Four copies of a label cannot fit into three columns, so every dilation is
/// empty although no single coordinate interval is empty.
#[test]
fn jointly_infeasible_gt_polytope_is_empty() {
    let lambda = [3, 3, 2, 1];
    let weight = [1, 1, 2, 4, 1];
    assert_eq!(
        gt_polytope_dim_full(&lambda, &[], &weight, None, None),
        None
    );
    assert_eq!(
        gt_polytope_affine_masks_masked(&lambda, &[], &weight, None, None, None),
        None
    );
    for dilation in 1..=3 {
        assert!(full_count(&lambda, &weight, None, dilation).is_zero());
    }
    for reciprocity in [false, true] {
        let polynomial = try_compute_ehrhart(
            &p(&lambda),
            &Partition::empty(),
            &weight,
            None,
            None,
            false,
            None,
            reciprocity,
        )
        .unwrap();
        assert!(polynomial.coeffs.iter().all(Zero::is_zero));
    }
}

/// Straight-shape grid with every composition, so every content permutation,
/// and fresh positive and interior samples.
#[test]
fn straight_gt_grid_matches_fresh_counts() {
    let mut checked = 0;
    for size in 1..=6_u32 {
        for lambda in Partition::all_of_size_bounded(size, 4, size) {
            let lambda = lambda.parts().to_vec();
            let mut dimension_by_sorted_weight = std::collections::HashMap::new();
            for weight in compositions(size, 5) {
                let full = full_count(&lambda, &weight, None, 1);
                let dimension = gt_polytope_dim_full(&lambda, &[], &weight, None, None);
                // A straight GT polytope is rationally nonempty exactly when
                // lambda dominates the sorted weight, exactly as for K > 0.
                assert_eq!(
                    dimension.is_some(),
                    !full.is_zero(),
                    "{lambda:?} {weight:?}"
                );
                let mut sorted = weight.clone();
                sorted.sort_unstable_by(|left, right| right.cmp(left));
                let previous = dimension_by_sorted_weight.insert(sorted, dimension);
                if let Some(previous) = previous {
                    assert_eq!(
                        previous, dimension,
                        "content permutation {lambda:?} {weight:?}"
                    );
                }
                let Some(dimension) = dimension else {
                    continue;
                };
                assert_eq!(
                    gt_polytope_dim_full(&scaled(&lambda, 2), &[], &scaled(&weight, 2), None, None),
                    Some(dimension)
                );
                let rows = lambda.len() as u32;
                assert_eq!(
                    gt_polytope_dim_full(
                        &lambda,
                        &[],
                        &weight,
                        Some(&vec![rows; weight.len()]),
                        Some(&vec![1; weight.len()]),
                    ),
                    Some(dimension)
                );
                if dimension > 6 {
                    continue;
                }
                let positive = try_compute_ehrhart(
                    &p(&lambda),
                    &Partition::empty(),
                    &weight,
                    None,
                    None,
                    false,
                    None,
                    false,
                )
                .unwrap();
                let reciprocal = try_compute_ehrhart(
                    &p(&lambda),
                    &Partition::empty(),
                    &weight,
                    None,
                    None,
                    false,
                    None,
                    true,
                )
                .unwrap();
                assert_eq!(positive.degree, dimension, "{lambda:?} {weight:?}");
                assert_eq!(positive.coeffs, reciprocal.coeffs, "{lambda:?} {weight:?}");
                let fresh = dimension as u32 + 2;
                assert_eq!(
                    evaluate(&positive, i64::from(fresh)),
                    rational(full_count(&lambda, &weight, None, fresh)),
                    "{lambda:?} {weight:?}"
                );
                let sign = if dimension.is_multiple_of(2) { 1 } else { -1 };
                assert_eq!(
                    evaluate(&positive, -2) * BigRational::from_integer(BigInt::from(sign)),
                    rational(interior_count(&lambda, &weight, None, 2)),
                    "{lambda:?} {weight:?}"
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 100);
}

/// Vertex 1 forces `x01 = N + x13`, vertex 0 forces `x01 + x02 = N`; hence
/// `x02 = x13 = 0` and the polytope is one point.
#[test]
fn flow_support_respects_saturated_balance_constraints() {
    let flow =
        FlowPolytope::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)], vec![1, -1, 1, -1]).unwrap();
    assert_eq!(flow.dimension().unwrap(), 0);
    assert_eq!(flow.ehrhart_poly(None).unwrap().coeffs.len(), 1);
    assert_eq!(flow.ehrhart_poly_positive(None).unwrap().degree, 0);
    for dilation in 1..=3 {
        assert_eq!(
            flow.count_lattice_points(dilation, None).unwrap(),
            BigUint::from(1_u32)
        );
        assert_eq!(
            flow.count_interior_lattice_points(dilation, None).unwrap(),
            BigUint::from(1_u32)
        );
    }
    let empty = FlowPolytope::new(3, vec![(0, 1)], vec![1, 0, -1]).unwrap();
    assert!(empty.dimension().is_err());
}

/// Every nonempty flow polytope on four vertices with increasing edges and
/// small netflows: dimension, both interpolations, and interiors against
/// fresh direct counts.
#[test]
fn four_vertex_flow_grid_matches_fresh_counts() {
    let all = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    let mut checked = 0;
    for mask in 0..64_u32 {
        let edges = all
            .iter()
            .enumerate()
            .filter(|(index, _)| mask & (1 << index) != 0)
            .map(|(_, edge)| *edge)
            .collect::<Vec<_>>();
        for a in -2..=2_i64 {
            for b in -2..=2_i64 {
                for c in -2..=2_i64 {
                    let netflow = vec![a, b, c, -a - b - c];
                    let flow = FlowPolytope::new(4, edges.clone(), netflow.clone()).unwrap();
                    if flow.count_lattice_points(1, None).unwrap().is_zero() {
                        assert!(flow.dimension().is_err());
                        continue;
                    }
                    let dimension = flow.dimension().unwrap();
                    let positive = flow.ehrhart_poly_positive(None).unwrap();
                    let reciprocal = flow.ehrhart_poly(None).unwrap();
                    assert_eq!(positive.degree, dimension, "{edges:?} {netflow:?}");
                    assert_eq!(positive.coeffs, reciprocal.coeffs, "{edges:?} {netflow:?}");
                    let fresh = dimension as u64 + 3;
                    assert_eq!(
                        evaluate(&positive, fresh as i64),
                        rational(flow.count_lattice_points(fresh, None).unwrap()),
                        "{edges:?} {netflow:?}"
                    );
                    let sign = if dimension.is_multiple_of(2) { 1 } else { -1 };
                    assert_eq!(
                        evaluate(&positive, -3) * BigRational::from_integer(BigInt::from(sign)),
                        rational(flow.count_interior_lattice_points(3, None).unwrap()),
                        "{edges:?} {netflow:?}"
                    );
                    checked += 1;
                }
            }
        }
    }
    assert!(checked > 1000);
}

/// The second partition at dilation `N` is `(a, 2147483648N - a)` with
/// `2147483647N <= a <= 2147483648N`, so the true polynomial is `N + 1`.
/// Dilation two leaves the supported `u32` range and must be reported.
#[test]
fn gt_dilation_does_not_wrap_u32_coordinates() {
    let lambda = p(&[2_147_483_648, 1]);
    let weight = [2_147_483_647, 1, 1];
    for reciprocity in [false, true] {
        match try_compute_ehrhart(
            &lambda,
            &Partition::empty(),
            &weight,
            None,
            None,
            false,
            None,
            reciprocity,
        ) {
            Ok(polynomial) => assert_eq!(evaluate(&polynomial, 3), rational(BigUint::from(4_u32))),
            Err(error) => assert!(error.contains("u32"), "{error}"),
        }
    }
    assert_eq!(scale_parts(&[u32::MAX], 1).unwrap(), vec![u32::MAX]);
    assert!(scale_parts(&[2_147_483_648], 2).is_err());
    assert!(scale_parts(&[1], u64::MAX).is_err());
}

#[test]
fn helper_counts_match_a_known_stretching_polynomial() {
    // K((2N, N), (N, N, N)) = N + 1.
    assert_eq!(full_count(&[2, 1], &[1, 1, 1], None, 5).to_u64(), Some(6));
}

/// The generic dimension routines have no 32-row limit.  A column of 33 boxes
/// with content `1^33` has one tableau, and the hook `(2, 1^32)` with standard
/// content has `binom(N + 32, 32)` points at dilation `N`.
#[test]
fn generic_gt_dimension_accepts_more_than_thirty_two_rows() {
    assert_eq!(gt_polytope_dim(&[1; 33], &[], &[1; 33]), Some(0));
    assert_eq!(gt_polytope_dim(&[1; 40], &[], &[1; 40]), Some(0));
    assert_eq!(
        gt_polytope_dim_full(&[1; 33], &[], &[1; 33], Some(&[33; 33]), Some(&[1; 33])),
        Some(0)
    );
    for reciprocity in [false, true] {
        let polynomial = try_compute_ehrhart(
            &p(&[1; 33]),
            &Partition::empty(),
            &[1; 33],
            None,
            None,
            false,
            None,
            reciprocity,
        )
        .unwrap();
        assert_eq!(polynomial.coeffs, vec![BigRational::from_integer(1.into())]);
    }

    let mut hook = vec![2];
    hook.extend([1; 32]);
    let weight = [1; 34];
    assert_eq!(gt_polytope_dim(&hook, &[], &weight), Some(32));
    for (dilation, expected) in [(1_u32, 33_u32), (2, 561), (3, 6545)] {
        assert_eq!(
            full_count(&hook, &weight, None, dilation),
            BigUint::from(expected)
        );
    }

    // Forbidding label 32 in row 32 (bit 31) empties the column.
    let mut forbidden = vec![0_u32; 33];
    forbidden[31] = 1 << 31;
    assert_eq!(
        gt_polytope_tight_inequalities_masked(
            &[1; 33],
            &[],
            &[1; 33],
            None,
            None,
            Some(&forbidden)
        ),
        None
    );
    // With 40 rows bit 31 still names row 32; bit 30 names a row that label
    // 32 never occupies, so that mask leaves the single point.
    for (bit, nonempty) in [(31, false), (30, true)] {
        let mut forbidden = vec![0_u32; 40];
        forbidden[31] = 1 << bit;
        let tight = gt_polytope_tight_inequalities_masked(
            &[1; 40],
            &[],
            &[1; 40],
            None,
            None,
            Some(&forbidden),
        );
        assert_eq!(tight.map(|tight| tight.dimension), nonempty.then_some(0));
    }
}

/// The audit's segment witness translated into a 33-row skew shape: rows 3
/// to 32 are empty, so the polytope is still a segment with `N + 1` points.
/// Boolean tightness and the legacy strict counter have no row limit, while
/// the documented bitmask interfaces refuse the shape instead of truncating.
#[test]
fn thirty_three_row_segment_keeps_exact_interiors() {
    let mut outer = vec![4, 3, 2];
    outer.extend([1; 30]);
    let inner = [1; 33];
    let weight = [1, 1, 3, 1];
    assert_eq!(gt_polytope_dim(&outer, &inner, &weight), Some(1));
    let tight =
        gt_polytope_tight_inequalities_masked(&outer, &inner, &weight, None, None, None).unwrap();
    assert_eq!(tight.dimension, 1);
    assert!(tight
        .lower
        .iter()
        .chain(&tight.diagonal)
        .all(|rows| rows.len() == 33));
    // Empty rows force every inequality below row 3 to equality.
    assert!(tight
        .lower
        .iter()
        .all(|rows| rows[3..].iter().all(|&is_tight| is_tight)));
    for dilation in 1..=4 {
        let scaled_outer = p(&scaled(&outer, dilation));
        let scaled_inner = p(&scaled(&inner, dilation));
        let scaled_weight = scaled(&weight, dilation);
        let full = try_flagged_skew_kostka(
            &scaled_outer,
            &scaled_inner,
            &scaled_weight,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(full, BigUint::from(dilation + 1));
        let interior =
            strict_skew_kostka_legacy(&scaled_outer, &scaled_inner, &scaled_weight, None, false);
        assert_eq!(interior, BigUint::from(dilation - 1));
    }
    let error = try_strict_skew_kostka(&p(&outer), &p(&inner), &weight, None, false).unwrap_err();
    assert!(error.contains("32 rows"), "{error}");
}

#[test]
#[should_panic(expected = "at most 32 rows")]
fn tight_inequality_bitmasks_refuse_more_than_thirty_two_rows() {
    gt_polytope_affine_masks_masked(&[1; 33], &[], &[1; 33], None, None, None);
}
