use crate::gt_dim::gt_polytope_dim_full;
use crate::kostka_dp::{
    flagged_skew_kostka_legacy, skew_kostka_legacy, strict_skew_kostka, strict_skew_kostka_legacy,
    try_flagged_skew_kostka, try_skew_kostka, try_strict_flagged_skew_kostka,
    try_strict_skew_kostka,
};
use crate::Partition;
/// Ehrhart polynomial computation for GT(lambda/mu, w).
///
/// By Rassart (2004), the function n ↦ K(n*lambda / n*mu, n*w) is a polynomial in n.
/// We:
///   1. Compute the degree d via gt_dim::gt_polytope_dim_full, including flags.
///   2. Collect d+1 sample points using Ehrhart-Macdonald reciprocity with an
///      adaptive strategy, including for flagged fixed-content polytopes.
///   3. Solve the resulting system over Q by Gaussian elimination.
///
/// The polynomial is stored as a Vec<BigRational> of length d+1,
/// where poly[k] is the coefficient of n^k (index 0 = constant term).
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_rational::BigRational;
use num_traits::{One, Zero};
use rayon::prelude::*;

pub struct EhrhartPoly {
    /// Coefficients of the polynomial in n: poly[k] = coeff of n^k.
    pub coeffs: Vec<BigRational>,
    /// Degree (index of highest non-zero coefficient).
    pub degree: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct FrozenRectangleReduction {
    lambda: Partition,
    mu: Partition,
    weight: Vec<u32>,
    upper_flags: Option<Vec<u32>>,
    lower_flags: Option<Vec<u32>>,
    cut_row: usize,
    cut_label: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct SaturatedSplitFactor {
    lambda: Partition,
    mu: Partition,
    weight: Vec<u32>,
    upper_flags: Option<Vec<u32>>,
    lower_flags: Option<Vec<u32>>,
}

#[derive(Debug, PartialEq, Eq)]
struct SaturatedRowLabelSplit {
    top: SaturatedSplitFactor,
    bottom: SaturatedSplitFactor,
    cut_row: usize,
    cut_label: usize,
}

fn optional_flags(
    upper: Vec<u32>,
    lower: Vec<u32>,
    rows: usize,
) -> Option<(Option<Vec<u32>>, Option<Vec<u32>>)> {
    if lower.iter().zip(&upper).any(|(lo, hi)| lo > hi) {
        return None;
    }
    let trivial =
        lower.iter().all(|flag| *flag == 1) && upper.iter().all(|flag| *flag == rows as u32);
    Some(((!trivial).then_some(upper), (!trivial).then_some(lower)))
}

/// Split a flagged tableau at a saturated row/label cut.
///
/// Later labels are barred from the top rows and the earlier labels have total
/// weight exactly equal to the top skew area.  Hence the earlier labels fill
/// the top block and the later labels fill the bottom block at every dilation.
/// Since every top label precedes every bottom label, all column inequalities
/// crossing the cut are automatic.  The lattice points therefore form the
/// Cartesian product of the two returned flagged tableau polytopes.
fn saturated_row_label_split(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
) -> Option<SaturatedRowLabelSplit> {
    let rows = lambda.num_parts();
    if rows < 2 || weight.len() < 2 || weight.contains(&0) {
        return None;
    }
    let upper = |label: usize| {
        upper_flags
            .and_then(|flags| flags.get(label))
            .copied()
            .unwrap_or(rows as u32)
            .min(rows as u32)
    };
    let lower = |label: usize| {
        lower_flags
            .and_then(|flags| flags.get(label))
            .copied()
            .unwrap_or(1)
            .max(1)
    };

    for cut_row in 1..rows {
        let top_area = (0..cut_row)
            .map(|row| lambda.part(row).saturating_sub(mu.part(row)))
            .sum::<u32>();
        let mut prefix_weight = 0u32;
        let Some(cut_label) = weight
            .iter()
            .position(|entry| {
                prefix_weight = prefix_weight.saturating_add(*entry);
                prefix_weight == top_area
            })
            .map(|index| index + 1)
        else {
            continue;
        };
        if cut_label == weight.len()
            || weight[cut_label..]
                .iter()
                .enumerate()
                .any(|(offset, _)| lower(cut_label + offset) <= cut_row as u32)
        {
            continue;
        }

        let top_upper = (0..cut_label)
            .map(|label| upper(label).min(cut_row as u32))
            .collect::<Vec<_>>();
        let top_lower = (0..cut_label).map(lower).collect::<Vec<_>>();
        let Some((top_upper, top_lower)) = optional_flags(top_upper, top_lower, cut_row) else {
            continue;
        };

        let bottom_rows = rows - cut_row;
        let bottom_upper = (cut_label..weight.len())
            .map(|label| upper(label).saturating_sub(cut_row as u32))
            .collect::<Vec<_>>();
        let bottom_lower = (cut_label..weight.len())
            .map(|label| lower(label).saturating_sub(cut_row as u32).max(1))
            .collect::<Vec<_>>();
        let Some((bottom_upper, bottom_lower)) =
            optional_flags(bottom_upper, bottom_lower, bottom_rows)
        else {
            continue;
        };

        return Some(SaturatedRowLabelSplit {
            top: SaturatedSplitFactor {
                lambda: Partition::new(lambda.parts()[..cut_row].to_vec()),
                mu: Partition::new((0..cut_row).map(|row| mu.part(row)).collect()),
                weight: weight[..cut_label].to_vec(),
                upper_flags: top_upper,
                lower_flags: top_lower,
            },
            bottom: SaturatedSplitFactor {
                lambda: Partition::new((cut_row..rows).map(|row| lambda.part(row)).collect()),
                mu: Partition::new((cut_row..rows).map(|row| mu.part(row)).collect()),
                weight: weight[cut_label..].to_vec(),
                upper_flags: bottom_upper,
                lower_flags: bottom_lower,
            },
            cut_row,
            cut_label,
        });
    }
    None
}

fn multiply_ehrhart(left: EhrhartPoly, right: EhrhartPoly) -> EhrhartPoly {
    let mut coeffs = vec![BigRational::zero(); left.coeffs.len() + right.coeffs.len() - 1];
    for (left_degree, left_coefficient) in left.coeffs.iter().enumerate() {
        for (right_degree, right_coefficient) in right.coeffs.iter().enumerate() {
            coeffs[left_degree + right_degree] += left_coefficient * right_coefficient;
        }
    }
    let degree = coeffs
        .iter()
        .enumerate()
        .rev()
        .find(|(_, coefficient)| !coefficient.is_zero())
        .map(|(degree, _)| degree)
        .unwrap_or(0);
    EhrhartPoly { coeffs, degree }
}

/// Delete a forced unit-width rectangular suffix from a flagged tableau.
///
/// If labels after `cut_label` are barred from the first `cut_row` rows and
/// the preceding labels have total weight equal to the top skew area, the
/// tableau splits at that row/label cut at every dilation.  When the remaining
/// rows all have length one, the remaining labels all have weight one, and
/// label `cut_label + j` is allowed in row `cut_row + j`, the bottom block is
/// the unique constant-row rectangle.  Removing it is therefore a
/// dilation-compatible lattice bijection and preserves the Ehrhart polynomial.
fn frozen_unit_rectangle_reduction(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
) -> Option<FrozenRectangleReduction> {
    let rows = lambda.num_parts();
    if rows < 2 || weight.is_empty() || weight.contains(&0) {
        return None;
    }

    // Prefer the earliest valid cut, which removes the largest forced suffix.
    for cut_row in 1..rows {
        if (cut_row..rows).any(|row| lambda.part(row) != 1 || mu.part(row) != 0) {
            continue;
        }
        let top_area = (0..cut_row)
            .map(|row| lambda.part(row).saturating_sub(mu.part(row)))
            .sum::<u32>();
        let mut prefix_weight = 0u32;
        let Some(cut_label) = weight
            .iter()
            .position(|entry| {
                prefix_weight = prefix_weight.saturating_add(*entry);
                prefix_weight == top_area
            })
            .map(|index| index + 1)
        else {
            continue;
        };
        if weight[..cut_label].iter().sum::<u32>() != top_area {
            continue;
        }

        let tail_rows = rows - cut_row;
        if weight.len() - cut_label != tail_rows
            || weight[cut_label..].iter().any(|entry| *entry != 1)
        {
            continue;
        }

        let upper = |label: usize| {
            upper_flags
                .and_then(|flags| flags.get(label))
                .copied()
                .unwrap_or(rows as u32)
                .min(rows as u32)
        };
        let lower = |label: usize| {
            lower_flags
                .and_then(|flags| flags.get(label))
                .copied()
                .unwrap_or(1)
                .max(1)
        };

        let mut suffix_is_forced = true;
        for offset in 0..tail_rows {
            let label = cut_label + offset;
            let assigned_row = (cut_row + offset + 1) as u32;
            if lower(label) <= cut_row as u32
                || lower(label) > assigned_row
                || upper(label) < assigned_row
            {
                suffix_is_forced = false;
                break;
            }
        }
        if !suffix_is_forced {
            continue;
        }

        let reduced_upper: Vec<u32> = (0..cut_label)
            .map(|label| upper(label).min(cut_row as u32))
            .collect();
        let reduced_lower: Vec<u32> = (0..cut_label).map(lower).collect();
        if reduced_lower
            .iter()
            .zip(&reduced_upper)
            .any(|(lo, hi)| lo > hi)
        {
            continue;
        }
        let flags_are_trivial = reduced_lower.iter().all(|flag| *flag == 1)
            && reduced_upper.iter().all(|flag| *flag == cut_row as u32);

        return Some(FrozenRectangleReduction {
            lambda: Partition::new(lambda.parts()[..cut_row].to_vec()),
            mu: Partition::new((0..cut_row).map(|row| mu.part(row)).collect::<Vec<_>>()),
            weight: weight[..cut_label].to_vec(),
            upper_flags: (!flags_are_trivial).then_some(reduced_upper),
            lower_flags: (!flags_are_trivial).then_some(reduced_lower),
            cut_row,
            cut_label,
        });
    }
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EhrhartInterpolation {
    AdaptiveReciprocity,
    PositiveOnly,
    Gorenstein,
}

impl EhrhartPoly {
    /// Evaluate the polynomial at a given n.
    pub fn eval(&self, n: u64) -> BigRational {
        let n_r = BigRational::from(BigInt::from(n));
        let mut result = BigRational::zero();
        let mut power = BigRational::one();
        for c in &self.coeffs {
            result += c * &power;
            power *= &n_r;
        }
        result
    }

    /// True if any coefficient of the polynomial is negative.
    pub fn has_negative_coefficient(&self) -> bool {
        self.coeffs.iter().any(|c| *c < BigRational::zero())
    }

    /// Display as a human-readable string, e.g. "(1/3)n^3 + n^2 + (5/3)n + 1".
    pub fn display(&self) -> String {
        let d = self.degree;
        if d == 0 {
            return format_rat(&self.coeffs[0]);
        }
        let mut terms: Vec<String> = Vec::new();
        for k in (0..=d).rev() {
            let c = &self.coeffs[k];
            if c.is_zero() {
                continue;
            }
            let c_str = format_rat(c);
            let term = match k {
                0 => c_str,
                1 => {
                    if c == &BigRational::one() {
                        "n".into()
                    } else {
                        format!("{}n", c_str)
                    }
                }
                _ => {
                    if c == &BigRational::one() {
                        format!("n^{}", k)
                    } else {
                        format!("{}n^{}", c_str, k)
                    }
                }
            };
            terms.push(term);
        }
        if terms.is_empty() {
            "0".into()
        } else {
            terms.join(" + ")
        }
    }

    /// Display as (1/d!) * (integer polynomial), where d = degree.
    /// Multiplying each coefficient by d! always yields integers for GT Ehrhart polynomials.
    pub fn display_factored(&self) -> String {
        let d = self.degree;
        if self.coeffs.iter().all(|c| c.is_zero()) {
            return "0".into();
        }

        // Compute d!
        let d_fact: BigInt = (1..=d as u64).fold(BigInt::one(), |acc, i| acc * BigInt::from(i));
        let d_fact_r = BigRational::from(d_fact.clone());

        // Integer coefficients: c_k * d!
        let int_coeffs: Vec<BigInt> = self.coeffs[..=d]
            .iter()
            .map(|c| (c * &d_fact_r).to_integer())
            .collect();

        let poly_str = format_int_poly(&int_coeffs);

        if d <= 1 {
            poly_str
        } else {
            format!("(1/{}!) * ({})", d, poly_str)
        }
    }
}

fn format_rat(r: &BigRational) -> String {
    if r.denom() == &BigInt::one() {
        r.numer().to_string()
    } else {
        format!("({}/{})", r.numer(), r.denom())
    }
}

/// Format a polynomial with integer coefficients as "a_d n^d + ... + a_1 n + a_0".
fn format_int_poly(coeffs: &[BigInt]) -> String {
    let d = coeffs.len().saturating_sub(1);
    let mut terms: Vec<String> = Vec::new();
    for k in (0..=d).rev() {
        let c = &coeffs[k];
        if c.is_zero() {
            continue;
        }
        let c_abs = c.magnitude().clone();
        let sign: String = if terms.is_empty() {
            if *c < BigInt::zero() {
                "-".into()
            } else {
                "".into()
            }
        } else {
            if *c < BigInt::zero() {
                " - ".into()
            } else {
                " + ".into()
            }
        };
        let mag_str = if c_abs == num_bigint::BigUint::from(1u32) && k > 0 {
            "".into()
        } else {
            c_abs.to_string()
        };
        let var_str = match k {
            0 => "".into(),
            1 => "n".into(),
            _ => format!("n^{}", k),
        };
        terms.push(format!("{}{}{}", sign, mag_str, var_str));
    }
    if terms.is_empty() {
        "0".into()
    } else {
        terms.join("")
    }
}

/// Compute the Ehrhart polynomial of GT(lambda/mu, w).
///
/// When `use_reciprocity` is true (the default), the computation uses
/// Ehrhart-Macdonald reciprocity with a sequential adaptive strategy:
///   - P(0) = 1 is free.
///   - At each step, choose the side (positive or negative t) whose last
///     raw DP count was smaller, evaluating one point at a time until
///     d+1 points are collected.  Negative side wins ties.
///   - P(-t) = (-1)^d * K_strict(t·λ, t·μ, t·w)
///   - Strict Kostka is 0 for small t when w has many 1s, making those
///     evaluations free and heavily favouring the negative side early on.
///
/// Reciprocity requires no row flags; with flags the method silently
/// falls back to the plain positive-dilation scheme.
#[allow(clippy::too_many_arguments)]
pub fn compute_ehrhart(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    verbose: bool,
    max_states: Option<usize>,
    use_reciprocity: bool,
) -> EhrhartPoly {
    try_compute_ehrhart(
        lambda,
        mu,
        w,
        upper_flags,
        lower_flags,
        verbose,
        max_states,
        use_reciprocity,
    )
    .expect("GT Ehrhart DP state limit exceeded")
}

/// Fallible variant of [`compute_ehrhart`] that reports a state-limit breach.
#[allow(clippy::too_many_arguments)]
pub fn try_compute_ehrhart(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    verbose: bool,
    max_states: Option<usize>,
    use_reciprocity: bool,
) -> Result<EhrhartPoly, String> {
    let mode = if use_reciprocity {
        EhrhartInterpolation::AdaptiveReciprocity
    } else {
        EhrhartInterpolation::PositiveOnly
    };
    try_compute_ehrhart_with_mode(
        lambda,
        mu,
        w,
        upper_flags,
        lower_flags,
        verbose,
        max_states,
        mode,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn compute_ehrhart_with_mode(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    verbose: bool,
    max_states: Option<usize>,
    mode: EhrhartInterpolation,
) -> EhrhartPoly {
    try_compute_ehrhart_with_mode(
        lambda,
        mu,
        w,
        upper_flags,
        lower_flags,
        verbose,
        max_states,
        mode,
    )
    .expect("GT Ehrhart DP state limit exceeded")
}

/// Fallible variant of [`compute_ehrhart_with_mode`] that reports a state-limit breach.
#[allow(clippy::too_many_arguments)]
pub fn try_compute_ehrhart_with_mode(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    verbose: bool,
    max_states: Option<usize>,
    mode: EhrhartInterpolation,
) -> Result<EhrhartPoly, String> {
    compute_ehrhart_impl(
        lambda,
        mu,
        w,
        upper_flags,
        lower_flags,
        verbose,
        max_states,
        mode,
        false,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn compute_ehrhart_legacy(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    verbose: bool,
    max_states: Option<usize>,
    use_reciprocity: bool,
) -> EhrhartPoly {
    let mode = if use_reciprocity {
        EhrhartInterpolation::AdaptiveReciprocity
    } else {
        EhrhartInterpolation::PositiveOnly
    };
    compute_ehrhart_legacy_with_mode(
        lambda,
        mu,
        w,
        upper_flags,
        lower_flags,
        verbose,
        max_states,
        mode,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn compute_ehrhart_legacy_with_mode(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    verbose: bool,
    max_states: Option<usize>,
    mode: EhrhartInterpolation,
) -> EhrhartPoly {
    compute_ehrhart_impl(
        lambda,
        mu,
        w,
        upper_flags,
        lower_flags,
        verbose,
        max_states,
        mode,
        true,
    )
    .expect("legacy GT Ehrhart DP state limit exceeded")
}

#[allow(clippy::too_many_arguments)]
fn compute_ehrhart_impl(
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    verbose: bool,
    max_states: Option<usize>,
    mode: EhrhartInterpolation,
    use_legacy_dp: bool,
) -> Result<EhrhartPoly, String> {
    // Early exit: sizes must be compatible for a non-empty polytope.
    let skew_size = lambda.size().saturating_sub(mu.size());
    let w_size: u32 = w.iter().sum();
    if skew_size != w_size {
        return Ok(EhrhartPoly {
            coeffs: vec![BigRational::zero()],
            degree: 0,
        });
    }

    // Only reorder w when no flag bounds are active (flags are tied to w's row ordering).
    // Weight reordering and Gorenstein mode require no flags.  The exact
    // strict DP now supports flags, so adaptive reciprocity does not require
    // their absence.
    let sort_weight = upper_flags.is_none() && lower_flags.is_none();
    let mode = match mode {
        EhrhartInterpolation::AdaptiveReciprocity => EhrhartInterpolation::AdaptiveReciprocity,
        EhrhartInterpolation::Gorenstein if sort_weight => EhrhartInterpolation::Gorenstein,
        EhrhartInterpolation::Gorenstein => {
            if verbose {
                eprintln!(
                    "warning: Gorenstein interpolation requires no row flags; falling back to plain positive interpolation"
                );
            }
            EhrhartInterpolation::PositiveOnly
        }
        EhrhartInterpolation::PositiveOnly => EhrhartInterpolation::PositiveOnly,
    };

    let d = match gt_polytope_dim_full(lambda.parts(), mu.parts(), w, upper_flags, lower_flags) {
        None => {
            return Ok(EhrhartPoly {
                coeffs: vec![BigRational::zero()],
                degree: 0,
            });
        }
        Some(d) => d,
    };

    // The legacy path intentionally retains the unreduced presentation so it
    // remains an independent comparison implementation for regression tests.
    if !use_legacy_dp {
        if let Some(reduced) =
            frozen_unit_rectangle_reduction(lambda, mu, w, upper_flags, lower_flags)
        {
            let reduced_dimension = gt_polytope_dim_full(
                reduced.lambda.parts(),
                reduced.mu.parts(),
                &reduced.weight,
                reduced.upper_flags.as_deref(),
                reduced.lower_flags.as_deref(),
            );
            if reduced_dimension == Some(d) {
                if verbose {
                    eprintln!(
                        "deleted forced rectangular suffix after row {} and label {}",
                        reduced.cut_row, reduced.cut_label
                    );
                }
                return compute_ehrhart_impl(
                    &reduced.lambda,
                    &reduced.mu,
                    &reduced.weight,
                    reduced.upper_flags.as_deref(),
                    reduced.lower_flags.as_deref(),
                    verbose,
                    max_states,
                    mode,
                    false,
                );
            } else if verbose {
                eprintln!(
                    "warning: skipped forced rectangular suffix reduction because dimensions differ: original {}, reduced {:?}",
                    d, reduced_dimension
                );
            }
        }

        if let Some(split) = saturated_row_label_split(lambda, mu, w, upper_flags, lower_flags) {
            let top_dimension = gt_polytope_dim_full(
                split.top.lambda.parts(),
                split.top.mu.parts(),
                &split.top.weight,
                split.top.upper_flags.as_deref(),
                split.top.lower_flags.as_deref(),
            );
            let bottom_dimension = gt_polytope_dim_full(
                split.bottom.lambda.parts(),
                split.bottom.mu.parts(),
                &split.bottom.weight,
                split.bottom.upper_flags.as_deref(),
                split.bottom.lower_flags.as_deref(),
            );
            if top_dimension
                .zip(bottom_dimension)
                .map(|(top, bottom)| top + bottom)
                == Some(d)
            {
                if verbose {
                    eprintln!(
                        "split saturated tableau after row {} and label {}",
                        split.cut_row, split.cut_label
                    );
                }
                let factor_mode = match mode {
                    EhrhartInterpolation::Gorenstein => EhrhartInterpolation::AdaptiveReciprocity,
                    other => other,
                };
                let top = compute_ehrhart_impl(
                    &split.top.lambda,
                    &split.top.mu,
                    &split.top.weight,
                    split.top.upper_flags.as_deref(),
                    split.top.lower_flags.as_deref(),
                    verbose,
                    max_states,
                    factor_mode,
                    false,
                )?;
                let bottom = compute_ehrhart_impl(
                    &split.bottom.lambda,
                    &split.bottom.mu,
                    &split.bottom.weight,
                    split.bottom.upper_flags.as_deref(),
                    split.bottom.lower_flags.as_deref(),
                    verbose,
                    max_states,
                    factor_mode,
                    false,
                )?;
                return Ok(multiply_ehrhart(top, bottom));
            } else if verbose {
                eprintln!(
                    "warning: skipped saturated tableau split because dimensions do not add: original {}, top {:?}, bottom {:?}",
                    d, top_dimension, bottom_dimension
                );
            }
        }
    }

    let eval_positive = |t: u64| -> Result<BigUint, String> {
        let tl = scale_partition(lambda, t);
        let tm_p = scale_partition(mu, t);
        let tw: Vec<u32> = w.iter().map(|&x| x * t as u32).collect();
        if use_legacy_dp {
            Ok(if upper_flags.is_some() || lower_flags.is_some() {
                flagged_skew_kostka_legacy(&tl, &tm_p, &tw, upper_flags, lower_flags, max_states)
            } else {
                skew_kostka_legacy(&tl, &tm_p, &tw, max_states, sort_weight)
            })
        } else if upper_flags.is_some() || lower_flags.is_some() {
            try_flagged_skew_kostka(&tl, &tm_p, &tw, upper_flags, lower_flags, max_states)
        } else {
            try_skew_kostka(&tl, &tm_p, &tw, max_states, sort_weight)
        }
    };

    let eval_strict = |t: u64| -> Result<BigUint, String> {
        let tl = scale_partition(lambda, t);
        let tm_p = scale_partition(mu, t);
        let tw: Vec<u32> = w.iter().map(|&x| x * t as u32).collect();
        if upper_flags.is_some() || lower_flags.is_some() {
            try_strict_flagged_skew_kostka(&tl, &tm_p, &tw, upper_flags, lower_flags, max_states)
        } else if use_legacy_dp {
            Ok(strict_skew_kostka_legacy(
                &tl, &tm_p, &tw, max_states, false,
            ))
        } else {
            try_strict_skew_kostka(&tl, &tm_p, &tw, max_states, false)
        }
    };

    if mode == EhrhartInterpolation::AdaptiveReciprocity {
        // P(0) = 1 always (the trivial chain μ=μ is the unique point at dilation 0).
        if d == 0 {
            // Constant polynomial.  No DP evaluation needed.
            return Ok(EhrhartPoly {
                coeffs: vec![BigRational::one()],
                degree: 0,
            });
        }

        // Sequential adaptive reciprocity strategy:
        //
        // P(0) = 1 is free.  Then greedily evaluate one point at a time, choosing
        // the side (positive t or negative t) whose previous raw DP count was smaller.
        // Negative side wins ties and goes first (strict Kostka ≤ ordinary Kostka,
        // and equals 0 for small t when w has many 1s — those zeros are free).
        //
        // P(-t) = (-1)^d * K_strict(t·λ, t·μ, t·w)  (Ehrhart-Macdonald reciprocity)
        let sign_pos = d % 2 == 0; // (-1)^d is +1 iff d is even

        let mut points: Vec<(i64, BigRational)> = Vec::with_capacity(d + 1);
        points.push((0, BigRational::one())); // P(0) = 1 is free

        let mut pos_t: u64 = 0;
        let mut neg_t: u64 = 0;
        let mut last_pos_count = BigUint::one();
        let mut last_neg_count = BigUint::one(); // equal → negative wins the first tie

        while points.len() <= d {
            if last_neg_count <= last_pos_count {
                // Evaluate the next negative point via strict Kostka
                neg_t += 1;
                let ks = eval_strict(neg_t)?;
                let ks_r = BigRational::from(ks.to_bigint().unwrap());
                let p_val = if sign_pos { ks_r } else { -ks_r };
                points.push((-(neg_t as i64), p_val));
                last_neg_count = ks;
            } else {
                // Evaluate the next positive point via ordinary Kostka
                pos_t += 1;
                let k = eval_positive(pos_t)?;
                let k_r = BigRational::from(k.to_bigint().unwrap());
                points.push((pos_t as i64, k_r));
                last_pos_count = k;
            }
        }

        let coeffs = poly_interpolate(&points);
        let true_degree = coeffs
            .iter()
            .enumerate()
            .rev()
            .find(|(_, c)| !c.is_zero())
            .map(|(i, _)| i)
            .unwrap_or(0);
        Ok(EhrhartPoly {
            coeffs,
            degree: true_degree,
        })
    } else if mode == EhrhartInterpolation::Gorenstein {
        let sign_pos = d % 2 == 0; // (-1)^d is +1 iff d is even

        let mut points: Vec<(i64, BigRational)> = Vec::with_capacity(d + 1);
        points.push((0, BigRational::one())); // P(0) = 1 is free

        let mut q: Option<u64> = None;
        let mut first_nonzero: Option<BigUint> = None;

        // Probe the negative side to detect the codegree q.
        // We keep the free zeros at -1, ..., -(q-1) as interpolation points.
        for neg_t in 1..=((d + 1) as u64) {
            let ks = eval_strict(neg_t)?;
            if ks.is_zero() {
                points.push((-(neg_t as i64), BigRational::zero()));
                if points.len() == d + 1 {
                    let coeffs = poly_interpolate(&points);
                    let true_degree = coeffs
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, c)| !c.is_zero())
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    return Ok(EhrhartPoly {
                        coeffs,
                        degree: true_degree,
                    });
                }
            } else {
                q = Some(neg_t);
                first_nonzero = Some(ks);
                break;
            }
        }

        let q = q.expect("gorenstein interpolation: codegree search exceeded d+1");
        let first_nonzero =
            first_nonzero.expect("gorenstein interpolation: missing first nonzero strict count");

        // If the codegree dilation does not have a unique interior point, the
        // explicit Gorenstein assumption has already failed. Fall back to the
        // exact adaptive reciprocity strategy rather than returning a bad polynomial.
        if first_nonzero != BigUint::one() {
            if verbose {
                eprintln!(
                    "warning: first nonzero strict count at codegree q={} is {}, not 1; falling back to adaptive reciprocity",
                    q, first_nonzero
                );
            }
            return compute_ehrhart_impl(
                lambda,
                mu,
                w,
                upper_flags,
                lower_flags,
                verbose,
                max_states,
                EhrhartInterpolation::AdaptiveReciprocity,
                use_legacy_dp,
            );
        }

        // Use P(-q) = (-1)^d P(0), then mirror larger negative points from small positives.
        points.push((
            -(q as i64),
            if sign_pos {
                BigRational::one()
            } else {
                -BigRational::one()
            },
        ));

        let remaining = d + 1 - points.len();
        let pair_count = remaining / 2;
        let has_extra_positive = remaining % 2 == 1;

        for t in 1..=pair_count {
            let t_u = t as u64;
            let k = eval_positive(t_u)?;
            let k_r = BigRational::from(k.to_bigint().unwrap());
            points.push((t as i64, k_r.clone()));
            let mirror = if sign_pos { k_r } else { -k_r };
            points.push((-(q as i64) - t as i64, mirror));
        }

        if has_extra_positive {
            let t = pair_count as u64 + 1;
            let k = eval_positive(t)?;
            let k_r = BigRational::from(k.to_bigint().unwrap());
            points.push((t as i64, k_r));
        }

        debug_assert_eq!(points.len(), d + 1);

        let coeffs = poly_interpolate(&points);
        let true_degree = coeffs
            .iter()
            .enumerate()
            .rev()
            .find(|(_, c)| !c.is_zero())
            .map(|(i, _)| i)
            .unwrap_or(0);
        Ok(EhrhartPoly {
            coeffs,
            degree: true_degree,
        })
    } else {
        // Plain method: evaluate at n = 1, ..., d+1.
        let values: Vec<BigRational> = (1..=(d + 1) as u64)
            .into_par_iter()
            .map(|n| {
                let k = eval_positive(n)?;
                Ok(BigRational::from(k.to_bigint().unwrap()))
            })
            .collect::<Result<Vec<_>, String>>()?;

        let coeffs = vandermonde_solve(&values);
        let true_degree = coeffs
            .iter()
            .enumerate()
            .rev()
            .find(|(_, c)| !c.is_zero())
            .map(|(i, _)| i)
            .unwrap_or(0);
        Ok(EhrhartPoly {
            coeffs,
            degree: true_degree,
        })
    }
}

fn scale_partition(p: &Partition, n: u64) -> Partition {
    Partition::from_sorted(p.parts().iter().map(|&x| x * n as u32).collect())
}

/// Interpolate a polynomial from d+1 arbitrary (x, y) sample points.
/// Returns coefficients [c_0, c_1, ..., c_d] for P(n) = c_0 + c_1·n + ... + c_d·n^d.
/// Uses Gaussian elimination over ℚ (d is small, typically ≤ 13).
fn poly_interpolate(points: &[(i64, BigRational)]) -> Vec<BigRational> {
    let d = points.len(); // d+1 unknowns, so this yields a degree-(d-1) polynomial
                          // Build the augmented Vandermonde matrix: M[i][k] = x_i^k, last column = y_i.
    let mut mat: Vec<Vec<BigRational>> = points
        .iter()
        .map(|&(x, ref y)| {
            let xb = BigInt::from(x);
            let mut row: Vec<BigRational> = Vec::with_capacity(d + 1);
            let mut power = BigInt::one();
            for _ in 0..d {
                row.push(BigRational::from(power.clone()));
                power *= &xb;
            }
            row.push(y.clone());
            row
        })
        .collect();

    // Gauss-Jordan elimination.
    for col in 0..d {
        let pivot_row = (col..d)
            .find(|&r| !mat[r][col].is_zero())
            .expect("poly_interpolate: singular system (duplicate x-values?)");
        mat.swap(col, pivot_row);
        let pivot = mat[col][col].clone();
        for entry in &mut mat[col][col..=d] {
            *entry /= &pivot;
        }
        let pivot_tail = mat[col][col..=d].to_vec();
        for (row_index, row_entries) in mat.iter_mut().enumerate().take(d) {
            if row_index == col {
                continue;
            }
            let factor = row_entries[col].clone();
            if factor.is_zero() {
                continue;
            }
            for (entry, pivot_entry) in row_entries[col..=d].iter_mut().zip(&pivot_tail) {
                *entry -= factor.clone() * pivot_entry;
            }
        }
    }
    mat.iter().map(|row| row[d].clone()).collect()
}

/// Fit P to sample points (1, v_1), (2, v_2), ..., (d+1, v_{d+1}).
fn vandermonde_solve(values: &[BigRational]) -> Vec<BigRational> {
    let points: Vec<(i64, BigRational)> = values
        .iter()
        .enumerate()
        .map(|(i, v)| ((i + 1) as i64, v.clone()))
        .collect();
    poly_interpolate(&points)
}

/// Verify Ehrhart-Macdonald reciprocity for `n_checks` values of t.
///
/// For each t = 1 ..= n_checks, checks:
///   (-1)^d * strict_skew_kostka(t*λ, t*μ, t*w)  ==  P_Ehrhart(-t)
///
/// Prints a summary and returns true iff all checks pass.
pub fn verify_reciprocity(
    poly: &EhrhartPoly,
    lambda: &Partition,
    mu: &Partition,
    w: &[u32],
    n_checks: u64,
    max_states: Option<usize>,
) -> bool {
    let d = poly.degree;
    let sign_pos = d.is_multiple_of(2); // (-1)^d is +1 iff d is even
    let sort_weight = true;

    // For degree-0 polytopes (a single point), the relative interior IS the point:
    // L_{P°}(t) = L_P(t) = constant.  Strict GT patterns always give 0, not 1, so
    // we cannot use the strict DP here — the check is trivially satisfied by P.
    if d == 0 {
        println!("  (degree 0: relative interior = polytope, check is trivial)");
        return true;
    }

    let mut all_ok = true;
    for t in 1..=n_checks {
        let tl = scale_partition(lambda, t);
        let tm = scale_partition(mu, t);
        let tw: Vec<u32> = w.iter().map(|&x| x * t as u32).collect();

        let interior = strict_skew_kostka(&tl, &tm, &tw, max_states, sort_weight);
        let interior_r = BigRational::from(interior.to_bigint().unwrap());

        // (-1)^d * interior
        let lhs = if sign_pos {
            interior_r.clone()
        } else {
            -interior_r.clone()
        };

        // P_Ehrhart(-t)
        let neg_t = BigRational::from(BigInt::from(-(t as i64)));
        let mut rhs = BigRational::zero();
        let mut power = BigRational::one();
        for c in &poly.coeffs {
            rhs += c * &power;
            power *= &neg_t;
        }

        let ok = lhs == rhs;
        if !ok {
            println!(
                "  FAIL t={}: (-1)^{} * L_{{P°}}({}) = {} but P(-{}) = {}",
                t,
                d,
                t,
                if sign_pos {
                    interior_r.clone()
                } else {
                    -interior_r.clone()
                },
                t,
                rhs
            );
            all_ok = false;
        } else {
            println!(
                "  OK   t={}: (-1)^{} * L_{{P°}}({}) = P(-{}) = {}",
                t, d, t, t, lhs
            );
        }
    }
    all_ok
}

/// Compute the h*-vector from the Ehrhart polynomial.
/// The h*-vector satisfies:
///   Σ_{n≥0} P(n) t^n  =  (Σ h*_k t^k) / (1-t)^{d+1}
/// Equivalently, express P(n) in the binomial basis C(n+d,d), C(n+d-1,d), ..., C(n,d):
///   P(n) = Σ_{k=0}^{d} h*_k * C(n+d-k, d)
/// The h*_k are the coefficients in this basis change.
pub fn compute_hstar(poly: &EhrhartPoly) -> Vec<BigInt> {
    let d = poly.degree;
    // Evaluate P(0), P(1), ..., P(d) (P(0) = h*_0 always).
    // Use the forward difference operator: h*_k = Δ^k P(0) / k! * ... (via finite differences).
    // Simpler: build the (d+1)×(d+1) basis-change matrix and solve.
    // For small d, just evaluate and use the known conversion:
    //   h*_k = Σ_{j=0}^{k} (-1)^{k-j} C(d+1, k-j) P(j)
    // This is the standard Ehrhart h*-vector formula.

    let d1 = d + 1;
    let mut hstar = vec![BigInt::zero(); d1];
    for (k, hstar_entry) in hstar.iter_mut().enumerate() {
        let mut val = BigInt::zero();
        for j in 0..=k {
            let p_j = poly.eval(j as u64);
            // p_j should be an integer (Kostka number at n=j).
            let p_j_int = p_j.numer().clone(); // denom should be 1
            let binom = binom_int(d1 as i64, (k - j) as i64);
            let sign = if (k - j) % 2 == 0 {
                BigInt::one()
            } else {
                -BigInt::one()
            };
            val += sign * binom * p_j_int;
        }
        *hstar_entry = val;
    }
    hstar
}

fn binom_int(n: i64, k: i64) -> BigInt {
    if k < 0 || k > n {
        return BigInt::zero();
    }
    let mut result = BigInt::one();
    for i in 0..k {
        result = result * BigInt::from(n - i) / BigInt::from(i + 1);
    }
    result
}

pub fn is_palindromic(v: &[BigInt]) -> bool {
    let v = trim_trailing_zeros(v);
    let n = v.len();
    (0..n / 2).all(|i| v[i] == v[n - 1 - i])
}

pub fn is_unimodal(v: &[BigInt]) -> bool {
    let v = trim_trailing_zeros(v);
    let n = v.len();
    if n <= 1 {
        return true;
    }
    let peak = v
        .iter()
        .enumerate()
        .max_by_key(|(_, x)| (*x).clone())
        .map(|(i, _)| i)
        .unwrap_or(0);
    (0..peak).all(|i| v[i] <= v[i + 1]) && (peak..n - 1).all(|i| v[i] >= v[i + 1])
}

fn trim_trailing_zeros(v: &[BigInt]) -> &[BigInt] {
    let end = v
        .iter()
        .rposition(|x| !x.is_zero())
        .map(|i| i + 1)
        .unwrap_or(0);
    &v[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(parts: &[u32]) -> Partition {
        Partition::new(parts.to_vec())
    }

    #[test]
    fn ehrhart_matches_legacy_dp() {
        let lambda = p(&[3, 1, 1]);
        let mu = Partition::empty();
        let w = [1, 1, 1, 1, 1];

        let fast = compute_ehrhart(&lambda, &mu, &w, None, None, false, None, true);
        let legacy = compute_ehrhart_legacy(&lambda, &mu, &w, None, None, false, None, true);

        assert_eq!(fast.degree, legacy.degree);
        assert_eq!(fast.coeffs, legacy.coeffs);
    }

    #[test]
    fn flagged_ehrhart_uses_flagged_dimension_and_counts() {
        let lambda = p(&[2, 1]);
        let mu = Partition::empty();
        let w = [1, 1, 1];
        let upper = [1, 1, 2];

        let poly = try_compute_ehrhart(&lambda, &mu, &w, Some(&upper), None, false, None, true)
            .expect("flagged Ehrhart interpolation");

        assert_eq!(poly.degree, 0);
        assert_eq!(poly.coeffs, vec![BigRational::one()]);
        assert_eq!(poly.eval(1), BigRational::one());
    }

    #[test]
    fn flagged_reciprocity_matches_positive_interpolation() {
        let lambda = p(&[3, 2, 1]);
        let mu = Partition::empty();
        let weight = [1, 1, 1, 1, 1, 1];
        let upper = [1, 2, 2, 3, 3, 3];
        let lower = [1, 1, 1, 1, 1, 2];

        let positive = try_compute_ehrhart(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&lower),
            false,
            None,
            false,
        )
        .expect("positive flagged interpolation");
        let reciprocal = try_compute_ehrhart(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&lower),
            false,
            None,
            true,
        )
        .expect("reciprocal flagged interpolation");
        let unflagged = try_compute_ehrhart(&lambda, &mu, &weight, None, None, false, None, false)
            .expect("unflagged interpolation");
        assert_eq!(reciprocal.degree, positive.degree);
        assert_eq!(reciprocal.coeffs, positive.coeffs);
        assert_ne!(positive.coeffs, unflagged.coeffs);
    }

    #[test]
    fn detects_forced_unit_rectangle_suffix() {
        let lambda = p(&[3, 1, 1]);
        let mu = p(&[1]);
        let weight = [1, 1, 1, 1];
        let upper = [3, 3, 3, 3];
        let lower = [1, 1, 1, 3];

        let reduced =
            frozen_unit_rectangle_reduction(&lambda, &mu, &weight, Some(&upper), Some(&lower))
                .expect("forced suffix");
        assert_eq!(reduced.lambda, p(&[3, 1]));
        assert_eq!(reduced.mu, p(&[1]));
        assert_eq!(reduced.weight, [1, 1, 1]);
        assert_eq!(reduced.upper_flags, None);
        assert_eq!(reduced.lower_flags, None);
        assert_eq!((reduced.cut_row, reduced.cut_label), (2, 3));
    }

    #[test]
    fn forced_rectangle_reduction_matches_unreduced_legacy_polynomial() {
        let lambda = p(&[3, 1, 1]);
        let mu = p(&[1]);
        let weight = [1, 1, 1, 1];
        let upper = [3, 3, 3, 3];
        let lower = [1, 1, 1, 3];

        let reduced = try_compute_ehrhart(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&lower),
            false,
            None,
            true,
        )
        .expect("reduced interpolation");
        let unreduced = compute_ehrhart_legacy(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&lower),
            false,
            None,
            true,
        );
        assert_eq!(reduced.degree, unreduced.degree);
        assert_eq!(reduced.coeffs, unreduced.coeffs);
    }

    #[test]
    fn forced_rectangle_reduction_rejects_near_misses() {
        let lambda = p(&[3, 1, 1]);
        let mu = p(&[1]);
        let weight = [1, 1, 1, 1];
        let upper = [3, 3, 3, 3];

        // The suffix label is still allowed in the top block.
        assert!(frozen_unit_rectangle_reduction(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&[1, 1, 1, 2]),
        )
        .is_none());

        // The suffix label is barred from its uniquely assigned row.
        assert!(frozen_unit_rectangle_reduction(
            &lambda,
            &mu,
            &weight,
            Some(&[3, 3, 3, 2]),
            Some(&[1, 1, 1, 3]),
        )
        .is_none());

        // The suffix multiplicity does not match a unit-width tail row.
        assert!(frozen_unit_rectangle_reduction(
            &p(&[4, 1, 1]),
            &mu,
            &[1, 1, 1, 2],
            Some(&upper),
            Some(&[1, 1, 1, 3]),
        )
        .is_none());
    }

    #[test]
    fn detects_nontrivial_saturated_product_split() {
        let lambda = p(&[3, 2, 1]);
        let mu = p(&[1]);
        let weight = [1, 1, 1, 1, 1];
        let upper = [3, 3, 3, 3, 3];
        let lower = [1, 1, 2, 2, 2];

        let split = saturated_row_label_split(&lambda, &mu, &weight, Some(&upper), Some(&lower))
            .expect("saturated split");
        assert_eq!((split.cut_row, split.cut_label), (1, 2));
        assert_eq!(split.top.lambda, p(&[3]));
        assert_eq!(split.top.mu, p(&[1]));
        assert_eq!(split.top.weight, [1, 1]);
        assert_eq!(split.bottom.lambda, p(&[2, 1]));
        assert_eq!(split.bottom.mu, Partition::empty());
        assert_eq!(split.bottom.weight, [1, 1, 1]);
        assert_eq!(split.top.upper_flags, None);
        assert_eq!(split.top.lower_flags, None);
        assert_eq!(split.bottom.upper_flags, None);
        assert_eq!(split.bottom.lower_flags, None);
    }

    #[test]
    fn saturated_product_split_matches_unreduced_legacy_polynomial() {
        let lambda = p(&[3, 2, 1]);
        let mu = p(&[1]);
        let weight = [1, 1, 1, 1, 1];
        let upper = [3, 3, 3, 3, 3];
        let lower = [1, 1, 2, 2, 2];

        let split = try_compute_ehrhart(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&lower),
            false,
            None,
            true,
        )
        .expect("split interpolation");
        let unreduced = compute_ehrhart_legacy(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&lower),
            false,
            None,
            true,
        );
        assert_eq!(split.degree, unreduced.degree);
        assert_eq!(split.coeffs, unreduced.coeffs);
    }

    #[test]
    fn saturated_product_split_multiplies_two_nontrivial_factors() {
        let lambda = p(&[3, 2, 2, 1]);
        let mu = p(&[1, 1]);
        let weight = [1, 1, 1, 1, 1, 1];
        let upper = [2, 2, 2, 4, 4, 4];
        let lower = [1, 1, 1, 3, 3, 3];

        let factors = saturated_row_label_split(&lambda, &mu, &weight, Some(&upper), Some(&lower))
            .expect("saturated split");
        let top_dimension = gt_polytope_dim_full(
            factors.top.lambda.parts(),
            factors.top.mu.parts(),
            &factors.top.weight,
            factors.top.upper_flags.as_deref(),
            factors.top.lower_flags.as_deref(),
        )
        .expect("nonempty top factor");
        let bottom_dimension = gt_polytope_dim_full(
            factors.bottom.lambda.parts(),
            factors.bottom.mu.parts(),
            &factors.bottom.weight,
            factors.bottom.upper_flags.as_deref(),
            factors.bottom.lower_flags.as_deref(),
        )
        .expect("nonempty bottom factor");
        assert!(top_dimension > 0 && bottom_dimension > 0);

        let split = try_compute_ehrhart(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&lower),
            false,
            None,
            true,
        )
        .expect("split interpolation");
        let unreduced = compute_ehrhart_legacy(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&lower),
            false,
            None,
            true,
        );
        assert_eq!(split.degree, unreduced.degree);
        assert_eq!(split.coeffs, unreduced.coeffs);
    }

    #[test]
    fn saturated_product_split_requires_later_labels_to_stay_below_cut() {
        let lambda = p(&[3, 2, 1]);
        let mu = p(&[1]);
        let weight = [1, 1, 1, 1, 1];
        let upper = [3, 3, 3, 3, 3];
        let lower = [1, 1, 1, 2, 2];

        assert!(
            saturated_row_label_split(&lambda, &mu, &weight, Some(&upper), Some(&lower),).is_none()
        );
    }

    #[test]
    fn state_limit_is_fallible_on_positive_and_strict_gt_paths() {
        let lambda = p(&[2, 1]);
        let mu = Partition::empty();
        let w = [1, 1, 1];

        let positive = try_compute_ehrhart_with_mode(
            &lambda,
            &mu,
            &w,
            None,
            None,
            false,
            Some(0),
            EhrhartInterpolation::PositiveOnly,
        );
        assert!(positive.is_err());

        let strict = try_compute_ehrhart_with_mode(
            &lambda,
            &mu,
            &w,
            None,
            None,
            false,
            Some(0),
            EhrhartInterpolation::AdaptiveReciprocity,
        );
        assert!(strict.is_err());
    }

    #[test]
    fn gorenstein_mode_matches_adaptive_for_birkhoff_three() {
        let lambda = p(&[3, 2, 1]);
        let mu = p(&[2, 1]);
        let w = [1, 1, 1];

        let adaptive = compute_ehrhart_with_mode(
            &lambda,
            &mu,
            &w,
            None,
            None,
            false,
            None,
            EhrhartInterpolation::AdaptiveReciprocity,
        );
        let gorenstein = compute_ehrhart_with_mode(
            &lambda,
            &mu,
            &w,
            None,
            None,
            false,
            None,
            EhrhartInterpolation::Gorenstein,
        );

        assert_eq!(adaptive.degree, gorenstein.degree);
        assert_eq!(adaptive.coeffs, gorenstein.coeffs);
    }

    #[test]
    fn gorenstein_mode_matches_adaptive_for_simplex_case() {
        let lambda = p(&[3, 1]);
        let mu = Partition::empty();
        let w = [1, 1, 1, 1];

        let adaptive = compute_ehrhart_with_mode(
            &lambda,
            &mu,
            &w,
            None,
            None,
            false,
            None,
            EhrhartInterpolation::AdaptiveReciprocity,
        );
        let gorenstein = compute_ehrhart_with_mode(
            &lambda,
            &mu,
            &w,
            None,
            None,
            false,
            None,
            EhrhartInterpolation::Gorenstein,
        );

        assert_eq!(adaptive.degree, gorenstein.degree);
        assert_eq!(adaptive.coeffs, gorenstein.coeffs);
    }
}
