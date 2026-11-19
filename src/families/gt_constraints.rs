//! Dilation-compatible GT chain and marked-free-entry adapters.

use super::{ConstraintPolytopeInput, DifferenceConstraint, GtInput, SubsetSum, VariableBounds};
use crate::exact::ExactResult;
use ehrcalc_kostka_engine::Partition;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

fn integer(value: u32) -> BigRational {
    BigRational::from_integer(value.into())
}

// A valid inconsistent system also represents empty fibres at dilation zero.
fn empty() -> ConstraintPolytopeInput {
    ConstraintPolytopeInput {
        variables: 1,
        bounds: vec![VariableBounds {
            lower: Some(integer(0)),
            upper: Some(integer(0)),
        }],
        differences: vec![DifferenceConstraint {
            left: 0,
            right: 0,
            upper: -integer(1),
        }],
        ..ConstraintPolytopeInput::default()
    }
}

/// Encode the full rectangular GT chain `mu = alpha^0 <= ... <= alpha^k = lambda`.
/// Variables are level-major, including both boundary levels; names are
/// `alpha_LEVEL_ROW` with zero-based indices. Flags are the existing one-based
/// per-label row intervals, clamped as in the Kostka counter. A forbidden label
/// in a row imposes equality across that step, even when neither entry is fixed.
/// Partitions are normalized exactly as in [`super::gt_ehrhart`]. Invalid flag
/// lengths are errors; incompatible shape/content/flags give an empty system.
pub fn gt_constraint_polytope(input: &GtInput) -> ExactResult<ConstraintPolytopeInput> {
    let lambda = Partition::new(input.lambda.clone());
    let mu = Partition::new(input.mu.clone());
    let rows = lambda.num_parts();
    let labels = input.weight.len();
    if input
        .upper_flags
        .as_ref()
        .is_some_and(|f| f.len() != labels)
        || input
            .lower_flags
            .as_ref()
            .is_some_and(|f| f.len() != labels)
    {
        return Err("flag lengths must match the weight length".into());
    }
    if !mu.partition_less_equal(&lambda)
        || mu.size_wide() + input.weight.iter().map(|&w| u128::from(w)).sum::<u128>()
            != lambda.size_wide()
    {
        return Ok(empty());
    }
    if rows == 0 {
        return Ok(ConstraintPolytopeInput::default());
    }
    let variables = labels
        .checked_add(1)
        .and_then(|l| l.checked_mul(rows))
        .ok_or("GT variable count overflow")?;
    let node = |level: usize, row: usize| level * rows + row;
    let mut result = ConstraintPolytopeInput {
        variables,
        names: (0..=labels)
            .flat_map(|l| (0..rows).map(move |r| format!("alpha_{l}_{r}")))
            .collect(),
        bounds: (0..=labels)
            .flat_map(|_| {
                (0..rows).map(|r| VariableBounds {
                    lower: Some(integer(mu.part(r))),
                    upper: Some(integer(lambda.part(r))),
                })
            })
            .collect(),
        ..ConstraintPolytopeInput::default()
    };
    for r in 0..rows {
        result.bounds[node(0, r)].upper = Some(integer(mu.part(r)));
        result.bounds[node(labels, r)].lower = Some(integer(lambda.part(r)));
    }
    let mut total = BigInt::from(mu.size_wide());
    for level in 0..=labels {
        if level > 0 {
            total += input.weight[level - 1];
        }
        result.sums.push(SubsetSum {
            variables: (0..rows).map(|r| node(level, r)).collect(),
            lower: Some(BigRational::from_integer(total.clone())),
            upper: Some(BigRational::from_integer(total.clone())),
        });
    }
    for step in 0..labels {
        let low = input
            .lower_flags
            .as_ref()
            .map_or(0, |f| (f[step] as usize).saturating_sub(1).min(rows));
        let high = input
            .upper_flags
            .as_ref()
            .map_or(rows, |f| (f[step] as usize).min(rows));
        for row in 0..rows {
            result.differences.push(DifferenceConstraint {
                left: node(step, row),
                right: node(step + 1, row),
                upper: integer(0),
            });
            if row > 0 {
                result.differences.push(DifferenceConstraint {
                    left: node(step + 1, row),
                    right: node(step, row - 1),
                    upper: integer(0),
                });
            }
            if input.weight[step] == 0 || row < low || row >= high {
                result.differences.push(DifferenceConstraint {
                    left: node(step + 1, row),
                    right: node(step, row),
                    upper: integer(0),
                });
            }
        }
    }
    // Only order propagation: no rounding of rational fibres and no inference
    // of dimension from lattice points. This exposes structurally frozen marks.
    for _ in 0..variables {
        let mut changed = false;
        for d in &result.differences {
            let lower = result.bounds[d.left].lower.clone().unwrap();
            let upper = result.bounds[d.right].upper.clone().unwrap();
            if lower > result.bounds[d.right].lower.clone().unwrap() {
                result.bounds[d.right].lower = Some(lower);
                changed = true;
            }
            if upper < result.bounds[d.left].upper.clone().unwrap() {
                result.bounds[d.left].upper = Some(upper);
                changed = true;
            }
        }
        if result.bounds.iter().any(|b| b.lower > b.upper) {
            return Ok(empty());
        }
        if !changed {
            break;
        }
    }
    result.normalized().map_err(|e| e.to_string())
}

/// Encode only structurally free GT entries. Frozen neighbours become marks
/// (bounds), and their values are subtracted from each level sum. Nonconstant
/// flag equalities are retained as two differences. This is an affine lattice
/// bijection at every dilation, including relative interiors; dimension is still
/// computed by the constraint engine's certified affine hull.
pub fn gt_free_constraint_polytope(input: &GtInput) -> ExactResult<ConstraintPolytopeInput> {
    let mut full = gt_constraint_polytope(input)?;
    loop {
        let reduced = remove_fixed(&full)?;
        if reduced.variables == full.variables {
            return Ok(reduced);
        }
        full = reduced;
    }
}

// Substituting one mark can leave a singleton level sum, exposing another
// fixed integral entry. Repeat until none remain (or the empty sentinel).
fn remove_fixed(full: &ConstraintPolytopeInput) -> ExactResult<ConstraintPolytopeInput> {
    let fixed = full
        .bounds
        .iter()
        .map(|b| {
            if b.lower == b.upper {
                b.lower.clone()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut indices = vec![None; full.variables];
    let mut result = ConstraintPolytopeInput::default();
    for v in 0..full.variables {
        if fixed[v].is_none() {
            indices[v] = Some(result.variables);
            result.variables += 1;
            result.bounds.push(full.bounds[v].clone());
            result.names.push(
                full.names
                    .get(v)
                    .cloned()
                    .unwrap_or_else(|| format!("x{v}")),
            );
        }
    }
    for d in &full.differences {
        match (indices[d.left], indices[d.right]) {
            (Some(left), Some(right)) => result.differences.push(DifferenceConstraint {
                left,
                right,
                upper: d.upper.clone(),
            }),
            (Some(v), None) => {
                let mark = fixed[d.right].as_ref().unwrap() + &d.upper;
                let bound = &mut result.bounds[v].upper;
                *bound = Some(bound.as_ref().unwrap().min(&mark).clone());
            }
            (None, Some(v)) => {
                let mark = fixed[d.left].as_ref().unwrap() - &d.upper;
                let bound = &mut result.bounds[v].lower;
                *bound = Some(bound.as_ref().unwrap().max(&mark).clone());
            }
            (None, None) => {
                if fixed[d.left].as_ref().unwrap() - fixed[d.right].as_ref().unwrap() > d.upper {
                    return Ok(empty());
                }
            }
        }
    }
    for s in &full.sums {
        let mark = s
            .variables
            .iter()
            .filter_map(|&v| fixed[v].as_ref())
            .sum::<BigRational>();
        let variables = s
            .variables
            .iter()
            .filter_map(|&v| indices[v])
            .collect::<Vec<_>>();
        let lower = s.lower.as_ref().map(|x| x - &mark);
        let upper = s.upper.as_ref().map(|x| x - &mark);
        if variables.is_empty() {
            if lower.as_ref().is_some_and(|x| x > &BigRational::zero())
                || upper.as_ref().is_some_and(|x| x < &BigRational::zero())
            {
                return Ok(empty());
            }
        } else {
            result.sums.push(SubsetSum {
                variables,
                lower,
                upper,
            });
        }
    }
    if result.bounds.iter().any(|b| b.lower > b.upper) {
        return Ok(empty());
    }
    result.normalized().map_err(|e| e.to_string())
}
