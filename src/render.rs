//! Shared plain-text, JSON, and LaTeX renderers for exact Ehrhart results.

use crate::exact::{format_rational, BinomialBasisPolynomial, EhrhartData, EhrhartPolynomial};
use clap::ValueEnum;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};
use serde_json::{json, Value};

/// Public output formats shared by CLI and future MCP operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lower")]
pub enum OutputFormat {
    Text,
    Json,
    Latex,
}

/// Render a complete Ehrhart calculation in the requested format.
pub fn render_ehrhart(data: &EhrhartData, format: OutputFormat) -> String {
    match format {
        OutputFormat::Text => render_ehrhart_text(data),
        OutputFormat::Json => serde_json::to_string_pretty(&ehrhart_json(data))
            .expect("Ehrhart JSON values are serializable"),
        OutputFormat::Latex => render_ehrhart_latex(data),
    }
}

/// Render an exact nonnegative count with a stable JSON form.
pub fn render_count(name: &str, value: &BigInt, format: OutputFormat) -> String {
    match format {
        OutputFormat::Text => format!("{name}: {value}\n"),
        OutputFormat::Json => serde_json::to_string_pretty(&json!({ name: value.to_string() }))
            .expect("count JSON is serializable"),
        OutputFormat::Latex => format!("{} = {}\n", latex_identifier(name), value),
    }
}

/// Render one exact transform polynomial in the requested format.
pub fn render_polynomial(
    label: &str,
    polynomial: &EhrhartPolynomial,
    binomial: Option<&BinomialBasisPolynomial>,
    format: OutputFormat,
) -> String {
    match format {
        OutputFormat::Text => {
            let mut output = format!("dimension: {}\n{label}: {}\n", polynomial.dimension(), format_text_polynomial(polynomial.power_coeffs(), "n"));
            if let Some(binomial) = binomial {
                output.push_str(&format!("binomial_basis: [{}]\n", join_bigints(binomial.coeffs())));
            }
            output
        }
        OutputFormat::Json => serde_json::to_string_pretty(&json!({
            "dimension": polynomial.dimension(),
            "label": label,
            "power_coefficients": rational_strings(polynomial.power_coeffs()),
            "binomial_coefficients": binomial.map(|value| bigint_strings(value.coeffs())),
        }))
        .expect("polynomial JSON is serializable"),
        OutputFormat::Latex => {
            let mut output = format!("{} = {}\n", latex_identifier(label), format_latex_polynomial(polynomial.power_coeffs(), "n"));
            if let Some(binomial) = binomial {
                output.push_str(&format!("\\text{{binomial basis}} = [{}]\n", join_bigints(binomial.coeffs())));
            }
            output
        }
    }
}

/// Stable structured representation for CLI and MCP callers.
pub fn ehrhart_json(data: &EhrhartData) -> Value {
    json!({
        "dimension": data.ehrhart.dimension(),
        "polynomial_degree": data.ehrhart.degree(),
        "hstar_degree": data.hstar.degree(),
        "ehrhart_power": rational_strings(data.ehrhart.power_coeffs()),
        "hstar": bigint_strings(data.hstar.coeffs()),
        "binomial_basis": data.ehrhart.to_binomial_basis().ok().map(|basis| bigint_strings(basis.coeffs())),
    })
}

fn render_ehrhart_text(data: &EhrhartData) -> String {
    format!(
        "dimension: {}\nEhrhart(n): {}\nhstar(t): [{}]\nbinomial_basis: [{}]\n",
        data.ehrhart.dimension(),
        format_text_polynomial(data.ehrhart.power_coeffs(), "n"),
        join_bigints(data.hstar.coeffs()),
        data.ehrhart
            .to_binomial_basis()
            .map(|basis| join_bigints(basis.coeffs()))
            .unwrap_or_else(|error| format!("unavailable ({error})")),
    )
}

fn render_ehrhart_latex(data: &EhrhartData) -> String {
    format!(
        "\\dim P = {}\\\\\nL_P(n) = {}\\\\\nh^*_P(t) = {}\n",
        data.ehrhart.dimension(),
        format_latex_polynomial(data.ehrhart.power_coeffs(), "n"),
        format_latex_polynomial_bigint(data.hstar.coeffs(), "t"),
    )
}

fn rational_strings(values: &[BigRational]) -> Vec<String> {
    values.iter().map(format_rational).collect()
}

fn bigint_strings(values: &[BigInt]) -> Vec<String> {
    values.iter().map(ToString::to_string).collect()
}

fn join_bigints(values: &[BigInt]) -> String {
    values.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
}

fn format_text_polynomial(coeffs: &[BigRational], variable: &str) -> String {
    format_polynomial(coeffs, variable, false)
}

fn format_latex_polynomial(coeffs: &[BigRational], variable: &str) -> String {
    format_polynomial(coeffs, variable, true)
}

fn format_latex_polynomial_bigint(coeffs: &[BigInt], variable: &str) -> String {
    let rational = coeffs.iter().cloned().map(BigRational::from).collect::<Vec<_>>();
    format_latex_polynomial(&rational, variable)
}

fn format_polynomial(coeffs: &[BigRational], variable: &str, latex: bool) -> String {
    let mut terms = Vec::new();
    for (degree, coefficient) in coeffs.iter().enumerate().rev() {
        if coefficient.is_zero() {
            continue;
        }
        let negative = coefficient.numer().is_negative();
        let magnitude = if negative { -coefficient.clone() } else { coefficient.clone() };
        let coefficient_text = if latex {
            format_latex_rational(&magnitude)
        } else {
            format_rational(&magnitude)
        };
        let variable_text = match degree {
            0 => String::new(),
            1 => variable.to_string(),
            _ if latex => format!("{}^{{{degree}}}", variable),
            _ => format!("{variable}^{degree}"),
        };
        let term = if degree == 0 {
            coefficient_text
        } else if magnitude == BigRational::from(BigInt::from(1)) {
            variable_text
        } else if latex {
            format!("{coefficient_text}{variable_text}")
        } else {
            format!("{coefficient_text} {variable_text}")
        };
        if terms.is_empty() {
            terms.push(if negative { format!("-{term}") } else { term });
        } else if negative {
            terms.push(format!(" - {term}"));
        } else {
            terms.push(format!(" + {term}"));
        }
    }
    if terms.is_empty() {
        "0".to_string()
    } else {
        terms.concat()
    }
}

fn format_latex_rational(value: &BigRational) -> String {
    if value.denom() == &BigInt::from(1) {
        value.numer().to_string()
    } else {
        format!("\\frac{{{}}}{{{}}}", value.numer(), value.denom())
    }
}

fn latex_identifier(text: &str) -> String {
    text.replace('_', "\\_")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exact::EhrhartPolynomial;
    use num_traits::One;

    #[test]
    fn renders_exact_data_in_all_public_formats() {
        let data = EhrhartData::new(
            EhrhartPolynomial::new(
                1,
                vec![BigRational::one(), BigRational::one()],
            )
            .expect("valid interval Ehrhart polynomial"),
        )
        .expect("integral h*");
        assert!(render_ehrhart(&data, OutputFormat::Text).contains("Ehrhart(n): n + 1"));
        assert!(render_ehrhart(&data, OutputFormat::Latex).contains("L_P(n) = n + 1"));
        let json = ehrhart_json(&data);
        assert_eq!(json["dimension"], 1);
        assert_eq!(json["hstar"], json!(["1", "0"]));
    }
}
