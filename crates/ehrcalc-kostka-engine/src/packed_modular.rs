//! Packed modular Kostka counting.
//!
//! This is the CPU reference for a future GPU backend. Intermediate partitions
//! are packed into one `u128`, and up to eight pairwise-coprime modular residues
//! travel together so that transition enumeration is shared across moduli.

use crate::Partition;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

/// Enough lanes for the currently targeted low-dimensional KTT calculations.
pub const MAX_MODULI: usize = 8;

/// Convenient large coprime moduli. All are prime and below `2^31`, so adding
/// two reduced residues cannot overflow a `u32`.
pub const DEFAULT_MODULI: [u32; MAX_MODULI] = [
    2_147_483_647,
    2_147_483_629,
    2_147_483_587,
    2_147_483_579,
    2_147_483_563,
    2_147_483_549,
    2_147_483_543,
    2_147_483_497,
];

const MAX_PACKED_ROWS: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModularKostkaStats {
    pub residues: Vec<u32>,
    pub moduli: Vec<u32>,
    pub peak_states: usize,
    pub level_states: Vec<usize>,
    pub level_transitions: Vec<u64>,
}

/// One contribution emitted while advancing a packed modular DP layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackedModularRecord {
    pub key: u128,
    pub value: u32,
}

/// Raw contributions and independently hash-reduced output for one real DP
/// layer. This diagnostic representation is intended for backend validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackedModularLayerTrace {
    pub rows: usize,
    pub bits_per_row: u32,
    pub modulus: u32,
    pub source_states: usize,
    pub records: Vec<PackedModularRecord>,
    pub reduced: Vec<PackedModularRecord>,
}

impl ModularKostkaStats {
    /// Reconstruct the unique nonnegative answer not exceeding `upper_bound`.
    ///
    /// The product of the moduli must be strictly larger than the supplied
    /// certified bound. This prevents an apparently stable, but ambiguous, CRT
    /// value from being accepted as exact.
    pub fn reconstruct_bounded(&self, upper_bound: &BigUint) -> Result<BigUint, String> {
        crt_reconstruct_bounded(&self.residues, &self.moduli, upper_bound)
    }
}

#[derive(Clone, Copy, Debug)]
struct PackedPartitions {
    rows: usize,
    bits: u32,
    mask: u128,
}

impl PackedPartitions {
    fn new(lambda: &Partition) -> Result<Self, String> {
        let rows = lambda.num_parts();
        if rows > MAX_PACKED_ROWS {
            return Err(format!(
                "partition has {rows} rows; packed backend supports at most {MAX_PACKED_ROWS}"
            ));
        }
        let largest = lambda.part(0);
        let bits = (u32::BITS - largest.leading_zeros()).max(1);
        if rows.saturating_mul(bits as usize) > u128::BITS as usize {
            return Err(format!(
                "partition needs {} packed bits; maximum is {}",
                rows * bits as usize,
                u128::BITS
            ));
        }
        let mask = if bits == u128::BITS {
            u128::MAX
        } else {
            (1_u128 << bits) - 1
        };
        Ok(Self { rows, bits, mask })
    }

    fn pack_partition(self, partition: &Partition) -> Result<u128, String> {
        if partition.num_parts() > self.rows {
            return Err("partition has more rows than the outer shape".to_string());
        }
        let mut parts = [0_u32; MAX_PACKED_ROWS];
        parts[..partition.num_parts()].copy_from_slice(partition.parts());
        self.pack_parts(&parts)
    }

    fn pack_parts(self, parts: &[u32; MAX_PACKED_ROWS]) -> Result<u128, String> {
        let mut key = 0_u128;
        for (row, &part) in parts.iter().take(self.rows).enumerate() {
            if u128::from(part) > self.mask {
                return Err(format!("row {row} value {part} exceeds packed field width"));
            }
            key |= u128::from(part) << (self.bits as usize * row);
        }
        Ok(key)
    }

    fn unpack(self, key: u128) -> [u32; MAX_PACKED_ROWS] {
        let mut parts = [0_u32; MAX_PACKED_ROWS];
        for (row, part) in parts.iter_mut().take(self.rows).enumerate() {
            *part = ((key >> (self.bits as usize * row)) & self.mask) as u32;
        }
        parts
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Residues([u32; MAX_MODULI]);

impl Residues {
    fn one(lanes: usize) -> Self {
        let mut values = [0; MAX_MODULI];
        values[..lanes].fill(1);
        Self(values)
    }

    fn add_assign(&mut self, other: Self, moduli: &[u32]) {
        for (lane, &modulus) in moduli.iter().enumerate() {
            let sum = self.0[lane] + other.0[lane];
            self.0[lane] = if sum >= modulus { sum - modulus } else { sum };
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PackedHasher(u64);

impl Default for PackedHasher {
    fn default() -> Self {
        Self(0x9e37_79b9_7f4a_7c15)
    }
}

impl Hasher for PackedHasher {
    fn finish(&self) -> u64 {
        avalanche(self.0)
    }

    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut word = 0_u64;
            for (shift, byte) in chunk.iter().enumerate() {
                word |= u64::from(*byte) << (8 * shift);
            }
            self.0 = self
                .0
                .wrapping_mul(0x9e37_79b9_7f4a_7c15)
                .wrapping_add(word.rotate_left(27));
        }
    }

    fn write_u128(&mut self, value: u128) {
        self.0 = value as u64 ^ ((value >> 64) as u64).rotate_left(29);
    }
}

fn avalanche(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

type PackedBuildHasher = BuildHasherDefault<PackedHasher>;
type StateMap = HashMap<u128, Residues, PackedBuildHasher>;

fn state_map_with_capacity(capacity: usize) -> StateMap {
    HashMap::with_capacity_and_hasher(capacity, PackedBuildHasher::default())
}

struct ExtensionContext<'a> {
    packer: PackedPartitions,
    lambda: &'a [u32; MAX_PACKED_ROWS],
    row_lo: usize,
    row_hi: usize,
}

impl ExtensionContext<'_> {
    fn visit_extensions<F>(
        &self,
        alpha: &[u32; MAX_PACKED_ROWS],
        strip_size: u32,
        mut visit: F,
    ) -> Result<(), String>
    where
        F: FnMut(u128),
    {
        let mut beta = *alpha;
        self.extend(alpha, strip_size, 0, &mut beta, &mut visit)
    }

    fn extend<F>(
        &self,
        alpha: &[u32; MAX_PACKED_ROWS],
        remaining: u32,
        row: usize,
        beta: &mut [u32; MAX_PACKED_ROWS],
        visit: &mut F,
    ) -> Result<(), String>
    where
        F: FnMut(u128),
    {
        if row == self.packer.rows {
            if remaining == 0 {
                visit(self.packer.pack_parts(beta)?);
            }
            return Ok(());
        }

        let base = alpha[row];
        if row < self.row_lo || row >= self.row_hi {
            beta[row] = base;
            return self.extend(alpha, remaining, row + 1, beta, visit);
        }

        let shape_capacity = self.lambda[row].saturating_sub(base);
        let strip_capacity = if row == 0 {
            remaining
        } else {
            alpha[row - 1].saturating_sub(base)
        };
        let maximum = remaining.min(shape_capacity).min(strip_capacity);
        for increment in 0..=maximum {
            beta[row] = base + increment;
            self.extend(alpha, remaining - increment, row + 1, beta, visit)?;
        }
        beta[row] = base;
        Ok(())
    }
}

/// Count a skew or flagged skew Kostka coefficient modulo several moduli.
///
/// When flags are absent, `sort_weight` may be used to reduce peak state count.
/// Flag entries use the same one-indexed row convention as
/// `kostka_dp::try_flagged_skew_kostka`.
#[allow(clippy::too_many_arguments)]
pub fn try_flagged_skew_kostka_modular_stats(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    moduli: &[u32],
    max_states: Option<usize>,
    sort_weight: bool,
) -> Result<ModularKostkaStats, String> {
    validate_moduli(moduli)?;
    if (upper_flags.is_some() || lower_flags.is_some()) && sort_weight {
        return Err("weight sorting is invalid when row flags are active".to_string());
    }

    let skew_size = lambda.size().saturating_sub(mu.size());
    let weight_size: u32 = weight.iter().sum();
    if skew_size != weight_size || !mu.partition_less_equal(lambda) {
        return Ok(zero_stats(moduli));
    }

    let packer = PackedPartitions::new(lambda)?;
    let lambda_key = packer.pack_partition(lambda)?;
    let mu_key = packer.pack_partition(mu)?;
    let mut lambda_parts = [0_u32; MAX_PACKED_ROWS];
    lambda_parts[..lambda.num_parts()].copy_from_slice(lambda.parts());

    let mut sorted_weight;
    let effective_weight = if sort_weight {
        sorted_weight = weight.to_vec();
        sorted_weight.sort_unstable_by(|left, right| right.cmp(left));
        sorted_weight.as_slice()
    } else {
        weight
    };

    let mut states = state_map_with_capacity(1);
    states.insert(mu_key, Residues::one(moduli.len()));
    let mut peak_states = 1;
    let mut level_states = vec![1];
    let mut level_transitions = Vec::with_capacity(effective_weight.len());

    for (label, &strip_size) in effective_weight.iter().enumerate() {
        if strip_size == 0 {
            level_states.push(states.len());
            level_transitions.push(states.len() as u64);
            continue;
        }

        let row_lo = lower_flags
            .and_then(|flags| flags.get(label))
            .map(|&flag| (flag as usize).saturating_sub(1).min(packer.rows))
            .unwrap_or(0);
        let row_hi = upper_flags
            .and_then(|flags| flags.get(label))
            .map(|&flag| (flag as usize).min(packer.rows))
            .unwrap_or(packer.rows);
        let context = ExtensionContext {
            packer,
            lambda: &lambda_parts,
            row_lo,
            row_hi,
        };
        let mut next = state_map_with_capacity(states.len().saturating_mul(2));
        let mut transitions = 0_u64;
        for (&key, &count) in &states {
            let alpha = packer.unpack(key);
            context.visit_extensions(&alpha, strip_size, |target| {
                transitions += 1;
                next.entry(target)
                    .or_insert(Residues([0; MAX_MODULI]))
                    .add_assign(count, moduli);
            })?;
        }
        if let Some(limit) = max_states {
            if next.len() > limit {
                return Err(format!(
                    "DP state count {} exceeds --max-states {}.",
                    next.len(),
                    limit
                ));
            }
        }
        peak_states = peak_states.max(next.len());
        level_states.push(next.len());
        level_transitions.push(transitions);
        states = next;
    }

    let residues = states
        .remove(&lambda_key)
        .unwrap_or(Residues([0; MAX_MODULI]));
    Ok(ModularKostkaStats {
        residues: residues.0[..moduli.len()].to_vec(),
        moduli: moduli.to_vec(),
        peak_states,
        level_states,
        level_transitions,
    })
}

/// Unflagged convenience wrapper.
pub fn try_skew_kostka_modular_stats(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    moduli: &[u32],
    max_states: Option<usize>,
    sort_weight: bool,
) -> Result<ModularKostkaStats, String> {
    try_flagged_skew_kostka_modular_stats(
        lambda,
        mu,
        weight,
        None,
        None,
        moduli,
        max_states,
        sort_weight,
    )
}

/// Materialize one real flagged-skew DP layer for validating an alternative
/// aggregation backend. `layer` is zero-indexed in the supplied weight order.
///
/// This intentionally supports one modulus: large traces are diagnostic data,
/// while the regular counter shares transition enumeration across all lanes.
#[allow(clippy::too_many_arguments)]
pub fn try_flagged_skew_kostka_modular_layer_trace(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    modulus: u32,
    layer: usize,
    max_states: Option<usize>,
) -> Result<PackedModularLayerTrace, String> {
    validate_moduli(&[modulus])?;
    if layer >= weight.len() {
        return Err(format!(
            "layer {layer} is outside weight length {}",
            weight.len()
        ));
    }
    let skew_size = lambda.size().saturating_sub(mu.size());
    let weight_size: u32 = weight.iter().sum();
    if skew_size != weight_size || !mu.partition_less_equal(lambda) {
        return Err("shape and weight do not define a nonempty compatible DP".to_string());
    }

    let packer = PackedPartitions::new(lambda)?;
    let mu_key = packer.pack_partition(mu)?;
    let mut lambda_parts = [0_u32; MAX_PACKED_ROWS];
    lambda_parts[..lambda.num_parts()].copy_from_slice(lambda.parts());
    let moduli = [modulus];
    let mut states = state_map_with_capacity(1);
    states.insert(mu_key, Residues::one(1));

    for (label, &strip_size) in weight.iter().enumerate().take(layer + 1) {
        let row_lo = lower_flags
            .and_then(|flags| flags.get(label))
            .map(|&flag| (flag as usize).saturating_sub(1).min(packer.rows))
            .unwrap_or(0);
        let row_hi = upper_flags
            .and_then(|flags| flags.get(label))
            .map(|&flag| (flag as usize).min(packer.rows))
            .unwrap_or(packer.rows);
        let context = ExtensionContext {
            packer,
            lambda: &lambda_parts,
            row_lo,
            row_hi,
        };
        let mut next = state_map_with_capacity(states.len().saturating_mul(2));
        let mut records = if label == layer {
            Some(Vec::new())
        } else {
            None
        };

        for (&key, &count) in &states {
            if strip_size == 0 {
                if let Some(records) = &mut records {
                    records.push(PackedModularRecord {
                        key,
                        value: count.0[0],
                    });
                }
                next.entry(key)
                    .or_insert(Residues([0; MAX_MODULI]))
                    .add_assign(count, &moduli);
                continue;
            }
            let alpha = packer.unpack(key);
            context.visit_extensions(&alpha, strip_size, |target| {
                if let Some(records) = &mut records {
                    records.push(PackedModularRecord {
                        key: target,
                        value: count.0[0],
                    });
                }
                next.entry(target)
                    .or_insert(Residues([0; MAX_MODULI]))
                    .add_assign(count, &moduli);
            })?;
        }

        if let Some(limit) = max_states {
            if next.len() > limit {
                return Err(format!(
                    "DP state count {} exceeds --max-states {}.",
                    next.len(),
                    limit
                ));
            }
        }

        if label == layer {
            let mut reduced: Vec<PackedModularRecord> = next
                .into_iter()
                .map(|(key, value)| PackedModularRecord {
                    key,
                    value: value.0[0],
                })
                .collect();
            reduced.sort_unstable_by_key(|record| record.key);
            return Ok(PackedModularLayerTrace {
                rows: packer.rows,
                bits_per_row: packer.bits,
                modulus,
                source_states: states.len(),
                records: records.expect("selected layer must collect records"),
                reduced,
            });
        }
        states = next;
    }

    unreachable!("validated layer must be visited")
}

fn zero_stats(moduli: &[u32]) -> ModularKostkaStats {
    ModularKostkaStats {
        residues: vec![0; moduli.len()],
        moduli: moduli.to_vec(),
        peak_states: 0,
        level_states: Vec::new(),
        level_transitions: Vec::new(),
    }
}

fn validate_moduli(moduli: &[u32]) -> Result<(), String> {
    if moduli.is_empty() || moduli.len() > MAX_MODULI {
        return Err(format!(
            "expected 1..={MAX_MODULI} moduli, received {}",
            moduli.len()
        ));
    }
    for (index, &modulus) in moduli.iter().enumerate() {
        if !(2..(1_u32 << 31)).contains(&modulus) {
            return Err(format!("modulus {modulus} must lie in 2..2^31"));
        }
        for &previous in &moduli[..index] {
            if gcd_u32(previous, modulus) != 1 {
                return Err(format!("moduli {previous} and {modulus} are not coprime"));
            }
        }
    }
    Ok(())
}

fn gcd_u32(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

/// Reconstruct a nonnegative integer from pairwise-coprime residues, rejecting
/// the result unless the combined modulus proves uniqueness below `upper_bound`.
pub fn crt_reconstruct_bounded(
    residues: &[u32],
    moduli: &[u32],
    upper_bound: &BigUint,
) -> Result<BigUint, String> {
    validate_moduli(moduli)?;
    if residues.len() != moduli.len() {
        return Err("residue and modulus lengths differ".to_string());
    }
    for (&residue, &modulus) in residues.iter().zip(moduli) {
        if residue >= modulus {
            return Err(format!("residue {residue} is not reduced modulo {modulus}"));
        }
    }

    let combined_modulus = moduli
        .iter()
        .fold(BigUint::one(), |product, &modulus| product * modulus);
    if combined_modulus <= *upper_bound {
        return Err(format!(
            "combined modulus has {} bits but does not exceed the certified bound",
            combined_modulus.bits()
        ));
    }

    let mut value = BigUint::zero();
    let mut product = BigUint::one();
    for (&residue, &modulus) in residues.iter().zip(moduli) {
        let value_mod = (&value % modulus)
            .to_u32()
            .expect("remainder modulo u32 must fit u32");
        let product_mod = (&product % modulus)
            .to_u32()
            .expect("remainder modulo u32 must fit u32");
        let delta = if residue >= value_mod {
            residue - value_mod
        } else {
            residue + modulus - value_mod
        };
        let inverse = inverse_mod(product_mod, modulus)
            .ok_or_else(|| format!("modulus {modulus} is not coprime to prior product"))?;
        let multiplier = (u64::from(delta) * u64::from(inverse) % u64::from(modulus)) as u32;
        value += &product * multiplier;
        product *= modulus;
    }

    if value > *upper_bound {
        return Err("CRT result exceeds the certified upper bound".to_string());
    }
    Ok(value)
}

fn inverse_mod(value: u32, modulus: u32) -> Option<u32> {
    let (mut old_r, mut r) = (i64::from(value), i64::from(modulus));
    let (mut old_s, mut s) = (1_i64, 0_i64);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }
    if old_r != 1 {
        return None;
    }
    Some(old_s.rem_euclid(i64::from(modulus)) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kostka_dp::{skew_kostka, try_flagged_skew_kostka};

    fn compositions(total: u32, parts: usize) -> Vec<Vec<u32>> {
        fn visit(total: u32, parts: usize, prefix: &mut Vec<u32>, output: &mut Vec<Vec<u32>>) {
            if parts == 1 {
                prefix.push(total);
                output.push(prefix.clone());
                prefix.pop();
                return;
            }
            for value in 0..=total {
                prefix.push(value);
                visit(total - value, parts - 1, prefix, output);
                prefix.pop();
            }
        }

        let mut output = Vec::new();
        visit(total, parts, &mut Vec::new(), &mut output);
        output
    }

    #[test]
    fn crt_requires_a_proving_bound() {
        let value = BigUint::from(9_876_543_210_123_456_789_u128);
        let moduli = &DEFAULT_MODULI[..3];
        let residues: Vec<u32> = moduli
            .iter()
            .map(|&modulus| (&value % modulus).to_u32().unwrap())
            .collect();
        assert_eq!(
            crt_reconstruct_bounded(&residues, moduli, &value).unwrap(),
            value
        );

        let ambiguous_bound = moduli
            .iter()
            .fold(BigUint::one(), |product, &modulus| product * modulus);
        assert!(crt_reconstruct_bounded(&residues, moduli, &ambiguous_bound).is_err());
    }

    #[test]
    fn modular_matches_small_skew_kostka_suite() {
        let moduli = &DEFAULT_MODULI[..2];
        for size in 0..=8 {
            for lambda in Partition::all_of_size(size) {
                for inner_size in 0..=size {
                    for mu in Partition::all_of_size(inner_size) {
                        if !mu.partition_less_equal(&lambda) {
                            continue;
                        }
                        let skew_size = size - inner_size;
                        for weight in compositions(skew_size, 3) {
                            let expected = skew_kostka(&lambda, &mu, &weight, None, false);
                            let actual = try_skew_kostka_modular_stats(
                                &lambda, &mu, &weight, moduli, None, false,
                            )
                            .unwrap();
                            for (residue, modulus) in actual.residues.iter().zip(moduli) {
                                assert_eq!(
                                    *residue,
                                    (&expected % *modulus).to_u32().unwrap(),
                                    "lambda={lambda} mu={mu} weight={weight:?}"
                                );
                            }
                            assert_eq!(
                                actual.reconstruct_bounded(&expected).unwrap(),
                                expected,
                                "lambda={lambda} mu={mu} weight={weight:?}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn modular_matches_flagged_counts() {
        let lambda = Partition::from_sorted(vec![4, 3, 1]);
        let mu = Partition::from_sorted(vec![1]);
        let weight = [2, 3, 2];
        let upper = [2, 3, 3];
        let lower = [1, 1, 2];
        let expected =
            try_flagged_skew_kostka(&lambda, &mu, &weight, Some(&upper), Some(&lower), None)
                .unwrap();
        let actual = try_flagged_skew_kostka_modular_stats(
            &lambda,
            &mu,
            &weight,
            Some(&upper),
            Some(&lower),
            &DEFAULT_MODULI[..2],
            None,
            false,
        )
        .unwrap();
        assert_eq!(actual.reconstruct_bounded(&expected).unwrap(), expected);
        assert_eq!(actual.level_states.len(), weight.len() + 1);
        assert_eq!(actual.level_transitions.len(), weight.len());
    }

    #[test]
    fn layer_trace_sort_reduction_matches_hash_reduction() {
        let lambda = Partition::from_sorted(vec![5, 4, 2]);
        let mu = Partition::from_sorted(vec![1]);
        let weight = [3, 4, 3];
        let modulus = DEFAULT_MODULI[0];
        let trace = try_flagged_skew_kostka_modular_layer_trace(
            &lambda, &mu, &weight, None, None, modulus, 1, None,
        )
        .unwrap();

        let mut records = trace.records;
        records.sort_unstable_by_key(|record| record.key);
        let mut reduced = Vec::<PackedModularRecord>::new();
        for record in records {
            if let Some(previous) = reduced.last_mut() {
                if previous.key == record.key {
                    let sum = previous.value + record.value;
                    previous.value = if sum >= modulus { sum - modulus } else { sum };
                    continue;
                }
            }
            reduced.push(record);
        }
        assert_eq!(reduced, trace.reduced);
    }

    #[test]
    fn rejects_non_coprime_moduli_and_too_wide_states() {
        let lambda = Partition::from_sorted(vec![1; 33]);
        assert!(try_skew_kostka_modular_stats(
            &lambda,
            &Partition::empty(),
            &[33],
            &[15, 21],
            None,
            false,
        )
        .is_err());
        assert!(try_skew_kostka_modular_stats(
            &lambda,
            &Partition::empty(),
            &[33],
            &DEFAULT_MODULI[..1],
            None,
            false,
        )
        .is_err());
    }
}
