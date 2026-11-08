//! Regression fixture: relaxing one row flag keeps the dilation-one lattice
//! points but raises the affine dimension, so the affine hull cannot be
//! inferred from the integer points of the first dilation.
use ehrcalc::families::{gt_ehrhart, GtInput};
use num_bigint::BigInt;
use num_rational::BigRational;

const SHAPE: [u32; 5] = [3, 2, 2, 1, 1];
const WEIGHT: [u32; 6] = [1, 2, 2, 2, 1, 1];
const TIGHT_FLAGS: [u32; 5] = [4, 4, 6, 6, 6];
const RELAXED_FLAGS: [u32; 5] = [4, 5, 6, 6, 6];

/// Lower flag of each entry value `j = 1..6`: the first row whose flag allows `j`.
fn lower_flags(row_flags: &[u32]) -> Vec<u32> {
    (1..=6)
        .map(|j| row_flags.iter().position(|&b| b >= j).unwrap() as u32 + 1)
        .collect()
}

fn ehrhart(row_flags: &[u32], use_reciprocity: bool) -> ehrcalc::exact::EhrhartPolynomial {
    gt_ehrhart(&GtInput {
        lambda: SHAPE.to_vec(),
        mu: vec![],
        weight: WEIGHT.to_vec(),
        upper_flags: None,
        lower_flags: Some(lower_flags(row_flags)),
        max_states: Some(100_000),
        use_reciprocity,
    })
    .unwrap()
    .ehrhart
}

fn integer(value: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(value))
}

#[test]
fn relaxed_flag_raises_dimension_with_equal_first_dilation() {
    let tight = ehrhart(&TIGHT_FLAGS, false);
    let relaxed = ehrhart(&RELAXED_FLAGS, false);
    // Both interpolation modes (fresh positive counts and reciprocity) agree.
    assert_eq!(tight, ehrhart(&TIGHT_FLAGS, true));
    assert_eq!(relaxed, ehrhart(&RELAXED_FLAGS, true));

    assert_eq!(tight.dimension(), 5);
    assert_eq!(relaxed.dimension(), 6);
    // Same lattice points at dilation one, different ones at dilation two.
    assert_eq!(tight.evaluate(1), relaxed.evaluate(1));
    assert_eq!(tight.evaluate(2), integer(27));
    assert_eq!(relaxed.evaluate(2), integer(28));
}

#[test]
fn difference_has_negative_linear_coefficient() {
    let tight = ehrhart(&TIGHT_FLAGS, false);
    let relaxed = ehrhart(&RELAXED_FLAGS, false);
    let as_strings = |p: &ehrcalc::exact::EhrhartPolynomial| {
        p.power_coeffs()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        as_strings(&tight),
        ["1", "149/60", "55/24", "1", "5/24", "1/60"]
    );
    assert_eq!(
        as_strings(&relaxed),
        ["1", "49/20", "203/90", "49/48", "35/144", "7/240", "1/720"]
    );
    // Relaxing the flag lowers the linear coefficient: 49/20 - 149/60 = -1/30.
    let linear_difference = &relaxed.power_coeffs()[1] - &tight.power_coeffs()[1];
    assert_eq!(
        linear_difference,
        BigRational::new(BigInt::from(-1), BigInt::from(30))
    );
}
