//! Error types for the Cobotium Token Program

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the Cobotium Token program
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum CobotiumError {
    /// Invalid instruction data passed to program
    #[error("Invalid instruction")]
    InvalidInstruction,

    /// Not rent exempt
    #[error("Not rent exempt")]
    NotRentExempt,

    /// Expected a different account owner
    #[error("Expected a different account owner")]
    IncorrectOwner,

    /// Account not initialized
    #[error("Account not initialized")]
    UninitializedAccount,

    /// Account already initialized
    #[error("Account already initialized")]
    AlreadyInitialized,

    /// Insufficient funds for the operation
    #[error("Insufficient funds")]
    InsufficientFunds,

    /// Invalid mint authority
    #[error("Invalid mint authority")]
    InvalidMintAuthority,

    /// Invalid token account
    #[error("Invalid token account")]
    InvalidTokenAccount,
}

impl From<CobotiumError> for ProgramError {
    fn from(e: CobotiumError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for CobotiumError {
    fn type_of() -> &'static str {
        "CobotiumError"
    }
}
