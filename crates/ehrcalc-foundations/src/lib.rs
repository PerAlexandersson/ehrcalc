//! Focused combinatorial foundations owned by the Ehrcalc workspace.
//!
//! This crate contains the partition, permutation, key-polynomial, and
//! order-polytope code required by Ehrcalc.  It was extracted from the broader
//! `combinatoric-core` workspace crate; see `../PROVENANCE.md`.

pub mod key_polynomial;
pub mod partition;
pub mod permutation;
pub mod poset;

pub use partition::Partition;
