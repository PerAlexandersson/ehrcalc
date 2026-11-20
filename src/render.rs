//! Shared plain-text, JSON, and LaTeX renderers for exact Ehrhart results.

use crate::exact::{format_rational, BinomialBasisPolynomial, EhrhartData, EhrhartPolynomial};
use crate::families::{
    ConstraintComputation, ConstraintCount, CountingPlan, OrderHeuristic, SamplePlan,
};
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

/// Stable spelling shared by constraint plans and interface parsers.
pub fn constraint_order_heuristic_name(heuristic: OrderHeuristic) -> &'static str {
    match heuristic {
        OrderHeuristic::Auto => "auto",
        OrderHeuristic::Greedy => "greedy",
        OrderHeuristic::SumContiguous => "sum-contiguous",
        OrderHeuristic::ReverseCuthillMcKee => "rcm",
        OrderHeuristic::ExactSubset => "exact-subset",
        OrderHeuristic::MinimumDegree => "min-degree",
        OrderHeuristic::MinimumFill => "min-fill",
        OrderHeuristic::CuthillMcKee => "cm",
        OrderHeuristic::GtLevel => "gt-level",
        OrderHeuristic::GtRow => "gt-row",
        OrderHeuristic::LocalImprovement => "local",
    }
}

/// Parse one stable ordering name for CLI and MCP callers.
pub fn parse_constraint_order_heuristic(name: &str) -> Result<OrderHeuristic, String> {
    match name {
        "auto" => Ok(OrderHeuristic::Auto),
        "greedy" => Ok(OrderHeuristic::Greedy),
        "sum-contiguous" => Ok(OrderHeuristic::SumContiguous),
        "rcm" => Ok(OrderHeuristic::ReverseCuthillMcKee),
        "exact-subset" => Ok(OrderHeuristic::ExactSubset),
        "min-degree" => Ok(OrderHeuristic::MinimumDegree),
        "min-fill" => Ok(OrderHeuristic::MinimumFill),
        "cm" => Ok(OrderHeuristic::CuthillMcKee),
        "gt-level" => Ok(OrderHeuristic::GtLevel),
        "gt-row" => Ok(OrderHeuristic::GtRow),
        "local" => Ok(OrderHeuristic::LocalImprovement),
        _ => Err(format!(
            "unknown order heuristic `{name}`; use auto, greedy, sum-contiguous, rcm, exact-subset, min-degree, min-fill, cm, gt-level, gt-row, local"
        )),
    }
}

/// Serialize a counting plan with candidate estimates and bounded pilot evidence.
pub fn counting_plan_json(plan: &CountingPlan) -> Value {
    json!({
        "dilation": plan.dilation, "strict": plan.strict, "order": plan.order,
        "heuristic": constraint_order_heuristic_name(plan.heuristic), "user_order": plan.user_order,
        "selection": plan.selection,
        "estimated_peak_states": plan.estimated_peak.to_string(),
        "estimated_work": plan.estimated_work.to_string(), "peak_frontier": plan.peak_frontier,
        "candidates": plan.candidates.iter().map(|c| json!({
            "heuristic": constraint_order_heuristic_name(c.heuristic), "order": c.order,
            "estimated_peak_states": c.estimated_peak.to_string(), "estimated_work": c.estimated_work.to_string(),
            "peak_frontier": c.peak_frontier,
            "variant": c.variant, "duplicate_of": c.duplicate_of,
            "predicted_peak_states": c.predicted_peak,
            "predicted_runtime_micros": c.predicted_runtime_micros,
            "prediction_basis": c.prediction_basis,
            "pilots": c.pilots.iter().map(|p| json!({
                "dilation": p.dilation, "strict": p.strict, "status": p.status,
                "actual_peak_states": p.peak_states, "layer_states": p.layer_states,
                "transitions": p.transitions, "elapsed_micros": p.elapsed_micros,
                "state_budget": p.state_budget, "transition_budget": p.transition_budget,
                "time_budget_micros": p.time_budget_micros,
            })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>()
    })
}

fn sample_plan_json(plan: &SamplePlan) -> Value {
    json!({
        "positive": plan.positive, "reciprocal": plan.reciprocal,
        "extra_dilations": plan.extra_dilations, "estimated_work": plan.estimated_work.to_string(),
        "ordinary": plan.ordinary_plans.iter().map(counting_plan_json).collect::<Vec<_>>(),
        "strict": plan.strict_plans.iter().map(counting_plan_json).collect::<Vec<_>>(),
    })
}

fn constraint_count_json(count: &ConstraintCount) -> Value {
    json!({
        "count": count.value.to_string(), "actual_peak_states": count.peak_states,
        "layer_states": count.layer_states,
        "plans": count.plans.iter().map(counting_plan_json).collect::<Vec<_>>(),
        "crt": count.modular.iter().map(|c| json!({
            "moduli_count": c.moduli.len(), "bound_bits": c.bound_bits,
            "upper_bound": c.upper_bound.to_string(), "moduli": c.moduli, "residues": c.residues,
            "combined_modulus": c.combined_modulus.to_string(),
        })).collect::<Vec<_>>()
    })
}

/// Warnings retained in every presentation, including a failed polynomial fit.
pub fn constraint_warnings(result: &ConstraintComputation) -> Vec<String> {
    let mut warnings = result.mismatches.iter().map(|m| format!(
        "extra-sample mismatch at dilation {}: fitted {}, counted {}; polynomiality assumption failed or counter/dimension error",
        m.dilation, format_rational(&m.expected), format_rational(&m.actual),
    )).collect::<Vec<_>>();
    warnings.extend(result.polynomiality_warnings.iter().cloned());
    if let Some(error) = &result.data_error {
        warnings.push(format!("Ehrhart transforms unavailable: {error}"));
    }
    if result.component_product_matches == Some(false) {
        warnings.push("component polynomial product differs from the full fit; polynomiality assumption may fail".into());
    }
    warnings
}

/// Whether a necessary polynomiality check or an extra sample failed.
pub fn constraint_polynomiality_failed(result: &ConstraintComputation) -> bool {
    !result.mismatches.is_empty()
        || !result.polynomiality_warnings.is_empty()
        || result.component_product_matches == Some(false)
}

/// Versioned constraint result used unchanged by the CLI and MCP tool.
/// Exact counts, estimates, coefficients, and CRT integers are decimal strings.
pub fn constraint_polytope_json(result: &ConstraintComputation) -> Value {
    let mut plan = sample_plan_json(&result.plan);
    plan["actual_peak_states"] = json!(result.samples.iter().map(|s| s.count.peak_states).max());
    json!({
        "schema": "ehrcalc.constraint-polytope-result.v1",
        "assumes_polynomial": result.assumes_polynomial, "empty": result.empty,
        "polynomiality_checks_failed": constraint_polynomiality_failed(result),
        "dimension": result.dimension, "implicit_equalities": result.implicit_equalities,
        "free_variables": result.free_variables,
        "fixed_values": result.fixed_values.iter().map(|q| q.as_ref().map(format_rational)).collect::<Vec<_>>(),
        "component_dimensions": result.component_dimensions, "plan": plan,
        "samples": result.samples.iter().map(|s| {
            let mut value = constraint_count_json(&s.count);
            value["dilation"] = json!(s.dilation);
            value["kind"] = json!(if s.strict { "interior" } else { "ordinary" });
            value["value"] = json!(format_rational(&s.value));
            value["interpolation"] = json!(s.interpolation);
            value
        }).collect::<Vec<_>>(),
        "ehrhart": result.data.as_ref().map(ehrhart_json),
        "ehrhart_power": result.polynomial.as_ref().map(|p| rational_strings(p.power_coeffs())),
        "hstar": result.data.as_ref().map(|d| bigint_strings(d.hstar.coeffs())),
        "binomial_basis": result.polynomial.as_ref().and_then(|p| p.to_binomial_basis().ok()).map(|b| bigint_strings(b.coeffs())),
        "mismatches": result.mismatches.iter().map(|m| json!({"dilation": m.dilation,
            "expected": format_rational(&m.expected), "actual": format_rational(&m.actual)})).collect::<Vec<_>>(),
        "component_product_matches": result.component_product_matches,
        "warnings": constraint_warnings(result),
    })
}

/// Render the shared constraint result, preserving plan-only and mismatch data.
pub fn render_constraint_polytope(result: &ConstraintComputation, format: OutputFormat) -> String {
    if format == OutputFormat::Json {
        return serde_json::to_string_pretty(&constraint_polytope_json(result))
            .expect("constraint result JSON");
    }
    let prefix = if format == OutputFormat::Latex {
        "% "
    } else {
        ""
    };
    let mut output = if result.empty {
        format!("{prefix}dimension: null\n{prefix}empty: true\n")
    } else if let Some(data) = &result.data {
        render_ehrhart(data, format)
    } else if let Some(poly) = &result.polynomial {
        render_polynomial("fitted_Ehrhart", poly, None, format)
    } else {
        format!(
            "{prefix}dimension: {}\n{prefix}empty: {}\n",
            result
                .dimension
                .map_or_else(|| "null".into(), |d| d.to_string()),
            result.empty
        )
    };
    output.push_str(&format!(
        "{prefix}assumes_polynomial: true\n{prefix}implicit_equalities: {:?}\n",
        result.implicit_equalities
    ));
    output.push_str(&format!(
        "{prefix}sample plan: {} positive, {} reciprocal, extras {:?}\n",
        result.plan.positive, result.plan.reciprocal, result.plan.extra_dilations
    ));
    for p in result
        .plan
        .ordinary_plans
        .iter()
        .chain(&result.plan.strict_plans)
    {
        output.push_str(&format!(
            "{prefix}{} order {:?}: estimated peak {}\n",
            if p.strict { "interior" } else { "ordinary" },
            p.order,
            p.estimated_peak
        ));
    }
    for s in &result.samples {
        output.push_str(&format!(
            "{prefix}sample {} ({}): count {}, value {}, actual peak {}\n",
            s.dilation,
            if s.strict { "interior" } else { "ordinary" },
            s.count.value,
            format_rational(&s.value),
            s.count.peak_states
        ));
        for c in &s.count.modular {
            output.push_str(&format!(
                "{prefix}CRT: {} moduli, {} bound bits\n",
                c.moduli.len(),
                c.bound_bits
            ));
        }
    }
    for warning in constraint_warnings(result) {
        output.push_str(&format!("{prefix}WARNING: {warning}\n"));
    }
    output
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

/// Render an Ehrhart calculation and record the algorithm that produced it.
///
/// JSON output gains the additive string field `method`; text and LaTeX are
/// identical to [`render_ehrhart`].
pub fn render_ehrhart_with_method(
    data: &EhrhartData,
    method: &str,
    format: OutputFormat,
) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ehrhart_json_with_method(data, method))
            .expect("Ehrhart JSON values are serializable"),
        _ => render_ehrhart(data, format),
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
            let mut output = format!(
                "dimension: {}\n{label}: {}\n",
                polynomial.dimension(),
                format_text_polynomial(polynomial.power_coeffs(), "n")
            );
            if let Some(binomial) = binomial {
                output.push_str(&format!(
                    "binomial_basis: [{}]\n",
                    join_bigints(binomial.coeffs())
                ));
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
            let mut output = format!(
                "{} = {}\n",
                latex_identifier(label),
                format_latex_polynomial(polynomial.power_coeffs(), "n")
            );
            if let Some(binomial) = binomial {
                output.push_str(&format!(
                    "\\text{{binomial basis}} = [{}]\n",
                    join_bigints(binomial.coeffs())
                ));
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

/// [`ehrhart_json`] plus the additive string field `method`.
pub fn ehrhart_json_with_method(data: &EhrhartData, method: &str) -> Value {
    let mut result = ehrhart_json(data);
    result["method"] = json!(method);
    result
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
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_text_polynomial(coeffs: &[BigRational], variable: &str) -> String {
    format_polynomial(coeffs, variable, false)
}

fn format_latex_polynomial(coeffs: &[BigRational], variable: &str) -> String {
    format_polynomial(coeffs, variable, true)
}

fn format_latex_polynomial_bigint(coeffs: &[BigInt], variable: &str) -> String {
    let rational = coeffs
        .iter()
        .cloned()
        .map(BigRational::from)
        .collect::<Vec<_>>();
    format_latex_polynomial(&rational, variable)
}

fn format_polynomial(coeffs: &[BigRational], variable: &str, latex: bool) -> String {
    let mut terms = Vec::new();
    for (degree, coefficient) in coeffs.iter().enumerate().rev() {
        if coefficient.is_zero() {
            continue;
        }
        let negative = coefficient.numer().is_negative();
        let magnitude = if negative {
            -coefficient.clone()
        } else {
            coefficient.clone()
        };
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
            EhrhartPolynomial::new(1, vec![BigRational::one(), BigRational::one()])
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
