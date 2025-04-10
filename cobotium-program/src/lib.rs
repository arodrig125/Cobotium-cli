//! Cobotium Token Program
//! A simple token program for the Cobotium blockchain

use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
};
use thiserror::Error;

/// Program entrypoint's implementation
pub mod processor;

/// Program state and account definitions
pub mod state;

/// Program instruction definitions
pub mod instruction;

/// Program errors
pub mod error;

// Export current SDK types for downstream users building with a different SDK version
pub use solana_program;

// Only include the entrypoint when the 'no-entrypoint' feature is not enabled
#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

/// Process instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Cobotium Token Program: process_instruction");
    processor::process_instruction(program_id, accounts, instruction_data)
}

#[cfg(test)]
mod tests {
    // Tests will be added here
}
