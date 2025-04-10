//! Instruction types for the Cobotium Token Program

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

/// Instructions supported by the Cobotium Token program
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub enum CobotiumInstruction {
    /// Initialize a new mint
    ///
    /// Accounts expected:
    /// 0. `[writable]` The mint account to initialize
    /// 1. `[]` Rent sysvar
    /// 2. `[signer]` The mint authority
    InitializeMint {
        /// The decimals of the mint
        decimals: u8,
    },

    /// Initialize a new token account
    ///
    /// Accounts expected:
    /// 0. `[writable]` The token account to initialize
    /// 1. `[]` The mint this account will be associated with
    /// 2. `[]` Rent sysvar
    /// 3. `[signer]` The owner of the token account
    InitializeAccount,

    /// Mint new tokens
    ///
    /// Accounts expected:
    /// 0. `[writable]` The mint
    /// 1. `[writable]` The account to mint tokens to
    /// 2. `[signer]` The mint authority
    MintTo {
        /// The amount of new tokens to mint
        amount: u64,
    },

    /// Transfer tokens
    ///
    /// Accounts expected:
    /// 0. `[writable]` The source account
    /// 1. `[writable]` The destination account
    /// 2. `[signer]` The source account's owner
    Transfer {
        /// The amount of tokens to transfer
        amount: u64,
    },

    /// Burn tokens
    ///
    /// Accounts expected:
    /// 0. `[writable]` The account to burn from
    /// 1. `[writable]` The mint
    /// 2. `[signer]` The account's owner
    Burn {
        /// The amount of tokens to burn
        amount: u64,
    },
}

/// Create an `InitializeMint` instruction
pub fn initialize_mint(
    program_id: &Pubkey,
    mint_pubkey: &Pubkey,
    mint_authority_pubkey: &Pubkey,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*mint_pubkey, false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        AccountMeta::new_readonly(*mint_authority_pubkey, true),
    ];

    let data = CobotiumInstruction::InitializeMint { decimals }.try_to_vec()?;

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create an `InitializeAccount` instruction
pub fn initialize_account(
    program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*account_pubkey, false),
        AccountMeta::new_readonly(*mint_pubkey, false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        AccountMeta::new_readonly(*owner_pubkey, true),
    ];

    let data = CobotiumInstruction::InitializeAccount.try_to_vec()?;

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create a `MintTo` instruction
pub fn mint_to(
    program_id: &Pubkey,
    mint_pubkey: &Pubkey,
    account_pubkey: &Pubkey,
    mint_authority_pubkey: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*mint_pubkey, false),
        AccountMeta::new(*account_pubkey, false),
        AccountMeta::new_readonly(*mint_authority_pubkey, true),
    ];

    let data = CobotiumInstruction::MintTo { amount }.try_to_vec()?;

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create a `Transfer` instruction
pub fn transfer(
    program_id: &Pubkey,
    source_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*source_pubkey, false),
        AccountMeta::new(*destination_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, true),
    ];

    let data = CobotiumInstruction::Transfer { amount }.try_to_vec()?;

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create a `Burn` instruction
pub fn burn(
    program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*account_pubkey, false),
        AccountMeta::new(*mint_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, true),
    ];

    let data = CobotiumInstruction::Burn { amount }.try_to_vec()?;

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
