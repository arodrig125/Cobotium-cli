use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use cobotium_sdk::CobotiumClient;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

/// Cobotium CLI to interact with the blockchain
#[derive(Parser)]
#[clap(name = "Cobotium CLI", version = "0.1.0", about = "Interact with the Cobotium blockchain")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// RPC endpoint URL
    #[clap(long, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    /// Path to payer keypair file
    #[clap(long, default_value = "~/.config/solana/id.json")]
    payer: String,

    /// Cobotium program ID
    #[clap(long, default_value = "11111111111111111111111111111111")]
    program_id: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Mint SOL to an address
    MintSol {
        #[clap(long)]
        to: String,
        #[clap(long)]
        amount: f64,
    },

    /// Transfer SOL to an address
    TransferSol {
        #[clap(long)]
        to: String,
        #[clap(long)]
        amount: f64,
    },

    /// Check wallet balance
    Balance {
        #[clap(long)]
        address: String,
    },

    /// Create a new Cobotium token mint
    CreateMint {
        /// Path to the mint keypair file
        #[clap(long)]
        mint_keypair: String,

        /// Decimals for the mint
        #[clap(long, default_value = "9")]
        decimals: u8,
    },

    /// Create a new token account
    CreateAccount {
        /// Path to the account keypair file
        #[clap(long)]
        account_keypair: String,

        /// Mint public key
        #[clap(long)]
        mint: String,

        /// Owner public key
        #[clap(long)]
        owner: String,
    },

    /// Mint tokens to an account
    MintTokens {
        /// Mint public key
        #[clap(long)]
        mint: String,

        /// Account public key
        #[clap(long)]
        account: String,

        /// Path to the mint authority keypair file
        #[clap(long)]
        mint_authority: String,

        /// Amount of tokens to mint
        #[clap(long)]
        amount: u64,
    },

    /// Transfer tokens from one account to another
    TransferTokens {
        /// Source account public key
        #[clap(long)]
        source: String,

        /// Destination account public key
        #[clap(long)]
        destination: String,

        /// Path to the owner keypair file
        #[clap(long)]
        owner: String,

        /// Amount of tokens to transfer
        #[clap(long)]
        amount: u64,
    },

    /// Burn tokens from an account
    BurnTokens {
        /// Account public key
        #[clap(long)]
        account: String,

        /// Mint public key
        #[clap(long)]
        mint: String,

        /// Path to the owner keypair file
        #[clap(long)]
        owner: String,

        /// Amount of tokens to burn
        #[clap(long)]
        amount: u64,
    },

    /// Deploy the Cobotium program
    DeployProgram {
        /// Path to the program keypair file
        #[clap(long)]
        program_keypair: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let payer = load_keypair(&cli.payer)?;
    let rpc = RpcClient::new(cli.rpc_url.clone());
    let cobotium_client = CobotiumClient::new(&cli.rpc_url, &cli.program_id)
        .map_err(|e| anyhow!("Failed to create Cobotium client: {}", e))?;

    match cli.command {
        Commands::MintSol { to, amount } => {
            mint_sol(&rpc, &payer, &to, amount)?;
        }

        Commands::TransferSol { to, amount } => {
            transfer_sol(&rpc, &payer, &to, amount)?;
        }

        Commands::Balance { address } => {
            check_balance(&rpc, &address)?;
        }

        Commands::CreateMint { mint_keypair, decimals } => {
            let mint = load_keypair(&mint_keypair)?;
            create_mint(&cobotium_client, &payer, &mint, decimals)?;
        }

        Commands::CreateAccount { account_keypair, mint, owner } => {
            let account = load_keypair(&account_keypair)?;
            let mint_pubkey = Pubkey::from_str(&mint)
                .map_err(|_| anyhow!("Invalid mint public key: {}", mint))?;
            let owner_pubkey = Pubkey::from_str(&owner)
                .map_err(|_| anyhow!("Invalid owner public key: {}", owner))?;
            create_account(&cobotium_client, &payer, &account, &mint_pubkey, &owner_pubkey)?;
        }

        Commands::MintTokens { mint, account, mint_authority, amount } => {
            let mint_pubkey = Pubkey::from_str(&mint)
                .map_err(|_| anyhow!("Invalid mint public key: {}", mint))?;
            let account_pubkey = Pubkey::from_str(&account)
                .map_err(|_| anyhow!("Invalid account public key: {}", account))?;
            let mint_authority_keypair = load_keypair(&mint_authority)?;
            mint_tokens(&cobotium_client, &payer, &mint_pubkey, &account_pubkey, &mint_authority_keypair, amount)?;
        }

        Commands::TransferTokens { source, destination, owner, amount } => {
            let source_pubkey = Pubkey::from_str(&source)
                .map_err(|_| anyhow!("Invalid source public key: {}", source))?;
            let destination_pubkey = Pubkey::from_str(&destination)
                .map_err(|_| anyhow!("Invalid destination public key: {}", destination))?;
            let owner_keypair = load_keypair(&owner)?;
            transfer_tokens(&cobotium_client, &payer, &source_pubkey, &destination_pubkey, &owner_keypair, amount)?;
        }

        Commands::BurnTokens { account, mint, owner, amount } => {
            let account_pubkey = Pubkey::from_str(&account)
                .map_err(|_| anyhow!("Invalid account public key: {}", account))?;
            let mint_pubkey = Pubkey::from_str(&mint)
                .map_err(|_| anyhow!("Invalid mint public key: {}", mint))?;
            let owner_keypair = load_keypair(&owner)?;
            burn_tokens(&cobotium_client, &payer, &account_pubkey, &mint_pubkey, &owner_keypair, amount)?;
        }

        Commands::DeployProgram { program_keypair } => {
            deploy_program(&rpc, &payer, program_keypair.as_deref())?;
        }
    }

    Ok(())
}

fn load_keypair(path: &str) -> Result<Keypair> {
    let expanded = shellexpand::tilde(path);
    read_keypair_file(expanded.as_ref())
        .map_err(|_| anyhow!("Failed to read keypair file: {}", path))
}

fn mint_sol(
    rpc: &RpcClient,
    payer: &Keypair,
    to: &str,
    amount: f64,
) -> Result<()> {
    let to_pubkey = Pubkey::from_str(to)
        .map_err(|_| anyhow!("Invalid public key: {}", to))?;
    let lamports = (amount * 1_000_000_000.0) as u64;

    let recent_blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(&payer.pubkey(), &to_pubkey, lamports)],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    let sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("✅ Minted {} SOL to {}\nSignature: {}", amount, to_pubkey, sig);
    Ok(())
}

fn transfer_sol(
    rpc: &RpcClient,
    payer: &Keypair,
    to: &str,
    amount: f64,
) -> Result<()> {
    let to_pubkey = Pubkey::from_str(to)
        .map_err(|_| anyhow!("Invalid public key: {}", to))?;
    let lamports = (amount * 1_000_000_000.0) as u64;

    let recent_blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(&payer.pubkey(), &to_pubkey, lamports)],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    let sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("✅ Transferred {} SOL to {}\nSignature: {}", amount, to_pubkey, sig);
    Ok(())
}

fn check_balance(
    rpc: &RpcClient,
    address: &str,
) -> Result<()> {
    let pubkey = Pubkey::from_str(address)
        .map_err(|_| anyhow!("Invalid public key: {}", address))?;
    let balance = rpc.get_balance(&pubkey)?;
    println!("💰 Balance for {}:\n   {} lamports (≈ {} SOL)", pubkey, balance, balance as f64 / 1_000_000_000.0);
    Ok(())
}

fn create_mint(
    client: &CobotiumClient,
    payer: &Keypair,
    mint: &Keypair,
    decimals: u8,
) -> Result<()> {
    let signature = client.initialize_mint(payer, mint, &payer.pubkey(), decimals)
        .map_err(|e| anyhow!("Failed to create mint: {}", e))?;

    println!("✅ Created mint {}\nDecimals: {}\nSignature: {}", mint.pubkey(), decimals, signature);
    Ok(())
}

fn create_account(
    client: &CobotiumClient,
    payer: &Keypair,
    account: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Result<()> {
    let signature = client.initialize_account(payer, account, mint, owner)
        .map_err(|e| anyhow!("Failed to create account: {}", e))?;

    println!("✅ Created account {}\nMint: {}\nOwner: {}\nSignature: {}", account.pubkey(), mint, owner, signature);
    Ok(())
}

fn mint_tokens(
    client: &CobotiumClient,
    payer: &Keypair,
    mint: &Pubkey,
    account: &Pubkey,
    mint_authority: &Keypair,
    amount: u64,
) -> Result<()> {
    let signature = client.mint_to(payer, mint, account, mint_authority, amount)
        .map_err(|e| anyhow!("Failed to mint tokens: {}", e))?;

    println!("✅ Minted {} tokens to {}\nMint: {}\nSignature: {}", amount, account, mint, signature);
    Ok(())
}

fn transfer_tokens(
    client: &CobotiumClient,
    payer: &Keypair,
    source: &Pubkey,
    destination: &Pubkey,
    owner: &Keypair,
    amount: u64,
) -> Result<()> {
    let signature = client.transfer(payer, source, destination, owner, amount)
        .map_err(|e| anyhow!("Failed to transfer tokens: {}", e))?;

    println!("✅ Transferred {} tokens from {} to {}\nSignature: {}", amount, source, destination, signature);
    Ok(())
}

fn burn_tokens(
    client: &CobotiumClient,
    payer: &Keypair,
    account: &Pubkey,
    mint: &Pubkey,
    owner: &Keypair,
    amount: u64,
) -> Result<()> {
    let signature = client.burn(payer, account, mint, owner, amount)
        .map_err(|e| anyhow!("Failed to burn tokens: {}", e))?;

    println!("✅ Burned {} tokens from {}\nMint: {}\nSignature: {}", amount, account, mint, signature);
    Ok(())
}

fn deploy_program(
    rpc: &RpcClient,
    payer: &Keypair,
    program_keypair_path: Option<&str>,
) -> Result<()> {
    // This is a simplified version - in a real implementation, you would use the solana CLI
    // or the solana_program_deploy crate to deploy the program
    println!("⚠️ Program deployment is not implemented in this CLI.\nPlease use the following command to deploy the program:\n");
    println!("solana program deploy --program-id <KEYPAIR_PATH> target/deploy/cobotium_program.so");

    if let Some(path) = program_keypair_path {
        println!("\nUsing program keypair: {}", path);
    } else {
        println!("\nNo program keypair provided. A new keypair will be generated.");
    }

    Ok(())
}
