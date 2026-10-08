//! Independent LP checks of Kogan-face interiors and union reciprocity.
use ehrcalc::exact::EhrhartPolynomial;
use ehrcalc_foundations::permutation::all_permutations_one_indexed;
use ehrcalc_foundations::{
    key_polynomial::{
        gt_patterns_with_equalities_n, key_ehrhart_polynomial, key_strict_lattice_point_count,
        reduced_kogan_faces, strict_gt_patterns_with_equalities_n, GtPattern,
    },
    Partition,
};
use ehrcalc_kostka_engine::affine_hull::{rational, AffineHull, RationalPolyhedron};
use num_bigint::BigInt;
use num_rational::BigRational;
use std::collections::HashSet;

fn positions(n: usize) -> Vec<(usize, usize)> {
    (1..n).flat_map(|i| (1..=i).map(move |j| (i, j))).collect()
}

fn flatten(p: &GtPattern) -> Vec<u32> {
    p.rows.iter().flatten().copied().collect()
}

#[test]
fn every_small_face_interior_matches_a_rational_affine_hull() {
    let mut tops = Vec::new();
    for a in 0..=2 {
        for b in 0..=a {
            for c in 0..=b {
                tops.push(vec![a, b, c]);
            }
        }
    }
    tops.extend([vec![3, 2, 1, 0], vec![2, 2, 1, 0], vec![1, 1, 1, 1]]);
    let mut checked = 0;
    for top in tops {
        let n = top.len();
        let pos = positions(n);
        let index = |i: usize, j: usize| i * (i - 1) / 2 + j - 1;
        for mask in 0..(1usize << pos.len()) {
            let eqs: Vec<_> = pos
                .iter()
                .enumerate()
                .filter_map(|(bit, &p)| (mask & (1 << bit) != 0).then_some(p))
                .collect();
            let scaled: Vec<_> = top.iter().map(|x| 2 * x).collect();
            let partition = Partition::new(scaled.clone());
            let mut system = RationalPolyhedron::new(n * (n + 1) / 2);
            for (j, &value) in scaled.iter().enumerate() {
                system.add_equality([(index(n, j + 1), rational(1))], rational(value));
            }
            let mut edges = Vec::new();
            for &(i, j) in &pos {
                let (a, b, c) = (index(i + 1, j), index(i, j), index(i + 1, j + 1));
                for (high, low) in [(a, b), (b, c)] {
                    system.add_inequality([(low, rational(1)), (high, rational(-1))], rational(0));
                    edges.push((high, low));
                }
                if eqs.contains(&(i, j)) {
                    system.add_equality([(a, rational(1)), (b, rational(-1))], rational(0));
                }
            }
            let AffineHull::Nonempty(hull) = system.affine_hull() else {
                panic!("Kogan face is nonempty");
            };
            let weak = gt_patterns_with_equalities_n(&partition, n, &eqs);
            let expected: HashSet<_> = weak
                .into_iter()
                .filter(|p| {
                    let values = flatten(p);
                    edges
                        .iter()
                        .zip(&hull.implicit_equalities)
                        .all(|(&(a, b), &tight)| tight || values[a] > values[b])
                })
                .collect();
            let actual: HashSet<_> = strict_gt_patterns_with_equalities_n(&partition, n, &eqs)
                .into_iter()
                .collect();
            assert_eq!(actual, expected, "top={top:?} eqs={eqs:?}");
            // For n=3, also check reciprocity using weak counts only.
            if n == 3 {
                let samples: Vec<_> = (0..=hull.dimension)
                    .map(|k| {
                        let lambda = Partition::new(top.iter().map(|x| x * k as u32).collect());
                        (
                            k as i64,
                            BigRational::from_integer(BigInt::from(
                                gt_patterns_with_equalities_n(&lambda, n, &eqs).len(),
                            )),
                        )
                    })
                    .collect();
                let poly = EhrhartPolynomial::interpolate(hull.dimension, &samples).unwrap();
                let sign = if hull.dimension % 2 == 0 { 1 } else { -1 };
                assert_eq!(
                    poly.evaluate(-2) * BigInt::from(sign),
                    BigRational::from_integer(actual.len().into())
                );
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 272);
}

#[test]
fn key_union_reciprocity_includes_shared_strata() {
    let permutations = [
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1],
    ];
    for a in 0..=3 {
        for b in 0..=a {
            for c in 0..=b {
                let lambda = Partition::new(vec![a, b, c]);
                for sigma in permutations {
                    let poly = key_ehrhart_polynomial(&lambda, &sigma, None);
                    let sign = if poly.degree.is_multiple_of(2) { 1 } else { -1 };
                    for k in 1..=3 {
                        let actual = key_strict_lattice_point_count(&lambda, &sigma, k);
                        assert_eq!(
                            poly.eval(-i64::from(k)) * BigInt::from(sign),
                            BigRational::from_integer(actual.into()),
                            "lambda={lambda:?} sigma={sigma:?} k={k}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn repeated_extreme_top_rows_and_empty_patterns_are_safe() {
    let lambda = Partition::new(vec![u32::MAX; 3]);
    let patterns = strict_gt_patterns_with_equalities_n(&lambda, 3, &[]);
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].weight(), vec![u32::MAX; 3]);
    assert_eq!(key_strict_lattice_point_count(&lambda, &[3, 2, 1], 1), 1);
    let empty = Partition::new(vec![]);
    assert_eq!(key_strict_lattice_point_count(&empty, &[], 1), 1);
    assert_eq!(
        key_ehrhart_polynomial(&empty, &[], None).eval(2),
        BigRational::from_integer(1.into())
    );
}

#[test]
fn four_row_key_unions_with_repeated_marks_satisfy_reciprocity() {
    for top in [vec![2, 1, 0, 0], vec![2, 2, 1, 0], vec![1, 1, 1, 1]] {
        let lambda = Partition::new(top);
        for sigma in all_permutations_one_indexed(4) {
            let poly = key_ehrhart_polynomial(&lambda, &sigma, None);
            let sign = if poly.degree.is_multiple_of(2) { 1 } else { -1 };
            for k in 1..=3 {
                assert_eq!(
                    poly.eval(-i64::from(k)) * BigInt::from(sign),
                    BigRational::from_integer(
                        key_strict_lattice_point_count(&lambda, &sigma, k).into()
                    ),
                    "lambda={lambda:?} sigma={sigma:?} k={k}"
                );
            }
        }
    }
}

#[test]
fn a_shared_edge_contributes_to_the_reciprocal_key_count() {
    // The two faces are a rectangle and a triangle meeting along an edge.
    // For lambda=(2,1,0), sigma=(3,1,2), P(t)=(t+1)(3t+2)/2.
    let scaled = Partition::new(vec![4, 2]);
    let face_interiors: HashSet<_> = [vec![(1, 1)], vec![(2, 2)]]
        .into_iter()
        .flat_map(|eqs| strict_gt_patterns_with_equalities_n(&scaled, 3, &eqs))
        .collect();
    assert_eq!(face_interiors.len(), 1);
    assert_eq!(
        key_strict_lattice_point_count(&Partition::new(vec![2, 1]), &[3, 1, 2], 2),
        2
    );
}

#[test]
#[should_panic(expected = "Kogan subset enumeration requires fewer than 64 positions")]
fn kogan_subset_bound_cannot_wrap_in_release() {
    reduced_kogan_faces(12, &(1..=12).collect::<Vec<_>>());
}
