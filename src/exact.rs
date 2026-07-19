//! Exact polynomial representations and Ehrhart transformations.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

/// Result type used by the exact, family-neutral layer.
pub type ExactResult<T> = Result<T, String>;

/// A polynomial in the dilation variable with an explicitly declared dimension.
///
/// `dimension` is not inferred from the trimmed power-basis degree.  Ehrhart
/// numerators can have trailing zero h* entries, so callers must retain the
/// geometrically meaningful dimension separately.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EhrhartPolynomial {
    dimension: usize,
    power_coeffs: Vec<BigRational>,
}

/// An h* numerator whose length records the declared Ehrhart dimension.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HStarPolynomial {
    dimension: usize,
    coeffs: Vec<BigInt>,
}

/// An integer-valued polynomial in the standard basis `binom(n, j)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinomialBasisPolynomial {
    coeffs: Vec<BigInt>,
}

/// An exact Ehrhart polynomial together with its h* numerator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EhrhartData {
    pub ehrhart: EhrhartPolynomial,
    pub hstar: HStarPolynomial,
}

impl EhrhartPolynomial {
    /// Construct a polynomial with a declared affine dimension.
    pub fn new(dimension: usize, power_coeffs: Vec<BigRational>) -> ExactResult<Self> {
        let power_coeffs = normalize_rational_coeffs(power_coeffs);
        let degree = power_coeffs.len().saturating_sub(1);
        if degree > dimension {
            return Err(format!(
                "polynomial degree {degree} exceeds declared dimension {dimension}"
            ));
        }
        Ok(Self {
            dimension,
            power_coeffs,
        })
    }

    /// Interpolate from exact values at distinct integral points.
    pub fn interpolate(
        dimension: usize,
        points: &[(i64, BigRational)],
    ) -> ExactResult<Self> {
        if points.len() != dimension + 1 {
            return Err(format!(
                "dimension {dimension} requires {} interpolation points, got {}",
                dimension + 1,
                points.len()
            ));
        }
        for (index, (x, _)) in points.iter().enumerate() {
            if points[..index].iter().any(|(previous, _)| previous == x) {
                return Err(format!("interpolation point {x} occurs more than once"));
            }
        }

        let mut result = vec![BigRational::zero(); dimension + 1];
        for (index, (x_i, y_i)) in points.iter().enumerate() {
            let mut basis = vec![BigRational::one()];
            let mut denominator = BigRational::one();
            for (other_index, (x_j, _)) in points.iter().enumerate() {
                if index == other_index {
                    continue;
                }
                basis = multiply_by_linear(&basis, -BigInt::from(*x_j));
                denominator *= BigRational::from(BigInt::from(*x_i - *x_j));
            }
            let scale = y_i.clone() / denominator;
            for (degree, coefficient) in basis.into_iter().enumerate() {
                result[degree] += scale.clone() * coefficient;
            }
        }
        Self::new(dimension, result)
    }

    /// Construct from consecutive values `P(0), ..., P(dimension)`.
    pub fn from_values(dimension: usize, values: &[BigInt]) -> ExactResult<Self> {
        if values.len() != dimension + 1 {
            return Err(format!(
                "dimension {dimension} requires {} values, got {}",
                dimension + 1,
                values.len()
            ));
        }
        let points = values
            .iter()
            .enumerate()
            .map(|(index, value)| (index as i64, BigRational::from(value.clone())))
            .collect::<Vec<_>>();
        Self::interpolate(dimension, &points)
    }

    /// The affine dimension declared by the family adapter or caller.
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// The degree after trimming trailing zero coefficients.
    pub fn degree(&self) -> usize {
        self.power_coeffs.len().saturating_sub(1)
    }

    /// Exact power-basis coefficients in ascending degree order.
    pub fn power_coeffs(&self) -> &[BigRational] {
        &self.power_coeffs
    }

    /// Evaluate at any integral dilation, including values used in reciprocity.
    pub fn evaluate(&self, dilation: i64) -> BigRational {
        let dilation = BigRational::from(BigInt::from(dilation));
        let mut value = BigRational::zero();
        for coefficient in self.power_coeffs.iter().rev() {
            value = value * dilation.clone() + coefficient.clone();
        }
        value
    }

    /// Convert to an h* numerator while preserving the declared dimension.
    pub fn to_hstar(&self) -> ExactResult<HStarPolynomial> {
        let mut coeffs = Vec::with_capacity(self.dimension + 1);
        for index in 0..=self.dimension {
            let mut value = BigRational::zero();
            for sample in 0..=index {
                let sign = if (index - sample) % 2 == 0 {
                    BigInt::one()
                } else {
                    -BigInt::one()
                };
                let factor = binomial_bigint(self.dimension + 1, index - sample);
                value += BigRational::from(sign * factor) * self.evaluate(sample as i64);
            }
            if value.denom() != &BigInt::one() {
                return Err(format!(
                    "h* coefficient at index {index} is not integral: {}/{}",
                    value.numer(),
                    value.denom()
                ));
            }
            coeffs.push(value.to_integer());
        }
        HStarPolynomial::new(self.dimension, coeffs)
    }

    /// Convert to the standard integer-valued basis `sum b_j binom(n, j)`.
    pub fn to_binomial_basis(&self) -> ExactResult<BinomialBasisPolynomial> {
        let mut values = (0..=self.degree())
            .map(|index| self.evaluate(index as i64))
            .collect::<Vec<_>>();
        let mut coeffs = Vec::with_capacity(values.len());
        while !values.is_empty() {
            let value = values[0].clone();
            if value.denom() != &BigInt::one() {
                return Err(format!(
                    "binomial-basis coefficient is not integral: {}/{}",
                    value.numer(),
                    value.denom()
                ));
            }
            coeffs.push(value.to_integer());
            values = values
                .windows(2)
                .map(|pair| pair[1].clone() - pair[0].clone())
                .collect();
        }
        Ok(BinomialBasisPolynomial::new(coeffs))
    }

    /// Compute `P(n + 1) - P(n)` exactly.
    pub fn finite_difference(&self) -> ExactResult<Self> {
        if self.degree() == 0 {
            return Self::new(0, vec![BigRational::zero()]);
        }
        let mut result = vec![BigRational::zero(); self.degree()];
        for (degree, coefficient) in self.power_coeffs.iter().enumerate().skip(1) {
            for lower_degree in 0..degree {
                let factor = BigRational::from(binomial_bigint(degree, lower_degree));
                result[lower_degree] += coefficient.clone() * factor;
            }
        }
        Self::new(self.degree().saturating_sub(1), result)
    }

    /// Compute `sum_{r=0}^{n-1} P(r)` using exact binomial coordinates.
    pub fn discrete_sum(&self) -> ExactResult<Self> {
        let mut binomial = self.to_binomial_basis()?.coeffs;
        binomial.insert(0, BigInt::zero());
        BinomialBasisPolynomial::new(binomial).to_polynomial()
    }

    /// Compute `P(scale * n + shift)` exactly.
    pub fn dilate_and_shift(&self, scale: i64, shift: i64) -> ExactResult<Self> {
        let mut result = vec![BigRational::zero(); self.degree() + 1];
        for (degree, coefficient) in self.power_coeffs.iter().enumerate() {
            for lower_degree in 0..=degree {
                let factor = binomial_bigint(degree, lower_degree)
                    * BigInt::from(scale).pow(lower_degree as u32)
                    * BigInt::from(shift).pow((degree - lower_degree) as u32);
                result[lower_degree] += coefficient.clone() * BigRational::from(factor);
            }
        }
        Self::new(self.dimension, result)
    }
}

impl HStarPolynomial {
    /// Construct an h* numerator of the declared dimension.
    pub fn new(dimension: usize, coeffs: Vec<BigInt>) -> ExactResult<Self> {
        if coeffs.len() != dimension + 1 {
            return Err(format!(
                "dimension {dimension} requires {} h* coefficients, got {}",
                dimension + 1,
                coeffs.len()
            ));
        }
        Ok(Self { dimension, coeffs })
    }

    /// The dimension of the denominator `(1 - t)^(dimension + 1)`.
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Coefficients in ascending numerator degree order.
    pub fn coeffs(&self) -> &[BigInt] {
        &self.coeffs
    }

    /// Largest index with nonzero coefficient, or zero for the zero numerator.
    pub fn degree(&self) -> usize {
        self.coeffs.iter().rposition(|value| !value.is_zero()).unwrap_or(0)
    }

    /// Convert from the shifted Ehrhart binomial basis to power coefficients.
    pub fn to_ehrhart(&self) -> ExactResult<EhrhartPolynomial> {
        let mut result = vec![BigRational::zero(); self.dimension + 1];
        let factorial = factorial_bigint(self.dimension);
        for (index, hstar) in self.coeffs.iter().enumerate() {
            if hstar.is_zero() {
                continue;
            }
            let shift = (self.dimension - index) as i64;
            let mut basis = vec![BigRational::one()];
            for offset in 0..self.dimension {
                basis = multiply_by_linear(&basis, BigInt::from(shift - offset as i64));
            }
            let scale = BigRational::from(hstar.clone()) / BigRational::from(factorial.clone());
            for (degree, coefficient) in basis.into_iter().enumerate() {
                result[degree] += scale.clone() * coefficient;
            }
        }
        EhrhartPolynomial::new(self.dimension, result)
    }
}

impl BinomialBasisPolynomial {
    /// Construct and trim standard binomial coefficients.
    pub fn new(coeffs: Vec<BigInt>) -> Self {
        let mut coeffs = coeffs;
        while coeffs.len() > 1 && coeffs.last().is_some_and(Zero::is_zero) {
            coeffs.pop();
        }
        if coeffs.is_empty() {
            coeffs.push(BigInt::zero());
        }
        Self { coeffs }
    }

    /// Coefficients of `binom(n, 0), binom(n, 1), ...`.
    pub fn coeffs(&self) -> &[BigInt] {
        &self.coeffs
    }

    /// Convert exactly to the power basis.
    pub fn to_polynomial(&self) -> ExactResult<EhrhartPolynomial> {
        let degree = self.coeffs.len().saturating_sub(1);
        let mut result = vec![BigRational::zero(); degree + 1];
        for (index, coefficient) in self.coeffs.iter().enumerate() {
            if coefficient.is_zero() {
                continue;
            }
            let mut basis = vec![BigRational::one()];
            for offset in 0..index {
                basis = multiply_by_linear(&basis, BigInt::from(-(offset as i64)));
            }
            let scale = BigRational::from(coefficient.clone())
                / BigRational::from(factorial_bigint(index));
            for (power, basis_coefficient) in basis.into_iter().enumerate() {
                result[power] += scale.clone() * basis_coefficient;
            }
        }
        EhrhartPolynomial::new(degree, result)
    }
}

impl EhrhartData {
    /// Build a complete result and verify integral h* coefficients.
    pub fn new(ehrhart: EhrhartPolynomial) -> ExactResult<Self> {
        let hstar = ehrhart.to_hstar()?;
        Ok(Self { ehrhart, hstar })
    }
}

/// Parse an exact integer or rational token such as `-7` or `13/24`.
pub fn parse_rational(text: &str) -> ExactResult<BigRational> {
    let text = text.trim();
    if let Some((numerator, denominator)) = text.split_once('/') {
        let numerator = parse_bigint(numerator)?;
        let denominator = parse_bigint(denominator)?;
        if denominator.is_zero() {
            return Err("rational denominator must be nonzero".to_string());
        }
        Ok(BigRational::new(numerator, denominator))
    } else {
        Ok(BigRational::from(parse_bigint(text)?))
    }
}

/// Parse comma-separated rational coefficients in ascending degree order.
pub fn parse_rational_list(text: &str) -> ExactResult<Vec<BigRational>> {
    if text.trim().is_empty() {
        return Err("coefficient list must not be empty".to_string());
    }
    text.split(',').map(parse_rational).collect()
}

/// Parse comma-separated arbitrary-size integers.
pub fn parse_bigint_list(text: &str) -> ExactResult<Vec<BigInt>> {
    if text.trim().is_empty() {
        return Err("integer list must not be empty".to_string());
    }
    text.split(',').map(parse_bigint).collect()
}

/// Format a rational in a compact machine-independent form.
pub fn format_rational(value: &BigRational) -> String {
    if value.denom() == &BigInt::one() {
        value.numer().to_string()
    } else {
        format!("{}/{}", value.numer(), value.denom())
    }
}

fn parse_bigint(text: &str) -> ExactResult<BigInt> {
    text.trim()
        .parse::<BigInt>()
        .map_err(|_| format!("invalid integer `{}`", text.trim()))
}

fn normalize_rational_coeffs(mut coeffs: Vec<BigRational>) -> Vec<BigRational> {
    while coeffs.len() > 1 && coeffs.last().is_some_and(Zero::is_zero) {
        coeffs.pop();
    }
    if coeffs.is_empty() {
        coeffs.push(BigRational::zero());
    }
    coeffs
}

fn multiply_by_linear(coeffs: &[BigRational], constant: BigInt) -> Vec<BigRational> {
    let mut result = vec![BigRational::zero(); coeffs.len() + 1];
    let constant = BigRational::from(constant);
    for (degree, coefficient) in coeffs.iter().enumerate() {
        result[degree] += coefficient.clone() * constant.clone();
        result[degree + 1] += coefficient.clone();
    }
    result
}

fn binomial_bigint(n: usize, k: usize) -> BigInt {
    if k > n {
        return BigInt::zero();
    }
    let k = k.min(n - k);
    let mut value = BigInt::one();
    for index in 0..k {
        value = value * BigInt::from(n - index) / BigInt::from(index + 1);
    }
    value
}

fn factorial_bigint(n: usize) -> BigInt {
    (1..=n).fold(BigInt::one(), |product, value| product * BigInt::from(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn integer(value: i64) -> BigRational {
        BigRational::from(BigInt::from(value))
    }

    #[test]
    fn interpolates_exactly_at_nonconsecutive_points() {
        let polynomial = EhrhartPolynomial::interpolate(
            2,
            &[(2, integer(9)), (3, integer(16)), (5, integer(36))],
        )
        .expect("quadratic interpolation should succeed");
        assert_eq!(polynomial.power_coeffs(), &[integer(1), integer(2), integer(1)]);
        assert_eq!(polynomial.evaluate(-2), integer(1));
    }

    #[test]
    fn rejects_duplicate_interpolation_points() {
        let error = EhrhartPolynomial::interpolate(1, &[(0, integer(1)), (0, integer(1))])
            .expect_err("duplicate points must be rejected");
        assert!(error.contains("more than once"));
    }

    #[test]
    fn hstar_preserves_trailing_zero_for_simplex() {
        let polynomial = EhrhartPolynomial::new(
            2,
            vec![integer(1), BigRational::new(BigInt::from(3), BigInt::from(2)), BigRational::new(BigInt::from(1), BigInt::from(2))],
        )
        .expect("valid two-dimensional Ehrhart polynomial");
        let hstar = polynomial.to_hstar().expect("h* should be integral");
        assert_eq!(hstar.coeffs(), &[BigInt::one(), BigInt::zero(), BigInt::zero()]);
        assert_eq!(hstar.to_ehrhart().expect("round trip").power_coeffs(), polynomial.power_coeffs());
    }

    #[test]
    fn binomial_round_trip_and_discrete_sum_are_exact() {
        let polynomial = EhrhartPolynomial::new(2, vec![integer(1), integer(3), integer(2)])
            .expect("valid polynomial");
        let binomial = polynomial.to_binomial_basis().expect("integer-valued polynomial");
        assert_eq!(binomial.coeffs(), &[BigInt::one(), BigInt::from(5), BigInt::from(4)]);
        assert_eq!(binomial.to_polynomial().expect("round trip"), polynomial);

        let sum = polynomial.discrete_sum().expect("sum should be integer-valued");
        assert_eq!(sum.evaluate(3), integer(22));
        assert_eq!(sum.evaluate(0), integer(0));
    }

    #[test]
    fn finite_difference_and_affine_substitution_are_exact() {
        let polynomial = EhrhartPolynomial::new(2, vec![integer(1), integer(0), integer(1)])
            .expect("valid polynomial");
        assert_eq!(
            polynomial.finite_difference().expect("difference").power_coeffs(),
            &[integer(1), integer(2)]
        );
        assert_eq!(
            polynomial
                .dilate_and_shift(2, 1)
                .expect("substitution")
                .power_coeffs(),
            &[integer(2), integer(4), integer(4)]
        );
    }

    #[test]
    fn parsing_uses_exact_rationals() {
        assert_eq!(parse_rational(" -7/ 9 ").expect("parse"), BigRational::new(BigInt::from(-7), BigInt::from(9)));
        assert!(parse_rational("1/0").is_err());
        assert_eq!(parse_bigint_list("1, 2, -9").expect("parse"), vec![BigInt::one(), BigInt::from(2), BigInt::from(-9)]);
    }
}
