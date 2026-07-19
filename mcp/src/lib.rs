//! MCP tools backed directly by the Ehrcalc library.

use ehrcalc::exact::{
    format_rational, parse_bigint_list, parse_rational_list, BinomialBasisPolynomial,
    EhrhartData, EhrhartPolynomial, HStarPolynomial,
};
use ehrcalc::families::{
    flow_ehrhart, gt_ehrhart, key_ehrhart, kostka_count, lr_count, order_ehrhart, FlowInput,
    GtInput, KeyInput, KostkaInput, LrInput, OrderInput,
};
use ehrcalc::render::ehrhart_json;
use num_rational::BigRational;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ErrorData as McpError, Json, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// MCP server for exact Ehrcalc operations.
#[derive(Debug, Clone)]
pub struct EhrcalcServer {
    tool_router: ToolRouter<Self>,
}

/// Exact transform operation supported by `ehrhart_transform`.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TransformOperation {
    Interpolate,
    ToHstar,
    FromHstar,
    Binomial,
    Delta,
    Sum,
}

/// One exact interpolation sample.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SamplePoint {
    pub dilation: i64,
    /// Exact integer or rational value, for example `"17/3"`.
    pub value: String,
}

/// Input for exact polynomial transforms.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EhrhartTransformRequest {
    pub operation: TransformOperation,
    pub dimension: Option<usize>,
    /// Ascending power-basis coefficients, each an exact integer or rational string.
    pub coefficients: Option<Vec<String>>,
    /// h* or standard binomial coefficients, depending on `operation`.
    pub integer_coefficients: Option<Vec<String>>,
    pub points: Option<Vec<SamplePoint>>,
}

/// Tagged input for one supported family calculation.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "family", rename_all = "snake_case")]
pub enum FamilyRequest {
    Gt {
        lambda: Vec<u32>,
        mu: Option<Vec<u32>>,
        weight: Vec<u32>,
        upper_flags: Option<Vec<u32>>,
        lower_flags: Option<Vec<u32>>,
        max_states: Option<usize>,
        positive_only: Option<bool>,
    },
    Kostka {
        lambda: Vec<u32>,
        mu: Option<Vec<u32>>,
        weight: Vec<u32>,
        upper_flags: Option<Vec<u32>>,
        lower_flags: Option<Vec<u32>>,
        max_states: Option<usize>,
    },
    Lr {
        lambda: Vec<u32>,
        mu: Vec<u32>,
        nu: Vec<u32>,
        max_states: Option<usize>,
    },
    Key {
        lambda: Vec<u32>,
        sigma: Vec<usize>,
        max_degree: Option<usize>,
    },
    OrderCovers {
        vertices: usize,
        covers: Vec<CoverInput>,
    },
    OrderChain {
        elements: usize,
    },
    OrderAntichain {
        elements: usize,
    },
    OrderFence {
        elements: usize,
    },
    OrderShape {
        lambda: Vec<u32>,
    },
    Flow {
        vertices: usize,
        edges: Vec<EdgeInput>,
        netflow: Vec<i64>,
        max_states: Option<usize>,
        positive_only: Option<bool>,
    },
}

/// One zero-based cover relation.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CoverInput {
    pub lower: usize,
    pub upper: usize,
}

/// One zero-based directed flow edge.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EdgeInput {
    pub tail: usize,
    pub head: usize,
}

#[tool_router(router = tool_router)]
impl EhrcalcServer {
    /// Construct a server with the registered exact tools.
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Run an exact Ehrhart transform: interpolate, convert to or from h*, convert binomial coordinates, take Delta, or take a discrete sum. Exact coefficients are strings."
    )]
    pub fn ehrhart_transform(
        &self,
        Parameters(request): Parameters<EhrhartTransformRequest>,
    ) -> Result<Json<Value>, McpError> {
        transform_json(&request).map(Json).map_err(invalid_params)
    }

    #[tool(
        description = "Compute exact data for one tagged family request: GT, Kostka, LR, key, order, or acyclic flow. Ehrhart responses include dimension, power coefficients, h*, and binomial coordinates."
    )]
    pub fn compute_family(
        &self,
        Parameters(request): Parameters<FamilyRequest>,
    ) -> Result<Json<Value>, McpError> {
        family_json(&request).map(Json).map_err(invalid_params)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for EhrcalcServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: Default::default(),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "ehrcalc".to_string(),
                title: Some("Ehrcalc".to_string()),
                version: env!("CARGO_PKG_VERSION").to_string(),
                description: Some("Exact Ehrhart and combinatorial counting tools.".to_string()),
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "Use exact string coefficients for large integers or rationals. Use compute_family for supported counting families and ehrhart_transform for family-neutral operations.".to_string(),
            ),
        }
    }
}

impl Default for EhrcalcServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Execute an exact transform without an MCP transport. Used by tests and tools.
pub fn transform_json(request: &EhrhartTransformRequest) -> Result<Value, String> {
    let polynomial = match request.operation {
        TransformOperation::Interpolate => {
            let dimension = required_dimension(request)?;
            let points = request
                .points
                .as_ref()
                .ok_or_else(|| "interpolate requires points".to_string())?
                .iter()
                .map(|point| Ok((point.dilation, parse_exact(&point.value)?)))
                .collect::<Result<Vec<_>, String>>()?;
            EhrhartPolynomial::interpolate(dimension, &points)?
        }
        TransformOperation::FromHstar => {
            let dimension = required_dimension(request)?;
            let coefficients = request
                .integer_coefficients
                .as_ref()
                .ok_or_else(|| "from_hstar requires integer_coefficients".to_string())?;
            HStarPolynomial::new(dimension, parse_integer_strings(coefficients)?)?.to_ehrhart()?
        }
        TransformOperation::Binomial => {
            let coefficients = request
                .integer_coefficients
                .as_ref()
                .ok_or_else(|| "binomial requires integer_coefficients".to_string())?;
            BinomialBasisPolynomial::new(parse_integer_strings(coefficients)?).to_polynomial()?
        }
        TransformOperation::ToHstar | TransformOperation::Delta | TransformOperation::Sum => {
            let polynomial = polynomial_from_request(request)?;
            match request.operation {
                TransformOperation::ToHstar => polynomial,
                TransformOperation::Delta => polynomial.finite_difference()?,
                TransformOperation::Sum => polynomial.discrete_sum()?,
                _ => unreachable!("matched above"),
            }
        }
    };

    match request.operation {
        TransformOperation::ToHstar | TransformOperation::FromHstar => {
            let mut result = ehrhart_json(&EhrhartData::new(polynomial)?);
            result["operation"] = json!(operation_name(&request.operation));
            Ok(result)
        }
        _ => Ok(polynomial_json(&polynomial, operation_name(&request.operation))),
    }
}

/// Execute one tagged family request without an MCP transport.
pub fn family_json(request: &FamilyRequest) -> Result<Value, String> {
    match request {
        FamilyRequest::Gt {
            lambda,
            mu,
            weight,
            upper_flags,
            lower_flags,
            max_states,
            positive_only,
        } => ehrhart_family_json(
            "gt",
            gt_ehrhart(&GtInput {
                lambda: lambda.clone(),
                mu: mu.clone().unwrap_or_default(),
                weight: weight.clone(),
                upper_flags: upper_flags.clone(),
                lower_flags: lower_flags.clone(),
                max_states: *max_states,
                use_reciprocity: !positive_only.unwrap_or(false),
            })?,
        ),
        FamilyRequest::Kostka {
            lambda,
            mu,
            weight,
            upper_flags,
            lower_flags,
            max_states,
        } => Ok(json!({
            "family": "kostka",
            "value": kostka_count(&KostkaInput {
                lambda: lambda.clone(),
                mu: mu.clone().unwrap_or_default(),
                weight: weight.clone(),
                upper_flags: upper_flags.clone(),
                lower_flags: lower_flags.clone(),
                max_states: *max_states,
            })?.to_string(),
        })),
        FamilyRequest::Lr {
            lambda,
            mu,
            nu,
            max_states,
        } => Ok(json!({
            "family": "lr",
            "value": lr_count(&LrInput {
                lambda: lambda.clone(),
                mu: mu.clone(),
                nu: nu.clone(),
                max_states: *max_states,
            })?.to_string(),
        })),
        FamilyRequest::Key {
            lambda,
            sigma,
            max_degree,
        } => ehrhart_family_json(
            "key",
            key_ehrhart(&KeyInput {
                lambda: lambda.clone(),
                sigma: sigma.clone(),
                max_degree: *max_degree,
            })?,
        ),
        FamilyRequest::OrderCovers { vertices, covers } => ehrhart_family_json(
            "order_covers",
            order_ehrhart(&OrderInput::Covers {
                vertices: *vertices,
                covers: covers.iter().map(|cover| (cover.lower, cover.upper)).collect(),
            })?,
        ),
        FamilyRequest::OrderChain { elements } => {
            ehrhart_family_json("order_chain", order_ehrhart(&OrderInput::Chain { elements: *elements })?)
        }
        FamilyRequest::OrderAntichain { elements } => ehrhart_family_json(
            "order_antichain",
            order_ehrhart(&OrderInput::Antichain { elements: *elements })?,
        ),
        FamilyRequest::OrderFence { elements } => {
            ehrhart_family_json("order_fence", order_ehrhart(&OrderInput::Fence { elements: *elements })?)
        }
        FamilyRequest::OrderShape { lambda } => {
            ehrhart_family_json("order_shape", order_ehrhart(&OrderInput::Shape { lambda: lambda.clone() })?)
        }
        FamilyRequest::Flow {
            vertices,
            edges,
            netflow,
            max_states,
            positive_only,
        } => ehrhart_family_json(
            "flow",
            flow_ehrhart(&FlowInput {
                vertices: *vertices,
                edges: edges.iter().map(|edge| (edge.tail, edge.head)).collect(),
                netflow: netflow.clone(),
                max_states: *max_states,
                use_reciprocity: !positive_only.unwrap_or(false),
            })?,
        ),
    }
}

fn required_dimension(request: &EhrhartTransformRequest) -> Result<usize, String> {
    request
        .dimension
        .ok_or_else(|| "operation requires a declared dimension".to_string())
}

fn polynomial_from_request(request: &EhrhartTransformRequest) -> Result<EhrhartPolynomial, String> {
    let dimension = required_dimension(request)?;
    let coefficients = request
        .coefficients
        .as_ref()
        .ok_or_else(|| "operation requires power-basis coefficients".to_string())?;
    EhrhartPolynomial::new(dimension, parse_rational_strings(coefficients)?)
}

fn parse_exact(text: &str) -> Result<BigRational, String> {
    parse_rational_list(text).and_then(|mut values| {
        if values.len() == 1 {
            Ok(values.remove(0))
        } else {
            Err(format!("expected one rational value, got `{text}`"))
        }
    })
}

fn parse_rational_strings(values: &[String]) -> Result<Vec<BigRational>, String> {
    values.iter().map(|value| parse_exact(value)).collect()
}

fn parse_integer_strings(values: &[String]) -> Result<Vec<num_bigint::BigInt>, String> {
    values
        .iter()
        .map(|value| {
            parse_bigint_list(value).and_then(|mut parsed| {
                if parsed.len() == 1 {
                    Ok(parsed.remove(0))
                } else {
                    Err(format!("expected one integer value, got `{value}`"))
                }
            })
        })
        .collect()
}

fn ehrhart_family_json(family: &str, data: EhrhartData) -> Result<Value, String> {
    let mut result = ehrhart_json(&data);
    result["family"] = json!(family);
    Ok(result)
}

fn polynomial_json(polynomial: &EhrhartPolynomial, operation: &str) -> Value {
    json!({
        "operation": operation,
        "dimension": polynomial.dimension(),
        "power_coefficients": polynomial.power_coeffs().iter().map(format_rational).collect::<Vec<_>>(),
        "binomial_coefficients": polynomial.to_binomial_basis().ok().map(|basis| basis.coeffs().iter().map(ToString::to_string).collect::<Vec<_>>()),
    })
}

fn operation_name(operation: &TransformOperation) -> &'static str {
    match operation {
        TransformOperation::Interpolate => "interpolate",
        TransformOperation::ToHstar => "to_hstar",
        TransformOperation::FromHstar => "from_hstar",
        TransformOperation::Binomial => "binomial",
        TransformOperation::Delta => "delta",
        TransformOperation::Sum => "sum",
    }
}

fn invalid_params(message: String) -> McpError {
    McpError::invalid_params(message, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_json_round_trips_hstar_data() {
        let output = transform_json(&EhrhartTransformRequest {
            operation: TransformOperation::ToHstar,
            dimension: Some(1),
            coefficients: Some(vec!["1".to_string(), "1".to_string()]),
            integer_coefficients: None,
            points: None,
        })
        .expect("h* transform");
        assert_eq!(output["hstar"], json!(["1", "0"]));
    }

    #[test]
    fn family_json_computes_kostka() {
        let output = family_json(&FamilyRequest::Kostka {
            lambda: vec![2, 1],
            mu: None,
            weight: vec![1, 1, 1],
            upper_flags: None,
            lower_flags: None,
            max_states: None,
        })
        .expect("Kostka MCP request");
        assert_eq!(output["value"], "2");
    }

    #[test]
    fn family_json_routes_order_and_flow_requests() {
        let order = family_json(&FamilyRequest::OrderChain { elements: 2 })
            .expect("order MCP request");
        assert_eq!(order["family"], "order_chain");
        assert_eq!(order["dimension"], 2);

        let flow = family_json(&FamilyRequest::Flow {
            vertices: 2,
            edges: vec![EdgeInput { tail: 0, head: 1 }, EdgeInput { tail: 0, head: 1 }],
            netflow: vec![1, -1],
            max_states: None,
            positive_only: Some(false),
        })
        .expect("flow MCP request");
        assert_eq!(flow["family"], "flow");
        assert_eq!(flow["hstar"], json!(["1", "0"]));
    }

    #[test]
    fn transform_json_rejects_ambiguous_or_missing_input() {
        let error = transform_json(&EhrhartTransformRequest {
            operation: TransformOperation::Interpolate,
            dimension: Some(1),
            coefficients: None,
            integer_coefficients: None,
            points: None,
        })
        .expect_err("interpolation without points must fail");
        assert!(error.contains("requires points"));
    }
}
