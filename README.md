# Cobotium Blockchain

Cobotium is a Solana-based blockchain project that provides a custom token program and tools for interacting with the blockchain.

## Project Structure

- `cobotium-program`: The on-chain Solana program for Cobotium tokens
- `sdk`: Client-side SDK for interacting with the Cobotium program
- `cli`: Command-line interface for interacting with the Cobotium blockchain

## Getting Started

### Prerequisites

- Rust and Cargo
- Solana CLI tools
- A Solana wallet with SOL for deployment and transactions

### Building the Project

```bash
# Clone the repository
git clone https://github.com/your-username/cobotium.git
cd cobotium

# Build the project
cargo build --release
```

### Deploying the Program

1. Build the program:

```bash
cd cobotium-program
cargo build-bpf
```

2. Deploy to devnet for testing:

```bash
solana program deploy --program-id <KEYPAIR_PATH> target/deploy/cobotium_program.so
```

3. Update the program ID in your CLI commands:

```bash
./target/release/cobotium-cli --program-id <PROGRAM_ID> <COMMAND>
```

## Mainnet Launch Checklist

### 1. Program Development and Testing

- [x] Implement the token program
- [x] Implement the SDK
- [x] Implement the CLI
- [ ] Write comprehensive tests
- [ ] Test on localnet
- [ ] Test on devnet

### 2. Security

- [ ] Conduct internal code review
- [ ] Conduct external security audit
- [ ] Fix all identified vulnerabilities
- [ ] Implement proper error handling

### 3. Deployment

- [ ] Generate a secure program keypair for mainnet
- [ ] Backup the program keypair securely
- [ ] Deploy to mainnet
- [ ] Verify the deployment

### 4. Post-Launch

- [ ] Monitor the program for any issues
- [ ] Set up alerts for unusual activity
- [ ] Prepare for potential upgrades

## Mainnet Deployment Instructions

1. Generate a new keypair for the program (if not already done):

```bash
solana-keygen new -o cobotium-program-keypair.json
```

2. Build the program for deployment:

```bash
cd cobotium-program
cargo build-bpf
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

5. Update your applications to use the mainnet program ID.

## Using the CLI

The Cobotium CLI provides various commands for interacting with the blockchain:

### Basic SOL Operations

```bash
# Mint SOL to an address
cobotium-cli --rpc-url https://api.mainnet-beta.solana.com mint-sol --to <ADDRESS> --amount <AMOUNT>

# Transfer SOL
cobotium-cli --rpc-url https://api.mainnet-beta.solana.com transfer-sol --to <ADDRESS> --amount <AMOUNT>

# Check balance
cobotium-cli --rpc-url https://api.mainnet-beta.solana.com balance --address <ADDRESS>
```

### Cobotium Token Operations

```bash
# Create a new mint
cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> create-mint --mint-keypair <KEYPAIR_PATH> --decimals 9

# Create a token account
cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> create-account --account-keypair <KEYPAIR_PATH> --mint <MINT_ADDRESS> --owner <OWNER_ADDRESS>

# Mint tokens
cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> mint-tokens --mint <MINT_ADDRESS> --account <ACCOUNT_ADDRESS> --mint-authority <AUTHORITY_KEYPAIR> --amount <AMOUNT>

# Transfer tokens
cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> transfer-tokens --source <SOURCE_ADDRESS> --destination <DESTINATION_ADDRESS> --owner <OWNER_KEYPAIR> --amount <AMOUNT>

# Burn tokens
cobotium-cli --rpc-url https://api.mainnet-beta.solana.com --program-id <PROGRAM_ID> burn-tokens --account <ACCOUNT_ADDRESS> --mint <MINT_ADDRESS> --owner <OWNER_KEYPAIR> --amount <AMOUNT>
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
