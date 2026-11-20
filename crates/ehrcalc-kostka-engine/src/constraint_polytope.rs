//! Exact bounded constraint polytopes: rational bounds, differences, and subset sums.
//!
//! All right sides scale with the dilation. Affine dimension and relative-interior
//! masks come from the certificate-checked [`crate::affine_hull`] implementation,
//! independently of interval propagation and lattice counts. See [`ConstraintPolytopeInput`]
//! for the JSON/text contract and [`PreparedConstraintPolytope`] for the counting API.
//!
//! Counts use a frontier of live difference endpoints and open sum accumulators.
//! Coordinates are arbitrary-size integers; small keys are packed into `u128`.
//! The modular backend shares transitions across prime residues and reconstructs
//! only when their product exceeds a certified bound. No floating-point arithmetic
//! enters feasibility, counting, or certification.

mod count;
mod model;
mod plan;
mod presolve;

pub use count::{ConstraintCount, ModularCertificate};
pub use model::{
    ConstraintPolytopeInput, DifferenceConstraint, SubsetSum, VariableBounds,
    MAX_CONSTRAINT_VARIABLES,
};
pub use plan::{
    ConstraintOptions, CountingPlan, OrderCandidate, OrderHeuristic, OrderPilot, SamplePlan,
    MAX_EXTRA_SAMPLES,
};
pub use presolve::{ConstraintRow, PreparedConstraintPolytope};

use std::fmt;

/// Validation, planning, or counting failure. A budget failure is never a zero count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintError {
    InvalidInput(String),
    Unbounded {
        variable: usize,
        name: String,
    },
    StateLimitExceeded {
        step: usize,
        variable: usize,
        limit: usize,
        frontier: usize,
        estimated_peak: String,
    },
    ModularCapacity {
        bound_bits: u64,
    },
    Arithmetic(String),
}

impl fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) | Self::Arithmetic(message) => f.write_str(message),
            Self::Unbounded { variable, name } => {
                write!(f, "variable {variable} ({name}) is unbounded")
            }
            Self::StateLimitExceeded {
                step,
                variable,
                limit,
                frontier,
                estimated_peak,
            } => write!(
                f,
                "max_states={limit} exceeded at step {step}, variable {variable}; \
                 frontier has {frontier} coordinates, estimated peak {estimated_peak} states"
            ),
            Self::ModularCapacity { bound_bits } => write!(
                f,
                "certified bound has {bound_bits} bits, exceeding available CRT primes; use exact"
            ),
        }
    }
}

impl std::error::Error for ConstraintError {}

type Result<T> = std::result::Result<T, ConstraintError>;

#[cfg(test)]
mod tests;
