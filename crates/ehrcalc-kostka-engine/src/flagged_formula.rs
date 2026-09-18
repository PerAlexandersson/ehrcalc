//! Modular flagged Jacobi--Trudi formulas for structured Kostka families.

use crate::packed_modular::{
    crt_reconstruct_bounded, inverse_mod, validate_crt_moduli, PackedBuildHasher, MAX_CRT_MODULI,
};
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use num_traits::{One, ToPrimitive};
use rayon::prelude::*;
use std::collections::HashMap;

const BLOCK_ROWS: usize = 5;
const TOTAL_ROWS: usize = 10;
const MAX_FORMULA_MODULI: usize = 16;

const FORMULA_MODULI: [u32; 24] = [
    2_147_483_647,
    2_147_483_629,
    2_147_483_587,
    2_147_483_579,
    2_147_483_563,
    2_147_483_549,
    2_147_483_543,
    2_147_483_497,
    2_147_483_489,
    2_147_483_477,
    2_147_483_423,
    2_147_483_399,
    2_147_483_353,
    2_147_483_323,
    2_147_483_269,
    2_147_483_249,
    2_147_483_237,
    2_147_483_179,
    2_147_483_171,
    2_147_483_137,
    2_147_483_123,
    2_147_483_077,
    2_147_483_069,
    2_147_483_059,
];

type LaneValues = [u32; MAX_FORMULA_MODULI];
type Matrix5 = [[u32; BLOCK_ROWS]; BLOCK_ROWS];
type StateMap = HashMap<u64, u128, PackedBuildHasher>;

#[derive(Clone, Copy)]
struct FastModulus {
    value: u32,
    reciprocal: u64,
}

impl FastModulus {
    fn new(value: u32) -> Self {
        Self {
            value,
            reciprocal: ((1_u128 << 64) / u128::from(value)) as u64,
        }
    }

    fn multiply(self, left: u32, right: u32) -> u32 {
        debug_assert!(left < self.value && right < self.value);
        let product = u64::from(left) * u64::from(right);
        let quotient = ((u128::from(product) * u128::from(self.reciprocal)) >> 64) as u64;
        let mut remainder = product - quotient * u64::from(self.value);
        if remainder >= u64::from(self.value) {
            remainder -= u64::from(self.value);
        }
        remainder as u32
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedFlagFormulaStats {
    pub value: BigUint,
    pub certified_upper_bound: BigUint,
    pub residues: Vec<u32>,
    pub moduli: Vec<u32>,
    pub state_counts: Vec<usize>,
}

fn add_mod(left: u32, right: u32, modulus: u32) -> u32 {
    let sum = left + right;
    if sum >= modulus {
        sum - modulus
    } else {
        sum
    }
}

fn sub_mod(left: u32, right: u32, modulus: u32) -> u32 {
    if left >= right {
        left - right
    } else {
        left + modulus - right
    }
}

fn mul_mod(left: u32, right: u32, modulus: u32) -> u32 {
    (u64::from(left) * u64::from(right) % u64::from(modulus)) as u32
}

fn pow_mod(mut base: u32, mut exponent: u32, modulus: u32) -> u32 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

fn determinant_mod(mut matrix: Matrix5, modulus: u32) -> u32 {
    let mut determinant = 1;
    for column in 0..BLOCK_ROWS {
        let Some(pivot_row) = (column..BLOCK_ROWS).find(|&row| matrix[row][column] != 0) else {
            return 0;
        };
        if pivot_row != column {
            matrix.swap(pivot_row, column);
            if determinant != 0 {
                determinant = modulus - determinant;
            }
        }
        let pivot = matrix[column][column];
        determinant = mul_mod(determinant, pivot, modulus);
        let inverse =
            inverse_mod(pivot, modulus).expect("nonzero residue modulo a prime must be invertible");
        let pivot_values = matrix[column];
        for row_values in matrix.iter_mut().skip(column + 1) {
            let factor = mul_mod(row_values[column], inverse, modulus);
            for (entry, &pivot_entry) in row_values
                .iter_mut()
                .zip(pivot_values.iter())
                .skip(column + 1)
            {
                *entry = sub_mod(*entry, mul_mod(factor, pivot_entry, modulus), modulus);
            }
        }
    }
    determinant
}

fn determinant_mod_division_free(matrix: Matrix5, modulus: FastModulus) -> u32 {
    let mut partial = [0_u32; 1 << BLOCK_ROWS];
    partial[0] = 1;
    for mask in 0_usize..(1 << BLOCK_ROWS) {
        let row = mask.count_ones() as usize;
        if row == BLOCK_ROWS {
            continue;
        }
        for (column, &entry) in matrix[row].iter().enumerate() {
            let bit = 1 << column;
            if mask & bit != 0 {
                continue;
            }
            let term = modulus.multiply(partial[mask], entry);
            let next = mask | bit;
            let earlier_greater_columns = (mask >> (column + 1)).count_ones();
            partial[next] = if earlier_greater_columns % 2 == 0 {
                add_mod(partial[next], term, modulus.value)
            } else {
                sub_mod(partial[next], term, modulus.value)
            };
        }
    }
    partial[(1 << BLOCK_ROWS) - 1]
}

fn complete_ones_residues(
    maximum_degree: usize,
    variables: u32,
    moduli: &[u32],
) -> Vec<LaneValues> {
    let mut exact = BigUint::one();
    let mut output = vec![[0; MAX_FORMULA_MODULI]; maximum_degree + 1];
    output[0][..moduli.len()].fill(1);
    for (degree, residues) in output.iter_mut().enumerate().skip(1) {
        exact *= BigUint::from(variables as usize + degree - 1);
        exact /= BigUint::from(degree);
        for (lane, &modulus) in moduli.iter().enumerate() {
            residues[lane] = (&exact % modulus)
                .to_u32()
                .expect("remainder modulo u32 fits u32");
        }
    }
    output
}

struct ModularBlockEvaluator {
    rows: Vec<Vec<[LaneValues; BLOCK_ROWS]>>,
    denominator_inverses: LaneValues,
    fast_moduli: Vec<FastModulus>,
}

impl ModularBlockEvaluator {
    fn new(dilation: u32, remaining_variables: u32, moduli: &[u32]) -> Result<Self, String> {
        let maximum_degree = 2 * dilation as usize + TOTAL_ROWS - 1;
        let first_row = complete_ones_residues(maximum_degree, 2, moduli);
        let remaining = complete_ones_residues(maximum_degree, remaining_variables, moduli);
        let mut rows = vec![
            vec![[[0; MAX_FORMULA_MODULI]; BLOCK_ROWS]; 2 * dilation as usize + 1];
            BLOCK_ROWS
        ];
        let mut denominator_inverses = [0; MAX_FORMULA_MODULI];

        for (lane, &modulus) in moduli.iter().enumerate() {
            let mut a = [[0; BLOCK_ROWS]; BLOCK_ROWS];
            let mut b = [[0; BLOCK_ROWS]; BLOCK_ROWS];
            for row in 0..BLOCK_ROWS {
                let table = if row == 0 { &first_row } else { &remaining };
                for column in 0..TOTAL_ROWS {
                    let degree = 2 * dilation as isize - row as isize + column as isize;
                    let value = if degree < 0 {
                        0
                    } else {
                        table[degree as usize][lane]
                    };
                    if column < BLOCK_ROWS {
                        a[row][column] = value;
                    } else {
                        b[row][column - BLOCK_ROWS] = value;
                    }
                }
            }
            let determinant_a = determinant_mod(a, modulus);
            if determinant_a == 0 {
                return Err(format!(
                    "constant Jacobi--Trudi block is singular modulo {modulus}"
                ));
            }
            denominator_inverses[lane] = inverse_mod(pow_mod(determinant_a, 4, modulus), modulus)
                .expect("nonzero fourth power modulo a prime is invertible");
            let mut adjugate_a_times_b = [[0; BLOCK_ROWS]; BLOCK_ROWS];
            for column in 0..BLOCK_ROWS {
                for variable in 0..BLOCK_ROWS {
                    let mut replaced = a;
                    for row in 0..BLOCK_ROWS {
                        replaced[row][variable] = b[row][column];
                    }
                    adjugate_a_times_b[variable][column] = determinant_mod(replaced, modulus);
                }
            }
            for (row, row_table) in rows.iter_mut().enumerate() {
                let original_row = row + BLOCK_ROWS;
                for (part, transformed_row) in row_table.iter_mut().enumerate() {
                    for (column, transformed_entry) in transformed_row.iter_mut().enumerate() {
                        let d_degree = part as isize - original_row as isize
                            + column as isize
                            + BLOCK_ROWS as isize;
                        let mut value = if d_degree < 0 {
                            0
                        } else {
                            mul_mod(determinant_a, remaining[d_degree as usize][lane], modulus)
                        };
                        for (left_column, adjugate_row) in adjugate_a_times_b.iter().enumerate() {
                            let c_degree =
                                part as isize - original_row as isize + left_column as isize;
                            if c_degree >= 0 {
                                value = sub_mod(
                                    value,
                                    mul_mod(
                                        remaining[c_degree as usize][lane],
                                        adjugate_row[column],
                                        modulus,
                                    ),
                                    modulus,
                                );
                            }
                        }
                        transformed_entry[lane] = value;
                    }
                }
            }
        }
        Ok(Self {
            rows,
            denominator_inverses,
            fast_moduli: moduli.iter().copied().map(FastModulus::new).collect(),
        })
    }

    fn evaluate(&self, bottom: &[u8; BLOCK_ROWS]) -> LaneValues {
        let mut output = [0; MAX_FORMULA_MODULI];
        for (lane, &modulus) in self.fast_moduli.iter().enumerate() {
            let mut matrix = [[0; BLOCK_ROWS]; BLOCK_ROWS];
            for (row, matrix_row) in matrix.iter_mut().enumerate() {
                for (column, entry) in matrix_row.iter_mut().enumerate() {
                    *entry = self.rows[row][bottom[row] as usize][column][lane];
                }
            }
            output[lane] = modulus.multiply(
                determinant_mod_division_free(matrix, modulus),
                self.denominator_inverses[lane],
            );
        }
        output
    }
}

fn pack_bottom(bottom: &[u8; BLOCK_ROWS]) -> u64 {
    bottom.iter().enumerate().fold(0, |packed, (index, &part)| {
        packed | (u64::from(part) << (8 * index))
    })
}

fn unpack_bottom(packed: u64) -> [u8; BLOCK_ROWS] {
    std::array::from_fn(|index| ((packed >> (8 * index)) & 0xff) as u8)
}

fn for_each_large_strip(
    bottom: &[u8; BLOCK_ROWS],
    cap: u32,
    callback: &mut impl FnMut([u8; BLOCK_ROWS]),
) {
    fn visit(
        bottom: &[u8; BLOCK_ROWS],
        current: &mut [u8; BLOCK_ROWS],
        suffix_capacity: &[u32; BLOCK_ROWS + 1],
        cap: u32,
        index: usize,
        used: u32,
        callback: &mut impl FnMut([u8; BLOCK_ROWS]),
    ) {
        if used + suffix_capacity[index] <= cap {
            return;
        }
        if index == BLOCK_ROWS {
            callback(*current);
            return;
        }
        let next = if index + 1 == BLOCK_ROWS {
            0
        } else {
            bottom[index + 1]
        };
        let maximum = bottom[index] - next;
        for delta in 0..=maximum {
            current[index] = bottom[index] - delta;
            visit(
                bottom,
                current,
                suffix_capacity,
                cap,
                index + 1,
                used + u32::from(delta),
                callback,
            );
        }
    }

    let mut suffix_capacity = [0_u32; BLOCK_ROWS + 1];
    for index in (0..BLOCK_ROWS).rev() {
        let next = if index + 1 == BLOCK_ROWS {
            0
        } else {
            bottom[index + 1]
        };
        suffix_capacity[index] = suffix_capacity[index + 1] + u32::from(bottom[index] - next);
    }
    let mut current = *bottom;
    visit(bottom, &mut current, &suffix_capacity, cap, 0, 0, callback);
}

fn large_strip_round(states: StateMap, dilation: u32) -> Result<StateMap, String> {
    if states.is_empty() {
        return Ok(HashMap::with_hasher(PackedBuildHasher::default()));
    }
    let entries: Vec<(u64, u128)> = states.into_iter().collect();
    let workers = rayon::current_num_threads().min(entries.len());
    let chunk_size = entries.len().div_ceil(workers);
    let mut partials: Vec<(StateMap, bool)> = entries
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut following = HashMap::with_hasher(PackedBuildHasher::default());
            let mut overflow = false;
            for &(packed, multiplicity) in chunk {
                let bottom = unpack_bottom(packed);
                for_each_large_strip(&bottom, dilation, &mut |next| {
                    let target = following.entry(pack_bottom(&next)).or_insert(0_u128);
                    if let Some(sum) = target.checked_add(multiplicity) {
                        *target = sum;
                    } else {
                        overflow = true;
                    }
                });
            }
            (following, overflow)
        })
        .collect();
    let largest = partials
        .iter()
        .enumerate()
        .max_by_key(|(_, (partial, _))| partial.len())
        .map(|(index, _)| index)
        .expect("a nonempty state map produces at least one partial map");
    let (mut following, mut overflow) = partials.swap_remove(largest);
    for (partial, partial_overflow) in partials {
        overflow |= partial_overflow;
        for (partition, multiplicity) in partial {
            let target = following.entry(partition).or_insert(0_u128);
            if let Some(sum) = target.checked_add(multiplicity) {
                *target = sum;
            } else {
                overflow = true;
            }
        }
    }
    if overflow {
        Err("128-bit singleton-strip multiplicity overflow".to_string())
    } else {
        Ok(following)
    }
}

fn modular_flagged_sum(
    states: &StateMap,
    dilation: u32,
    remaining_variables: u32,
    moduli: &[u32],
) -> Result<LaneValues, String> {
    let evaluator = ModularBlockEvaluator::new(dilation, remaining_variables, moduli)?;
    Ok(states
        .par_iter()
        .map(|(&packed, &multiplicity)| {
            let values = evaluator.evaluate(&unpack_bottom(packed));
            let mut weighted = [0; MAX_FORMULA_MODULI];
            for (lane, &modulus) in evaluator.fast_moduli.iter().enumerate() {
                let multiplicity = (multiplicity % u128::from(modulus.value)) as u32;
                weighted[lane] = modulus.multiply(values[lane], multiplicity);
            }
            weighted
        })
        .reduce(
            || [0; MAX_FORMULA_MODULI],
            |mut left, right| {
                for (lane, &modulus) in moduli.iter().enumerate() {
                    left[lane] = add_mod(left[lane], right[lane], modulus);
                }
                left
            },
        ))
}

fn rectangle_ssyt_bound(dilation: u32) -> BigUint {
    let width = 2 * dilation as usize;
    let mut bound = BigRational::one();
    for upper_row in 0..TOTAL_ROWS {
        for lower_row in TOTAL_ROWS..17 {
            bound *= BigRational::new(
                BigInt::from(width + lower_row - upper_row),
                BigInt::from(lower_row - upper_row),
            );
        }
    }
    debug_assert!(bound.is_integer());
    bound
        .to_integer()
        .to_biguint()
        .expect("SSYT dimension is nonnegative")
}

fn choose_moduli(upper_bound: &BigUint) -> Result<Vec<u32>, String> {
    let mut product = BigUint::one();
    let mut selected = Vec::new();
    for &modulus in &FORMULA_MODULI {
        selected.push(modulus);
        product *= modulus;
        if product > *upper_bound {
            break;
        }
    }
    if product <= *upper_bound {
        return Err("formula modulus table does not prove the SSYT bound".to_string());
    }
    if selected.len() > MAX_FORMULA_MODULI || selected.len() > MAX_CRT_MODULI {
        return Err("retained-flag formula needs too many CRT moduli".to_string());
    }
    validate_crt_moduli(&selected)?;
    Ok(selected)
}

/// Compute the r5 zero-nonflag-edge candidate's lattice count by packed
/// machine-word arithmetic followed by certified CRT reconstruction.
///
/// After zero-label deletion the shape is `(2t)^10`; the first row has flag
/// two, and five distinguished labels have multiplicity `t`. The remaining
/// flagged Schur counts use a block-reduced Jacobi--Trudi determinant modulo
/// several primes. The unrestricted `s_(2t)^10(1^17)` count is a certified
/// upper bound for CRT uniqueness.
pub fn count_retained_flag_five_singletons(
    dilation: u32,
) -> Result<RetainedFlagFormulaStats, String> {
    if dilation == 0 {
        return Ok(RetainedFlagFormulaStats {
            value: BigUint::one(),
            certified_upper_bound: BigUint::one(),
            residues: vec![1],
            moduli: vec![FORMULA_MODULI[0]],
            state_counts: vec![1],
        });
    }
    if dilation > 127 {
        return Err("dilation exceeds the packed retained-flag limit 127".to_string());
    }
    let certified_upper_bound = rectangle_ssyt_bound(dilation);
    let moduli = choose_moduli(&certified_upper_bound)?;
    let rectangle = [2 * dilation as u8; BLOCK_ROWS];
    let mut states = HashMap::with_hasher(PackedBuildHasher::default());
    states.insert(pack_bottom(&rectangle), 1);
    let mut state_counts = vec![1];
    let mut residues = [0; MAX_FORMULA_MODULI];
    const BINOMIAL_FIVE: [u32; 6] = [1, 5, 10, 10, 5, 1];
    for (selected, &coefficient) in BINOMIAL_FIVE.iter().enumerate() {
        let term = modular_flagged_sum(&states, dilation, 17 - selected as u32, &moduli)?;
        for (lane, &modulus) in moduli.iter().enumerate() {
            let weighted = mul_mod(term[lane], coefficient, modulus);
            residues[lane] = if selected % 2 == 0 {
                add_mod(residues[lane], weighted, modulus)
            } else {
                sub_mod(residues[lane], weighted, modulus)
            };
        }
        if selected + 1 != BINOMIAL_FIVE.len() {
            states = large_strip_round(states, dilation)?;
            state_counts.push(states.len());
        }
    }
    let residues = residues[..moduli.len()].to_vec();
    let value = crt_reconstruct_bounded(&residues, &moduli, &certified_upper_bound)?;
    Ok(RetainedFlagFormulaStats {
        value,
        certified_upper_bound,
        residues,
        moduli,
        state_counts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_modulus_matches_integer_remainder() {
        for value in FORMULA_MODULI.iter().copied().take(8) {
            let modulus = FastModulus::new(value);
            for left in [0, 1, 2, value / 2, value - 2, value - 1] {
                for right in [0, 1, 3, value / 3, value - 2, value - 1] {
                    assert_eq!(
                        modulus.multiply(left, right),
                        (u64::from(left) * u64::from(right) % u64::from(value)) as u32
                    );
                }
            }
        }
    }

    #[test]
    fn rectangle_bound_covers_known_count() {
        let count = BigUint::parse_bytes(
            b"9679019246255852537723258153459155465584838415220597693274170",
            10,
        )
        .unwrap();
        assert!(rectangle_ssyt_bound(31) > count);
    }

    #[test]
    fn division_free_determinant_matches_elimination() {
        let matrix = [
            [4, 1, 7, 9, 2],
            [3, 8, 5, 0, 6],
            [2, 7, 1, 8, 4],
            [9, 0, 6, 3, 5],
            [1, 5, 2, 7, 8],
        ];
        for modulus in FORMULA_MODULI.iter().copied().take(4) {
            assert_eq!(
                determinant_mod_division_free(matrix, FastModulus::new(modulus)),
                determinant_mod(matrix, modulus)
            );
        }
    }

    #[test]
    fn retained_flag_five_singleton_regression() {
        let expected = [
            "1",
            "4880700",
            "1043929527355",
            "26844209466341800",
            "158034549841185046640",
            "313717199442962194213394",
            "269556454373800066261747475",
            "118939336002898491799729593726",
        ];
        let expected_states = [
            vec![1],
            vec![1, 1, 1, 1, 1, 1],
            vec![1, 2, 4, 7, 12, 18],
            vec![1, 3, 9, 23, 50, 99],
            vec![1, 4, 16, 52, 141, 337],
            vec![1, 5, 25, 99, 318, 892],
            vec![1, 6, 36, 167, 622, 1999],
            vec![1, 7, 49, 261, 1103, 3993],
        ];
        for dilation in 0..=7 {
            let stats = count_retained_flag_five_singletons(dilation).unwrap();
            assert_eq!(stats.value.to_string(), expected[dilation as usize]);
            assert_eq!(stats.state_counts, expected_states[dilation as usize]);
            assert!(stats.value <= stats.certified_upper_bound);
        }
    }
}
