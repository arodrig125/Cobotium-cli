# Cobotium Documentation

Welcome to the official documentation for the Cobotium project, a Solana-based token platform.

**Official Website**: [cobotium.io](https://cobotium.io)

## Table of Contents

1. [Introduction](#introduction)
2. [Architecture](#architecture)
3. [Getting Started](#getting-started)
4. [Token Program](#token-program)
5. [SDK](#sdk)
6. [CLI](#cli)
7. [Security Features](#security-features)
8. [Deployment Guide](#deployment-guide)
9. [Monitoring](#monitoring)
10. [Troubleshooting](#troubleshooting)

## Introduction

Cobotium is a custom token platform built on the Solana blockchain. It provides a secure, efficient, and feature-rich token program that allows for:

- Creating token mints with configurable decimals
- Minting tokens to accounts
- Transferring tokens between accounts
- Burning tokens
- Freezing and thawing accounts for security purposes

For more information, visit our website at [cobotium.io](https://cobotium.io).

## Architecture

The Cobotium project consists of three main components:

1. **Token Program**: An on-chain Solana program that handles token operations
2. **SDK**: A client-side library for interacting with the token program
3. **CLI**: A command-line interface for easy interaction with the token program

### Component Diagram

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│                 │     │                 │     │                 │
│  Cobotium CLI   │────▶│  Cobotium SDK   │────▶│ Cobotium Token  │
│                 │     │                 │     │    Program      │
└─────────────────┘     └─────────────────┘     └─────────────────┘
        │                                               │
        │                                               │
        │                                               │
        ▼                                               ▼
┌─────────────────┐                           ┌─────────────────┐
│                 │                           │                 │
│  Solana Client  │                           │  Solana Runtime │
│                 │                           │                 │
└─────────────────┘                           └─────────────────┘
```

## Getting Started

### Prerequisites

- Rust and Cargo
- Solana CLI tools (v1.17.0 or later)
- A Solana wallet with SOL for transactions

### Installation

1. Clone the repository:

```bash
git clone https://github.com/arodrig125/Cobotium-cli.git
cd Cobotium-cli
```

2. Build the project:

```bash
cargo build --release
```

3. Run the CLI:

```bash
./target/release/cobotium-cli --help
```

## Token Program

The Cobotium Token Program is a Solana program that implements token functionality. It supports the following operations:

### Initialize Mint

Creates a new token mint with specified decimals and optional freeze authority.

### Initialize Account

Creates a new token account associated with a specific mint.

### Mint To

Mints new tokens to a specified account.

### Transfer

Transfers tokens from one account to another.

### Burn

Burns (destroys) tokens from an account.

### Freeze Account

Freezes an account, preventing any token operations until thawed.

### Thaw Account

Thaws a frozen account, allowing token operations again.

## SDK

The Cobotium SDK provides a convenient way to interact with the Cobotium Token Program from Rust applications.

### Usage Example

```rust
use cobotium_sdk::CobotiumClient;
use solana_sdk::signature::{Keypair, Signer};

// Create a client
let client = CobotiumClient::new("https://api.devnet.solana.com", "YOUR_PROGRAM_ID")?;

// Initialize a mint
let payer = Keypair::from_file("path/to/keypair.json")?;
let mint = Keypair::new();
let decimals = 9;
let freeze_authority = Some(&payer.pubkey());

client.initialize_mint(&payer, &mint, &payer.pubkey(), freeze_authority, decimals)?;

// Create an account
let account = Keypair::new();
client.initialize_account(&payer, &account, &mint.pubkey(), &payer.pubkey())?;

// Mint tokens
let amount = 1000000000; // 1 token with 9 decimals
client.mint_to(&payer, &mint.pubkey(), &account.pubkey(), &payer, amount)?;
```

## CLI

The Cobotium CLI provides a command-line interface for interacting with the Cobotium Token Program.

### Commands

#### Create a Mint

```bash
cobotium-cli --program-id <PROGRAM_ID> create-mint --mint-keypair <KEYPAIR_PATH> --decimals 9 --freeze-authority <FREEZE_AUTH_KEYPAIR_PATH>
```

#### Create an Account

```bash
cobotium-cli --program-id <PROGRAM_ID> create-account --account-keypair <KEYPAIR_PATH> --mint <MINT_ADDRESS> --owner <OWNER_ADDRESS>
```

#### Mint Tokens

```bash
cobotium-cli --program-id <PROGRAM_ID> mint-tokens --mint <MINT_ADDRESS> --account <ACCOUNT_ADDRESS> --mint-authority <AUTHORITY_KEYPAIR> --amount <AMOUNT>
```

#### Transfer Tokens

```bash
cobotium-cli --program-id <PROGRAM_ID> transfer-tokens --source <SOURCE_ADDRESS> --destination <DESTINATION_ADDRESS> --owner <OWNER_KEYPAIR> --amount <AMOUNT>
```

#### Burn Tokens

```bash
cobotium-cli --program-id <PROGRAM_ID> burn-tokens --account <ACCOUNT_ADDRESS> --mint <MINT_ADDRESS> --owner <OWNER_KEYPAIR> --amount <AMOUNT>
```

#### Freeze Account

```bash
cobotium-cli --program-id <PROGRAM_ID> freeze-account --account <ACCOUNT_ADDRESS> --mint <MINT_ADDRESS> --freeze-authority <FREEZE_AUTH_KEYPAIR>
```

#### Thaw Account

```bash
cobotium-cli --program-id <PROGRAM_ID> thaw-account --account <ACCOUNT_ADDRESS> --mint <MINT_ADDRESS> --freeze-authority <FREEZE_AUTH_KEYPAIR>
```

## Security Features

The Cobotium Token Program includes several security features:

### Freeze Authority

The freeze authority is a special role that can freeze and thaw accounts. This is useful for:

- Emergency situations where suspicious activity is detected
- Compliance requirements
- Upgrading the protocol

### Overflow Protection

All arithmetic operations are checked for overflow/underflow to prevent potential exploits.

### Ownership Verification

The program verifies ownership of accounts to ensure only authorized users can perform operations.

### Initialization Checks

The program prevents re-initialization of accounts to avoid potential security issues.

## Deployment Guide

### Devnet Deployment

1. Build the program for BPF target:

```bash
cd cobotium-program
cargo build-bpf
```

2. Generate a keypair for the program (if not already done):

```bash
solana-keygen new -o cobotium-program-keypair.json
```

3. Deploy to devnet:

```bash
solana config set --url https://api.devnet.solana.com
solana program deploy --program-id cobotium-program-keypair.json target/deploy/cobotium_program.so
```

4. Save the program ID:

```bash
solana address -k cobotium-program-keypair.json
```

### Mainnet Deployment

1. Build the program for BPF target:

```bash
cd cobotium-program
cargo build-bpf
```

2. Generate a keypair for the program (if not already done):

```bash
solana-keygen new -o cobotium-program-keypair.json
```

3. Deploy to mainnet:

```bash
solana config set --url https://api.mainnet-beta.solana.com
solana program deploy --program-id cobotium-program-keypair.json target/deploy/cobotium_program.so
```

4. Save the program ID:

```bash
solana address -k cobotium-program-keypair.json
```

## Monitoring

### Transaction Monitoring

You can monitor transactions involving your program using the Solana CLI:

```bash
solana logs --url https://api.mainnet-beta.solana.com <PROGRAM_ID>
```

### Automated Monitoring

For automated monitoring, consider setting up:

1. A dedicated server running `solana logs` and parsing the output
2. Alerts for specific events (e.g., large transfers, freeze events)
3. Integration with monitoring services like Datadog, Prometheus, or Grafana

### Health Checks

Regularly perform health checks on your program:

1. Verify the program is still deployed
2. Test basic functionality
3. Monitor account growth

## Troubleshooting

### Common Errors

#### "Invalid Program ID"

Ensure you're using the correct program ID in your commands.

#### "Insufficient Funds"

Ensure the account has enough tokens for the operation.

#### "Account Already Initialized"

The account you're trying to initialize already exists.

#### "Account Frozen"

The account is frozen and cannot perform token operations until thawed.

### Getting Help

If you encounter issues not covered in this documentation, please:

1. Visit our website at [cobotium.io](https://cobotium.io) for the latest information
2. Check the GitHub repository for open issues
3. Open a new issue with detailed information about your problem
4. Contact the Cobotium team for support
