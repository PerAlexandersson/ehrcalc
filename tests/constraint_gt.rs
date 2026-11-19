//! Full/reduced GT adapters versus independent maintained Kostka counters.
use ehrcalc::families::{
    constraint_polytope_ehrhart, gt_constraint_polytope, gt_ehrhart, gt_free_constraint_polytope,
    ConstraintOptions, GtInput, OrderHeuristic, PreparedConstraintPolytope,
};
use ehrcalc_kostka_engine::{
    gt_dim::gt_polytope_dim_full,
    kostka_dp::{
        profile_kostka_state_peak, try_flagged_skew_kostka, try_strict_flagged_skew_kostka,
    },
    Partition,
};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

fn input(
    lambda: &[u32],
    mu: &[u32],
    weight: &[u32],
    upper: Option<Vec<u32>>,
    lower: Option<Vec<u32>>,
) -> GtInput {
    GtInput {
        lambda: lambda.to_vec(),
        mu: mu.to_vec(),
        weight: weight.to_vec(),
        upper_flags: upper,
        lower_flags: lower,
        max_states: Some(100_000),
        use_reciprocity: true,
    }
}

/// Each case: 24 ordinary/strict exact/CRT adapter-count comparisons, four
/// service fits, and two existing-engine fits (or emptiness errors).
fn compare(case: &GtInput) -> bool {
    let lambda = Partition::new(case.lambda.clone());
    let mu = Partition::new(case.mu.clone());
    let dimension = gt_polytope_dim_full(
        lambda.parts(),
        mu.parts(),
        &case.weight,
        case.upper_flags.as_deref(),
        case.lower_flags.as_deref(),
    );
    let options = ConstraintOptions {
        order_heuristic: OrderHeuristic::Greedy,
        ..ConstraintOptions::default()
    };
    let full = gt_constraint_polytope(case).unwrap();
    let reduced = gt_free_constraint_polytope(case).unwrap();
    let prepared = [&full, &reduced].map(|s| PreparedConstraintPolytope::new(s).unwrap());
    if !prepared[1].is_empty() {
        assert!(
            reduced.bounds.iter().all(|b| b.lower != b.upper),
            "fixed entry survived reduction: {case:?}"
        );
    }
    for p in &prepared {
        assert_eq!(p.dimension(), dimension, "dimension: {case:?}");
    }
    for dilation in 1..=3_u32 {
        let outer = Partition::new(lambda.parts().iter().map(|x| x * dilation).collect());
        let inner = Partition::new(mu.parts().iter().map(|x| x * dilation).collect());
        let weight = case.weight.iter().map(|x| x * dilation).collect::<Vec<_>>();
        let ordinary = try_flagged_skew_kostka(
            &outer,
            &inner,
            &weight,
            case.upper_flags.as_deref(),
            case.lower_flags.as_deref(),
            case.max_states,
        )
        .unwrap();
        let strict = try_strict_flagged_skew_kostka(
            &outer,
            &inner,
            &weight,
            case.upper_flags.as_deref(),
            case.lower_flags.as_deref(),
            case.max_states,
        )
        .unwrap();
        for (interior, expected) in [(false, &ordinary), (true, &strict)] {
            for p in &prepared {
                for exact in [false, true] {
                    let actual = p
                        .count(
                            u64::from(dilation),
                            interior,
                            &ConstraintOptions {
                                exact,
                                ..options.clone()
                            },
                        )
                        .unwrap();
                    assert_eq!(
                        &actual.value, expected,
                        "count N={dilation}, strict={interior}, exact={exact}: {case:?}"
                    );
                }
            }
        }
    }
    for positive_only in [false, true] {
        let mut reference_input = case.clone();
        reference_input.use_reciprocity = !positive_only;
        let reference = gt_ehrhart(&reference_input);
        for system in [&full, &reduced] {
            let result = constraint_polytope_ehrhart(
                system,
                &ConstraintOptions {
                    positive_only,
                    ..options.clone()
                },
            )
            .unwrap();
            assert_eq!(result.dimension, dimension, "{case:?}");
            assert!(
                result.mismatches.is_empty(),
                "{case:?}: {:?}",
                result.mismatches
            );
            if dimension.is_some() {
                assert_eq!(
                    result.data.as_ref(),
                    Some(reference.as_ref().unwrap()),
                    "polynomial, positive_only={positive_only}: {case:?}"
                );
                assert_ne!(result.component_product_matches, Some(false), "{case:?}");
            } else {
                assert!(reference.is_err(), "{case:?}");
                assert!(result.empty);
                assert!(result
                    .polynomial
                    .unwrap()
                    .power_coeffs()
                    .iter()
                    .all(Zero::is_zero));
            }
        }
    }
    dimension.is_some()
}

struct Random(u64);
impl Random {
    fn below(&mut self, n: u32) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 32) % u64::from(n)) as u32
    }
}

#[test]
fn thousand_deterministic_flagged_skew_comparisons() {
    let mut rng = Random(0x00ef_2026_1008);
    let mut nonempty = 0;
    let mut skew = 0;
    let mut positive_dimension = 0;
    for index in 0..1024 {
        let rows = 1 + rng.below(3) as usize;
        let labels = rows + 1;
        let mut lambda = (0..rows).map(|_| 1 + rng.below(3)).collect::<Vec<_>>();
        lambda.sort_unstable_by(|a, b| b.cmp(a));
        let mut mu = if index % 2 == 0 {
            vec![0; rows]
        } else {
            (0..rows)
                .map(|r| rng.below(lambda[r] + 1))
                .collect::<Vec<_>>()
        };
        mu.sort_unstable_by(|a, b| b.cmp(a));
        for r in 0..rows {
            mu[r] = mu[r].min(lambda[r]);
        }
        skew += usize::from(mu.iter().any(|&m| m > 0));
        // Build a small semistandard tableau, using row+1 plus an isotone
        // 0/1 offset. Randomly relaxed label intervals retain that witness.
        let mut offsets = vec![vec![0; 3]; rows];
        let mut weight = vec![0; labels];
        let mut first = vec![rows as u32 + 1; labels];
        let mut last = vec![0; labels];
        for r in 0..rows {
            for c in mu[r] as usize..lambda[r] as usize {
                let left = if c > mu[r] as usize {
                    offsets[r][c - 1]
                } else {
                    0
                };
                let above = if r > 0 { offsets[r - 1][c] } else { 0 };
                let offset = left.max(above).max(rng.below(2));
                offsets[r][c] = offset;
                let label = r + offset as usize;
                weight[label] += 1;
                first[label] = first[label].min(r as u32 + 1);
                last[label] = last[label].max(r as u32 + 1);
            }
        }
        let lower = (0..labels)
            .map(|l| {
                if weight[l] == 0 {
                    1
                } else {
                    1 + rng.below(first[l])
                }
            })
            .collect::<Vec<_>>();
        let mut upper = (0..labels)
            .map(|l| {
                if weight[l] == 0 {
                    rows as u32
                } else {
                    last[l] + rng.below(rows as u32 - last[l] + 1)
                }
            })
            .collect::<Vec<_>>();
        if index % 4 == 0 {
            if let Some(l) = weight.iter().position(|&w| w > 0) {
                upper[l] = 0;
            }
        }
        let case = input(&lambda, &mu, &weight, Some(upper), Some(lower));
        nonempty += usize::from(compare(&case));
        positive_dimension += usize::from(
            gt_polytope_dim_full(
                &lambda,
                &mu,
                &weight,
                case.upper_flags.as_deref(),
                case.lower_flags.as_deref(),
            )
            .is_some_and(|d| d > 0),
        );
    }
    assert!(nonempty >= 700);
    assert!(skew >= 350);
    assert!(positive_dimension >= 30);
    eprintln!("GT gate: 1024 cases, {nonempty} nonempty, {} empty, {skew} skew, {positive_dimension} positive-dimensional; 24576 count comparisons; 4096 service fits; 2048 existing-engine fits/empty errors", 1024 - nonempty);
}

#[test]
fn regression_2026_10_08_and_broad_gt_corners() {
    for row_flags in [[4, 4, 6, 6, 6], [4, 5, 6, 6, 6]] {
        let lower = (1..=6)
            .map(|j| row_flags.iter().position(|&b| b >= j).unwrap() as u32 + 1)
            .collect();
        let case = input(
            &[3, 2, 2, 1, 1],
            &[],
            &[1, 2, 2, 2, 1, 1],
            None,
            Some(lower),
        );
        assert!(compare(&case));
    }
    let cases = [
        input(&[], &[], &[], None, None),
        input(&[], &[], &[0, 0], Some(vec![0, 0]), Some(vec![99, 99])),
        input(&[2, 1], &[2, 1], &[], None, None),
        input(&[2, 1], &[], &[0, 1, 0, 1, 1], Some(vec![2; 5]), None),
        input(&[1, 1], &[], &[2], None, None),
        input(&[3, 1], &[1], &[2, 1], Some(vec![1, 2]), Some(vec![1, 2])),
        input(&[3, 3], &[2, 2], &[0, 2, 0], None, None),
        input(&[2, 1], &[3], &[0], None, None),
        input(&[2, 1], &[], &[2], None, None),
        input(
            &[2, 1],
            &[],
            &[1, 1, 1],
            Some(vec![2, 2, 2]),
            Some(vec![2, 1, 1]),
        ),
        input(
            &[3, 1],
            &[1],
            &[1, 1, 1],
            Some(vec![99; 3]),
            Some(vec![0; 3]),
        ),
        input(&[0, 1, 2], &[0], &[1, 1, 1], None, None),
    ];
    for case in &cases {
        compare(case);
    }
    let mut bad = cases[0].clone();
    bad.upper_flags = Some(vec![1]);
    assert!(gt_constraint_polytope(&bad).is_err());
    assert!(gt_free_constraint_polytope(&bad).is_err());
    // A forbidden row can leave two equal but nonfixed entries. Reduction must
    // retain the equality rather than mistakenly freezing either endpoint.
    let case = input(&[3, 2], &[1], &[1, 1, 1, 1], Some(vec![2, 1, 2, 2]), None);
    compare(&case);
    let full = gt_constraint_polytope(&case).unwrap();
    let free = gt_free_constraint_polytope(&case).unwrap();
    assert!(free.variables < full.variables);
    eprintln!("GT corner gate: 15 cases including both flag-relaxation fixtures; 360 count comparisons and 60 service fits");
}

#[test]
#[ignore = "bounded serial wall-time comparison; run explicitly with --ignored --nocapture"]
fn timing_full_reduced_and_existing_gt_engine() {
    let cases = [
        (
            "small flagged",
            input(&[3, 2, 1], &[], &[2, 2, 2], Some(vec![2, 3, 3]), None),
        ),
        (
            "skew flagged",
            input(
                &[4, 3, 1],
                &[1],
                &[2, 2, 2, 1],
                Some(vec![2, 3, 3, 3]),
                Some(vec![1, 1, 1, 2]),
            ),
        ),
        (
            "relaxed regression",
            input(
                &[3, 2, 2, 1, 1],
                &[],
                &[1, 2, 2, 2, 1, 1],
                None,
                Some(vec![1, 1, 1, 1, 2, 3]),
            ),
        ),
    ];
    eprintln!("instance,engine,wall_us,peak_states,dimension");
    for (name, case) in cases {
        let start = std::time::Instant::now();
        let (reference, peak) = profile_kostka_state_peak(|| gt_ehrhart(&case));
        let reference = reference.unwrap();
        eprintln!(
            "{name},gt_ehrhart,{},{peak},{}",
            start.elapsed().as_micros(),
            reference.ehrhart.dimension()
        );
        for reduced in [false, true] {
            let start = std::time::Instant::now();
            let system = if reduced {
                gt_free_constraint_polytope(&case)
            } else {
                gt_constraint_polytope(&case)
            }
            .unwrap();
            let result =
                constraint_polytope_ehrhart(&system, &ConstraintOptions::default()).unwrap();
            let elapsed = start.elapsed().as_micros();
            assert_eq!(result.data.as_ref(), Some(&reference));
            let peak = result
                .samples
                .iter()
                .map(|s| s.count.peak_states)
                .max()
                .unwrap_or(0);
            eprintln!(
                "{name},{},{elapsed},{peak},{}",
                if reduced { "reduced" } else { "full" },
                reference.ehrhart.dimension()
            );
            assert!(result.mismatches.is_empty());
            for n in 1..=3 {
                assert!(
                    reference.ehrhart.evaluate(n) >= BigRational::from_integer(BigInt::from(0))
                );
            }
        }
    }
}

#[test]
fn state_profile_matches_existing_stats_and_restores_nested_scopes() {
    use ehrcalc_kostka_engine::kostka_dp::try_skew_kostka_stats;
    let outer = Partition::new(vec![4, 3, 1]);
    let inner = Partition::empty();
    let ((result, nested), peak) = profile_kostka_state_peak(|| {
        profile_kostka_state_peak(|| {
            try_skew_kostka_stats(&outer, &inner, &[2, 2, 2, 2], None, false).unwrap()
        })
    });
    assert!(peak > 1);
    assert_eq!(peak, result.peak_states);
    assert_eq!(peak, nested);
    let (_, empty) = profile_kostka_state_peak(|| {});
    assert_eq!(empty, 0);
    let _ = std::panic::catch_unwind(|| profile_kostka_state_peak(|| panic!("profile cleanup")));
    assert_eq!(profile_kostka_state_peak(|| ()).1, 0);
}
