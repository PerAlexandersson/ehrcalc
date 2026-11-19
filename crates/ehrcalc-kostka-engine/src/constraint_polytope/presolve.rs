use super::{ConstraintError, ConstraintPolytopeInput, Result, VariableBounds};
use crate::affine_hull::{exact_rank, AffineHull, AffineHullData, RationalPolyhedron};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use std::collections::BTreeMap;

/// One normalized inequality `sum sign*x_variable <= rhs`.
///
/// Rows occur in this order: lower then upper explicit bounds by variable,
/// differences in lexicographic order, lower then upper subset sums in
/// lexicographic subset order. Fixed coordinates have already been substituted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstraintRow {
    pub terms: Vec<(usize, i8)>,
    pub rhs: BigRational,
}

/// Validated, presolved polytope, reusable across dilations and orders.
///
/// `free_variables` maps reduced indices to original input indices. The hull's
/// mask indexes `rows`, including constant rows left after substitution. Empty
/// systems have no hull or components. A nonempty zero-dimensional system does.
#[derive(Clone, Debug)]
pub struct PreparedConstraintPolytope {
    pub(super) input: ConstraintPolytopeInput,
    pub(super) free_variables: Vec<usize>,
    pub(super) fixed_values: Vec<Option<BigRational>>,
    pub(super) bounds: Vec<VariableBounds>,
    pub(super) rows: Vec<ConstraintRow>,
    pub(super) hull: Option<AffineHullData>,
    pub(super) components: Vec<Vec<usize>>,
    pub(super) component_dimensions: Vec<usize>,
}

pub(super) fn polyhedron(variables: usize, rows: &[ConstraintRow]) -> RationalPolyhedron {
    let mut polyhedron = RationalPolyhedron::new(variables);
    for row in rows {
        polyhedron.add_inequality(
            row.terms
                .iter()
                .map(|&(v, sign)| (v, BigRational::from_integer(sign.into()))),
            row.rhs.clone(),
        );
    }
    polyhedron
}

fn rows(input: &ConstraintPolytopeInput) -> Vec<ConstraintRow> {
    let mut rows = Vec::new();
    for (v, bound) in input.bounds.iter().enumerate() {
        if let Some(lower) = &bound.lower {
            rows.push(ConstraintRow {
                terms: vec![(v, -1)],
                rhs: -lower,
            });
        }
        if let Some(upper) = &bound.upper {
            rows.push(ConstraintRow {
                terms: vec![(v, 1)],
                rhs: upper.clone(),
            });
        }
    }
    for difference in &input.differences {
        let mut terms = vec![(difference.left, 1), (difference.right, -1)];
        terms.sort_unstable();
        if difference.left == difference.right {
            terms.clear();
        }
        rows.push(ConstraintRow {
            terms,
            rhs: difference.upper.clone(),
        });
    }
    for sum in &input.sums {
        if let Some(lower) = &sum.lower {
            rows.push(ConstraintRow {
                terms: sum.variables.iter().map(|&v| (v, -1)).collect(),
                rhs: -lower,
            });
        }
        if let Some(upper) = &sum.upper {
            rows.push(ConstraintRow {
                terms: sum.variables.iter().map(|&v| (v, 1)).collect(),
                rhs: upper.clone(),
            });
        }
    }
    rows
}

fn update_lower(bound: &mut VariableBounds, value: BigRational) -> bool {
    if bound.lower.as_ref().is_none_or(|old| &value > old) {
        bound.lower = Some(value);
        true
    } else {
        false
    }
}

fn update_upper(bound: &mut VariableBounds, value: BigRational) -> bool {
    if bound.upper.as_ref().is_none_or(|old| &value < old) {
        bound.upper = Some(value);
        true
    } else {
        false
    }
}

fn contradictory(bounds: &[VariableBounds]) -> bool {
    bounds
        .iter()
        .any(|b| matches!((&b.lower, &b.upper), (Some(a), Some(z)) if a > z))
}

fn propagate(bounds: &mut [VariableBounds], rows: &[ConstraintRow]) {
    // A cap is necessary: rational bound consistency can converge only in the limit.
    // Every intermediate endpoint is sound; only the exact LP decides dimension.
    for _ in 0..bounds.len().saturating_mul(8).saturating_add(32) {
        let mut changed = false;
        for row in rows {
            for &(v, sign) in &row.terms {
                let minimum = row.terms.iter().filter(|&&(w, _)| w != v).try_fold(
                    BigRational::zero(),
                    |total, &(w, s)| {
                        let b = if s > 0 {
                            &bounds[w].lower
                        } else {
                            &bounds[w].upper
                        };
                        b.as_ref().map(|b| total + b * BigInt::from(s))
                    },
                );
                if let Some(minimum) = minimum {
                    let endpoint = &row.rhs - minimum;
                    changed |= if sign > 0 {
                        update_upper(&mut bounds[v], endpoint)
                    } else {
                        update_lower(&mut bounds[v], -endpoint)
                    };
                }
            }
        }
        if !changed || contradictory(bounds) {
            break;
        }
    }
}

fn complete_bounds(
    input: &ConstraintPolytopeInput,
    bounds: &mut [VariableBounds],
    rows: &[ConstraintRow],
) -> Result<()> {
    // Interval propagation is incomplete even for bounded systems. Certify missing
    // directions using the recession cone, then find a finite enclosing integer
    // endpoint by exponential, certificate-checked feasibility searches.
    for (v, bound) in bounds.iter_mut().enumerate() {
        for sign in [-1_i8, 1] {
            let missing = if sign > 0 {
                bound.upper.is_none()
            } else {
                bound.lower.is_none()
            };
            if !missing {
                continue;
            }
            let mut recession = RationalPolyhedron::new(input.variables);
            for row in rows {
                recession.add_inequality(
                    row.terms
                        .iter()
                        .map(|&(w, s)| (w, BigRational::from_integer(s.into()))),
                    BigRational::zero(),
                );
            }
            recession.add_equality(
                [(v, BigRational::one())],
                BigRational::from_integer(sign.into()),
            );
            if matches!(recession.affine_hull(), AffineHull::Nonempty(_)) {
                let name = input
                    .names
                    .get(v)
                    .cloned()
                    .unwrap_or_else(|| format!("x{v}"));
                return Err(ConstraintError::Unbounded { variable: v, name });
            }
            let mut endpoint = BigInt::one();
            loop {
                let mut candidate = polyhedron(input.variables, rows);
                candidate.add_inequality(
                    [(v, BigRational::from_integer((-sign).into()))],
                    BigRational::from_integer(-&endpoint),
                );
                if matches!(candidate.affine_hull(), AffineHull::Empty) {
                    break;
                }
                endpoint *= 2;
            }
            if sign > 0 {
                bound.upper = Some(BigRational::from_integer(endpoint));
            } else {
                bound.lower = Some(BigRational::from_integer(-endpoint));
            }
        }
    }
    Ok(())
}

impl PreparedConstraintPolytope {
    /// Normalize, propagate sound bounds, substitute fixed variables, certify the
    /// affine hull, and decompose the remaining constraint hypergraph.
    pub fn new(input: &ConstraintPolytopeInput) -> Result<Self> {
        let input = input.normalized()?;
        let original_rows = rows(&input);
        let mut bounds = input.bounds.clone();
        propagate(&mut bounds, &original_rows);
        let mut result = Self {
            fixed_values: vec![None; input.variables],
            input,
            free_variables: Vec::new(),
            bounds: Vec::new(),
            rows: Vec::new(),
            hull: None,
            components: Vec::new(),
            component_dimensions: Vec::new(),
        };
        if contradictory(&bounds)
            || original_rows
                .iter()
                .any(|r| r.terms.is_empty() && r.rhs < BigRational::zero())
        {
            return Ok(result);
        }
        if bounds
            .iter()
            .any(|b| b.lower.is_none() || b.upper.is_none())
        {
            // Check feasibility first so an empty unbounded presentation returns empty.
            if matches!(
                polyhedron(result.input.variables, &original_rows).affine_hull(),
                AffineHull::Empty
            ) {
                return Ok(result);
            }
            complete_bounds(&result.input, &mut bounds, &original_rows)?;
            propagate(&mut bounds, &original_rows);
        }
        let mut reduced_index = vec![0; result.input.variables];
        for (v, bound) in bounds.into_iter().enumerate() {
            if bound.lower == bound.upper {
                result.fixed_values[v] = bound.lower;
            } else {
                reduced_index[v] = result.free_variables.len();
                result.free_variables.push(v);
                result.bounds.push(bound);
            }
        }
        for row in original_rows {
            let mut reduced = ConstraintRow {
                terms: Vec::new(),
                rhs: row.rhs,
            };
            for (v, sign) in row.terms {
                if let Some(value) = &result.fixed_values[v] {
                    reduced.rhs -= value * BigInt::from(sign);
                } else {
                    reduced.terms.push((reduced_index[v], sign));
                }
            }
            reduced.terms.sort_unstable();
            result.rows.push(reduced);
        }
        let n = result.free_variables.len();
        let AffineHull::Nonempty(hull) = polyhedron(n, &result.rows).affine_hull() else {
            return Ok(result);
        };
        let mut labels = (0..n).collect::<Vec<_>>();
        for row in &result.rows {
            if let Some(&(first, _)) = row.terms.first() {
                for &(v, _) in &row.terms {
                    let from = labels[v];
                    let to = labels[first];
                    for label in &mut labels {
                        if *label == from {
                            *label = to;
                        }
                    }
                }
            }
        }
        let mut components = BTreeMap::<usize, Vec<usize>>::new();
        for (v, label) in labels.into_iter().enumerate() {
            components.entry(label).or_default().push(v);
        }
        result.components = components.into_values().collect();
        for component in &result.components {
            let equations = result
                .rows
                .iter()
                .zip(&hull.implicit_equalities)
                .filter(|(row, implicit)| {
                    **implicit
                        && row
                            .terms
                            .first()
                            .is_some_and(|(v, _)| component.contains(v))
                })
                .map(|(row, _)| {
                    component
                        .iter()
                        .map(|v| {
                            BigRational::from_integer(
                                row.terms
                                    .iter()
                                    .find(|(w, _)| w == v)
                                    .map_or(0_i8, |(_, s)| *s)
                                    .into(),
                            )
                        })
                        .collect()
                })
                .collect();
            result
                .component_dimensions
                .push(component.len() - exact_rank(equations));
        }
        if result.component_dimensions.iter().sum::<usize>() != hull.dimension {
            return Err(ConstraintError::Arithmetic(
                "component dimensions do not add".into(),
            ));
        }
        result.hull = Some(hull);
        Ok(result)
    }

    /// `None` denotes an empty polytope, distinct from dimension zero.
    pub fn dimension(&self) -> Option<usize> {
        self.hull.as_ref().map(|hull| hull.dimension)
    }

    /// Whether exact feasibility or sound propagation proved the system empty.
    pub fn is_empty(&self) -> bool {
        self.hull.is_none()
    }

    /// Normalized source model, before fixed-variable substitution.
    pub fn input(&self) -> &ConstraintPolytopeInput {
        &self.input
    }

    /// Reduced-to-original variable map.
    pub fn free_variables(&self) -> &[usize] {
        &self.free_variables
    }

    /// Fixed rational values, indexed by original variable number.
    pub fn fixed_values(&self) -> &[Option<BigRational>] {
        &self.fixed_values
    }

    /// Sound rational bounds on reduced variables; these are pruning data.
    pub fn bounds(&self) -> &[VariableBounds] {
        &self.bounds
    }

    /// Normalized inequalities after fixed-variable substitution.
    pub fn rows(&self) -> &[ConstraintRow] {
        &self.rows
    }

    /// Certificate-checked hull and implicit-equality mask, indexed by `rows()`.
    pub fn affine_hull(&self) -> Option<&AffineHullData> {
        self.hull.as_ref()
    }

    /// Independent components, using reduced variable numbers.
    pub fn components(&self) -> &[Vec<usize>] {
        &self.components
    }

    /// Dimensions of the independent factors; their sum is checked against the hull.
    pub fn component_dimensions(&self) -> &[usize] {
        &self.component_dimensions
    }

    /// Extract one lattice-product factor without rerunning an affine-hull LP.
    /// Fixed coordinates are checked by the parent counter, not duplicated here.
    pub fn component(&self, index: usize) -> Result<Self> {
        let vars = self
            .components
            .get(index)
            .ok_or_else(|| ConstraintError::InvalidInput("component index out of range".into()))?;
        let hull = self.hull.as_ref().ok_or_else(|| {
            ConstraintError::InvalidInput("empty polytope has no components".into())
        })?;
        let mut factor = Self {
            input: self.input.clone(),
            free_variables: vars.iter().map(|&v| self.free_variables[v]).collect(),
            fixed_values: vec![None; self.input.variables],
            bounds: vars.iter().map(|&v| self.bounds[v].clone()).collect(),
            rows: Vec::new(),
            hull: None,
            components: vec![(0..vars.len()).collect()],
            component_dimensions: vec![self.component_dimensions[index]],
        };
        let mut mask = Vec::new();
        for (row, &implicit) in self.rows.iter().zip(&hull.implicit_equalities) {
            if row.terms.first().is_some_and(|(v, _)| vars.contains(v)) {
                factor.rows.push(ConstraintRow {
                    terms: row
                        .terms
                        .iter()
                        .map(|&(v, s)| {
                            (
                                vars.binary_search(&v)
                                    .expect("component contains whole row"),
                                s,
                            )
                        })
                        .collect(),
                    rhs: row.rhs.clone(),
                });
                mask.push(implicit);
            }
        }
        factor.hull = Some(AffineHullData {
            dimension: self.component_dimensions[index],
            implicit_equalities: mask,
            relative_interior_point: vars
                .iter()
                .map(|&v| hull.relative_interior_point[v].clone())
                .collect(),
        });
        Ok(factor)
    }
}
