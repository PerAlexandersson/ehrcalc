use num_bigint::BigUint;
use num_traits::One;
use std::fmt;

/// A partition of a non-negative integer: a weakly decreasing sequence of positive integers.
///
/// Stored as `Vec<u32>` in weakly decreasing order with no trailing zeros.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Partition(Vec<u32>);

impl Partition {
    /// Create a partition from parts. Sorts descending and strips zeros.
    pub fn new(mut parts: Vec<u32>) -> Self {
        parts.sort_unstable_by(|a, b| b.cmp(a));
        parts.retain(|&x| x > 0);
        Partition(parts)
    }

    /// Create a partition from parts that are already sorted descending.
    /// Strips trailing zeros but does NOT re-sort. Caller must guarantee order.
    pub fn from_sorted(mut parts: Vec<u32>) -> Self {
        parts.retain(|&x| x > 0);
        Partition(parts)
    }

    /// The empty partition (partition of 0).
    pub fn empty() -> Self {
        Partition(vec![])
    }

    /// The parts as a slice.
    pub fn parts(&self) -> &[u32] {
        &self.0
    }

    /// Number of (nonzero) parts, i.e. the length of the partition.
    pub fn num_parts(&self) -> usize {
        self.0.len()
    }

    /// The i-th part (0-indexed). Returns 0 if i >= num_parts.
    pub fn part(&self, i: usize) -> u32 {
        if i < self.0.len() {
            self.0[i]
        } else {
            0
        }
    }

    /// The size |lambda| = sum of all parts.
    pub fn size(&self) -> u32 {
        self.0.iter().sum()
    }

    /// Whether this is the empty partition.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Conjugate (transpose) partition.
    pub fn conjugate_partition(&self) -> Partition {
        if self.is_empty() {
            return Partition::empty();
        }
        let max_part = self.0[0] as usize;
        let mut conj = vec![0u32; max_part];
        for &p in &self.0 {
            for x in conj.iter_mut().take(p as usize) {
                *x += 1;
            }
        }
        Partition(conj)
    }

    /// Test Young diagram containment: self ⊆ other (entrywise).
    pub fn partition_less_equal(&self, other: &Partition) -> bool {
        let n = self.num_parts().max(other.num_parts());
        for i in 0..n {
            if self.part(i) > other.part(i) {
                return false;
            }
        }
        true
    }

    /// Count standard Young tableaux of shape lambda via the hook-length formula.
    pub fn count_syt(&self) -> BigUint {
        let n = self.size() as u64;
        let mut numerator = BigUint::one();
        for i in 2..=n {
            numerator *= i;
        }
        for row in &self.hook_lengths() {
            for &h in row {
                numerator /= h as u64;
            }
        }
        numerator
    }

    /// Arm length at box (r, c) (0-indexed): lambda_r - c - 1.
    pub fn partition_arm(&self, row: usize, col: usize) -> Option<u32> {
        let part_r = self.part(row);
        if col < part_r as usize {
            Some(part_r - col as u32 - 1)
        } else {
            None
        }
    }

    /// Leg length at box (r, c) (0-indexed): lambda'_c - r - 1.
    pub fn partition_leg(&self, row: usize, col: usize) -> Option<u32> {
        let conj = self.conjugate_partition();
        let part_c = conj.part(col);
        if row < part_c as usize {
            Some(part_c - row as u32 - 1)
        } else {
            None
        }
    }

    /// Hook length at box (r, c): arm + leg + 1.
    pub fn hook_length(&self, row: usize, col: usize) -> Option<u32> {
        match (self.partition_arm(row, col), self.partition_leg(row, col)) {
            (Some(a), Some(l)) => Some(a + l + 1),
            _ => None,
        }
    }

    /// Table of all hook lengths.
    pub fn hook_lengths(&self) -> Vec<Vec<u32>> {
        let conj = self.conjugate_partition();
        self.0
            .iter()
            .enumerate()
            .map(|(r, &part_r)| {
                (0..part_r as usize)
                    .map(|c| {
                        let arm = part_r - c as u32 - 1;
                        let leg = conj.part(c) - r as u32 - 1;
                        arm + leg + 1
                    })
                    .collect()
            })
            .collect()
    }

    /// All partitions of n (in reverse lexicographic order).
    pub fn all_of_size(n: u32) -> Vec<Partition> {
        Self::all_of_size_bounded(n, n as usize, n)
    }

    /// All partitions of n with at most `max_parts` parts, each at most `max_part`.
    pub fn all_of_size_bounded(n: u32, max_parts: usize, max_part: u32) -> Vec<Partition> {
        let mut result = Vec::new();
        Self::enumerate_helper(n, max_parts, max_part, &mut vec![], &mut result);
        result
    }

    fn enumerate_helper(
        remaining: u32,
        max_parts: usize,
        max_part: u32,
        current: &mut Vec<u32>,
        results: &mut Vec<Partition>,
    ) {
        if remaining == 0 {
            results.push(Partition(current.clone()));
            return;
        }
        if max_parts == 0 {
            return;
        }
        let upper = remaining.min(max_part);
        for part in (1..=upper).rev() {
            current.push(part);
            Self::enumerate_helper(remaining - part, max_parts - 1, part, current, results);
            current.pop();
        }
    }

    /// Parse "5,3,1" or "5.3.1" into a Partition.
    pub fn parse(s: &str) -> Result<Partition, String> {
        if s.is_empty() || s == "0" {
            return Ok(Partition::empty());
        }
        let sep = if s.contains(',') { ',' } else { '.' };
        let parts: Result<Vec<u32>, _> = s.split(sep).map(|x| x.trim().parse::<u32>()).collect();
        match parts {
            Ok(p) => Ok(Partition::new(p)),
            Err(e) => Err(format!("Failed to parse partition '{}': {}", s, e)),
        }
    }

    /// Display as comma-separated parts, or "∅" for empty.
    pub fn display(&self) -> String {
        if self.is_empty() {
            "∅".to_string()
        } else {
            self.0
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",")
        }
    }
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// Parse a comma-separated list of non-negative integers into a Partition.
pub fn parse_partition(s: &str) -> Result<Partition, String> {
    let mut parts: Vec<u32> = s
        .split(',')
        .map(|x| x.trim().parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid partition: {}", e))?;
    parts.sort_unstable_by(|a, b| b.cmp(a));
    Ok(Partition::new(parts))
}

/// Parse a comma-separated list into a weight vector (composition); zeros allowed, order kept.
pub fn parse_weight(s: &str) -> Result<Vec<u32>, String> {
    s.split(',')
        .map(|x| {
            x.trim()
                .parse::<u32>()
                .map_err(|e| format!("Invalid weight: {}", e))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let p = Partition::new(vec![3, 5, 1]);
        assert_eq!(p.parts(), &[5, 3, 1]);
        assert_eq!(p.size(), 9);
        assert_eq!(p.num_parts(), 3);
    }

    #[test]
    fn test_conjugate() {
        let p = Partition::new(vec![4, 2, 1]);
        assert_eq!(p.conjugate_partition(), Partition::new(vec![3, 2, 1, 1]));
        assert_eq!(p.conjugate_partition().conjugate_partition(), p);
    }

    #[test]
    fn test_count_syt() {
        let p = Partition::new(vec![3, 2, 1]);
        assert_eq!(p.count_syt(), BigUint::from(16u32));
    }

    #[test]
    fn test_enumeration() {
        let parts = Partition::all_of_size(5);
        assert_eq!(parts.len(), 7);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Partition::parse("5,3,1").unwrap(),
            Partition::new(vec![5, 3, 1])
        );
        assert_eq!(Partition::parse("0").unwrap(), Partition::empty());
    }
}
