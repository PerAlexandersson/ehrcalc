use crate::Partition;
/// Standard Young tableaux count via the hook-length formula.
///   f^lambda = n! / prod of all hook lengths
use num_bigint::BigUint;
use num_traits::One;

/// Compute hook lengths for all cells of lambda.
/// hook(i, j) = lambda[i] - j + lambda'[j] - i - 1  (0-indexed i, j).
pub fn hook_lengths(lambda: &Partition) -> Vec<Vec<u32>> {
    let conj = lambda.conjugate_partition();
    lambda
        .parts()
        .iter()
        .enumerate()
        .map(|(i, &row_len)| {
            (0..row_len as usize)
                .map(|j| {
                    let arm = row_len - j as u32 - 1;
                    let leg = conj.part(j).saturating_sub(i as u32 + 1);
                    arm + leg + 1
                })
                .collect()
        })
        .collect()
}

/// Count SYT of shape lambda using the hook-length formula.
pub fn count_syt(lambda: &Partition) -> BigUint {
    let n = lambda.size() as u64;
    // n!
    let mut numerator = BigUint::one();
    for i in 2..=n {
        numerator *= i;
    }
    // divide by each hook length
    let hooks = hook_lengths(lambda);
    for row in &hooks {
        for &h in row {
            numerator /= h as u64;
        }
    }
    numerator
}
