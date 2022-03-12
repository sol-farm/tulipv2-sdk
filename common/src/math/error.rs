//! Error types

use anchor_lang::solana_program::{decode_error::DecodeError, program_error::ProgramError};
use num_derive::FromPrimitive;
use thiserror::Error;

/// Errors that may be returned by the TokenLending program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum MathError {
    #[error("Math operation overflow")]
    MathOverflow,
}

impl From<MathError> for ProgramError {
    fn from(e: MathError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MathError {
    fn type_of() -> &'static str {
        "Math Error"
    }
}
