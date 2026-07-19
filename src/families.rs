//! Adapters for exact counting families already implemented in the workspace.

use crate::exact::{EhrhartData, EhrhartPolynomial, ExactResult};
use ehrcalc_foundations::key_polynomial::key_ehrhart_polynomial;
use ehrcalc_foundations::poset::Poset;
use ehrcalc_foundations::Partition as CorePartition;
use ehrcalc_kostka_engine::ehrhart::compute_ehrhart;
use ehrcalc_kostka_engine::flow::FlowPolytope;
use ehrcalc_kostka_engine::gt_dim::gt_polytope_dim_full;
use ehrcalc_kostka_engine::kostka_dp::{flagged_skew_kostka, skew_kostka};
use ehrcalc_kostka_engine::lr::lr_dp;
use ehrcalc_kostka_engine::Partition as KostkaPartition;
use num_bigint::{BigInt, BigUint, ToBigInt};

/// Input for a GT/Kostka Ehrhart calculation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GtInput {
    pub lambda: Vec<u32>,
    pub mu: Vec<u32>,
    pub weight: Vec<u32>,
    pub upper_flags: Option<Vec<u32>>,
    pub lower_flags: Option<Vec<u32>>,
    pub max_states: Option<usize>,
    pub use_reciprocity: bool,
}

/// Input for a direct Kostka or flagged Kostka count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KostkaInput {
    pub lambda: Vec<u32>,
    pub mu: Vec<u32>,
    pub weight: Vec<u32>,
    pub upper_flags: Option<Vec<u32>>,
    pub lower_flags: Option<Vec<u32>>,
    pub max_states: Option<usize>,
}

/// Input for an LR coefficient calculation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LrInput {
    pub lambda: Vec<u32>,
    pub mu: Vec<u32>,
    pub nu: Vec<u32>,
    pub max_states: Option<usize>,
}

/// Supported constructors for order polytopes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderInput {
    Covers {
        vertices: usize,
        covers: Vec<(usize, usize)>,
    },
    Chain {
        elements: usize,
    },
    Antichain {
        elements: usize,
    },
    Fence {
        elements: usize,
    },
    Shape {
        lambda: Vec<u32>,
    },
}

/// Input for an acyclic flow-polytope calculation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowInput {
    pub vertices: usize,
    pub edges: Vec<(usize, usize)>,
    pub netflow: Vec<i64>,
    pub max_states: Option<usize>,
    pub use_reciprocity: bool,
}

/// Input for the key-polynomial Kogan-face evaluator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyInput {
    pub lambda: Vec<u32>,
    /// One-based permutation notation, for example `[2, 1, 3]`.
    pub sigma: Vec<usize>,
    /// A proven degree bound. `None` uses the full GT upper bound.
    pub max_degree: Option<usize>,
}

/// Compute exact GT Ehrhart and h* data.
pub fn gt_ehrhart(input: &GtInput) -> ExactResult<EhrhartData> {
    let lambda = KostkaPartition::new(input.lambda.clone());
    let mu = KostkaPartition::new(input.mu.clone());
    let dimension = gt_polytope_dim_full(
        lambda.parts(),
        mu.parts(),
        &input.weight,
        input.upper_flags.as_deref(),
        input.lower_flags.as_deref(),
    )
    .ok_or_else(|| "the requested GT polytope is empty".to_string())?;
    let polynomial = compute_ehrhart(
        &lambda,
        &mu,
        &input.weight,
        input.upper_flags.as_deref(),
        input.lower_flags.as_deref(),
        false,
        input.max_states,
        input.use_reciprocity,
    );
    EhrhartData::new(EhrhartPolynomial::new(dimension, polynomial.coeffs)?)
}

/// Compute a straight, skew, or flagged Kostka coefficient exactly.
pub fn kostka_count(input: &KostkaInput) -> ExactResult<BigInt> {
    let lambda = KostkaPartition::new(input.lambda.clone());
    let mu = KostkaPartition::new(input.mu.clone());
    let count = if input.upper_flags.is_some() || input.lower_flags.is_some() {
        flagged_skew_kostka(
            &lambda,
            &mu,
            &input.weight,
            input.upper_flags.as_deref(),
            input.lower_flags.as_deref(),
            input.max_states,
        )
    } else {
        skew_kostka(&lambda, &mu, &input.weight, input.max_states, true)
    };
    biguint_to_bigint(count)
}

/// Compute an LR coefficient by the existing augmented GT dynamic program.
pub fn lr_count(input: &LrInput) -> ExactResult<BigInt> {
    let lambda = KostkaPartition::new(input.lambda.clone());
    let mu = KostkaPartition::new(input.mu.clone());
    let nu = KostkaPartition::new(input.nu.clone());
    biguint_to_bigint(lr_dp(&lambda, &mu, &nu, input.max_states))
}

/// Compute exact Ehrhart and h* data for an order polytope.
pub fn order_ehrhart(input: &OrderInput) -> ExactResult<EhrhartData> {
    let poset = build_poset(input);
    let polynomial = poset.order_polytope_ehrhart();
    EhrhartData::new(EhrhartPolynomial::new(poset.num_elements(), polynomial)?)
}

/// Compute exact Ehrhart and h* data for an acyclic flow polytope.
pub fn flow_ehrhart(input: &FlowInput) -> ExactResult<EhrhartData> {
    let flow = FlowPolytope::new(input.vertices, input.edges.clone(), input.netflow.clone())?;
    let dimension = flow.dimension()?;
    let polynomial = if input.use_reciprocity {
        flow.ehrhart_poly(input.max_states)?
    } else {
        flow.ehrhart_poly_positive(input.max_states)?
    };
    EhrhartData::new(EhrhartPolynomial::new(dimension, polynomial.coeffs)?)
}

/// Compute exact key-Ehrhart and h* data using the current Kogan-face engine.
pub fn key_ehrhart(input: &KeyInput) -> ExactResult<EhrhartData> {
    validate_permutation(&input.sigma)?;
    let lambda = CorePartition::new(input.lambda.clone());
    let polynomial = key_ehrhart_polynomial(&lambda, &input.sigma, input.max_degree);
    // The existing key engine exposes the proven degree of this Ehrhart
    // polynomial. A separate Kogan-face dimension routine is future work.
    EhrhartData::new(EhrhartPolynomial::new(polynomial.degree, polynomial.coeffs)?)
}

fn build_poset(input: &OrderInput) -> Poset {
    match input {
        OrderInput::Covers { vertices, covers } => Poset::new(*vertices, covers),
        OrderInput::Chain { elements } => Poset::chain(*elements),
        OrderInput::Antichain { elements } => Poset::antichain(*elements),
        OrderInput::Fence { elements } => Poset::fence(*elements),
        OrderInput::Shape { lambda } => Poset::from_shape(&CorePartition::new(lambda.clone())),
    }
}

fn validate_permutation(sigma: &[usize]) -> ExactResult<()> {
    if sigma.is_empty() {
        return Err("key permutation must not be empty".to_string());
    }
    let mut sorted = sigma.to_vec();
    sorted.sort_unstable();
    let expected = (1..=sigma.len()).collect::<Vec<_>>();
    if sorted != expected {
        return Err(format!(
            "key permutation must contain each value 1 through {} exactly once",
            sigma.len()
        ));
    }
    Ok(())
}

fn biguint_to_bigint(value: BigUint) -> ExactResult<BigInt> {
    value
        .to_bigint()
        .ok_or_else(|| "BigUint to BigInt conversion failed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::{One, Zero};

    #[test]
    fn computes_kostka_and_lr_counts() {
        let kostka = kostka_count(&KostkaInput {
            lambda: vec![2, 1],
            mu: vec![],
            weight: vec![1, 1, 1],
            upper_flags: None,
            lower_flags: None,
            max_states: None,
        })
        .expect("Kostka count");
        assert_eq!(kostka, BigInt::from(2));

        let lr = lr_count(&LrInput {
            lambda: vec![2, 1],
            mu: vec![1],
            nu: vec![2],
            max_states: None,
        })
        .expect("LR count");
        assert_eq!(lr, BigInt::one());
    }

    #[test]
    fn computes_gt_order_and_flow_ehrhart_data() {
        let gt = gt_ehrhart(&GtInput {
            lambda: vec![2, 1],
            mu: vec![],
            weight: vec![1, 1, 1],
            upper_flags: None,
            lower_flags: None,
            max_states: None,
            use_reciprocity: true,
        })
        .expect("GT Ehrhart data");
        assert_eq!(gt.ehrhart.evaluate(0), num_rational::BigRational::one());

        let order = order_ehrhart(&OrderInput::Chain { elements: 2 })
            .expect("order-polytope data");
        assert_eq!(order.hstar.coeffs(), &[BigInt::one(), BigInt::zero(), BigInt::zero()]);

        let flow = flow_ehrhart(&FlowInput {
            vertices: 2,
            edges: vec![(0, 1), (0, 1)],
            netflow: vec![1, -1],
            max_states: None,
            use_reciprocity: true,
        })
        .expect("flow-polytope data");
        assert_eq!(flow.ehrhart.evaluate(3), num_rational::BigRational::from(BigInt::from(4)));
    }

    #[test]
    fn flagged_gt_data_agrees_with_direct_flagged_count_at_one() {
        let input = GtInput {
            lambda: vec![2, 1],
            mu: vec![],
            weight: vec![1, 1, 1],
            upper_flags: Some(vec![1, 2, 2]),
            lower_flags: None,
            max_states: None,
            use_reciprocity: false,
        };
        let data = gt_ehrhart(&input).expect("flagged GT data");
        let count = kostka_count(&KostkaInput {
            lambda: input.lambda,
            mu: input.mu,
            weight: input.weight,
            upper_flags: input.upper_flags,
            lower_flags: input.lower_flags,
            max_states: input.max_states,
        })
        .expect("flagged count");
        assert_eq!(data.ehrhart.evaluate(1), num_rational::BigRational::from(count));
    }

    #[test]
    fn computes_a_small_key_family() {
        let data = key_ehrhart(&KeyInput {
            lambda: vec![1],
            sigma: vec![1],
            max_degree: None,
        })
        .expect("key Ehrhart data");
        assert_eq!(data.ehrhart.evaluate(0), num_rational::BigRational::one());
    }

    #[test]
    fn rejects_non_permutations_for_key_data() {
        let error = key_ehrhart(&KeyInput {
            lambda: vec![2, 1],
            sigma: vec![1, 1],
            max_degree: None,
        })
        .expect_err("invalid permutation must be rejected");
        assert!(error.contains("exactly once"));
    }
}
