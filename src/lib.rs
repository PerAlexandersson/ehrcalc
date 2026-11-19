//! Exact Ehrhart computations and related combinatorial counting tools.
//!
//! This crate is intentionally a small scaffold while the family-neutral exact
//! algebra and adapters described in `EHRHART_TOOL_SPEC.md` are implemented.
//! Public APIs will be added only with exact tests and stable documentation.

pub mod cli;
pub mod exact;
pub mod families;
pub mod key_scan;
pub mod render;

/// Core model, planner, and exact constraint-polytope counters.
pub use ehrcalc_kostka_engine::constraint_polytope;

/// The package version exposed to callers before the first functional API.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
