//! CLI command definitions, input parsing, dispatch, and generated reference docs.

use crate::exact::{
    format_rational, parse_bigint_list, parse_rational, parse_rational_list,
    BinomialBasisPolynomial, EhrhartData, EhrhartPolynomial, HStarPolynomial,
};
use crate::families::{
    flow_ehrhart, gt_ehrhart, key_ehrhart, kostka_count, lr_count, order_ehrhart, FlowInput,
    GtInput, KeyInput, KostkaInput, LrInput, OrderInput,
};
use crate::render::{render_count, render_ehrhart, render_polynomial, OutputFormat};
use clap::{Args, CommandFactory, Parser, Subcommand};
use num_rational::BigRational;
use serde_json::json;
use std::path::PathBuf;

/// Exact Ehrhart computations and related combinatorial counting tools.
#[derive(Debug, Parser)]
#[command(
    name = "ehrcalc",
    version,
    about = "Exact Ehrhart computations and related combinatorial counts",
    long_about = "Ehrcalc computes Ehrhart polynomials, h*-vectors, dimensions, and related exact combinatorial data.\n\nEvery computational command uses exact arbitrary-size integers and rationals. The JSON format is the stable machine-readable contract; formula-producing commands also support LaTeX.",
    after_long_help = "Use `ehrcalc <command> --help` for exact input syntax and examples.\n\nFor generated Markdown command documentation, run `ehrcalc docs cli`."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Public command families and exact transform operations.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Compute Gelfand-Tsetlin Ehrhart, h*, and dimension data.
    Gt(GtArgs),
    /// Compute Kostka, skew Kostka, and flagged Kostka coefficients.
    Kostka(KostkaArgs),
    /// Compute a Littlewood-Richardson coefficient.
    Lr(LrArgs),
    /// Compute key-polynomial Ehrhart families from Kogan faces.
    Key(KeyArgs),
    /// Compute order-polytope Ehrhart, h*, and dimension data.
    Order(OrderArgs),
    /// Compute acyclic flow-polytope Ehrhart, h*, and dimension data.
    Flow(FlowArgs),
    /// Interpolate an exact polynomial from distinct integral sample points.
    Interpolate(InterpolateArgs),
    /// Convert exact Ehrhart data to or from an h* numerator.
    Hstar(HstarArgs),
    /// Convert between the power basis and standard binomial basis.
    Binomial(BinomialArgs),
    /// Compute the finite difference of an exact polynomial.
    Delta(PolynomialArgs),
    /// Compute the discrete sum of an integer-valued polynomial.
    Sum(PolynomialArgs),
    /// Evaluate an exact polynomial at an integral argument.
    Eval(EvalArgs),
    /// Verify an exact polynomial against one independently supplied value.
    Verify(VerifyArgs),
    /// Render generated documentation from the command model.
    Docs {
        #[command(subcommand)]
        target: DocsTarget,
    },
}

impl Command {
    /// Return the stable CLI spelling for diagnostics and documentation.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Gt(_) => "gt",
            Self::Kostka(_) => "kostka",
            Self::Lr(_) => "lr",
            Self::Key(_) => "key",
            Self::Order(_) => "order",
            Self::Flow(_) => "flow",
            Self::Interpolate(_) => "interpolate",
            Self::Hstar(_) => "hstar",
            Self::Binomial(_) => "binomial",
            Self::Delta(_) => "delta",
            Self::Sum(_) => "sum",
            Self::Eval(_) => "eval",
            Self::Verify(_) => "verify",
            Self::Docs { .. } => "docs",
        }
    }
}

/// Output options shared by computational commands.
#[derive(Clone, Debug, Args)]
pub struct OutputArgs {
    /// Render as human-readable text, stable JSON, or LaTeX formulas.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

/// Arguments for a GT Ehrhart calculation.
#[derive(Clone, Debug, Args)]
pub struct GtArgs {
    /// Outer partition, for example `4,3,1`.
    #[arg(long)]
    lambda: String,
    /// Optional inner partition for a skew shape.
    #[arg(long, default_value = "")]
    mu: String,
    /// Weight composition, for example `2,2,1`.
    #[arg(long)]
    weight: String,
    /// Per-label upper row flags.
    #[arg(long)]
    upper_flags: Option<String>,
    /// Per-label lower row flags.
    #[arg(long)]
    lower_flags: Option<String>,
    /// Abort the underlying DP if a level has more than this many states.
    #[arg(long)]
    max_states: Option<usize>,
    /// Use only nonnegative dilation samples instead of reciprocity.
    #[arg(long)]
    positive_only: bool,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for a direct Kostka count.
#[derive(Clone, Debug, Args)]
pub struct KostkaArgs {
    /// Outer partition, for example `4,3,1`.
    #[arg(long)]
    lambda: String,
    /// Optional inner partition for a skew shape.
    #[arg(long, default_value = "")]
    mu: String,
    /// Weight composition, for example `2,2,1`.
    #[arg(long)]
    weight: String,
    /// Per-label upper row flags.
    #[arg(long)]
    upper_flags: Option<String>,
    /// Per-label lower row flags.
    #[arg(long)]
    lower_flags: Option<String>,
    /// Abort the underlying DP if a level has more than this many states.
    #[arg(long)]
    max_states: Option<usize>,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for an LR coefficient.
#[derive(Clone, Debug, Args)]
pub struct LrArgs {
    /// Outer partition lambda.
    #[arg(long)]
    lambda: String,
    /// Inner partition mu.
    #[arg(long)]
    mu: String,
    /// Content partition nu.
    #[arg(long)]
    nu: String,
    /// Abort the underlying DP if a level has more than this many states.
    #[arg(long)]
    max_states: Option<usize>,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for a key-Ehrhart calculation.
#[derive(Clone, Debug, Args)]
pub struct KeyArgs {
    /// Dominant partition lambda.
    #[arg(long)]
    lambda: String,
    /// One-based permutation, for example `2,4,5,3,1`.
    #[arg(long)]
    sigma: String,
    /// A proven degree bound; the default is the full GT bound.
    #[arg(long)]
    max_degree: Option<usize>,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for an order-polytope calculation.
#[derive(Clone, Debug, Args)]
pub struct OrderArgs {
    /// Construct the chain on this many elements.
    #[arg(long, conflicts_with_all = ["antichain", "fence", "shape", "vertices", "covers"])]
    chain: Option<usize>,
    /// Construct the antichain on this many elements.
    #[arg(long, conflicts_with_all = ["chain", "fence", "shape", "vertices", "covers"])]
    antichain: Option<usize>,
    /// Construct the fence on this many elements.
    #[arg(long, conflicts_with_all = ["chain", "antichain", "shape", "vertices", "covers"])]
    fence: Option<usize>,
    /// Construct the Ferrers-poset order polytope of this partition shape.
    #[arg(long, conflicts_with_all = ["chain", "antichain", "fence", "vertices", "covers"])]
    shape: Option<String>,
    /// Number of vertices for an explicit cover-relation input.
    #[arg(long, requires = "covers")]
    vertices: Option<usize>,
    /// Comma-separated zero-based covers, for example `0<1,0<2,1<3,2<3`.
    #[arg(long, requires = "vertices")]
    covers: Option<String>,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for an acyclic flow-polytope calculation.
#[derive(Clone, Debug, Args)]
pub struct FlowArgs {
    /// Number of zero-based graph vertices.
    #[arg(long)]
    vertices: usize,
    /// Comma-separated directed edges, for example `0->1,0->2,1->3,2->3`.
    #[arg(long)]
    edges: String,
    /// Comma-separated netflow vector whose entries sum to zero.
    #[arg(long)]
    netflow: String,
    /// Abort the underlying DP if it exceeds this many states.
    #[arg(long)]
    max_states: Option<usize>,
    /// Use only nonnegative dilation samples instead of reciprocity.
    #[arg(long)]
    positive_only: bool,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for exact interpolation.
#[derive(Clone, Debug, Args)]
pub struct InterpolateArgs {
    /// Declared affine dimension and interpolation degree bound.
    #[arg(long)]
    dimension: usize,
    /// Comma-separated `dilation:value` pairs, for example `0:1,1:3,2:7`.
    #[arg(long)]
    points: String,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for h* conversion.
#[derive(Clone, Debug, Args)]
pub struct HstarArgs {
    /// Declared affine dimension.
    #[arg(long)]
    dimension: usize,
    /// Power-basis coefficients in ascending degree order.
    #[arg(long, conflicts_with = "hstar", required_unless_present = "hstar")]
    coefficients: Option<String>,
    /// h* coefficients in ascending degree order, including trailing zeros.
    #[arg(long, conflicts_with = "coefficients", required_unless_present = "coefficients")]
    hstar: Option<String>,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for a power/binomial-basis conversion.
#[derive(Clone, Debug, Args)]
pub struct BinomialArgs {
    /// Power-basis coefficients in ascending degree order.
    #[arg(long, conflicts_with = "binomial", required_unless_present = "binomial")]
    coefficients: Option<String>,
    /// Standard binomial-basis coefficients for `sum b_j binom(n,j)`.
    #[arg(long, conflicts_with = "coefficients", required_unless_present = "coefficients")]
    binomial: Option<String>,
    /// Declared dimension when converting from the power basis.
    #[arg(long)]
    dimension: Option<usize>,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for transforms of a supplied power-basis polynomial.
#[derive(Clone, Debug, Args)]
pub struct PolynomialArgs {
    /// Declared affine dimension.
    #[arg(long)]
    dimension: usize,
    /// Power-basis coefficients in ascending degree order.
    #[arg(long)]
    coefficients: String,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for exact evaluation.
#[derive(Clone, Debug, Args)]
pub struct EvalArgs {
    /// Declared affine dimension.
    #[arg(long)]
    dimension: usize,
    /// Power-basis coefficients in ascending degree order.
    #[arg(long)]
    coefficients: String,
    /// Integral dilation argument.
    #[arg(long)]
    at: i64,
    #[command(flatten)]
    output: OutputArgs,
}

/// Arguments for one independent exact evaluation check.
#[derive(Clone, Debug, Args)]
pub struct VerifyArgs {
    /// Declared affine dimension.
    #[arg(long)]
    dimension: usize,
    /// Power-basis coefficients in ascending degree order.
    #[arg(long)]
    coefficients: String,
    /// Check one `dilation:value` pair, for example `4:105`.
    #[arg(long)]
    point: String,
    #[command(flatten)]
    output: OutputArgs,
}

/// Targets that can be rendered from the command model.
#[derive(Debug, Subcommand)]
pub enum DocsTarget {
    /// Render the generated Markdown CLI reference to standard output.
    Cli(DocsCliArgs),
}

/// Arguments controlling generated CLI-reference output.
#[derive(Clone, Debug, Args)]
pub struct DocsCliArgs {
    /// Write the generated Markdown reference to this path instead of standard output.
    #[arg(long)]
    write: Option<PathBuf>,
}

/// Run a parsed command. `None` asks the binary to display top-level help.
pub fn run(cli: Cli) -> Result<Option<String>, String> {
    let Some(command) = cli.command else {
        return Ok(None);
    };
    let output = match command {
        Command::Gt(args) => {
            let weight = parse_u32_list(&args.weight, "weight")?;
            let upper_flags = parse_optional_u32_list(args.upper_flags.as_deref(), "upper flags")?;
            let lower_flags = parse_optional_u32_list(args.lower_flags.as_deref(), "lower flags")?;
            validate_flag_lengths(&weight, upper_flags.as_deref(), lower_flags.as_deref())?;
            let data = gt_ehrhart(&GtInput {
                lambda: parse_u32_list(&args.lambda, "lambda")?,
                mu: parse_optional_u32_list(Some(&args.mu), "mu")?.unwrap_or_default(),
                weight,
                upper_flags,
                lower_flags,
                max_states: args.max_states,
                use_reciprocity: !args.positive_only,
            })?;
            render_ehrhart(&data, args.output.format)
        }
        Command::Kostka(args) => {
            let weight = parse_u32_list(&args.weight, "weight")?;
            let upper_flags = parse_optional_u32_list(args.upper_flags.as_deref(), "upper flags")?;
            let lower_flags = parse_optional_u32_list(args.lower_flags.as_deref(), "lower flags")?;
            validate_flag_lengths(&weight, upper_flags.as_deref(), lower_flags.as_deref())?;
            let count = kostka_count(&KostkaInput {
                lambda: parse_u32_list(&args.lambda, "lambda")?,
                mu: parse_optional_u32_list(Some(&args.mu), "mu")?.unwrap_or_default(),
                weight,
                upper_flags,
                lower_flags,
                max_states: args.max_states,
            })?;
            render_count("kostka", &count, args.output.format)
        }
        Command::Lr(args) => {
            let count = lr_count(&LrInput {
                lambda: parse_u32_list(&args.lambda, "lambda")?,
                mu: parse_u32_list(&args.mu, "mu")?,
                nu: parse_u32_list(&args.nu, "nu")?,
                max_states: args.max_states,
            })?;
            render_count("littlewood_richardson", &count, args.output.format)
        }
        Command::Key(args) => {
            let data = key_ehrhart(&KeyInput {
                lambda: parse_u32_list(&args.lambda, "lambda")?,
                sigma: parse_usize_list(&args.sigma, "sigma")?,
                max_degree: args.max_degree,
            })?;
            render_ehrhart(&data, args.output.format)
        }
        Command::Order(args) => {
            let data = order_ehrhart(&parse_order_input(&args)?)?;
            render_ehrhart(&data, args.output.format)
        }
        Command::Flow(args) => {
            let data = flow_ehrhart(&FlowInput {
                vertices: args.vertices,
                edges: parse_edges(&args.edges)?,
                netflow: parse_i64_list(&args.netflow, "netflow")?,
                max_states: args.max_states,
                use_reciprocity: !args.positive_only,
            })?;
            render_ehrhart(&data, args.output.format)
        }
        Command::Interpolate(args) => {
            let polynomial = EhrhartPolynomial::interpolate(args.dimension, &parse_points(&args.points)?)?;
            render_polynomial("interpolate", &polynomial, polynomial.to_binomial_basis().ok().as_ref(), args.output.format)
        }
        Command::Hstar(args) => {
            let data = if let Some(coefficients) = args.coefficients {
                EhrhartData::new(EhrhartPolynomial::new(args.dimension, parse_rational_list(&coefficients)?)?)?
            } else {
                let hstar = HStarPolynomial::new(args.dimension, parse_bigint_list(args.hstar.as_deref().expect("clap requires hstar"))?)?;
                EhrhartData::new(hstar.to_ehrhart()?)?
            };
            render_ehrhart(&data, args.output.format)
        }
        Command::Binomial(args) => {
            let polynomial = if let Some(coefficients) = args.coefficients {
                let coefficients = parse_rational_list(&coefficients)?;
                let dimension = args.dimension.unwrap_or(coefficients.len().saturating_sub(1));
                EhrhartPolynomial::new(dimension, coefficients)?
            } else {
                BinomialBasisPolynomial::new(parse_bigint_list(args.binomial.as_deref().expect("clap requires binomial"))?).to_polynomial()?
            };
            let basis = polynomial.to_binomial_basis()?;
            render_polynomial("Ehrhart", &polynomial, Some(&basis), args.output.format)
        }
        Command::Delta(args) => {
            let polynomial = parse_polynomial_args(&args)?;
            let result = polynomial.finite_difference()?;
            render_polynomial("Delta", &result, result.to_binomial_basis().ok().as_ref(), args.output.format)
        }
        Command::Sum(args) => {
            let polynomial = parse_polynomial_args(&args)?;
            let result = polynomial.discrete_sum()?;
            render_polynomial("sum", &result, result.to_binomial_basis().ok().as_ref(), args.output.format)
        }
        Command::Eval(args) => {
            let polynomial = EhrhartPolynomial::new(args.dimension, parse_rational_list(&args.coefficients)?)?;
            render_value("value", args.at, &polynomial.evaluate(args.at), args.output.format)
        }
        Command::Verify(args) => {
            let polynomial = EhrhartPolynomial::new(args.dimension, parse_rational_list(&args.coefficients)?)?;
            let (point, expected) = parse_one_point(&args.point)?;
            render_verification(point, &expected, &polynomial.evaluate(point), args.output.format)
        }
        Command::Docs { target } => match target {
            DocsTarget::Cli(args) => {
                let reference = cli_reference_markdown();
                if let Some(path) = args.write {
                    std::fs::write(&path, reference)
                        .map_err(|error| format!("could not write {}: {error}", path.display()))?;
                    format!("wrote {}\n", path.display())
                } else {
                    reference
                }
            }
        },
    };
    Ok(Some(output))
}

/// Render Markdown command documentation directly from the Clap model.
pub fn cli_reference_markdown() -> String {
    let command = Cli::command();
    let mut markdown = String::from(
        "<!-- This file is generated by `ehrcalc docs cli`. Do not edit it manually. -->\n\n",
    );
    markdown.push_str("# Ehrcalc CLI Reference\n\n");
    markdown.push_str(
        "This reference is generated from the `clap` command model. It is the source-of-truth command reference; the README provides the higher-level project overview.\n\n",
    );
    markdown.push_str("## Top-Level Usage\n\n```text\n");
    let mut root = command.clone();
    markdown.push_str(&root.render_long_help().to_string());
    markdown.push_str("\n```\n\n## Commands\n\n");

    for subcommand in command.get_subcommands() {
        let mut detailed = subcommand.clone();
        markdown.push_str(&format!("### `ehrcalc {}`\n\n", subcommand.get_name()));
        markdown.push_str("```text\n");
        markdown.push_str(&detailed.render_long_help().to_string());
        markdown.push_str("\n```\n\n");
    }
    markdown
}

fn parse_polynomial_args(args: &PolynomialArgs) -> Result<EhrhartPolynomial, String> {
    EhrhartPolynomial::new(args.dimension, parse_rational_list(&args.coefficients)?)
}

fn parse_order_input(args: &OrderArgs) -> Result<OrderInput, String> {
    match (&args.chain, &args.antichain, &args.fence, &args.shape, &args.vertices, &args.covers) {
        (Some(elements), None, None, None, None, None) => Ok(OrderInput::Chain { elements: *elements }),
        (None, Some(elements), None, None, None, None) => Ok(OrderInput::Antichain { elements: *elements }),
        (None, None, Some(elements), None, None, None) => Ok(OrderInput::Fence { elements: *elements }),
        (None, None, None, Some(shape), None, None) => Ok(OrderInput::Shape { lambda: parse_u32_list(shape, "shape")? }),
        (None, None, None, None, Some(vertices), Some(covers)) => Ok(OrderInput::Covers {
            vertices: *vertices,
            covers: parse_covers(covers, *vertices)?,
        }),
        _ => Err("select exactly one order-polytope constructor: --chain, --antichain, --fence, --shape, or --vertices with --covers".to_string()),
    }
}

fn parse_u32_list(text: &str, name: &str) -> Result<Vec<u32>, String> {
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    text.split(',')
        .map(|item| item.trim().parse::<u32>().map_err(|_| format!("invalid {name} entry `{}`", item.trim())))
        .collect()
}

fn parse_optional_u32_list(text: Option<&str>, name: &str) -> Result<Option<Vec<u32>>, String> {
    text.filter(|value| !value.trim().is_empty())
        .map(|value| parse_u32_list(value, name))
        .transpose()
}

fn parse_usize_list(text: &str, name: &str) -> Result<Vec<usize>, String> {
    if text.trim().is_empty() {
        return Err(format!("{name} must not be empty"));
    }
    text.split(',')
        .map(|item| item.trim().parse::<usize>().map_err(|_| format!("invalid {name} entry `{}`", item.trim())))
        .collect()
}

fn parse_i64_list(text: &str, name: &str) -> Result<Vec<i64>, String> {
    if text.trim().is_empty() {
        return Err(format!("{name} must not be empty"));
    }
    text.split(',')
        .map(|item| item.trim().parse::<i64>().map_err(|_| format!("invalid {name} entry `{}`", item.trim())))
        .collect()
}

fn parse_edges(text: &str) -> Result<Vec<(usize, usize)>, String> {
    if text.trim().is_empty() {
        return Err("edge list must not be empty".to_string());
    }
    text.split(',')
        .map(|edge| {
            let (tail, head) = edge
                .trim()
                .split_once("->")
                .ok_or_else(|| format!("invalid edge `{}`; use tail->head", edge.trim()))?;
            Ok((
                tail.trim().parse::<usize>().map_err(|_| format!("invalid edge tail `{}`", tail.trim()))?,
                head.trim().parse::<usize>().map_err(|_| format!("invalid edge head `{}`", head.trim()))?,
            ))
        })
        .collect()
}

fn parse_covers(text: &str, vertices: usize) -> Result<Vec<(usize, usize)>, String> {
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    text.split(',')
        .map(|cover| {
            let (lower, upper) = cover
                .trim()
                .split_once('<')
                .ok_or_else(|| format!("invalid cover `{}`; use lower<upper", cover.trim()))?;
            let lower = lower.trim().parse::<usize>().map_err(|_| format!("invalid cover vertex `{}`", lower.trim()))?;
            let upper = upper.trim().parse::<usize>().map_err(|_| format!("invalid cover vertex `{}`", upper.trim()))?;
            if lower >= vertices || upper >= vertices || lower == upper {
                return Err(format!("cover {lower}<{upper} is outside the vertex set 0..{}", vertices.saturating_sub(1)));
            }
            Ok((lower, upper))
        })
        .collect()
}

fn parse_points(text: &str) -> Result<Vec<(i64, BigRational)>, String> {
    if text.trim().is_empty() {
        return Err("point list must not be empty".to_string());
    }
    text.split(',').map(parse_one_point).collect()
}

fn parse_one_point(text: &str) -> Result<(i64, BigRational), String> {
    let (point, value) = text
        .trim()
        .split_once(':')
        .ok_or_else(|| format!("invalid point `{}`; use dilation:value", text.trim()))?;
    Ok((
        point.trim().parse::<i64>().map_err(|_| format!("invalid dilation `{}`", point.trim()))?,
        parse_rational(value)?,
    ))
}

fn validate_flag_lengths(weight: &[u32], upper: Option<&[u32]>, lower: Option<&[u32]>) -> Result<(), String> {
    for (name, flags) in [("upper flags", upper), ("lower flags", lower)] {
        if let Some(flags) = flags {
            if flags.len() != weight.len() {
                return Err(format!("{name} has length {}, but weight has length {}", flags.len(), weight.len()));
            }
        }
    }
    Ok(())
}

fn render_value(label: &str, point: i64, value: &BigRational, format: OutputFormat) -> String {
    match format {
        OutputFormat::Text => format!("{label}({point}): {}\n", format_rational(value)),
        OutputFormat::Json => serde_json::to_string_pretty(&json!({ "point": point, label: format_rational(value) })).expect("JSON value"),
        OutputFormat::Latex => format!("{}({}) = {}\n", label, point, format_rational(value)),
    }
}

fn render_verification(point: i64, expected: &BigRational, actual: &BigRational, format: OutputFormat) -> String {
    let passed = expected == actual;
    match format {
        OutputFormat::Text => format!("point: {point}\nexpected: {}\nactual: {}\npassed: {passed}\n", format_rational(expected), format_rational(actual)),
        OutputFormat::Json => serde_json::to_string_pretty(&json!({ "point": point, "expected": format_rational(expected), "actual": format_rational(actual), "passed": passed })).expect("JSON verification"),
        OutputFormat::Latex => format!("P({point}) = {}\\quad\\text{{expected }}{}\\quad\\text{{passed: {passed}}}\n", format_rational(actual), format_rational(expected)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cover_and_edge_syntax() {
        assert_eq!(parse_covers("0<1,0<2", 3).expect("covers"), vec![(0, 1), (0, 2)]);
        assert_eq!(parse_edges("0->1,1->2").expect("edges"), vec![(0, 1), (1, 2)]);
        assert!(parse_covers("0<3", 3).is_err());
        assert!(parse_edges("0-1").is_err());
    }

    #[test]
    fn invokes_transform_and_family_commands() {
        let output = run(Cli {
            command: Some(Command::Interpolate(InterpolateArgs {
                dimension: 2,
                points: "0:1,1:3,2:7".to_string(),
                output: OutputArgs {
                    format: OutputFormat::Json,
                },
            })),
        })
        .expect("interpolate command")
        .expect("output");
        assert!(output.contains("power_coefficients"));

        let output = run(Cli {
            command: Some(Command::Kostka(KostkaArgs {
                lambda: "2,1".to_string(),
                mu: String::new(),
                weight: "1,1,1".to_string(),
                upper_flags: None,
                lower_flags: None,
                max_states: None,
                output: OutputArgs {
                    format: OutputFormat::Text,
                },
            })),
        })
        .expect("Kostka command")
        .expect("output");
        assert_eq!(output, "kostka: 2\n");
    }

    #[test]
    fn dispatches_all_primary_ehrhart_families() {
        let gt = run(Cli {
            command: Some(Command::Gt(GtArgs {
                lambda: "2,1".to_string(),
                mu: String::new(),
                weight: "1,1,1".to_string(),
                upper_flags: None,
                lower_flags: None,
                max_states: None,
                positive_only: false,
                output: OutputArgs { format: OutputFormat::Json },
            })),
        })
        .expect("GT dispatch")
        .expect("GT output");
        assert!(gt.contains("ehrhart_power"));

        let lr = run(Cli {
            command: Some(Command::Lr(LrArgs {
                lambda: "2,1".to_string(),
                mu: "1".to_string(),
                nu: "2".to_string(),
                max_states: None,
                output: OutputArgs { format: OutputFormat::Json },
            })),
        })
        .expect("LR dispatch")
        .expect("LR output");
        assert!(lr.contains("littlewood_richardson"));

        let key = run(Cli {
            command: Some(Command::Key(KeyArgs {
                lambda: "1".to_string(),
                sigma: "1".to_string(),
                max_degree: None,
                output: OutputArgs { format: OutputFormat::Json },
            })),
        })
        .expect("key dispatch")
        .expect("key output");
        assert!(key.contains("hstar"));

        let order = run(Cli {
            command: Some(Command::Order(OrderArgs {
                chain: None,
                antichain: None,
                fence: None,
                shape: None,
                vertices: Some(2),
                covers: Some("0<1".to_string()),
                output: OutputArgs { format: OutputFormat::Json },
            })),
        })
        .expect("order dispatch")
        .expect("order output");
        assert!(order.contains("dimension"));

        let flow = run(Cli {
            command: Some(Command::Flow(FlowArgs {
                vertices: 2,
                edges: "0->1,0->1".to_string(),
                netflow: "1,-1".to_string(),
                max_states: None,
                positive_only: false,
                output: OutputArgs { format: OutputFormat::Json },
            })),
        })
        .expect("flow dispatch")
        .expect("flow output");
        assert!(flow.contains("hstar"));
    }

    #[test]
    fn transform_and_verification_dispatches_report_exact_data() {
        let hstar = run(Cli {
            command: Some(Command::Hstar(HstarArgs {
                dimension: 1,
                coefficients: None,
                hstar: Some("1,0".to_string()),
                output: OutputArgs { format: OutputFormat::Json },
            })),
        })
        .expect("hstar dispatch")
        .expect("hstar output");
        assert!(hstar.contains("ehrhart_power"));

        let sum = run(Cli {
            command: Some(Command::Sum(PolynomialArgs {
                dimension: 1,
                coefficients: "1,1".to_string(),
                output: OutputArgs { format: OutputFormat::Json },
            })),
        })
        .expect("sum dispatch")
        .expect("sum output");
        assert!(sum.contains("power_coefficients"));

        let verification = run(Cli {
            command: Some(Command::Verify(VerifyArgs {
                dimension: 1,
                coefficients: "1,1".to_string(),
                point: "3:4".to_string(),
                output: OutputArgs { format: OutputFormat::Json },
            })),
        })
        .expect("verification dispatch")
        .expect("verification output");
        assert!(verification.contains("\"passed\": true"));
    }

    #[test]
    fn invalid_flag_lengths_are_rejected_before_counting() {
        let error = run(Cli {
            command: Some(Command::Kostka(KostkaArgs {
                lambda: "2,1".to_string(),
                mu: String::new(),
                weight: "1,1,1".to_string(),
                upper_flags: Some("1,2".to_string()),
                lower_flags: None,
                max_states: None,
                output: OutputArgs { format: OutputFormat::Text },
            })),
        })
        .expect_err("invalid flags must fail before calculation");
        assert!(error.contains("length"));
    }

    #[test]
    fn generated_reference_contains_detailed_help() {
        let reference = cli_reference_markdown();
        assert!(reference.contains("--upper-flags"));
        assert!(reference.contains("ehrcalc docs cli"));
        assert_eq!(reference, include_str!("../docs/cli.md"));
    }
}
