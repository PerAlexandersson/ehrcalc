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

/// Maximum number of sequential residue batches accepted by CRT
/// reconstruction. Device DPs remain limited to [`MAX_MODULI`] simultaneous
/// lanes, while exact drivers may combine many independent batches.
pub const MAX_CRT_MODULI: usize = 64;

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
    level_lower: &'a [u32],
    level_upper: &'a [u32],
    row_lo: usize,
    row_hi: usize,
    forbidden_mask: u32,
    strict_lower_mask: u32,
    strict_diagonal_mask: u32,
}

impl ExtensionContext<'_> {
    fn visit_extensions<F>(
        &self,
        alpha: &[u32; MAX_PACKED_ROWS],
        strip_size: u32,
        mut visit: F,
    ) -> Result<(), String>
    where
        F: FnMut(u128) -> Result<(), String>,
    {
        let mut beta = *alpha;
        let mut row_minimum = [0_u32; MAX_PACKED_ROWS];
        let mut row_maximum = [0_u32; MAX_PACKED_ROWS];
        let mut suffix_minimum = [0_u32; MAX_PACKED_ROWS + 1];
        let mut suffix_maximum = [0_u32; MAX_PACKED_ROWS + 1];
        for row in 0..self.packer.rows {
            let Some((minimum, maximum)) = self.extension_bounds(alpha, row) else {
                return Ok(());
            };
            row_minimum[row] = minimum;
            row_maximum[row] = maximum;
        }
        for row in (0..self.packer.rows).rev() {
            suffix_minimum[row] = suffix_minimum[row + 1].saturating_add(row_minimum[row]);
            suffix_maximum[row] = suffix_maximum[row + 1]
                .saturating_add(row_maximum[row])
                .min(strip_size);
        }
        if strip_size < suffix_minimum[0] || strip_size > suffix_maximum[0] {
            return Ok(());
        }
        self.extend(
            alpha,
            strip_size,
            0,
            &row_minimum,
            &row_maximum,
            &suffix_minimum,
            &suffix_maximum,
            &mut beta,
            &mut visit,
        )
    }

    fn extension_bounds(&self, alpha: &[u32; MAX_PACKED_ROWS], row: usize) -> Option<(u32, u32)> {
        let forced_zero =
            row < self.row_lo || row >= self.row_hi || self.forbidden_mask & (1_u32 << row) != 0;
        let need_strict_lower = self.strict_lower_mask & (1_u32 << row) != 0;
        let need_strict_diagonal = row > 0 && self.strict_diagonal_mask & (1_u32 << row) != 0;
        let base = alpha[row];
        let gap = if row == 0 {
            u32::MAX
        } else {
            alpha[row - 1].saturating_sub(base)
        };
        let (mut minimum, mut maximum) = if forced_zero {
            if need_strict_lower || (need_strict_diagonal && gap == 0) {
                return None;
            }
            (0, 0)
        } else {
            let minimum = u32::from(need_strict_lower);
            let diagonal_capacity = if need_strict_diagonal {
                gap.checked_sub(1)?
            } else {
                gap
            };
            let maximum = self.lambda[row].saturating_sub(base).min(diagonal_capacity);
            (minimum, maximum)
        };

        let level_upper = self.level_upper[row];
        if base > level_upper {
            return None;
        }
        minimum = minimum.max(self.level_lower[row].saturating_sub(base));
        maximum = maximum.min(level_upper - base);
        (minimum <= maximum).then_some((minimum, maximum))
    }

    #[allow(clippy::too_many_arguments)]
    fn extend<F>(
        &self,
        alpha: &[u32; MAX_PACKED_ROWS],
        remaining: u32,
        row: usize,
        row_minimum: &[u32; MAX_PACKED_ROWS],
        row_maximum: &[u32; MAX_PACKED_ROWS],
        suffix_minimum: &[u32; MAX_PACKED_ROWS + 1],
        suffix_maximum: &[u32; MAX_PACKED_ROWS + 1],
        beta: &mut [u32; MAX_PACKED_ROWS],
        visit: &mut F,
    ) -> Result<(), String>
    where
        F: FnMut(u128) -> Result<(), String>,
    {
        if row == self.packer.rows {
            if remaining == 0 {
                visit(self.packer.pack_parts(beta)?)?;
            }
            return Ok(());
        }
        if remaining < suffix_minimum[row] || remaining > suffix_maximum[row] {
            return Ok(());
        }

        let base = alpha[row];
        let minimum = row_minimum[row].max(remaining.saturating_sub(suffix_maximum[row + 1]));
        let maximum = remaining
            .min(row_maximum[row])
            .min(remaining.saturating_sub(suffix_minimum[row + 1]));
        for increment in minimum..=maximum {
            beta[row] = base + increment;
            self.extend(
                alpha,
                remaining - increment,
                row + 1,
                row_minimum,
                row_maximum,
                suffix_minimum,
                suffix_maximum,
                beta,
                visit,
            )?;
        }
        beta[row] = base;
        Ok(())
    }
}

fn partition_size_u64(partition: &Partition) -> u64 {
    partition.parts().iter().map(|&part| u64::from(part)).sum()
}

fn weight_size_u64(weight: &[u32]) -> Result<u64, String> {
    weight.iter().try_fold(0_u64, |total, &part| {
        total
            .checked_add(u64::from(part))
            .ok_or_else(|| "weight size exceeds u64".to_string())
    })
}

fn validate_constraint_masks(
    name: &str,
    masks: Option<&[u32]>,
    weight_len: usize,
    rows: usize,
) -> Result<(), String> {
    let Some(masks) = masks else {
        return Ok(());
    };
    if masks.len() != weight_len {
        return Err(format!(
            "{name} length {} does not match weight length {weight_len}",
            masks.len()
        ));
    }
    let allowed = if rows == u32::BITS as usize {
        u32::MAX
    } else {
        (1_u32 << rows) - 1
    };
    if let Some((label, mask)) = masks
        .iter()
        .enumerate()
        .find(|(_, mask)| **mask & !allowed != 0)
    {
        return Err(format!(
            "{name} for label {} contains a bit outside the {rows} shape rows: {mask:#x}",
            label + 1
        ));
    }
    Ok(())
}

/// Count a constrained skew Kostka coefficient modulo several moduli.
///
/// The row-mask convention agrees with
/// `kostka_dp::try_masked_flagged_skew_kostka`. This supports individual
/// Kogan faces after a complement-row lift, as well as explicit
/// relative-interior masks. Weight sorting is valid only when every constraint
/// argument is absent.
#[allow(clippy::too_many_arguments)]
pub fn try_masked_flagged_skew_kostka_modular_stats(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
    strict_lower_masks: Option<&[u32]>,
    strict_diagonal_masks: Option<&[u32]>,
    moduli: &[u32],
    max_states: Option<usize>,
    sort_weight: bool,
) -> Result<ModularKostkaStats, String> {
    validate_moduli(moduli)?;
    if (upper_flags.is_some()
        || lower_flags.is_some()
        || forbidden_row_masks.is_some()
        || strict_lower_masks.is_some()
        || strict_diagonal_masks.is_some())
        && sort_weight
    {
        return Err("weight sorting is invalid when row constraints are active".to_string());
    }

    if !mu.partition_less_equal(lambda) {
        return Ok(zero_stats(moduli));
    }
    let skew_size = partition_size_u64(lambda) - partition_size_u64(mu);
    let weight_size = weight_size_u64(weight)?;
    if skew_size != weight_size {
        return Ok(zero_stats(moduli));
    }

    let packer = PackedPartitions::new(lambda)?;
    if upper_flags.is_some_and(|flags| flags.len() != weight.len())
        || lower_flags.is_some_and(|flags| flags.len() != weight.len())
    {
        return Err("flag lengths must match the weight length".to_string());
    }
    validate_constraint_masks(
        "forbidden-row mask",
        forbidden_row_masks,
        weight.len(),
        packer.rows,
    )?;
    validate_constraint_masks(
        "strict-lower mask",
        strict_lower_masks,
        weight.len(),
        packer.rows,
    )?;
    validate_constraint_masks(
        "strict-diagonal mask",
        strict_diagonal_masks,
        weight.len(),
        packer.rows,
    )?;
    if strict_diagonal_masks.is_some_and(|masks| masks.iter().any(|mask| mask & 1 != 0)) {
        return Err("strict-diagonal masks cannot contain the first-row bit".to_string());
    }
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
    let Some((_, level_lower_bounds, level_upper_bounds)) =
        crate::gt_dim::gt_polytope_bounds_masked(
            lambda.parts(),
            mu.parts(),
            effective_weight,
            upper_flags,
            lower_flags,
            forbidden_row_masks,
        )
    else {
        return Ok(zero_stats(moduli));
    };

    let mut states = state_map_with_capacity(1);
    states.insert(mu_key, Residues::one(moduli.len()));
    let mut peak_states = 1;
    let mut level_states = vec![1];
    let mut level_transitions = Vec::with_capacity(effective_weight.len());

    for (label, &strip_size) in effective_weight.iter().enumerate() {
        if strip_size == 0 {
            if strict_lower_masks.is_some_and(|masks| masks[label] != 0)
                || strict_diagonal_masks.is_some_and(|masks| masks[label] != 0)
            {
                return Err("zero-weight labels must have zero strictness masks".to_string());
            }
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
        let (level_lower, level_upper) = if label + 1 == effective_weight.len() {
            (&lambda_parts[..packer.rows], &lambda_parts[..packer.rows])
        } else {
            (
                level_lower_bounds[label].as_slice(),
                level_upper_bounds[label].as_slice(),
            )
        };
        let context = ExtensionContext {
            packer,
            lambda: &lambda_parts,
            level_lower,
            level_upper,
            row_lo,
            row_hi,
            forbidden_mask: forbidden_row_masks.map_or(0, |masks| masks[label]),
            strict_lower_mask: strict_lower_masks.map_or(0, |masks| masks[label]),
            strict_diagonal_mask: strict_diagonal_masks.map_or(0, |masks| masks[label]),
        };
        let mut next = state_map_with_capacity(states.len().saturating_mul(2));
        let mut transitions = 0_u64;
        for (&key, &count) in &states {
            let alpha = packer.unpack(key);
            context.visit_extensions(&alpha, strip_size, |target| {
                transitions = transitions
                    .checked_add(1)
                    .ok_or_else(|| "transition count exceeds u64".to_string())?;
                next.entry(target)
                    .or_insert(Residues([0; MAX_MODULI]))
                    .add_assign(count, moduli);
                if let Some(limit) = max_states {
                    if next.len() > limit {
                        return Err(format!(
                            "DP state count {} exceeds --max-states {}.",
                            next.len(),
                            limit
                        ));
                    }
                }
                Ok(())
            })?;
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

/// Count an ordinary or row-flagged skew Kostka coefficient modulo several
/// moduli.
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
    try_masked_flagged_skew_kostka_modular_stats(
        lambda,
        mu,
        weight,
        upper_flags,
        lower_flags,
        None,
        None,
        None,
        moduli,
        max_states,
        sort_weight,
    )
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
pub fn try_masked_flagged_skew_kostka_modular_layer_trace(
    lambda: &Partition,
    mu: &Partition,
    weight: &[u32],
    upper_flags: Option<&[u32]>,
    lower_flags: Option<&[u32]>,
    forbidden_row_masks: Option<&[u32]>,
    strict_lower_masks: Option<&[u32]>,
    strict_diagonal_masks: Option<&[u32]>,
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
    if !mu.partition_less_equal(lambda) {
        return Err("shape and weight do not define a nonempty compatible DP".to_string());
    }
    let skew_size = partition_size_u64(lambda) - partition_size_u64(mu);
    let weight_size = weight_size_u64(weight)?;
    if skew_size != weight_size {
        return Err("shape and weight do not define a nonempty compatible DP".to_string());
    }

    let packer = PackedPartitions::new(lambda)?;
    if upper_flags.is_some_and(|flags| flags.len() != weight.len())
        || lower_flags.is_some_and(|flags| flags.len() != weight.len())
    {
        return Err("flag lengths must match the weight length".to_string());
    }
    validate_constraint_masks(
        "forbidden-row mask",
        forbidden_row_masks,
        weight.len(),
        packer.rows,
    )?;
    validate_constraint_masks(
        "strict-lower mask",
        strict_lower_masks,
        weight.len(),
        packer.rows,
    )?;
    validate_constraint_masks(
        "strict-diagonal mask",
        strict_diagonal_masks,
        weight.len(),
        packer.rows,
    )?;
    if strict_diagonal_masks.is_some_and(|masks| masks.iter().any(|mask| mask & 1 != 0)) {
        return Err("strict-diagonal masks cannot contain the first-row bit".to_string());
    }
    let mu_key = packer.pack_partition(mu)?;
    let mut lambda_parts = [0_u32; MAX_PACKED_ROWS];
    lambda_parts[..lambda.num_parts()].copy_from_slice(lambda.parts());
    let (_, level_lower_bounds, level_upper_bounds) = crate::gt_dim::gt_polytope_bounds_masked(
        lambda.parts(),
        mu.parts(),
        weight,
        upper_flags,
        lower_flags,
        forbidden_row_masks,
    )
    .ok_or_else(|| "shape and constraints define an empty DP".to_string())?;
    let moduli = [modulus];
    let mut states = state_map_with_capacity(1);
    states.insert(mu_key, Residues::one(1));

    for (label, &strip_size) in weight.iter().enumerate().take(layer + 1) {
        if strip_size == 0
            && (strict_lower_masks.is_some_and(|masks| masks[label] != 0)
                || strict_diagonal_masks.is_some_and(|masks| masks[label] != 0))
        {
            return Err("zero-weight labels must have zero strictness masks".to_string());
        }
        let row_lo = lower_flags
            .and_then(|flags| flags.get(label))
            .map(|&flag| (flag as usize).saturating_sub(1).min(packer.rows))
            .unwrap_or(0);
        let row_hi = upper_flags
            .and_then(|flags| flags.get(label))
            .map(|&flag| (flag as usize).min(packer.rows))
            .unwrap_or(packer.rows);
        let (level_lower, level_upper) = if label + 1 == weight.len() {
            (&lambda_parts[..packer.rows], &lambda_parts[..packer.rows])
        } else {
            (
                level_lower_bounds[label].as_slice(),
                level_upper_bounds[label].as_slice(),
            )
        };
        let context = ExtensionContext {
            packer,
            lambda: &lambda_parts,
            level_lower,
            level_upper,
            row_lo,
            row_hi,
            forbidden_mask: forbidden_row_masks.map_or(0, |masks| masks[label]),
            strict_lower_mask: strict_lower_masks.map_or(0, |masks| masks[label]),
            strict_diagonal_mask: strict_diagonal_masks.map_or(0, |masks| masks[label]),
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
                if let Some(limit) = max_states {
                    if next.len() > limit {
                        return Err(format!(
                            "DP state count {} exceeds --max-states {}.",
                            next.len(),
                            limit
                        ));
                    }
                }
                Ok(())
            })?;
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

/// Materialize one ordinary or row-flagged skew DP layer.
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
    try_masked_flagged_skew_kostka_modular_layer_trace(
        lambda,
        mu,
        weight,
        upper_flags,
        lower_flags,
        None,
        None,
        None,
        modulus,
        layer,
        max_states,
    )
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
    validate_moduli_with_limit(moduli, MAX_MODULI)
}

fn validate_crt_moduli(moduli: &[u32]) -> Result<(), String> {
    validate_moduli_with_limit(moduli, MAX_CRT_MODULI)
}

fn validate_moduli_with_limit(moduli: &[u32], maximum: usize) -> Result<(), String> {
    if moduli.is_empty() || moduli.len() > maximum {
        return Err(format!(
            "expected 1..={maximum} moduli, received {}",
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
    validate_crt_moduli(moduli)?;
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
    use crate::kostka_dp::{skew_kostka, try_flagged_skew_kostka, try_masked_flagged_skew_kostka};

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
    fn crt_combines_more_batches_than_device_lane_limit() {
        let moduli = [
            2_147_483_647,
            2_147_483_629,
            2_147_483_587,
            2_147_483_579,
            2_147_483_563,
            2_147_483_549,
            2_147_483_543,
            2_147_483_497,
            2_147_483_489,
        ];
        let value = (BigUint::one() << 260_u32) + BigUint::from(12_345_u32);
        let residues = moduli
            .iter()
            .map(|&modulus| (&value % BigUint::from(modulus)).to_u32().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            crt_reconstruct_bounded(&residues, &moduli, &value).unwrap(),
            value
        );
        assert!(validate_moduli(&moduli).is_err());
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
    fn modular_matches_masked_face_and_strict_counts() {
        let lambda = Partition::from_sorted(vec![2, 1]);
        let mu = Partition::empty();
        let weight = [1, 1, 1];
        let forbidden = [0, 1, 0];
        let strict_lower = [0, 0, 1];
        let strict_diagonal = [0, 0, 0];
        let expected = try_masked_flagged_skew_kostka(
            &lambda,
            &mu,
            &weight,
            None,
            None,
            Some(&forbidden),
            Some(&strict_lower),
            Some(&strict_diagonal),
            None,
        )
        .unwrap();
        let actual = try_masked_flagged_skew_kostka_modular_stats(
            &lambda,
            &mu,
            &weight,
            None,
            None,
            Some(&forbidden),
            Some(&strict_lower),
            Some(&strict_diagonal),
            &DEFAULT_MODULI[..2],
            None,
            false,
        )
        .unwrap();
        assert_eq!(actual.reconstruct_bounded(&expected).unwrap(), expected);
    }

    #[test]
    fn modular_uses_wide_shape_totals_for_packable_partitions() {
        let lambda = Partition::from_sorted(vec![2_147_483_648, 2_147_483_648]);
        let mu = Partition::from_sorted(vec![2_147_483_648, 2_147_483_647]);
        let actual =
            try_skew_kostka_modular_stats(&lambda, &mu, &[1], &DEFAULT_MODULI[..2], None, false)
                .unwrap();
        assert_eq!(actual.residues, vec![1, 1]);
        assert_eq!(actual.level_transitions, vec![1]);
    }

    #[test]
    fn modular_uses_wide_intermediate_level_totals() {
        let lambda = Partition::from_sorted(vec![
            1_073_741_825,
            1_073_741_825,
            1_073_741_825,
            1_073_741_823,
        ]);
        let mu = Partition::from_sorted(vec![
            1_073_741_825,
            1_073_741_825,
            1_073_741_823,
            1_073_741_822,
        ]);
        let stats =
            try_skew_kostka_modular_stats(&lambda, &mu, &[1, 2], &DEFAULT_MODULI[..2], None, false)
                .unwrap();

        assert_eq!(stats.residues, vec![2, 2]);
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
