//! ACVM execution is independent of the proving backend against which the ACIR code is being proven.
//! However there are currently a few opcodes for which there is currently no rust implementation so we must
//! use the C++ implementations included in Aztec Lab's Barretenberg library.
//!
//! As [`acvm`] includes rust implementations for these opcodes, this module can be removed.

pub mod barretenberg_structures;
pub mod pedersen;
pub mod scalar_mul;
pub mod schnorr;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    FromFeature(#[from] FeatureError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum FeatureError {
    #[error("Value {scalar_as_hex} is not a valid grumpkin scalar")]
    InvalidGrumpkinScalar { scalar_as_hex: String },
    #[error("Limb {limb_as_hex} is not less than 2^128")]
    InvalidGrumpkinScalarLimb { limb_as_hex: String },
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct BackendError(#[from] Error);

impl From<FeatureError> for BackendError {
    fn from(value: FeatureError) -> Self {
        value.into()
    }
}

#[derive(Debug)]
pub struct BlackboxSolver {}

impl BlackboxSolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BlackboxSolver {
    fn default() -> Self {
        Self::new()
    }
}
