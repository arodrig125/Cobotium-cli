//! State definitions for the Cobotium Token Program

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// Mint account data
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Mint {
    /// Is this mint initialized
    pub is_initialized: bool,
    /// Decimals of the mint
    pub decimals: u8,
    /// Authority that can mint new tokens
    pub mint_authority: Pubkey,
    /// Total supply of tokens
    pub supply: u64,
}

impl Sealed for Mint {}

impl IsInitialized for Mint {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Mint {
    const LEN: usize = 1 + 1 + 32 + 8; // is_initialized + decimals + mint_authority + supply

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let data = self.try_to_vec().unwrap();
        dst[..data.len()].copy_from_slice(&data);
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mint = Mint::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(mint)
    }
}

/// Token account data
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct TokenAccount {
    /// Is this account initialized
    pub is_initialized: bool,
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account
    pub owner: Pubkey,
    /// The amount of tokens this account holds
    pub amount: u64,
}

impl Sealed for TokenAccount {}

impl IsInitialized for TokenAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for TokenAccount {
    const LEN: usize = 1 + 32 + 32 + 8; // is_initialized + mint + owner + amount

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let data = self.try_to_vec().unwrap();
        dst[..data.len()].copy_from_slice(&data);
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let account = TokenAccount::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(account)
    }
}
