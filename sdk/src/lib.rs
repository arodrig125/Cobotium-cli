//! Cobotium SDK for interacting with the Cobotium Token Program

use borsh::BorshSerialize;
use cobotium_program::instruction as program_instruction;
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::Instruction;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use std::str::FromStr;
use thiserror::Error;

/// Errors that may be returned by the Cobotium SDK
#[derive(Debug, Error)]
pub enum CobotiumSdkError {
    #[error("Client error: {0}")]
    ClientError(#[from] solana_client::client_error::ClientError),

    #[error("Program error: {0}")]
    ProgramError(#[from] solana_program::program_error::ProgramError),

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),
}

/// Result type for Cobotium SDK operations
pub type CobotiumResult<T> = Result<T, CobotiumSdkError>;

/// Cobotium client for interacting with the Cobotium Token Program
pub struct CobotiumClient {
    /// RPC client for interacting with the Solana blockchain
    pub rpc_client: RpcClient,
    /// Program ID of the Cobotium Token Program
    pub program_id: Pubkey,
}

impl CobotiumClient {
    /// Create a new Cobotium client
    pub fn new(rpc_url: &str, program_id: &str) -> CobotiumResult<Self> {
        let program_id = Pubkey::from_str(program_id)
            .map_err(|_| CobotiumSdkError::InvalidPublicKey(program_id.to_string()))?;

        Ok(Self {
            rpc_client: RpcClient::new(rpc_url.to_string()),
            program_id,
        })
    }

    /// Initialize a new mint
    pub fn initialize_mint(
        &self,
        payer: &Keypair,
        mint: &Keypair,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
        decimals: u8,
    ) -> CobotiumResult<Signature> {
        let instruction = program_instruction::initialize_mint(
            &self.program_id,
            &mint.pubkey(),
            mint_authority,
            freeze_authority,
            decimals,
        )?;

        self.send_transaction(&[instruction], &[payer, mint])
    }

    /// Initialize a new token account
    pub fn initialize_account(
        &self,
        payer: &Keypair,
        account: &Keypair,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> CobotiumResult<Signature> {
        let instruction = program_instruction::initialize_account(
            &self.program_id,
            &account.pubkey(),
            mint,
            owner,
        )?;

        self.send_transaction(&[instruction], &[payer, account])
    }

    /// Mint tokens to an account
    pub fn mint_to(
        &self,
        payer: &Keypair,
        mint: &Pubkey,
        account: &Pubkey,
        mint_authority: &Keypair,
        amount: u64,
    ) -> CobotiumResult<Signature> {
        let instruction = program_instruction::mint_to(
            &self.program_id,
            mint,
            account,
            &mint_authority.pubkey(),
            amount,
        )?;

        self.send_transaction(&[instruction], &[payer, mint_authority])
    }

    /// Transfer tokens from one account to another
    pub fn transfer(
        &self,
        payer: &Keypair,
        source: &Pubkey,
        destination: &Pubkey,
        owner: &Keypair,
        amount: u64,
    ) -> CobotiumResult<Signature> {
        let instruction = program_instruction::transfer(
            &self.program_id,
            source,
            destination,
            &owner.pubkey(),
            amount,
        )?;

        self.send_transaction(&[instruction], &[payer, owner])
    }

    /// Burn tokens from an account
    pub fn burn(
        &self,
        payer: &Keypair,
        account: &Pubkey,
        mint: &Pubkey,
        owner: &Keypair,
        amount: u64,
    ) -> CobotiumResult<Signature> {
        let instruction = program_instruction::burn(
            &self.program_id,
            account,
            mint,
            &owner.pubkey(),
            amount,
        )?;

        self.send_transaction(&[instruction], &[payer, owner])
    }

    /// Freeze an account
    pub fn freeze_account(
        &self,
        payer: &Keypair,
        account: &Pubkey,
        mint: &Pubkey,
        freeze_authority: &Keypair,
    ) -> CobotiumResult<Signature> {
        let instruction = program_instruction::freeze_account(
            &self.program_id,
            account,
            mint,
            &freeze_authority.pubkey(),
        )?;

        self.send_transaction(&[instruction], &[payer, freeze_authority])
    }

    /// Thaw (unfreeze) an account
    pub fn thaw_account(
        &self,
        payer: &Keypair,
        account: &Pubkey,
        mint: &Pubkey,
        freeze_authority: &Keypair,
    ) -> CobotiumResult<Signature> {
        let instruction = program_instruction::thaw_account(
            &self.program_id,
            account,
            mint,
            &freeze_authority.pubkey(),
        )?;

        self.send_transaction(&[instruction], &[payer, freeze_authority])
    }

    /// Send a transaction with the given instructions and signers
    fn send_transaction(
        &self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> CobotiumResult<Signature> {
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;

        let transaction = Transaction::new_signed_with_payer(
            instructions,
            Some(&signers[0].pubkey()),
            signers,
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        Ok(signature)
    }
}
