# Cobotium Mainnet Deployment Guide

This guide provides detailed instructions for deploying your Cobotium token program to the Solana mainnet.

**Official Website**: [cobotium.io](https://cobotium.io)

## Table of Contents

1. [Introduction](#introduction)
2. [Pre-Deployment Checklist](#pre-deployment-checklist)
3. [Deployment Process](#deployment-process)
4. [Post-Deployment Verification](#post-deployment-verification)
5. [Website Integration](#website-integration)
6. [Ongoing Maintenance](#ongoing-maintenance)

## Introduction

Deploying to mainnet is a significant milestone for your Cobotium project. This guide will walk you through the process to ensure a smooth and secure deployment.

## Pre-Deployment Checklist

Before deploying to mainnet, ensure you have completed the following:

### Security Audit

- [x] Code review completed
- [x] Security vulnerabilities addressed
- [x] Overflow protection implemented
- [x] Freeze functionality tested
- [ ] External audit completed (recommended)

### Testing

- [ ] All functionality tested on devnet
- [ ] Edge cases tested
- [ ] Performance testing completed
- [ ] Stress testing completed

### Documentation

- [x] User documentation completed
- [x] Developer documentation completed
- [x] Monitoring documentation completed
- [ ] Website documentation updated

### Infrastructure

- [ ] Monitoring system set up
- [ ] Alerting system configured
- [ ] Backup procedures established
- [ ] Emergency response plan documented

### Legal and Compliance

- [ ] Terms of service finalized
- [ ] Privacy policy finalized
- [ ] Regulatory compliance verified (if applicable)
- [ ] Token economics finalized

## Deployment Process

### Step 1: Final Build

Build the program for deployment:

```bash
cd cobotium-program
cargo build-bpf --release
```

This will create the program binary at `target/deploy/cobotium_program.so`.

### Step 2: Program Keypair

Generate a keypair for your program (if not already done):

```bash
solana-keygen new -o cobotium-program-keypair.json
```

**IMPORTANT**: Back up this keypair securely. It is required for any future program upgrades.

Recommended backup methods:
- Hardware security module
- Multiple encrypted backups in separate physical locations
- Paper backup in a secure location

### Step 3: Fund the Deployer Account

Ensure your deployer account has sufficient SOL for deployment:

```bash
solana balance --keypair <PATH_TO_DEPLOYER_KEYPAIR>
```

If needed, transfer SOL to the deployer account:

```bash
solana transfer --keypair <FUNDING_KEYPAIR> <DEPLOYER_ADDRESS> <AMOUNT_SOL> --allow-unfunded-recipient
```

### Step 4: Deploy to Mainnet

Switch to the mainnet endpoint:

```bash
solana config set --url https://api.mainnet-beta.solana.com
```

Deploy the program:

```bash
solana program deploy --keypair <PATH_TO_DEPLOYER_KEYPAIR> --program-id cobotium-program-keypair.json target/deploy/cobotium_program.so
```

This command will output the program ID. Save this ID as it will be needed for all interactions with your program.

### Step 5: Verify Deployment

Verify that the program was deployed successfully:

```bash
solana program show <PROGRAM_ID>
```

This should display information about your deployed program, including:
- Program ID
- Owner
- Program data account
- Last deployed slot

## Post-Deployment Verification

### Functional Testing

Perform basic functional tests to ensure the program is working correctly:

1. Create a mint:

```bash
./target/release/cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> create-mint --mint-keypair <KEYPAIR_PATH> --decimals 9
```

2. Create a token account:

```bash
./target/release/cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> create-account --account-keypair <KEYPAIR_PATH> --mint <MINT_ADDRESS> --owner <OWNER_ADDRESS>
```

3. Mint tokens:

```bash
./target/release/cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> mint-tokens --mint <MINT_ADDRESS> --account <ACCOUNT_ADDRESS> --mint-authority <AUTHORITY_KEYPAIR> --amount <AMOUNT>
```

### Security Verification

1. Verify freeze functionality:

```bash
./target/release/cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> freeze-account --account <ACCOUNT_ADDRESS> --mint <MINT_ADDRESS> --freeze-authority <FREEZE_AUTH_KEYPAIR>
```

2. Verify thaw functionality:

```bash
./target/release/cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> thaw-account --account <ACCOUNT_ADDRESS> --mint <MINT_ADDRESS> --freeze-authority <FREEZE_AUTH_KEYPAIR>
```

## Website Integration

Update your website at [cobotium.io](https://cobotium.io) with the following information:

### Program Information

- Program ID
- Deployment date
- Current version

### Integration Guide

Create a developer integration guide that includes:

1. SDK installation instructions:

```bash
npm install @cobotium/sdk
# or
yarn add @cobotium/sdk
```

2. Basic usage examples:

```javascript
import { CobotiumClient } from '@cobotium/sdk';

// Initialize client
const client = new CobotiumClient('https://api.mainnet-beta.solana.com', 'YOUR_PROGRAM_ID');

// Create a mint
const mint = await client.createMint(wallet, 9);

// Create an account
const account = await client.createAccount(wallet, mint.publicKey, wallet.publicKey);

// Mint tokens
await client.mintTo(wallet, mint.publicKey, account.publicKey, wallet, 1000000000);
```

### Explorer Links

Add links to Solana explorers for your program:

- [Solana Explorer](https://explorer.solana.com/address/YOUR_PROGRAM_ID)
- [Solscan](https://solscan.io/account/YOUR_PROGRAM_ID)
- [Solana Beach](https://solanabeach.io/address/YOUR_PROGRAM_ID)

## Ongoing Maintenance

### Monitoring

Start your monitoring system:

```bash
cd monitoring
pm2 start monitor.js
```

Refer to the [Monitoring Guide](MONITORING.md) for detailed instructions.

### Regular Health Checks

Schedule regular health checks:

1. Daily:
   - Check program logs for errors
   - Verify transaction volume is within expected range

2. Weekly:
   - Perform basic functionality tests
   - Review account growth

3. Monthly:
   - Comprehensive security review
   - Performance optimization review

### Upgrade Planning

For future upgrades:

1. Develop and test on devnet
2. Conduct security audit
3. Announce upgrade to users
4. Deploy upgrade to mainnet
5. Verify upgrade

## Conclusion

Congratulations on deploying your Cobotium token program to mainnet! By following this guide, you've ensured a secure and well-documented deployment process.

Remember to keep your program keypair secure, monitor your program regularly, and stay responsive to user feedback and potential issues.

For more information and updates, visit [cobotium.io](https://cobotium.io).
