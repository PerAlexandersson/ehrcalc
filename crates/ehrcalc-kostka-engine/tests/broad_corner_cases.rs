//! Deterministic boundary and metamorphic tests from the second correctness review.
use ehrcalc_kostka_engine::{
    affine_hull::{rational, AffineHull, RationalPolyhedron},
    flow::FlowPolytope,
    gt_dim::{gt_polytope_dim, gt_polytope_dim_full},
    kostka_dp::{
        skew_kostka_legacy, strict_skew_kostka_legacy, try_flagged_skew_kostka, try_skew_kostka,
        try_strict_skew_kostka,
    },
    packed_modular::{crt_reconstruct_bounded, try_masked_flagged_skew_kostka_packed_exact_stats},
    Partition,
};
use num_bigint::BigUint;

fn p(parts: &[u32]) -> Partition {
    Partition::new(parts.to_vec())
}

#[test]
fn row_and_packing_boundaries_with_unique_tableaux() {
    for rows in [0, 1, 2, 30, 31, 32, 33, 63, 64, 65] {
        let shape = vec![1; rows];
        let weight = vec![1; rows];
        assert_eq!(gt_polytope_dim(&shape, &[], &weight), Some(0));
        assert_eq!(
            try_skew_kostka(&p(&shape), &p(&[]), &weight, Some(1000), false).unwrap(),
            1u32.into()
        );
        assert_eq!(
            strict_skew_kostka_legacy(&p(&shape), &p(&[]), &weight, Some(1000), false),
            1u32.into()
        );
    }
    // Constant-height rectangles have one tableau with content (width,...,width).
    // These have 124, 128, and 132 bits at width 15, respectively.
    for rows in [31, 32, 33] {
        let shape = p(&vec![15; rows]);
        let result = try_masked_flagged_skew_kostka_packed_exact_stats(
            &shape,
            &p(&[]),
            &vec![15; rows],
            None,
            None,
            None,
            None,
            None,
            Some(1000),
        );
        if rows <= 32 {
            assert_eq!(result.unwrap().value, BigUint::from(1u32));
        } else {
            assert!(result.is_err());
        }
    }
}

#[test]
fn zero_weight_insertion_preserves_the_polytope() {
    let shape = p(&[3, 2, 1]);
    for slot in 0..=4 {
        let mut weight = vec![1, 1, 3, 1];
        weight.insert(slot, 0);
        assert_eq!(gt_polytope_dim(shape.parts(), &[], &weight), Some(1));
        assert_eq!(
            try_skew_kostka(&shape, &p(&[]), &weight, None, false).unwrap(),
            2u32.into()
        );
        let mut upper = vec![3; weight.len()];
        let mut lower = vec![1; weight.len()];
        // An unused label may be forbidden everywhere without changing the fiber.
        upper[slot] = 0;
        lower[slot] = 4;
        assert_eq!(
            gt_polytope_dim_full(shape.parts(), &[], &weight, Some(&upper), Some(&lower)),
            Some(1)
        );
        assert_eq!(
            try_flagged_skew_kostka(&shape, &p(&[]), &weight, Some(&upper), Some(&lower), None)
                .unwrap(),
            2u32.into()
        );
    }
}

#[test]
fn large_horizontal_translation_preserves_small_skew_fiber() {
    let shift = u32::MAX / 3;
    let outer = p(&[shift + 3, shift + 2, shift + 1]);
    let inner = p(&[shift; 3]);
    let weight = [1, 1, 3, 1];
    assert_eq!(
        gt_polytope_dim(outer.parts(), inner.parts(), &weight),
        Some(1)
    );
    assert_eq!(
        try_skew_kostka(&outer, &inner, &weight, Some(100), false).unwrap(),
        2u32.into()
    );
    assert_eq!(
        try_strict_skew_kostka(&outer, &inner, &weight, Some(100), false).unwrap(),
        0u32.into()
    );
}

#[test]
#[ignore = "BCA-3: unchecked u32 shape totals; see docs/BROAD_CORNER_AUDIT.md"]
fn legacy_translation_does_not_overflow_total_shape_size() {
    let shift = u32::MAX / 3;
    let outer = p(&[shift + 3, shift + 2, shift + 1]);
    let inner = p(&[shift; 3]);
    assert_eq!(
        skew_kostka_legacy(&outer, &inner, &[1, 1, 3, 1], Some(100), false),
        2u32.into()
    );
}

#[test]
#[ignore = "BCA-3: unchecked u32 shape totals; see docs/BROAD_CORNER_AUDIT.md"]
fn public_lr_counter_handles_a_large_translated_single_box() {
    let shift = u32::MAX / 3;
    let outer = p(&[shift + 1, shift, shift]);
    let inner = p(&[shift; 3]);
    if let Ok(count) = ehrcalc_kostka_engine::lr::try_lr_dp(&outer, &inner, &p(&[1]), Some(100)) {
        assert_eq!(count, BigUint::from(1u32));
    }
}

#[test]
#[ignore = "BCA-1: unchecked i64 netflow sum; see docs/BROAD_CORNER_AUDIT.md"]
fn flow_constructor_rejects_nonzero_sum_without_wrapping() {
    assert!(FlowPolytope::new(2, vec![], vec![i64::MIN, i64::MIN]).is_err());
}

#[test]
#[ignore = "BCA-1: unchecked i64 netflow sum; see docs/BROAD_CORNER_AUDIT.md"]
fn flow_constructor_accepts_balanced_extreme_entries_without_panicking() {
    let flow = FlowPolytope::new(3, vec![(0, 1), (1, 2)], vec![i64::MAX, 1, i64::MIN]);
    // An explicit supported-range error is acceptable; a panic is not.
    if let Ok(flow) = flow {
        assert_eq!(flow.dimension().unwrap(), 0);
    }
}

#[test]
#[ignore = "BCA-2: unchecked i64 flow accumulation; see docs/BROAD_CORNER_AUDIT.md"]
fn flow_counter_does_not_report_zero_when_an_intermediate_flow_overflows() {
    // The node order keeps constructor partial sums within i64. The unique
    // flow carries 2^63 along 2->1, which the counting DP cannot store in i64.
    let flow = FlowPolytope::new(3, vec![(0, 2), (2, 1)], vec![i64::MAX, i64::MIN, 1]).unwrap();
    assert_eq!(flow.dimension().unwrap(), 0);
    if let Ok(count) = flow.count_lattice_points(1, None) {
        assert_eq!(
            count,
            BigUint::from(1u32),
            "return an error if the representation is too narrow"
        );
    }
}

#[test]
fn parallel_edge_flows_have_the_known_simplex_counts() {
    for edges in 1..=5 {
        for supply in 0..=4i64 {
            let flow = FlowPolytope::new(2, vec![(0, 1); edges], vec![supply, -supply]).unwrap();
            assert_eq!(
                flow.dimension().unwrap(),
                if supply == 0 { 0 } else { edges - 1 }
            );
            for n in 1..=5u64 {
                let amount = supply as u64 * n;
                let expected = (1..edges as u64).fold(1u64, |v, i| v * (amount + i) / i);
                assert_eq!(
                    flow.count_lattice_points(n, Some(10000)).unwrap(),
                    expected.into()
                );
            }
        }
    }
}

#[test]
fn flow_relabeling_and_edge_order_preserve_counts() {
    let edges = [(0, 1), (0, 2), (1, 3), (2, 3)];
    let netflow = [1, -1, 1, -1];
    for labels in [[0, 1, 2, 3], [3, 2, 1, 0], [2, 0, 3, 1], [1, 3, 0, 2]] {
        let mut a = vec![0; 4];
        for i in 0..4 {
            a[labels[i]] = netflow[i];
        }
        let mut relabeled = edges.map(|(u, v)| (labels[u], labels[v])).to_vec();
        relabeled.reverse();
        let flow = FlowPolytope::new(4, relabeled, a).unwrap();
        assert_eq!(flow.dimension().unwrap(), 0);
        for n in 1..=5 {
            assert_eq!(flow.count_lattice_points(n, None).unwrap(), 1u32.into());
            assert_eq!(
                flow.count_interior_lattice_points(n, None).unwrap(),
                1u32.into()
            );
        }
    }
}

#[test]
fn exact_hull_handles_redundancy_scaling_and_rational_singletons() {
    for denominator in 1..=7 {
        let mut system = RationalPolyhedron::new(3);
        system.add_equality([(0, rational(denominator))], rational(1));
        system.add_equality([(0, rational(2 * denominator))], rational(2));
        // x1 in [0,1]; x2 unconstrained. Dimension is exactly two.
        system.add_inequality([(1, rational(-1))], rational(0));
        system.add_inequality([(1, rational(1))], rational(1));
        system.add_inequality([], rational(0));
        system.add_inequality([], rational(1));
        let AffineHull::Nonempty(hull) = system.affine_hull() else {
            panic!("nonempty");
        };
        assert_eq!(hull.dimension, 2);
        assert_eq!(hull.implicit_equalities, [false, false, true, false]);
        assert_eq!(
            hull.relative_interior_point[0],
            rational(1) / rational(denominator)
        );
        system.add_inequality([(0, rational(denominator))], rational(0));
        assert_eq!(system.affine_hull(), AffineHull::Empty);
    }
}

#[test]
fn crt_handles_boundaries_and_coprime_composite_moduli() {
    for value in 0..72u32 {
        assert_eq!(
            crt_reconstruct_bounded(&[value % 8, value % 9], &[8, 9], &71u32.into()).unwrap(),
            value.into()
        );
    }
    assert!(crt_reconstruct_bounded(&[0, 0], &[8, 9], &72u32.into()).is_err());
    assert!(crt_reconstruct_bounded(&[0, 0], &[6, 9], &1u32.into()).is_err());
    assert!(crt_reconstruct_bounded(&[8, 0], &[8, 9], &71u32.into()).is_err());
    assert!(crt_reconstruct_bounded(&[], &[], &0u32.into()).is_err());
}
