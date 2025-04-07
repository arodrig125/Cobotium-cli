use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    rpc_url: String,
    payer: String,
}

impl AppConfig {
    fn load() -> Option<Self> {
        let config_path = Self::config_path();
        if config_path.exists() {
            let contents = fs::read_to_string(config_path).ok()?;
            serde_json::from_str(&contents).ok()
        } else {
            None
        }
    }

    fn config_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".cobotium/config.json");
        path
    }
}

/// Cobotium CLI: Interact with your on-chain program and CBT token
#[derive(Parser)]
#[clap(name = "Cobotium CLI", version = "0.1.0", about = "Interact with the Cobotium blockchain")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(long, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    #[clap(long, default_value = "~/.config/solana/id.json")]
    payer: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Ping the on-chain Cobotium program
    Ping,

    /// Check SOL balance of a wallet
    Balance {
        #[clap(long)]
        address: String,
    },

    /// Transfer SOL to another wallet
    Transfer {
        #[clap(long)]
        to: String,
        #[clap(long)]
        amount: f64,
    },

    /// Check CBT token balance
    BalanceCbt {
        #[clap(long)]
        owner: String,
    },

    /// Transfer CBT tokens to another wallet
    TransferCbt {
        #[clap(long)]
        to: String,
        #[clap(long)]
        amount: f64,
    },

    /// Faucet: Send free CBT to a user
    Faucet {
        #[clap(long)]
        to: String,
        #[clap(long)]
        amount: f64,
    },
    Config(ConfigCommand),
    },
    #[derive(Subcommand)]
enum ConfigCommand {
    /// Set RPC URL and payer path
    Set {
        #[clap(long)]
        payer: Option<String>,
        #[clap(long)]
        rpc: Option<String>,
    },

    /// Show the current config
    Get,
}

fn main() {
    let cli = Cli::parse();
    let config = AppConfig::load();

    let rpc_url = cli
        .rpc_url
        .clone()
        .or_else(|| config.as_ref().map(|c| c.rpc_url.clone()))
        .unwrap_or_else(|| "https://api.devnet.solana.com".to_string());

    let payer_path = cli
        .payer
        .clone()
        .or_else(|| config.as_ref().map(|c| c.payer.clone()))
        .unwrap_or_else(|| "~/.config/solana/id.json".to_string());

    let payer = load_keypair(&payer_path);
    let rpc = RpcClient::new(rpc_url.clone());

    match cli.command {
        Commands::Ping => {
            if let Err(err) = ping_program(&rpc, &payer) {
                eprintln!("❌ Ping failed: {}", err);
            }
        }
        Commands::Balance { address } => {
            if let Err(err) = check_balance(&rpc, &address) {
                eprintln!("❌ Balance check failed: {}", err);
            }
        }
        Commands::Transfer { to, amount } => {
            if let Err(err) = transfer_tokens(&rpc, &payer, &to, amount) {
                eprintln!("❌ Transfer failed: {}", err);
            }
        }
        Commands::BalanceCbt { owner } => {
            if let Err(err) = balance_cbt(&rpc, &owner) {
                eprintln!("❌ CBT balance check failed: {}", err);
            }
        }
        Commands::TransferCbt { to, amount } => {
            if let Err(err) = transfer_cbt(&rpc, &payer, &to, amount) {
                eprintln!("❌ CBT transfer failed: {}", err);
            }
        }
        Commands::Faucet { to, amount } => {
            if let Err(err) = mint_cbt(&rpc, &payer, &to, amount) {
                eprintln!("❌ Faucet mint failed: {}", err);
            }
        }
        Commands::Config(cmd) => match cmd {
            ConfigCommand::Set { payer, rpc } => {
                if let Err(err) = update_config(payer, rpc) {
                    eprintln!("❌ Config update failed: {}", err);
                }
            }
            ConfigCommand::Get => {
                match AppConfig::load() {
                    Some(config) => {
                        println!("📄 Current config:");
                        println!("🔑 Payer: {}", config.payer);
                        println!("🌐 RPC URL: {}", config.rpc_url);
                    }
                    None => println!("⚠️ No config file found at ~/.cobotium/config.json"),
                }
            }
        }
        }
    }

fn load_keypair(path: &str) -> Keypair {
    let expanded = shellexpand::tilde(path);
    read_keypair_file(expanded.as_ref()).expect("Failed to read keypair file")
}

fn ping_program(rpc: &RpcClient, payer: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str("YOUR_PROGRAM_ID_HERE")?;
    let instruction = Instruction::new_with_bytes(program_id, b"ping", vec![]);

    let blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(&[instruction], Some(&payer.pubkey()), &[payer], blockhash);

    let sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("✅ Ping sent! Signature: {}", sig);
    Ok(())
}

fn check_balance(rpc: &RpcClient, address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pubkey = Pubkey::from_str(address)?;
    let lamports = rpc.get_balance(&pubkey)?;
    println!("💰 Balance for {}: {} lamports ({} SOL)", address, lamports, lamports as f64 / 1_000_000_000.0);
    Ok(())
}

fn transfer_tokens(
    rpc: &RpcClient,
    payer: &Keypair,
    to: &str,
    amount: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let to_pubkey = Pubkey::from_str(to)?;
    let lamports = (amount * 1_000_000_000.0) as u64;
    let blockhash = rpc.get_latest_blockhash()?;

    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(&payer.pubkey(), &to_pubkey, lamports)],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    let sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("✅ Transferred {} SOL to {}\nSignature: {}", amount, to, sig);
    Ok(())
}

fn balance_cbt(rpc: &RpcClient, owner: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str("HAy9rwsDCb6qpVDzLmuNuipWwQacPiw2xefoHSjA9ETW")?;
    let owner_pubkey = Pubkey::from_str(owner)?;
    let ata = spl_associated_token_account::get_associated_token_address(&owner_pubkey, &mint);

    let balance = rpc.get_token_account_balance(&ata)?;
    println!("📦 CBT balance for {}: {}", owner, balance.ui_amount_string);
    Ok(())
}

fn transfer_cbt(
    rpc: &RpcClient,
    payer: &Keypair,
    to: &str,
    amount: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str("HAy9rwsDCb6qpVDzLmuNuipWwQacPiw2xefoHSjA9ETW")?;
    let to_pubkey = Pubkey::from_str(to)?;
    let from_ata = spl_associated_token_account::get_associated_token_address(&payer.pubkey(), &mint);
    let to_ata = spl_associated_token_account::get_associated_token_address(&to_pubkey, &mint);
    let amount = (amount * 1_000_000_000.0) as u64;

    let ix = spl_token::instruction::transfer_checked(
        &spl_token::id(),
        &from_ata,
        &mint,
        &to_ata,
        &payer.pubkey(),
        &[],
        amount,
        9,
    )?;

    let blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], blockhash);

    let sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("✅ Transferred {} CBT to {}\nSignature: {}", amount, to, sig);
    Ok(())
}

fn mint_cbt(
    rpc: &RpcClient,
    payer: &Keypair,
    to: &str,
    amount: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mint = Pubkey::from_str("HAy9rwsDCb6qpVDzLmuNuipWwQacPiw2xefoHSjA9ETW")?;
    let to_pubkey = Pubkey::from_str(to)?;
    let to_ata = spl_associated_token_account::get_associated_token_address(&to_pubkey, &mint);
    let amount = (amount * 1_000_000_000.0) as u64;

    if rpc.get_account(&to_ata).is_err() {
        let ix = spl_associated_token_account::instruction::create_associated_token_account(
            &payer.pubkey(),
            &to_pubkey,
            &mint,
            &spl_token::id(),
        );
        let blockhash = rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], blockhash);
        rpc.send_and_confirm_transaction(&tx)?;
    }

    let mint_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint,
        &to_ata,
        &payer.pubkey(),
        &[],
        amount,
    )?;

    let blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(&[mint_ix], Some(&payer.pubkey()), &[payer], blockhash);

    let sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("✅ Minted {} CBT to {}\nSignature: {}", amount, to, sig);
    Ok(())
}

fn update_config(
    payer: Option<String>,
    rpc_url: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = AppConfig::load().unwrap_or(AppConfig {
        payer: "~/.config/solana/id.json".to_string(),
        rpc_url: "https://api.devnet.solana.com".to_string(),
    });

    if let Some(p) = payer {
        config.payer = p;
    }

    if let Some(r) = rpc_url {
        config.rpc_url = r;
    }

    let path = AppConfig::config_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(path, serde_json::to_string_pretty(&config)?)?;

    println!("✅ Config updated:");
    println!("  RPC: {}", config.rpc_url);
    println!("  Payer: {}", config.payer);
    Ok(())
}