use super::{ConstraintError, Result};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

/// Optional rational endpoints. Missing endpoints must be bounded by other constraints.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VariableBounds {
    pub lower: Option<BigRational>,
    pub upper: Option<BigRational>,
}

/// The inequality `x_left - x_right <= upper`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DifferenceConstraint {
    pub left: usize,
    pub right: usize,
    pub upper: BigRational,
}

/// A subset sum between optional endpoints; equal endpoints specify an equality.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubsetSum {
    pub variables: Vec<usize>,
    pub lower: Option<BigRational>,
    pub upper: Option<BigRational>,
}

/// Input for `ehrcalc.constraint-polytope.v1`.
///
/// Serde uses integers or `"p/q"` strings for rational right sides, never floats.
/// `bounds` is an array of `[lower, upper]` pairs (null means absent),
/// `differences` contains `[i,j,c]`, and `sums` contains
/// `{"vars":[i,...],"lower":l,"upper":u}`. `schema` is optional on input.
/// Missing bounds means no explicit bounds; otherwise exactly one pair is required
/// per variable. Names may be omitted, or must be distinct and have length `variables`.
/// Duplicate members of a subset are rejected, rather than interpreted as coefficients.
/// Text input supports `vars`, `bound`, `le`, and `sum`, with `#` comments.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConstraintPolytopeInput {
    pub variables: usize,
    pub names: Vec<String>,
    pub bounds: Vec<VariableBounds>,
    pub differences: Vec<DifferenceConstraint>,
    pub sums: Vec<SubsetSum>,
}

fn invalid(message: impl Into<String>) -> ConstraintError {
    ConstraintError::InvalidInput(message.into())
}

pub(super) fn parse_rational(text: &str) -> Result<BigRational> {
    let (numerator, denominator) = text.split_once('/').unwrap_or((text, "1"));
    let numerator =
        BigInt::from_str(numerator).map_err(|_| invalid("invalid rational numerator"))?;
    let denominator =
        BigInt::from_str(denominator).map_err(|_| invalid("invalid rational denominator"))?;
    if denominator.is_zero() {
        return Err(invalid("zero rational denominator"));
    }
    Ok(BigRational::new(numerator, denominator))
}

fn rational(value: &Value) -> Result<BigRational> {
    match value {
        Value::String(text) => parse_rational(text),
        Value::Number(number) if number.is_i64() || number.is_u64() => {
            parse_rational(&number.to_string())
        }
        _ => Err(invalid("right sides must be integers or rational strings")),
    }
}

fn optional(value: Option<&Value>) -> Result<Option<BigRational>> {
    value.filter(|v| !v.is_null()).map(rational).transpose()
}

fn index(value: &Value) -> Result<usize> {
    value
        .as_u64()
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| invalid("expected a nonnegative variable index"))
}

fn array(value: &Value) -> Result<&Vec<Value>> {
    value.as_array().ok_or_else(|| invalid("expected an array"))
}

fn tighten_lower(current: &mut Option<BigRational>, other: &Option<BigRational>) {
    if let Some(value) = other {
        if current.as_ref().is_none_or(|old| value > old) {
            *current = Some(value.clone());
        }
    }
}

fn tighten_upper(current: &mut Option<BigRational>, other: &Option<BigRational>) {
    if let Some(value) = other {
        if current.as_ref().is_none_or(|old| value < old) {
            *current = Some(value.clone());
        }
    }
}

impl ConstraintPolytopeInput {
    /// Parse and validate JSON using the same contract as the serde implementation.
    pub fn from_json(text: &str) -> Result<Self> {
        serde_json::from_str(text).map_err(|error| invalid(error.to_string()))
    }

    /// Merge repeated constraints and singleton sums, retaining the tightest endpoints.
    /// A contradiction between different valid constraints is retained as an empty system.
    pub fn normalized(&self) -> Result<Self> {
        let n = self.variables;
        if !self.names.is_empty()
            && (self.names.len() != n || self.names.iter().collect::<BTreeSet<_>>().len() != n)
        {
            return Err(invalid("names must be distinct and have length variables"));
        }
        if !self.bounds.is_empty() && self.bounds.len() != n {
            return Err(invalid("bounds must have length variables"));
        }
        let mut result = self.clone();
        if result.bounds.is_empty() {
            result.bounds.resize_with(n, VariableBounds::default);
        }
        let ordered = |lo: &Option<BigRational>, hi: &Option<BigRational>| match (lo, hi) {
            (Some(a), Some(b)) => a <= b,
            _ => true,
        };
        if self.bounds.iter().any(|b| !ordered(&b.lower, &b.upper)) {
            return Err(invalid("bound lower endpoint exceeds upper endpoint"));
        }
        let mut differences = BTreeMap::<(usize, usize), BigRational>::new();
        for difference in &self.differences {
            if difference.left >= n || difference.right >= n {
                return Err(invalid("difference variable index out of range"));
            }
            differences
                .entry((difference.left, difference.right))
                .and_modify(|c| *c = c.clone().min(difference.upper.clone()))
                .or_insert_with(|| difference.upper.clone());
        }
        result.differences = differences
            .into_iter()
            .map(|((left, right), upper)| DifferenceConstraint { left, right, upper })
            .collect();
        let mut sums = BTreeMap::<Vec<usize>, VariableBounds>::new();
        for sum in &self.sums {
            let mut vars = sum.variables.clone();
            vars.sort_unstable();
            if vars.is_empty()
                || vars.iter().any(|&v| v >= n)
                || vars.windows(2).any(|w| w[0] == w[1])
            {
                return Err(invalid(
                    "sum must contain distinct, in-range variable indices",
                ));
            }
            if !ordered(&sum.lower, &sum.upper) || (sum.lower.is_none() && sum.upper.is_none()) {
                return Err(invalid("sum endpoints are missing or reversed"));
            }
            let target = if vars.len() == 1 {
                &mut result.bounds[vars[0]]
            } else {
                sums.entry(vars).or_default()
            };
            tighten_lower(&mut target.lower, &sum.lower);
            tighten_upper(&mut target.upper, &sum.upper);
        }
        result.sums = sums
            .into_iter()
            .map(|(variables, bounds)| SubsetSum {
                variables,
                lower: bounds.lower,
                upper: bounds.upper,
            })
            .collect();
        if result.bounds.iter().any(|b| !ordered(&b.lower, &b.upper))
            || result.sums.iter().any(|s| !ordered(&s.lower, &s.upper))
            || result
                .differences
                .iter()
                .any(|d| d.left == d.right && d.upper < BigRational::zero())
        {
            // A valid pair of incompatible constraints denotes an empty system.
            // Keep a valid, idempotent model instead of reversed merged endpoints.
            result.bounds = vec![
                VariableBounds {
                    lower: Some(BigRational::zero()),
                    upper: Some(BigRational::zero()),
                };
                n
            ];
            result.differences = vec![DifferenceConstraint {
                left: 0,
                right: 0,
                upper: -BigRational::one(),
            }];
            result.sums.clear();
        }
        Ok(result)
    }

    fn from_value(value: Value) -> Result<Self> {
        let object = value
            .as_object()
            .ok_or_else(|| invalid("expected an input object"))?;
        for key in object.keys() {
            if ![
                "schema",
                "variables",
                "names",
                "bounds",
                "differences",
                "sums",
            ]
            .contains(&key.as_str())
            {
                return Err(invalid(format!("unknown field {key}")));
            }
        }
        if object
            .get("schema")
            .is_some_and(|s| s != "ehrcalc.constraint-polytope.v1")
        {
            return Err(invalid("unsupported constraint-polytope schema"));
        }
        let mut input = Self {
            variables: index(
                object
                    .get("variables")
                    .ok_or_else(|| invalid("missing variables"))?,
            )?,
            ..Self::default()
        };
        if let Some(names) = object.get("names") {
            input.names = array(names)?
                .iter()
                .map(|n| {
                    n.as_str()
                        .map(str::to_owned)
                        .ok_or_else(|| invalid("name must be a string"))
                })
                .collect::<Result<_>>()?;
        }
        if let Some(bounds) = object.get("bounds") {
            for bound in array(bounds)? {
                let b = array(bound)?;
                if b.len() != 2 {
                    return Err(invalid("a bound needs two endpoints"));
                }
                input.bounds.push(VariableBounds {
                    lower: optional(b.first())?,
                    upper: optional(b.get(1))?,
                });
            }
        }
        if let Some(differences) = object.get("differences") {
            for difference in array(differences)? {
                let d = array(difference)?;
                if d.len() != 3 {
                    return Err(invalid("a difference needs i,j,c"));
                }
                input.differences.push(DifferenceConstraint {
                    left: index(&d[0])?,
                    right: index(&d[1])?,
                    upper: rational(&d[2])?,
                });
            }
        }
        if let Some(sums) = object.get("sums") {
            for sum in array(sums)? {
                let s = sum
                    .as_object()
                    .ok_or_else(|| invalid("expected a sum object"))?;
                if s.keys()
                    .any(|k| !["vars", "lower", "upper"].contains(&k.as_str()))
                {
                    return Err(invalid("unknown sum field"));
                }
                input.sums.push(SubsetSum {
                    variables: array(s.get("vars").ok_or_else(|| invalid("missing vars"))?)?
                        .iter()
                        .map(index)
                        .collect::<Result<_>>()?,
                    lower: optional(s.get("lower"))?,
                    upper: optional(s.get("upper"))?,
                });
            }
        }
        input.normalized()?;
        Ok(input)
    }
}

impl FromStr for ConstraintPolytopeInput {
    type Err = ConstraintError;

    fn from_str(text: &str) -> Result<Self> {
        let mut input = None;
        for (line_index, line) in text.lines().enumerate() {
            let words = line
                .split('#')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .collect::<Vec<_>>();
            if words.is_empty() {
                continue;
            }
            let bad = || {
                invalid(format!(
                    "invalid constraint syntax on line {}",
                    line_index + 1
                ))
            };
            let idx = |word: &str| word.parse::<usize>().map_err(|_| bad());
            if words[0] == "vars" && words.len() == 2 && input.is_none() {
                let n = idx(words[1])?;
                input = Some(Self {
                    variables: n,
                    bounds: vec![VariableBounds::default(); n],
                    ..Self::default()
                });
                continue;
            }
            let system = input.as_mut().ok_or_else(bad)?;
            match words[0] {
                "bound" if words.len() == 4 => {
                    let v = idx(words[1])?;
                    let lo = parse_rational(words[2])?;
                    let hi = parse_rational(words[3])?;
                    if lo > hi {
                        return Err(bad());
                    }
                    let bounds = system.bounds.get_mut(v).ok_or_else(bad)?;
                    tighten_lower(&mut bounds.lower, &Some(lo));
                    tighten_upper(&mut bounds.upper, &Some(hi));
                }
                "le" if words.len() == 3 || words.len() == 4 => {
                    system.differences.push(DifferenceConstraint {
                        left: idx(words[1])?,
                        right: idx(words[2])?,
                        upper: if words.len() == 4 {
                            parse_rational(words[3])?
                        } else {
                            BigRational::zero()
                        },
                    });
                }
                "sum" if words.len() >= 4 => {
                    let op = words[words.len() - 2];
                    let rhs = parse_rational(words[words.len() - 1])?;
                    if !["=", "<=", ">="].contains(&op) {
                        return Err(bad());
                    }
                    system.sums.push(SubsetSum {
                        variables: words[1..words.len() - 2]
                            .iter()
                            .map(|w| idx(w))
                            .collect::<Result<_>>()?,
                        lower: (op != "<=").then(|| rhs.clone()),
                        upper: (op != ">=").then_some(rhs),
                    });
                }
                _ => return Err(bad()),
            }
        }
        let input = input.ok_or_else(|| invalid("missing vars declaration"))?;
        input.normalized()?;
        Ok(input)
    }
}

impl Serialize for ConstraintPolytopeInput {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        fn q(value: &BigRational) -> Value {
            // Strings preserve arbitrary-size integers as well as fractions.
            Value::String(if value.denom().is_one() {
                value.numer().to_string()
            } else {
                format!("{}/{}", value.numer(), value.denom())
            })
        }
        json!({
            "schema": "ehrcalc.constraint-polytope.v1", "variables": self.variables,
            "names": self.names,
            "bounds": self.bounds.iter().map(|b| [b.lower.as_ref().map(q), b.upper.as_ref().map(q)])
                .collect::<Vec<_>>(),
            "differences": self.differences.iter().map(|d| json!([d.left, d.right, q(&d.upper)]))
                .collect::<Vec<_>>(),
            "sums": self.sums.iter().map(|s| json!({"vars": s.variables,
                "lower": s.lower.as_ref().map(q), "upper": s.upper.as_ref().map(q)}))
                .collect::<Vec<_>>()
        })
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ConstraintPolytopeInput {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_value(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
