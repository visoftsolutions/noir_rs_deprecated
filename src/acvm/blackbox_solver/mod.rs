//! This module deals with the execution of the ACVM.
//!
//! The ACVM execution is independent of the proving backend against which the ACIR code is being proven.
//! However, there are a few opcodes that currently lack a Rust implementation, so the C++ implementations
//! included in Aztec Lab's Barretenberg library are used.
//!
//! Since [`acvm`] provides Rust implementations for these opcodes, this module may be deprecated in the future.

use crate::bindings::BackendError;

pub mod barretenberg_structures;
pub mod pedersen;
pub mod scalar_mul;
pub mod schnorr;

#[derive(Debug, thiserror::Error)]
pub(crate) enum RuntimeError {
    #[error("BackendError")]
    BackendError(#[from] BackendError),
    #[error("Value {scalar_as_hex} is not a valid grumpkin scalar")]
    InvalidGrumpkinScalar { scalar_as_hex: String },
    #[error("Limb {limb_as_hex} is not less than 2^128")]
    InvalidGrumpkinScalarLimb { limb_as_hex: String },
}

/// Represents a blackbox opcodes solver for the [`acvm`].
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
