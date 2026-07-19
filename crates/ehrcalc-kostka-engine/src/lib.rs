//! Exact GT, Kostka, LR, flow-polytope, and Ehrhart algorithms for Ehrcalc.
//!
//! The implementation was migrated from the legacy `kostka` project.  See
//! `../PROVENANCE.md` for migration scope and compatibility policy.

pub mod ehrhart;
pub mod flow;
pub mod gt_dim;
pub mod kostka_dp;
pub mod lr;
pub mod partition;
pub mod syt;

pub use partition::Partition;
