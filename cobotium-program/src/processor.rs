//! Program instruction processor

use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    error::CobotiumError,
    instruction::CobotiumInstruction,
    state::{Mint, TokenAccount},
};

/// Program state handler.
pub struct Processor {}

impl Processor {
    /// Process an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = CobotiumInstruction::try_from_slice(input)?;

        match instruction {
            CobotiumInstruction::InitializeMint { decimals, freeze_authority } => {
                msg!("Instruction: InitializeMint");
                Self::process_initialize_mint(accounts, decimals, freeze_authority, program_id)
            }
            CobotiumInstruction::InitializeAccount => {
                msg!("Instruction: InitializeAccount");
                Self::process_initialize_account(accounts, program_id)
            }
            CobotiumInstruction::MintTo { amount } => {
                msg!("Instruction: MintTo");
                Self::process_mint_to(accounts, amount, program_id)
            }
            CobotiumInstruction::Transfer { amount } => {
                msg!("Instruction: Transfer");
                Self::process_transfer(accounts, amount, program_id)
            }
            CobotiumInstruction::Burn { amount } => {
                msg!("Instruction: Burn");
                Self::process_burn(accounts, amount, program_id)
            }
        }
    }

    /// Process InitializeMint instruction
    pub fn process_initialize_mint(
        accounts: &[AccountInfo],
        decimals: u8,
        freeze_authority: Option<Pubkey>,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let mint_info = next_account_info(account_info_iter)?;
        let rent_info = next_account_info(account_info_iter)?;
        let mint_authority_info = next_account_info(account_info_iter)?;

        // Get optional freeze authority
        let freeze_authority_info = if freeze_authority.is_some() {
            Some(next_account_info(account_info_iter)?)
        } else {
            None
        };

        // Check program ownership
        if mint_info.owner != program_id {
            return Err(CobotiumError::IncorrectOwner.into());
        }

        // Check rent exemption
        let rent = &Rent::from_account_info(rent_info)?;
        if !rent.is_exempt(mint_info.lamports(), mint_info.data_len()) {
            return Err(CobotiumError::NotRentExempt.into());
        }

        // Check if the mint account is already initialized
        if mint_info.data_len() >= Mint::LEN {
            let mint = Mint::unpack_from_slice(&mint_info.data.borrow())?;
            if mint.is_initialized {
                return Err(CobotiumError::AlreadyInitialized.into());
            }
        }

        // Validate decimals (typically 0-18 for tokens)
        if decimals > 18 {
            return Err(ProgramError::InvalidArgument);
        }

        // Verify freeze authority if provided
        if let Some(freeze_authority) = freeze_authority {
            if let Some(freeze_authority_info) = freeze_authority_info {
                if freeze_authority != *freeze_authority_info.key {
                    return Err(CobotiumError::InvalidProgramAuthority.into());
                }
                if !freeze_authority_info.is_signer {
                    return Err(ProgramError::MissingRequiredSignature);
                }
            } else {
                return Err(ProgramError::InvalidArgument);
            }
        }

        // Create and save mint
        let mut mint_data = mint_info.data.borrow_mut();
        let mint = Mint {
            is_initialized: true,
            decimals,
            mint_authority: *mint_authority_info.key,
            freeze_authority,
            supply: 0,
        };
        mint.pack_into_slice(&mut mint_data);

        msg!("Mint initialized with {} decimals", decimals);
        if freeze_authority.is_some() {
            msg!("Freeze authority set");
        }
        Ok(())
    }

    /// Process InitializeAccount instruction
    pub fn process_initialize_account(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let account_info = next_account_info(account_info_iter)?;
        let mint_info = next_account_info(account_info_iter)?;
        let rent_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;

        // Check program ownership
        if account_info.owner != program_id {
            return Err(CobotiumError::IncorrectOwner.into());
        }

        // Check mint ownership
        if mint_info.owner != program_id {
            return Err(CobotiumError::IncorrectOwner.into());
        }

        // Check rent exemption
        let rent = &Rent::from_account_info(rent_info)?;
        if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
            return Err(CobotiumError::NotRentExempt.into());
        }

        // Check if the account is already initialized
        if account_info.data_len() >= TokenAccount::LEN {
            let token_account = TokenAccount::unpack_from_slice(&account_info.data.borrow())?;
            if token_account.is_initialized {
                return Err(CobotiumError::AlreadyInitialized.into());
            }
        }

        // Validate account size
        if account_info.data_len() < TokenAccount::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check mint is initialized
        if mint_info.data_len() < Mint::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let mint = Mint::unpack_from_slice(&mint_info.data.borrow())?;
        if !mint.is_initialized {
            return Err(CobotiumError::UninitializedAccount.into());
        }

        // Create and save token account
        let mut account_data = account_info.data.borrow_mut();
        let token_account = TokenAccount {
            is_initialized: true,
            mint: *mint_info.key,
            owner: *owner_info.key,
            amount: 0,
            is_frozen: false,
        };
        token_account.pack_into_slice(&mut account_data);

        msg!("Token account initialized for mint: {}", mint_info.key);
        Ok(())
    }

    /// Process MintTo instruction
    pub fn process_mint_to(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let mint_info = next_account_info(account_info_iter)?;
        let account_info = next_account_info(account_info_iter)?;
        let mint_authority_info = next_account_info(account_info_iter)?;

        // Validate amount
        if amount == 0 {
            return Err(ProgramError::InvalidArgument);
        }

        // Check program ownership
        if mint_info.owner != program_id || account_info.owner != program_id {
            return Err(CobotiumError::IncorrectOwner.into());
        }

        // Validate account sizes
        if mint_info.data_len() < Mint::LEN || account_info.data_len() < TokenAccount::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check mint authority is signer
        if !mint_authority_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Check accounts are initialized
        let mut mint = Mint::unpack_from_slice(&mint_info.data.borrow())?;
        let mut token_account = TokenAccount::unpack_from_slice(&account_info.data.borrow())?;

        if !mint.is_initialized || !token_account.is_initialized {
            return Err(CobotiumError::UninitializedAccount.into());
        }

        // Check mint authority
        if mint.mint_authority != *mint_authority_info.key {
            return Err(CobotiumError::InvalidMintAuthority.into());
        }

        // Check account's mint matches
        if token_account.mint != *mint_info.key {
            return Err(CobotiumError::InvalidTokenAccount.into());
        }

        // Check if account is frozen
        if token_account.is_frozen {
            return Err(CobotiumError::AccountFrozen.into());
        }

        // Mint tokens with overflow check
        token_account.amount = token_account.amount.checked_add(amount)
            .ok_or(CobotiumError::Overflow)?;
        mint.supply = mint.supply.checked_add(amount)
            .ok_or(CobotiumError::Overflow)?;

        // Save updated accounts
        mint.pack_into_slice(&mut mint_info.data.borrow_mut());
        token_account.pack_into_slice(&mut account_info.data.borrow_mut());

        msg!("Minted {} tokens to account {}", amount, account_info.key);
        Ok(())
    }

    /// Process Transfer instruction
    pub fn process_transfer(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let source_info = next_account_info(account_info_iter)?;
        let destination_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;

        // Check program ownership
        if source_info.owner != program_id || destination_info.owner != program_id {
            return Err(CobotiumError::IncorrectOwner.into());
        }

        // Check owner is signer
        if !owner_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Check accounts are initialized
        let mut source = TokenAccount::unpack_from_slice(&source_info.data.borrow())?;
        let mut destination = TokenAccount::unpack_from_slice(&destination_info.data.borrow())?;

        if !source.is_initialized || !destination.is_initialized {
            return Err(CobotiumError::UninitializedAccount.into());
        }

        // Check owner
        if source.owner != *owner_info.key {
            return Err(CobotiumError::IncorrectOwner.into());
        }

        // Check accounts are for the same mint
        if source.mint != destination.mint {
            return Err(CobotiumError::InvalidTokenAccount.into());
        }

        // Check if accounts are frozen
        if source.is_frozen || destination.is_frozen {
            return Err(CobotiumError::AccountFrozen.into());
        }

        // Check sufficient funds
        if source.amount < amount {
            return Err(CobotiumError::InsufficientFunds.into());
        }

        // Transfer tokens
        source.amount = source.amount.checked_sub(amount).ok_or(ProgramError::InvalidArgument)?;
        destination.amount = destination.amount.checked_add(amount).ok_or(ProgramError::InvalidArgument)?;

        // Save updated accounts
        source.pack_into_slice(&mut source_info.data.borrow_mut());
        destination.pack_into_slice(&mut destination_info.data.borrow_mut());

        Ok(())
    }

    /// Process Burn instruction
    pub fn process_burn(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let account_info = next_account_info(account_info_iter)?;
        let mint_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;

        // Check program ownership
        if account_info.owner != program_id || mint_info.owner != program_id {
            return Err(CobotiumError::IncorrectOwner.into());
        }

        // Check owner is signer
        if !owner_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Check accounts are initialized
        let mut token_account = TokenAccount::unpack_from_slice(&account_info.data.borrow())?;
        let mut mint = Mint::unpack_from_slice(&mint_info.data.borrow())?;

        if !token_account.is_initialized || !mint.is_initialized {
            return Err(CobotiumError::UninitializedAccount.into());
        }

        // Check owner
        if token_account.owner != *owner_info.key {
            return Err(CobotiumError::IncorrectOwner.into());
        }

        // Check account's mint matches
        if token_account.mint != *mint_info.key {
            return Err(CobotiumError::InvalidTokenAccount.into());
        }

        // Check if account is frozen
        if token_account.is_frozen {
            return Err(CobotiumError::AccountFrozen.into());
        }

        // Check sufficient funds
        if token_account.amount < amount {
            return Err(CobotiumError::InsufficientFunds.into());
        }

        // Burn tokens
        token_account.amount = token_account.amount.checked_sub(amount).ok_or(ProgramError::InvalidArgument)?;
        mint.supply = mint.supply.checked_sub(amount).ok_or(ProgramError::InvalidArgument)?;

        // Save updated accounts
        token_account.pack_into_slice(&mut account_info.data.borrow_mut());
        mint.pack_into_slice(&mut mint_info.data.borrow_mut());

        Ok(())
    }
}

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process_instruction(program_id, accounts, instruction_data)
}
