//! Exact hypersimplex, transportation, Birkhoff and rank-polytope fixtures.
use ehrcalc::{
    exact::{parse_rational_list, EhrhartPolynomial},
    families::{
        constraint_polytope_ehrhart, ConstraintOptions, ConstraintPolytopeInput, OrderHeuristic,
        PreparedConstraintPolytope, SubsetSum, VariableBounds,
    },
};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, ToPrimitive};
use std::collections::HashSet;

fn q(x: usize) -> BigRational {
    BigRational::from_integer(x.into())
}
fn boxed(n: usize, max: usize) -> ConstraintPolytopeInput {
    ConstraintPolytopeInput {
        variables: n,
        bounds: vec![
            VariableBounds {
                lower: Some(q(0)),
                upper: Some(q(max))
            };
            n
        ],
        ..ConstraintPolytopeInput::default()
    }
}
fn sum(variables: Vec<usize>, lower: Option<usize>, upper: usize) -> SubsetSum {
    SubsetSum {
        variables,
        lower: lower.map(q),
        upper: Some(q(upper)),
    }
}
fn options() -> ConstraintOptions {
    ConstraintOptions {
        order_heuristic: OrderHeuristic::Greedy,
        ..ConstraintOptions::default()
    }
}

fn binomial(n: i64, k: usize) -> BigInt {
    if n < k as i64 {
        return 0.into();
    }
    (0..k).fold(BigInt::one(), |x, i| x * (n - i as i64) / (i + 1))
}
fn hypersimplex_count(k: usize, n: usize, dilation: i64) -> BigInt {
    (0..=n)
        .map(|j| {
            let term = binomial(n as i64, j)
                * binomial(
                    k as i64 * dilation - j as i64 * (dilation + 1) + n as i64 - 1,
                    n - 1,
                );
            if j % 2 == 0 {
                term
            } else {
                -term
            }
        })
        .sum()
}

#[test]
fn hypersimplices_against_inclusion_exclusion() {
    let mut cases = 0;
    for n in 2..=6 {
        for k in 0..=n {
            let mut input = boxed(n, 1);
            input.sums.push(sum((0..n).collect(), Some(k), k));
            let prepared = PreparedConstraintPolytope::new(&input).unwrap();
            assert_eq!(
                prepared.dimension(),
                Some(if k == 0 || k == n { 0 } else { n - 1 })
            );
            for exact in [false, true] {
                for dilation in 0..=4 {
                    let count = prepared
                        .count(dilation, false, &ConstraintOptions { exact, ..options() })
                        .unwrap();
                    assert_eq!(
                        BigInt::from(count.value),
                        hypersimplex_count(k, n, dilation as i64)
                    );
                }
            }
            for positive_only in [false, true] {
                let result = constraint_polytope_ehrhart(
                    &input,
                    &ConstraintOptions {
                        positive_only,
                        ..options()
                    },
                )
                .unwrap();
                assert!(result.mismatches.is_empty());
                let data = result.data.unwrap();
                for dilation in 0..=6 {
                    assert_eq!(
                        data.ehrhart.evaluate(dilation),
                        BigRational::from_integer(hypersimplex_count(k, n, dilation))
                    );
                }
            }
            cases += 1;
        }
    }
    eprintln!("hypersimplex gate: {cases} systems, 250 exact/CRT count checks, 50 fits, 350 formula evaluations");
}

fn transportation(rows: &[usize], columns: &[usize]) -> ConstraintPolytopeInput {
    let n = columns.len();
    let mut input = boxed(rows.len() * n, 0);
    input.bounds = rows
        .iter()
        .flat_map(|&r| {
            columns.iter().map(move |&c| VariableBounds {
                lower: Some(q(0)),
                upper: Some(q(r.min(c))),
            })
        })
        .collect();
    input.sums.extend(
        rows.iter()
            .enumerate()
            .map(|(r, &total)| sum((0..n).map(|c| r * n + c).collect(), Some(total), total)),
    );
    input
        .sums
        .extend(columns.iter().enumerate().map(|(c, &total)| {
            sum(
                (0..rows.len()).map(|r| r * n + c).collect(),
                Some(total),
                total,
            )
        }));
    input
}

// Full original box enumeration; deliberately independent of presolve and DP.
fn brute_box(input: &ConstraintPolytopeInput, dilation: usize) -> usize {
    let widths = input
        .bounds
        .iter()
        .map(|b| b.upper.as_ref().unwrap().to_integer().to_usize().unwrap() * dilation)
        .collect::<Vec<_>>();
    let mut point = vec![0; input.variables];
    let mut count = 0;
    loop {
        if input.sums.iter().all(|s| {
            let total = s.variables.iter().map(|&v| point[v]).sum::<usize>();
            s.lower.as_ref().is_none_or(|a| q(total) >= a * q(dilation))
                && s.upper.as_ref().is_none_or(|b| q(total) <= b * q(dilation))
        }) {
            count += 1;
        }
        let mut i = 0;
        while i < point.len() && point[i] == widths[i] {
            point[i] = 0;
            i += 1;
        }
        if i == point.len() {
            break;
        }
        point[i] += 1;
    }
    count
}

#[test]
fn birkhoff_three_and_four_published_polynomials() {
    // Beck--Pixton, Discrete Comput. Geom. 30 (2003), 623--637.
    // Author's exact coefficient table (checked 2026-11-19):
    // https://matthbeck.github.io/birkhoff/polynomials.html
    let fixtures = [
        (3, "1,9/4,15/8,3/4,1/8", vec![1, 1, 1, 0, 0]),
        (
            4,
            "1,65/18,379/63,35117/5670,43/10,1109/540,2/3,19/135,11/630,11/11340",
            vec![1, 14, 87, 148, 87, 14, 1, 0, 0, 0],
        ),
    ];
    for (n, coefficients, hstar) in fixtures {
        let input = transportation(&vec![1; n], &vec![1; n]);
        let expected =
            EhrhartPolynomial::new((n - 1).pow(2), parse_rational_list(coefficients).unwrap())
                .unwrap();
        for positive_only in [false, true] {
            let result = constraint_polytope_ehrhart(
                &input,
                &ConstraintOptions {
                    positive_only,
                    ..options()
                },
            )
            .unwrap();
            assert!(result.mismatches.is_empty());
            let data = result.data.unwrap();
            assert_eq!(data.ehrhart, expected);
            assert_eq!(
                data.hstar.coeffs(),
                hstar.iter().map(|&x| BigInt::from(x)).collect::<Vec<_>>()
            );
        }
        let prepared = PreparedConstraintPolytope::new(&input).unwrap();
        for exact in [false, true] {
            for dilation in 1..=3 {
                let count = prepared
                    .count(dilation, false, &ConstraintOptions { exact, ..options() })
                    .unwrap();
                assert_eq!(
                    BigRational::from_integer(count.value.into()),
                    expected.evaluate(dilation as i64)
                );
            }
        }
    }
    eprintln!("Birkhoff gate: B3/B4, four published-polynomial fits and twelve exact/CRT counts");
}

#[test]
fn transportation_against_original_box_brute_force() {
    for (rows, cols) in [
        (vec![1, 2], vec![1, 1, 1]),
        (vec![2, 2], vec![1, 3]),
        (vec![0, 2], vec![1, 1]),
        (vec![1, 1], vec![1, 2]),
    ] {
        let input = transportation(&rows, &cols);
        let prepared = PreparedConstraintPolytope::new(&input).unwrap();
        for dilation in 1..=3 {
            let expected = brute_box(&input, dilation);
            for exact in [false, true] {
                let count = prepared
                    .count(
                        dilation as u64,
                        false,
                        &ConstraintOptions { exact, ..options() },
                    )
                    .unwrap();
                assert_eq!(
                    count.value,
                    expected.into(),
                    "{rows:?}/{cols:?}, N={dilation}"
                );
            }
        }
        for positive_only in [false, true] {
            let result = constraint_polytope_ehrhart(
                &input,
                &ConstraintOptions {
                    positive_only,
                    ..options()
                },
            )
            .unwrap();
            assert!(result.mismatches.is_empty());
            for n in 1..=3 {
                assert_eq!(
                    result.polynomial.as_ref().unwrap().evaluate(n),
                    q(brute_box(&input, n as usize))
                );
            }
        }
    }
    eprintln!("transportation gate: four systems (including empty and degenerate), 24 exact/CRT counts, eight fits");
}

fn graphic_rank(vertices: usize, edges: &[(usize, usize)], mask: usize) -> usize {
    let mut component = (0..vertices).collect::<Vec<_>>();
    let mut rank = 0;
    for (e, &(a, b)) in edges.iter().enumerate() {
        if mask & (1 << e) == 0 || component[a] == component[b] {
            continue;
        }
        let old = component[b];
        let new = component[a];
        for c in &mut component {
            if *c == old {
                *c = new;
            }
        }
        rank += 1;
    }
    rank
}

fn check_rank_polytope(n: usize, rank: impl Fn(usize) -> usize) {
    for base in [false, true] {
        let mut input = boxed(n, 1);
        for mask in 1_usize..1 << n {
            let subset = (0..n).filter(|&v| mask & (1 << v) != 0).collect::<Vec<_>>();
            input.sums.push(sum(
                subset,
                (base && mask == (1 << n) - 1).then(|| rank(mask)),
                rank(mask),
            ));
        }
        let vertices = (0_usize..1 << n)
            .filter(|&s| {
                s.count_ones() as usize == rank(s) && (!base || rank(s) == rank((1 << n) - 1))
            })
            .map(|s| {
                (0..n)
                    .map(|v| u8::from(s & (1 << v) != 0))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let prepared = PreparedConstraintPolytope::new(&input).unwrap();
        let mut brute: HashSet<Vec<u8>> = [vec![0; n]].into();
        for dilation in 1..=3 {
            // Matroid polytopes have the integer decomposition property: brute
            // force every sum of N independent-set / basis indicator vectors.
            brute = brute
                .iter()
                .flat_map(|p| {
                    vertices
                        .iter()
                        .map(move |v| p.iter().zip(v).map(|(a, b)| a + b).collect())
                })
                .collect();
            for exact in [false, true] {
                let count = prepared
                    .count(dilation, false, &ConstraintOptions { exact, ..options() })
                    .unwrap();
                assert_eq!(count.value, brute.len().into(), "base={base}, N={dilation}");
            }
        }
        for positive_only in [false, true] {
            let result = constraint_polytope_ehrhart(
                &input,
                &ConstraintOptions {
                    positive_only,
                    ..options()
                },
            )
            .unwrap();
            assert!(result.mismatches.is_empty());
            assert_eq!(result.polynomial.unwrap().evaluate(3), q(brute.len()));
        }
    }
}

#[test]
fn uniform_and_graphic_matroids_from_rank_functions() {
    for n in 1..=5 {
        for k in 0..=n {
            check_rank_polytope(n, |s| k.min(s.count_ones() as usize));
        }
    }
    for (vertices, edges) in [
        (3, vec![(0, 1), (1, 2), (2, 0)]),
        (4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]),
        (4, vec![(0, 1), (1, 2), (2, 0), (2, 3), (0, 0), (0, 1)]),
    ] {
        check_rank_polytope(edges.len(), |s| graphic_rank(vertices, &edges, s));
    }
    eprintln!("matroid gate: 20 uniform + 3 graphic matroids, 46 base/independence systems, 276 exact/CRT counts, 92 fits");
}
