//! Adapters for exact counting families already implemented in the workspace.

use crate::exact::{EhrhartData, EhrhartPolynomial, ExactResult, HStarPolynomial};
use clap::ValueEnum;
use ehrcalc_foundations::key_polynomial::key_ehrhart_polynomial;
use ehrcalc_foundations::poset::{IdealDpError, Poset};
use ehrcalc_foundations::Partition as CorePartition;
use ehrcalc_kostka_engine::ehrhart::try_compute_ehrhart;
use ehrcalc_kostka_engine::flow::FlowPolytope;
use ehrcalc_kostka_engine::gt_dim::gt_polytope_dim_full;
use ehrcalc_kostka_engine::kostka_dp::{try_flagged_skew_kostka, try_skew_kostka};
use ehrcalc_kostka_engine::lr::try_lr_dp;
use ehrcalc_kostka_engine::Partition as KostkaPartition;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::Zero;

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

/// Default budget of stored `(order ideal, last element)` rows for the
/// order-ideal method.
///
/// Each row holds a descent-count vector of at most `|P|` entries, so the
/// budget bounds both the work and the peak memory of one attempt.  Two
/// million rows keep an attempt to a few seconds and a few hundred megabytes
/// for posets of up to a few dozen elements, while every long narrow poset of
/// practical interest (for example the `8 x 8` rectangle, which needs fewer
/// than `10^5` rows) fits many times over.  Wider posets, such as antichains
/// with more than about 18 elements, exceed it and are handed to the
/// frontier method by [`OrderMethod::Auto`].
pub const DEFAULT_ORDER_IDEAL_MAX_STATES: usize = 2_000_000;

/// Algorithm used to obtain the h* numerator of an order polytope.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lower")]
pub enum OrderMethod {
    /// Try the order-ideal method within its state budget; otherwise use the
    /// frontier method.
    #[default]
    Auto,
    /// Stanley's theorem: h* is the descent generating function of the
    /// linear extensions of a naturally labeled poset, computed by a dynamic
    /// program over order ideals.  Fails when the state budget is exceeded.
    Ideals,
    /// Count (strict) order-preserving maps by a frontier dynamic program,
    /// use Stanley-Macdonald reciprocity for negative dilations, and
    /// interpolate.
    Frontier,
}

impl OrderMethod {
    /// Stable lowercase name used in CLI flags and JSON provenance.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Ideals => "ideals",
            Self::Frontier => "frontier",
        }
    }
}

/// Method controls for an order-polytope calculation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OrderOptions {
    pub method: OrderMethod,
    /// Budget of stored `(ideal, last element)` rows for the order-ideal
    /// method; `None` selects [`DEFAULT_ORDER_IDEAL_MAX_STATES`].  It is
    /// rejected together with [`OrderMethod::Frontier`], which has no such
    /// budget.
    pub max_states: Option<usize>,
}

/// An order-polytope result together with the method that produced it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderComputation {
    pub data: EhrhartData,
    /// Never [`OrderMethod::Auto`]: the method that actually ran.
    pub method: OrderMethod,
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
    let polynomial = try_compute_ehrhart(
        &lambda,
        &mu,
        &input.weight,
        input.upper_flags.as_deref(),
        input.lower_flags.as_deref(),
        false,
        input.max_states,
        input.use_reciprocity,
    )?;
    EhrhartData::new(EhrhartPolynomial::new(dimension, polynomial.coeffs)?)
}

/// Compute a straight, skew, or flagged Kostka coefficient exactly.
pub fn kostka_count(input: &KostkaInput) -> ExactResult<BigInt> {
    let lambda = KostkaPartition::new(input.lambda.clone());
    let mu = KostkaPartition::new(input.mu.clone());
    let count = if input.upper_flags.is_some() || input.lower_flags.is_some() {
        try_flagged_skew_kostka(
            &lambda,
            &mu,
            &input.weight,
            input.upper_flags.as_deref(),
            input.lower_flags.as_deref(),
            input.max_states,
        )?
    } else {
        try_skew_kostka(&lambda, &mu, &input.weight, input.max_states, true)?
    };
    biguint_to_bigint(count)
}

/// Compute an LR coefficient by the existing augmented GT dynamic program.
pub fn lr_count(input: &LrInput) -> ExactResult<BigInt> {
    let lambda = KostkaPartition::new(input.lambda.clone());
    let mu = KostkaPartition::new(input.mu.clone());
    let nu = KostkaPartition::new(input.nu.clone());
    biguint_to_bigint(try_lr_dp(&lambda, &mu, &nu, input.max_states)?)
}

/// Compute exact Ehrhart and h* data for an order polytope with the default
/// method selection ([`OrderMethod::Auto`]).
pub fn order_ehrhart(input: &OrderInput) -> ExactResult<EhrhartData> {
    Ok(order_ehrhart_with(input, &OrderOptions::default())?.data)
}

/// Compute exact Ehrhart and h* data for an order polytope, reporting which
/// method produced it.
///
/// Both methods return the same [`EhrhartData`]: the dimension is the number
/// of poset elements and the h* numerator keeps its trailing zeros.
pub fn order_ehrhart_with(
    input: &OrderInput,
    options: &OrderOptions,
) -> ExactResult<OrderComputation> {
    let poset = build_poset(input)?;
    match options.method {
        OrderMethod::Frontier => {
            if options.max_states.is_some() {
                return Err(
                    "max_states applies only to the ideals method, not to frontier".to_string(),
                );
            }
            order_by_frontier(&poset)
        }
        OrderMethod::Ideals => {
            let hstar =
                ideal_hstar(&poset, options.max_states).map_err(|error| error.to_string())?;
            order_from_hstar(&poset, hstar)
        }
        OrderMethod::Auto => match ideal_hstar(&poset, options.max_states) {
            Ok(hstar) => order_from_hstar(&poset, hstar),
            // Too many states (or elements for the bitset keys): the frontier
            // method does not depend on the number of order ideals.
            Err(IdealDpError::StateLimitExceeded { .. } | IdealDpError::TooManyElements { .. }) => {
                order_by_frontier(&poset)
            }
        },
    }
}

fn order_by_frontier(poset: &Poset) -> ExactResult<OrderComputation> {
    let polynomial = poset.order_polytope_ehrhart();
    Ok(OrderComputation {
        data: EhrhartData::new(EhrhartPolynomial::new(poset.num_elements(), polynomial)?)?,
        method: OrderMethod::Frontier,
    })
}

/// Stanley: for a naturally labeled poset `h*(O(P), t)` is the P-Eulerian
/// polynomial, whose coefficients the order-ideal DP computes directly.
fn ideal_hstar(poset: &Poset, max_states: Option<usize>) -> Result<Vec<BigInt>, IdealDpError> {
    poset.p_eulerian_polynomial_ideal_dp(Some(max_states.unwrap_or(DEFAULT_ORDER_IDEAL_MAX_STATES)))
}

/// Declare the full numerator (`dimension + 1` entries, trailing zeros
/// included) and recover the Ehrhart polynomial from it.
fn order_from_hstar(poset: &Poset, mut coeffs: Vec<BigInt>) -> ExactResult<OrderComputation> {
    let dimension = poset.num_elements();
    coeffs.resize(dimension + 1, BigInt::zero());
    Ok(OrderComputation {
        data: EhrhartData::from_hstar(HStarPolynomial::new(dimension, coeffs)?)?,
        method: OrderMethod::Ideals,
    })
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
    EhrhartData::new(EhrhartPolynomial::new(
        polynomial.degree,
        polynomial.coeffs,
    )?)
}

fn build_poset(input: &OrderInput) -> ExactResult<Poset> {
    Ok(match input {
        OrderInput::Covers { vertices, covers } => Poset::try_new(*vertices, covers)?,
        OrderInput::Chain { elements } => Poset::chain(*elements),
        OrderInput::Antichain { elements } => Poset::antichain(*elements),
        OrderInput::Fence { elements } => Poset::fence(*elements),
        OrderInput::Shape { lambda } => Poset::from_shape(&CorePartition::new(lambda.clone())),
    })
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
    fn kostka_state_limit_returns_an_error() {
        let result = kostka_count(&KostkaInput {
            lambda: vec![2, 1],
            mu: vec![],
            weight: vec![1, 1, 1],
            upper_flags: None,
            lower_flags: None,
            max_states: Some(0),
        });
        assert!(result.is_err());
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

        let order = order_ehrhart(&OrderInput::Chain { elements: 2 }).expect("order-polytope data");
        assert_eq!(
            order.hstar.coeffs(),
            &[BigInt::one(), BigInt::zero(), BigInt::zero()]
        );

        let flow = flow_ehrhart(&FlowInput {
            vertices: 2,
            edges: vec![(0, 1), (0, 1)],
            netflow: vec![1, -1],
            max_states: None,
            use_reciprocity: true,
        })
        .expect("flow-polytope data");
        assert_eq!(
            flow.ehrhart.evaluate(3),
            num_rational::BigRational::from(BigInt::from(4))
        );
    }

    #[test]
    fn gt_and_flow_adapters_report_exact_dimensions_and_hstar() {
        // Audit witness: the polytope is a segment with N + 1 points.
        for use_reciprocity in [false, true] {
            for lower_flags in [None, Some(vec![1, 1, 1, 2])] {
                let gt = gt_ehrhart(&GtInput {
                    lambda: vec![3, 2, 1],
                    mu: vec![],
                    weight: vec![1, 1, 3, 1],
                    upper_flags: None,
                    lower_flags,
                    max_states: None,
                    use_reciprocity,
                })
                .expect("GT Ehrhart data");
                assert_eq!(gt.ehrhart.dimension(), 1);
                assert_eq!(gt.hstar.coeffs(), &[BigInt::one(), BigInt::zero()]);
                assert_eq!(
                    gt.ehrhart.evaluate(9),
                    num_rational::BigRational::from(BigInt::from(10))
                );
            }
        }
        let empty = gt_ehrhart(&GtInput {
            lambda: vec![3, 3, 2, 1],
            mu: vec![],
            weight: vec![1, 1, 2, 4, 1],
            upper_flags: None,
            lower_flags: None,
            max_states: None,
            use_reciprocity: true,
        });
        assert!(empty.is_err());

        // Balance at vertices 0 and 1 forces x02 = x13 = 0: one point.
        for use_reciprocity in [false, true] {
            let flow = flow_ehrhart(&FlowInput {
                vertices: 4,
                edges: vec![(0, 1), (0, 2), (1, 3), (2, 3)],
                netflow: vec![1, -1, 1, -1],
                max_states: None,
                use_reciprocity,
            })
            .expect("flow-polytope data");
            assert_eq!(flow.ehrhart.dimension(), 0);
            assert_eq!(flow.hstar.coeffs(), &[BigInt::one()]);
        }
    }

    #[test]
    fn order_rejects_cyclic_covers() {
        let result = order_ehrhart(&OrderInput::Covers {
            vertices: 2,
            covers: vec![(0, 1), (1, 0)],
        });
        assert!(result.is_err());
    }

    #[test]
    fn flagged_gt_data_agrees_with_direct_flagged_count_at_one() {
        let input = GtInput {
            lambda: vec![2, 1],
            mu: vec![],
            weight: vec![1, 1, 1],
            upper_flags: Some(vec![1, 1, 2]),
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
        assert_eq!(
            data.ehrhart.evaluate(1),
            num_rational::BigRational::from(count.clone())
        );
        assert_eq!(count, BigInt::one());
        assert_eq!(data.ehrhart.dimension(), 0);
    }

    #[test]
    fn gt_state_limit_returns_an_error_for_both_interpolation_paths() {
        for use_reciprocity in [false, true] {
            let result = gt_ehrhart(&GtInput {
                lambda: vec![2, 1],
                mu: vec![],
                weight: vec![1, 1, 1],
                upper_flags: None,
                lower_flags: None,
                max_states: Some(0),
                use_reciprocity,
            });
            assert!(result.is_err(), "use_reciprocity={use_reciprocity}");
        }
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

    fn order_with(
        input: &OrderInput,
        method: OrderMethod,
        max_states: Option<usize>,
    ) -> ExactResult<OrderComputation> {
        order_ehrhart_with(input, &OrderOptions { method, max_states })
    }

    fn hstar_strings(data: &EhrhartData) -> Vec<String> {
        data.hstar
            .coeffs()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    /// Deterministic random poset given by covers in a non-natural labeling.
    fn random_covers(seed: u64, n: usize) -> OrderInput {
        let mut state = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let mut next = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            state >> 33
        };
        let density = 1 + next() % 4;
        let mut covers = Vec::new();
        for a in 0..n {
            for b in a + 1..n {
                if next() % 8 < density {
                    covers.push((a, b));
                }
            }
        }
        let mut label: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            label.swap(i, (next() % (i as u64 + 1)) as usize);
        }
        OrderInput::Covers {
            vertices: n,
            covers: covers
                .into_iter()
                .map(|(a, b)| (label[a], label[b]))
                .collect(),
        }
    }

    #[test]
    fn order_ideals_and_frontier_give_identical_data() {
        let mut inputs = vec![
            OrderInput::Chain { elements: 0 },
            OrderInput::Chain { elements: 1 },
            OrderInput::Chain { elements: 6 },
            OrderInput::Antichain { elements: 1 },
            OrderInput::Antichain { elements: 7 },
            OrderInput::Fence { elements: 2 },
            OrderInput::Fence { elements: 9 },
            OrderInput::Shape { lambda: vec![] },
            OrderInput::Shape {
                lambda: vec![3, 3, 3],
            },
            OrderInput::Shape {
                lambda: vec![4, 3, 1],
            },
            OrderInput::Shape { lambda: vec![5, 5] },
            // Product of chains 3 x 4, and a poset with a non-natural labeling.
            OrderInput::Shape {
                lambda: vec![4, 4, 4],
            },
            OrderInput::Covers {
                vertices: 5,
                covers: vec![(4, 3), (4, 2), (3, 1), (2, 1), (1, 0)],
            },
        ];
        for seed in 0..60 {
            inputs.push(random_covers(seed, 1 + (seed % 10) as usize));
        }
        for input in inputs {
            let ideals = order_with(&input, OrderMethod::Ideals, None).expect("ideals");
            let frontier = order_with(&input, OrderMethod::Frontier, None).expect("frontier");
            assert_eq!(ideals.method, OrderMethod::Ideals);
            assert_eq!(frontier.method, OrderMethod::Frontier);
            // Same dimension, power coefficients, h* (with trailing zeros).
            assert_eq!(ideals.data, frontier.data, "{input:?}");
            let dimension = ideals.data.ehrhart.dimension();
            assert_eq!(ideals.data.hstar.coeffs().len(), dimension + 1);
            assert_eq!(
                crate::render::ehrhart_json(&ideals.data),
                crate::render::ehrhart_json(&frontier.data)
            );
        }
    }

    #[test]
    fn order_ideals_method_matches_known_values() {
        let hstar = |input: OrderInput| {
            hstar_strings(&order_with(&input, OrderMethod::Ideals, None).unwrap().data)
        };
        assert_eq!(
            hstar(OrderInput::Chain { elements: 3 }),
            ["1", "0", "0", "0"]
        );
        // Antichain: Eulerian numbers A(4, k), padded to dimension + 1.
        assert_eq!(
            hstar(OrderInput::Antichain { elements: 4 }),
            ["1", "11", "11", "1", "0"]
        );
        assert_eq!(
            hstar(OrderInput::Shape { lambda: vec![2, 2] }),
            ["1", "1", "0", "0", "0"]
        );
        // 2 x 4 rectangle: Narayana numbers 1, 6, 6, 1.
        assert_eq!(
            hstar(OrderInput::Shape { lambda: vec![4, 4] }),
            ["1", "6", "6", "1", "0", "0", "0", "0", "0"]
        );
        let empty = order_with(
            &OrderInput::Chain { elements: 0 },
            OrderMethod::Ideals,
            None,
        )
        .unwrap();
        assert_eq!(empty.data.ehrhart.dimension(), 0);
        assert_eq!(hstar_strings(&empty.data), ["1"]);
    }

    #[test]
    fn order_ideals_data_evaluates_to_the_order_ideal_count() {
        // L(1) is the number of lattice points of O(P), i.e. of order ideals.
        let covers = vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4)];
        let ideals = Poset::new(5, &covers).order_ideals().len();
        let data = order_with(
            &OrderInput::Covers {
                vertices: 5,
                covers,
            },
            OrderMethod::Ideals,
            None,
        )
        .unwrap()
        .data;
        assert_eq!(
            data.ehrhart.evaluate(1),
            num_rational::BigRational::from(BigInt::from(ideals))
        );
    }

    #[test]
    fn order_auto_uses_ideals_within_budget_and_frontier_beyond_it() {
        let input = OrderInput::Antichain { elements: 8 };
        let auto = order_with(&input, OrderMethod::Auto, None).unwrap();
        assert_eq!(auto.method, OrderMethod::Ideals);
        assert_eq!(order_ehrhart(&input).unwrap(), auto.data);

        // Exactly 8 * 2^7 = 1024 rows are needed.
        assert_eq!(
            order_with(&input, OrderMethod::Auto, Some(1024))
                .unwrap()
                .method,
            OrderMethod::Ideals
        );
        let fallback = order_with(&input, OrderMethod::Auto, Some(1023)).unwrap();
        assert_eq!(fallback.method, OrderMethod::Frontier);
        assert_eq!(fallback.data, auto.data);

        let error = order_with(&input, OrderMethod::Ideals, Some(1023)).expect_err("budget");
        assert!(error.contains("exceeded the limit of 1023"), "{error}");
    }

    #[test]
    fn order_frontier_rejects_a_state_budget_and_cycles_are_rejected_by_all_methods() {
        let error = order_with(
            &OrderInput::Chain { elements: 3 },
            OrderMethod::Frontier,
            Some(10),
        )
        .expect_err("frontier has no state budget");
        assert!(error.contains("max_states"), "{error}");
        let cyclic = OrderInput::Covers {
            vertices: 2,
            covers: vec![(0, 1), (1, 0)],
        };
        for method in [
            OrderMethod::Auto,
            OrderMethod::Ideals,
            OrderMethod::Frontier,
        ] {
            assert!(order_with(&cyclic, method, None).is_err());
        }
    }

    #[test]
    fn order_method_names_are_stable() {
        assert_eq!(OrderMethod::default(), OrderMethod::Auto);
        assert_eq!(OrderMethod::Auto.as_str(), "auto");
        assert_eq!(OrderMethod::Ideals.as_str(), "ideals");
        assert_eq!(OrderMethod::Frontier.as_str(), "frontier");
    }

    #[test]
    fn order_ideals_handles_large_narrow_posets_quickly() {
        // 5 x 5 rectangle: 25 elements, 252 ideals.
        let data = order_with(
            &OrderInput::Shape { lambda: vec![5; 5] },
            OrderMethod::Ideals,
            None,
        )
        .unwrap()
        .data;
        assert_eq!(data.ehrhart.dimension(), 25);
        // The h* entries sum to the number of standard Young tableaux of 5^5.
        let total: BigInt = data.hstar.coeffs().iter().sum();
        assert_eq!(total, BigInt::from(701_149_020u64));
    }
}
