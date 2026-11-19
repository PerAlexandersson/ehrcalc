use super::*;
use crate::affine_hull::{exact_rank, AffineHull, RationalPolyhedron};
use num::Integer;
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn independent_hull(input: &ConstraintPolytopeInput) -> AffineHull {
    let input = input.normalized().unwrap();
    let mut polyhedron = RationalPolyhedron::new(input.variables);
    for (v, b) in input.bounds.iter().enumerate() {
        if let Some(a) = &b.lower {
            polyhedron.add_inequality([(v, q(-1, 1))], -a);
        }
        if let Some(b) = &b.upper {
            polyhedron.add_inequality([(v, q(1, 1))], b.clone());
        }
    }
    for d in &input.differences {
        polyhedron.add_inequality([(d.left, q(1, 1)), (d.right, q(-1, 1))], d.upper.clone());
    }
    for sum in &input.sums {
        if let Some(a) = &sum.lower {
            polyhedron.add_inequality(sum.variables.iter().map(|&v| (v, q(-1, 1))), -a);
        }
        if let Some(b) = &sum.upper {
            polyhedron.add_inequality(sum.variables.iter().map(|&v| (v, q(1, 1))), b.clone());
        }
    }
    polyhedron.affine_hull()
}

/// Independent reference: enumerate the original box and evaluate rational
/// inequalities directly. No integer presolve, translation, or DP transitions.
fn brute_points(
    input: &ConstraintPolytopeInput,
    dilation: u64,
    strict: bool,
) -> Vec<Vec<BigRational>> {
    let AffineHull::Nonempty(hull) = independent_hull(input) else {
        return Vec::new();
    };
    let input = input.normalized().unwrap();
    if dilation == 0 {
        return vec![vec![BigRational::zero(); input.variables]];
    }
    let scale = BigInt::from(dilation);
    let ranges = input
        .bounds
        .iter()
        .map(|b| {
            let lo = b.lower.as_ref().expect("reference needs a supplied box") * &scale;
            let hi = b.upper.as_ref().expect("reference needs a supplied box") * &scale;
            (
                lo.numer().div_ceil(lo.denom()).to_i64().unwrap(),
                hi.numer().div_floor(hi.denom()).to_i64().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut points = Vec::new();
    fn visit(
        input: &ConstraintPolytopeInput,
        ranges: &[(i64, i64)],
        point: &mut Vec<BigRational>,
        scale: &BigInt,
        strict: bool,
        mask: &[bool],
        points: &mut Vec<Vec<BigRational>>,
    ) {
        if point.len() != ranges.len() {
            let (lo, hi) = ranges[point.len()];
            for x in lo..=hi {
                point.push(BigRational::from_integer(x.into()));
                visit(input, ranges, point, scale, strict, mask, points);
                point.pop();
            }
            return;
        }
        let mut inequalities = Vec::new();
        for (v, b) in input.bounds.iter().enumerate() {
            if let Some(a) = &b.lower {
                inequalities.push((-&point[v], -a * scale));
            }
            if let Some(b) = &b.upper {
                inequalities.push((point[v].clone(), b * scale));
            }
        }
        for d in &input.differences {
            inequalities.push((&point[d.left] - &point[d.right], &d.upper * scale));
        }
        for sum in &input.sums {
            let total = sum
                .variables
                .iter()
                .map(|&v| point[v].clone())
                .sum::<BigRational>();
            if let Some(a) = &sum.lower {
                inequalities.push((-&total, -a * scale));
            }
            if let Some(b) = &sum.upper {
                inequalities.push((total.clone(), b * scale));
            }
        }
        assert_eq!(inequalities.len(), mask.len());
        if inequalities
            .iter()
            .zip(mask)
            .all(|((lhs, rhs), &implicit)| {
                if strict && !implicit {
                    lhs < rhs
                } else {
                    lhs <= rhs
                }
            })
        {
            points.push(point.clone());
        }
    }
    visit(
        &input,
        &ranges,
        &mut Vec::new(),
        &scale,
        strict,
        &hull.implicit_equalities,
        &mut points,
    );
    points
}

struct Random(u64);
impl Random {
    fn next(&mut self, max: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % max
    }
    fn rhs(&mut self) -> BigRational {
        q(
            self.next(7) as i64 - 2,
            if self.next(3) == 0 { 2 } else { 1 },
        )
    }
}

fn random_input(random: &mut Random, case: usize) -> ConstraintPolytopeInput {
    let n = 1 + random.next(4) as usize;
    let mut input = ConstraintPolytopeInput {
        variables: n,
        ..ConstraintPolytopeInput::default()
    };
    for _ in 0..n {
        let lo = q(-(random.next(3) as i64), 2);
        let hi = if case.is_multiple_of(13) {
            lo.clone()
        } else {
            q(2 + random.next(3) as i64, 2)
        };
        input.bounds.push(VariableBounds {
            lower: Some(lo),
            upper: Some(hi),
        });
    }
    for i in 0..n {
        for j in i + 1..n {
            if random.next(3) == 0 {
                let (left, right) = if random.next(2) == 0 { (i, j) } else { (j, i) };
                input.differences.push(DifferenceConstraint {
                    left,
                    right,
                    upper: random.rhs(),
                });
            }
        }
    }
    for _ in 0..random.next(4) {
        let variables = (0..n).filter(|_| random.next(2) == 0).collect::<Vec<_>>();
        if variables.len() < 2 {
            continue;
        }
        let lo = random.rhs();
        let hi = lo.clone() + q(random.next(5) as i64, 2);
        let kind = random.next(4);
        input.sums.push(SubsetSum {
            variables,
            lower: (kind != 0).then_some(lo.clone()),
            upper: if kind == 1 {
                None
            } else if kind == 2 {
                Some(lo)
            } else {
                Some(hi)
            },
        });
    }
    input
}

#[test]
fn random_dp_brute_force_order_and_crt_gate() {
    let mut random = Random(0xb6e3_984a_d912_a821);
    let mut comparisons = 0;
    let mut nonempty = 0;
    for case in 0..360 {
        let input = random_input(&mut random, case);
        let prepared = PreparedConstraintPolytope::new(&input).unwrap();
        match independent_hull(&input) {
            AffineHull::Empty => assert!(prepared.is_empty()),
            AffineHull::Nonempty(hull) => {
                assert_eq!(prepared.dimension(), Some(hull.dimension));
                assert_eq!(
                    prepared.affine_hull().unwrap().implicit_equalities,
                    hull.implicit_equalities
                );
            }
        }
        nonempty += usize::from(prepared.dimension().is_some());
        for dilation in [1, 2, 3] {
            for strict in [false, true] {
                let expected = BigUint::from(brute_points(&input, dilation, strict).len());
                for heuristic in [
                    OrderHeuristic::Greedy,
                    OrderHeuristic::SumContiguous,
                    OrderHeuristic::ReverseCuthillMcKee,
                    OrderHeuristic::ExactSubset,
                ] {
                    let options = ConstraintOptions {
                        exact: true,
                        order_heuristic: heuristic,
                        ..ConstraintOptions::default()
                    };
                    let exact = prepared.count(dilation, strict, &options).unwrap();
                    assert_eq!(
                        exact.value, expected,
                        "case {case}, N={dilation}, strict={strict}, \
                        order={heuristic:?}, input={input:?}"
                    );
                    let modular = prepared
                        .count(
                            dilation,
                            strict,
                            &ConstraintOptions {
                                exact: false,
                                ..options
                            },
                        )
                        .unwrap();
                    assert_eq!(modular.value, exact.value);
                    for certificate in &modular.modular {
                        assert!(certificate.combined_modulus > certificate.upper_bound);
                        assert!(modular.value.is_zero() || !certificate.moduli.is_empty());
                    }
                    comparisons += 1;
                }
                let override_count = prepared
                    .count(
                        dilation,
                        strict,
                        &ConstraintOptions {
                            exact: true,
                            order: Some((0..input.variables).rev().collect()),
                            ..ConstraintOptions::default()
                        },
                    )
                    .unwrap();
                assert_eq!(override_count.value, expected);
            }
        }
        for plan in prepared
            .plan(3, false, &ConstraintOptions::default())
            .unwrap()
        {
            let greedy = plan
                .candidates
                .iter()
                .find(|p| p.heuristic == OrderHeuristic::Greedy)
                .unwrap();
            let exact = plan
                .candidates
                .iter()
                .find(|p| p.heuristic == OrderHeuristic::ExactSubset)
                .unwrap();
            assert!(exact.estimated_peak <= greedy.estimated_peak);
        }
    }
    assert!(nonempty >= 180);
    eprintln!(
        "RANDOM_COUNTER_GATE: systems=360 nonempty={nonempty} N=1,2,3 strict+ordinary \
        candidate_exact_brute_crt_comparisons={comparisons} override_comparisons=2160"
    );
}

#[test]
fn parsing_normalization_and_validation() {
    let input: ConstraintPolytopeInput = "vars 3\nbound 0 -1/2 2\n\
        bound 1 0 3\nbound 2 0 3\nle 0 1\nle 0 1 -1/2\n\
        sum 0 2 = 2\nsum 1 2 <= 4 # comment\n"
        .parse()
        .unwrap();
    let json = serde_json::to_string(&input).unwrap();
    assert_eq!(ConstraintPolytopeInput::from_json(&json).unwrap(), input);
    let normalized = input.normalized().unwrap();
    assert_eq!(normalized.differences.len(), 1);
    assert_eq!(normalized.differences[0].upper, q(-1, 2));
    let incompatible = ConstraintPolytopeInput::from_json(
        r#"{"variables":2,
        "bounds":[[0,1],[0,1]],"sums":[{"vars":[0,1],"lower":2},
        {"vars":[1,0],"upper":1}]}"#,
    )
    .unwrap()
    .normalized()
    .unwrap();
    assert_eq!(incompatible.normalized().unwrap(), incompatible);
    assert!(PreparedConstraintPolytope::new(&incompatible)
        .unwrap()
        .is_empty());
    for text in [
        r#"{"variables":1,"bounds":[[2,1]]}"#,
        r#"{"variables":1,"bounds":[[0,1.5]]}"#,
        r#"{"variables":2,"names":["x","x"]}"#,
        r#"{"variables":1,"differences":[[0,1,0]]}"#,
        r#"{"variables":1,"sums":[{"vars":[],"upper":1}]}"#,
        r#"{"variables":1,"sums":[{"vars":[0,0],"upper":1}]}"#,
        r#"{"variables":1,"sums":[{"vars":[0],"lower":2,"upper":1}]}"#,
        r#"{"variables":1,"bounds":[[0,"1/0"]]}"#,
        r#"{"variables":1,"unexpected":1}"#,
        r#"{"schema":"v2","variables":1}"#,
    ] {
        assert!(ConstraintPolytopeInput::from_json(text).is_err(), "{text}");
    }
    for text in ["", "bound 0 0 1", "vars 1\nle 0 2", "vars 1\nsum 0 = 1/0"] {
        assert!(text.parse::<ConstraintPolytopeInput>().is_err(), "{text}");
    }
}

#[test]
fn affine_hull_implicit_equalities_fixed_and_empty() {
    // Nonnegativity and x+y <= 0 force two implicit lower bounds jointly.
    let input = ConstraintPolytopeInput::from_json(
        r#"{"variables":3,
        "bounds":[[0,1],[0,1],[0,1]],"sums":[{"vars":[0,1],"upper":0}]}"#,
    )
    .unwrap();
    let prepared = PreparedConstraintPolytope::new(&input).unwrap();
    assert_eq!(prepared.dimension(), Some(1));
    assert_eq!(prepared.free_variables, vec![2]);
    assert_eq!(
        prepared
            .count(2, true, &ConstraintOptions::default())
            .unwrap()
            .value,
        1_u32.into()
    );
    let fractional = ConstraintPolytopeInput::from_json(
        r#"{"variables":2,
        "bounds":[["1/2","1/2"],[0,1]]}"#,
    )
    .unwrap();
    let prepared = PreparedConstraintPolytope::new(&fractional).unwrap();
    assert_eq!(prepared.dimension(), Some(1));
    assert_eq!(
        prepared
            .count(1, false, &ConstraintOptions::default())
            .unwrap()
            .value,
        BigUint::zero()
    );
    assert_eq!(
        prepared
            .count(2, false, &ConstraintOptions::default())
            .unwrap()
            .value,
        3_u32.into()
    );
    assert_eq!(
        prepared
            .count(2, true, &ConstraintOptions::default())
            .unwrap()
            .value,
        1_u32.into()
    );
    let empty = ConstraintPolytopeInput::from_json(
        r#"{"variables":2,"bounds":[[0,1],[0,1]],
        "differences":[[0,1,-2]]}"#,
    )
    .unwrap();
    let prepared = PreparedConstraintPolytope::new(&empty).unwrap();
    assert_eq!(prepared.dimension(), None);
    assert_eq!(
        prepared
            .count(0, false, &ConstraintOptions::default())
            .unwrap()
            .value,
        BigUint::zero()
    );
    // No lattice point at N=1, yet an exact one-dimensional rational hull.
    let slice = ConstraintPolytopeInput::from_json(
        r#"{"variables":2,"bounds":[[0,1],[0,1]],
        "sums":[{"vars":[0,1],"lower":"1/2","upper":"1/2"}]}"#,
    )
    .unwrap();
    let prepared = PreparedConstraintPolytope::new(&slice).unwrap();
    assert_eq!(prepared.dimension(), Some(1));
    assert_eq!(
        prepared
            .count(1, false, &ConstraintOptions::default())
            .unwrap()
            .value,
        BigUint::zero()
    );
}

#[test]
fn bound_completion_and_unbounded_diagnostics() {
    let bounded = ConstraintPolytopeInput::from_json(
        r#"{"variables":2,
        "differences":[[0,1,0],[1,0,0]],
        "sums":[{"vars":[0,1],"lower":0,"upper":2}]}"#,
    )
    .unwrap();
    let prepared = PreparedConstraintPolytope::new(&bounded).unwrap();
    assert_eq!(prepared.dimension(), Some(1));
    assert_eq!(
        prepared
            .count(2, false, &ConstraintOptions::default())
            .unwrap()
            .value,
        3_u32.into()
    );
    assert_eq!(
        prepared
            .count(2, true, &ConstraintOptions::default())
            .unwrap()
            .value,
        1_u32.into()
    );
    let unbounded =
        ConstraintPolytopeInput::from_json(r#"{"variables":1,"names":["height"]}"#).unwrap();
    assert!(matches!(PreparedConstraintPolytope::new(&unbounded),
        Err(ConstraintError::Unbounded { name,.. }) if name == "height"));
    let sums_bounded = ConstraintPolytopeInput::from_json(
        r#"{"variables":2,
        "bounds":[[0,null],[0,null]],"sums":[{"vars":[0,1],"upper":2}]}"#,
    )
    .unwrap();
    assert_eq!(
        PreparedConstraintPolytope::new(&sums_bounded)
            .unwrap()
            .count(1, false, &ConstraintOptions::default())
            .unwrap()
            .value,
        6_u32.into()
    );
}

#[test]
fn affine_hull_matches_spanning_brute_points() {
    let mut random = Random(0x78da_22c8_1234);
    for _ in 0..48 {
        let n = 1 + random.next(5) as usize;
        let mut input = ConstraintPolytopeInput {
            variables: n,
            bounds: vec![
                VariableBounds {
                    lower: Some(q(0, 1)),
                    upper: Some(q(1, 1))
                };
                n
            ],
            ..ConstraintPolytopeInput::default()
        };
        for i in 0..n {
            for j in i + 1..n {
                if random.next(3) == 0 {
                    input.differences.push(DifferenceConstraint {
                        left: i,
                        right: j,
                        upper: q(0, 1),
                    });
                }
            }
        }
        let prepared = PreparedConstraintPolytope::new(&input).unwrap();
        let points = brute_points(&input, 1, false);
        let matrix = points
            .iter()
            .skip(1)
            .map(|p| p.iter().zip(&points[0]).map(|(x, y)| x - y).collect())
            .collect();
        assert_eq!(Some(exact_rank(matrix)), prepared.dimension());
        let mut independent = RationalPolyhedron::new(n);
        for v in 0..n {
            independent.add_inequality([(v, q(-1, 1))], q(0, 1));
            independent.add_inequality([(v, q(1, 1))], q(1, 1));
        }
        for d in &input.differences {
            independent.add_inequality([(d.left, q(1, 1)), (d.right, q(-1, 1))], d.upper.clone());
        }
        let AffineHull::Nonempty(hull) = independent.affine_hull() else {
            panic!("unit order polytope");
        };
        assert_eq!(hull.dimension, prepared.dimension().unwrap());
    }
}

#[test]
fn state_budget_and_order_validation() {
    let input = ConstraintPolytopeInput::from_json(
        r#"{"variables":3,
        "bounds":[[0,3],[0,3],[0,3]],"sums":[{"vars":[0,1,2],"upper":4}]}"#,
    )
    .unwrap();
    let prepared = PreparedConstraintPolytope::new(&input).unwrap();
    let options = ConstraintOptions {
        max_states: Some(1),
        ..ConstraintOptions::default()
    };
    let error = prepared.count(1, false, &options).unwrap_err();
    assert!(matches!(
        error,
        ConstraintError::StateLimitExceeded {
            step: 1,
            limit: 1,
            ..
        }
    ));
    assert!(error.to_string().contains("frontier"));
    assert!(prepared.sample_plan(&options).is_ok());
    assert!(prepared
        .plan(
            1,
            false,
            &ConstraintOptions {
                order: Some(vec![0, 0, 1]),
                ..options
            }
        )
        .is_err());
    assert!(prepared
        .count(0, true, &ConstraintOptions::default())
        .is_err());
}

#[test]
fn equality_bound_and_multiple_crt_lanes() {
    let input = ConstraintPolytopeInput::from_json(
        r#"{"variables":3,
        "bounds":[[0,20],[0,20],[0,20]],
        "sums":[{"vars":[0,1,2],"lower":20,"upper":20}]}"#,
    )
    .unwrap();
    let prepared = PreparedConstraintPolytope::new(&input).unwrap();
    let result = prepared
        .count(1, false, &ConstraintOptions::default())
        .unwrap();
    assert_eq!(result.value, 231_u32.into());
    assert_eq!(result.modular[0].upper_bound, 441_u32.into());
    let huge: BigInt = BigInt::one() << 280_usize;
    let input = ConstraintPolytopeInput {
        variables: 1,
        bounds: vec![VariableBounds {
            lower: Some(q(0, 1)),
            upper: Some(BigRational::from_integer(huge.clone())),
        }],
        ..ConstraintPolytopeInput::default()
    };
    let prepared = PreparedConstraintPolytope::new(&input).unwrap();
    let modular = prepared
        .count(1, false, &ConstraintOptions::default())
        .unwrap();
    let exact = prepared
        .count(
            1,
            false,
            &ConstraintOptions {
                exact: true,
                ..ConstraintOptions::default()
            },
        )
        .unwrap();
    assert_eq!(modular.value, exact.value);
    assert_eq!(modular.value, (huge + BigInt::one()).to_biguint().unwrap());
    assert!(modular.modular[0].moduli.len() > 8);
    let cert = &modular.modular[0];
    assert!(crate::packed_modular::crt_reconstruct_bounded(
        &cert.residues,
        &cert.moduli,
        &cert.combined_modulus
    )
    .is_err());
}

#[test]
fn interval_smoke() {
    let input = ConstraintPolytopeInput::from_json(r#"{"variables":1,"bounds":[[0,2]]}"#).unwrap();
    let prepared = PreparedConstraintPolytope::new(&input).unwrap();
    assert_eq!(prepared.dimension(), Some(1));
    assert_eq!(
        prepared
            .count(3, false, &ConstraintOptions::default())
            .unwrap()
            .value,
        7_u32.into()
    );
    assert_eq!(
        prepared
            .count(3, true, &ConstraintOptions::default())
            .unwrap()
            .value,
        5_u32.into()
    );
}
