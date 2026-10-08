//! Exact-transform identities and small public-adapter corner cases.
use ehrcalc::{
    exact::{BinomialBasisPolynomial, EhrhartPolynomial, HStarPolynomial},
    families::{key_ehrhart, lr_count, order_ehrhart, KeyInput, LrInput, OrderInput},
};
use ehrcalc_foundations::{
    key_polynomial::{key_lattice_point_count, key_strict_lattice_point_count},
    Partition,
};
use num_bigint::BigInt;
use num_rational::BigRational;

#[test]
fn exact_transform_roundtrips_and_calculus_at_signed_points() {
    let mut cases = 0;
    for a in -2..=2 {
        for b in -2..=2 {
            for c in -2..=2 {
                let coefficients = vec![a.into(), b.into(), c.into()];
                let basis = BinomialBasisPolynomial::new(coefficients);
                let poly = basis.to_polynomial().unwrap();
                assert_eq!(poly.to_binomial_basis().unwrap(), basis);
                assert_eq!(poly.to_hstar().unwrap().to_ehrhart().unwrap(), poly);
                let difference = poly.finite_difference().unwrap();
                let sum = poly.discrete_sum().unwrap();
                let shifted = poly.dilate_and_shift(-2, 3).unwrap();
                for n in -5..=5 {
                    assert_eq!(
                        difference.evaluate(n),
                        poly.evaluate(n + 1) - poly.evaluate(n)
                    );
                    assert_eq!(sum.evaluate(n + 1) - sum.evaluate(n), poly.evaluate(n));
                    assert_eq!(shifted.evaluate(n), poly.evaluate(-2 * n + 3));
                }
                cases += 1;
            }
        }
    }
    eprintln!("integer-valued polynomial transform cases: {cases}");
    let constant = EhrhartPolynomial::new(3, vec![BigRational::from_integer(1.into())]).unwrap();
    assert_eq!(
        constant.to_hstar().unwrap().coeffs(),
        &[1.into(), (-3).into(), 3.into(), (-1).into()]
    );
    assert_eq!(constant.to_hstar().unwrap().to_ehrhart().unwrap(), constant);
}

#[test]
fn interpolation_rejects_duplicate_abscissae_and_uses_exact_extremes() {
    let one = BigRational::from_integer(1.into());
    assert!(EhrhartPolynomial::interpolate(1, &[(0, one.clone()), (0, one)]).is_err());
    let points = [i64::MIN, 0, i64::MAX].map(|x| {
        let xbig = BigInt::from(x);
        (x, BigRational::from_integer(&xbig * &xbig + 1))
    });
    let poly = EhrhartPolynomial::interpolate(2, &points).unwrap();
    assert_eq!(
        poly.power_coeffs(),
        &[1.into(), 0.into(), 1.into()].map(BigRational::from_integer)
    );
    assert!(HStarPolynomial::new(2, vec![1.into()]).is_err());
}

#[test]
fn key_identity_longest_and_repeated_top_rows() {
    for n in 1..=4u32 {
        // pi_s1(x1^(2n)) has 2n+1 terms.
        assert_eq!(
            key_lattice_point_count(&Partition::new(vec![2]), &[2, 1], n),
            u64::from(2 * n + 1)
        );
        assert_eq!(
            key_lattice_point_count(&Partition::new(vec![2]), &[1, 2], n),
            1
        );
        let data = key_ehrhart(&KeyInput {
            lambda: vec![1, 1],
            sigma: vec![2, 1],
            max_degree: None,
        })
        .unwrap();
        assert_eq!(data.ehrhart.dimension(), 0);
        assert_eq!(
            data.ehrhart.evaluate(i64::from(n)),
            BigRational::from_integer(1.into())
        );
    }
}

#[test]
#[ignore = "BCA-4: key interior misses implicit equalities; see docs/BROAD_CORNER_AUDIT.md"]
fn key_strict_counter_counts_the_relative_interior_of_a_point() {
    // GT(1,1) is one point; every interlacing inequality is implicit.
    for n in 1..=3 {
        assert_eq!(
            key_strict_lattice_point_count(&Partition::new(vec![1, 1]), &[2, 1], n),
            1
        );
    }
}

#[test]
#[ignore = "BCA-3: unchecked u32 shape totals; see docs/BROAD_CORNER_AUDIT.md"]
fn lr_adapter_does_not_silently_drop_a_translated_box() {
    let shift = u32::MAX / 3;
    let result = lr_count(&LrInput {
        lambda: vec![shift + 1, shift, shift],
        mu: vec![shift; 3],
        nu: vec![1],
        max_states: Some(100),
    });
    if let Ok(count) = result {
        assert_eq!(count, BigInt::from(1));
    }
}

#[test]
fn order_polytopes_match_closed_form_counts_beyond_machine_words() {
    for elements in [0, 1, 5, 20] {
        let cube = order_ehrhart(&OrderInput::Antichain { elements }).unwrap();
        assert_eq!(cube.ehrhart.dimension(), elements);
        for n in [0, 1, 2, 97] {
            let expected = BigInt::from(n + 1).pow(elements as u32);
            assert_eq!(
                cube.ehrhart.evaluate(n),
                BigRational::from_integer(expected)
            );
        }
    }
    for elements in [0, 1, 5, 12] {
        let simplex = order_ehrhart(&OrderInput::Chain { elements }).unwrap();
        for n in [0, 1, 2, 97] {
            let expected = (1..=elements).fold(BigInt::from(1), |v, i| v * (n as usize + i) / i);
            assert_eq!(
                simplex.ehrhart.evaluate(n),
                BigRational::from_integer(expected)
            );
        }
    }
}

#[test]
fn order_polytopes_ignore_redundant_edges_and_relabeling() {
    let diamond = [(0, 1), (0, 2), (1, 3), (2, 3)];
    let base = order_ehrhart(&OrderInput::Covers {
        vertices: 4,
        covers: diamond.to_vec(),
    })
    .unwrap();
    for labels in [[0, 1, 2, 3], [3, 2, 1, 0], [2, 0, 3, 1]] {
        let mut covers = diamond.map(|(u, v)| (labels[u], labels[v])).to_vec();
        covers.extend([(labels[0], labels[3]), (labels[0], labels[1])]);
        covers.reverse();
        let data = order_ehrhart(&OrderInput::Covers {
            vertices: 4,
            covers,
        })
        .unwrap();
        assert_eq!(data.ehrhart, base.ehrhart);
    }
    for (vertices, covers) in [
        (2, vec![(0, 2)]),
        (2, vec![(0, 1), (1, 0)]),
        (1, vec![(0, 0)]),
    ] {
        assert!(order_ehrhart(&OrderInput::Covers { vertices, covers }).is_err());
    }
}
