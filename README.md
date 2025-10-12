# Custom Decentralized Ledger (Blockchain) in Rust

A Bitcoin-inspired blockchain implementation built from scratch in Rust for educational purposes. This project demonstrates core blockchain concepts including UTXO model, Proof-of-Work consensus, cryptographic signatures, and peer-to-peer networking.

## 📚 Project Overview

This is a complete, working blockchain system with:
- **Node**: Full blockchain validation and P2P networking
- **Miner**: Proof-of-Work mining client
- **Wallet**: Terminal UI for managing keys and sending transactions
- **Core Library**: Reusable blockchain primitives

Based on the book **"Building Bitcoin in Rust"**, this implementation provides a hands-on learning experience for understanding how cryptocurrencies work under the hood.

## ✨ Features

### Blockchain Core
- ✅ **UTXO Model** - Unspent Transaction Output tracking (like Bitcoin)
- ✅ **Proof-of-Work** - SHA-256 based mining with dynamic difficulty adjustment
- ✅ **Merkle Trees** - Efficient transaction commitment in blocks
- ✅ **Cryptographic Signatures** - ECDSA with Secp256k1 curve
- ✅ **Block Validation** - Comprehensive transaction and block verification
- ✅ **Halving Schedule** - Block rewards decrease over time

### Network
- ✅ **P2P Protocol** - TCP-based peer-to-peer communication
- ✅ **Blockchain Sync** - Download and validate blockchain from peers
- ✅ **Transaction Broadcasting** - Propagate transactions across the network
- ✅ **Mempool** - Transaction pool with fee-based prioritization

### Applications
- ✅ **Full Node** - Maintain blockchain state and serve requests
- ✅ **Miner** - Mine blocks and earn rewards
- ✅ **Wallet** - User-friendly TUI for managing funds

## 🏗️ Architecture

```
custom-dlt-rs/
├── lib/              # Core blockchain library (btclib)
│   ├── src/          # Core logic (crypto, networking, validation)
│   └── types/        # Data structures (Block, Transaction, Blockchain)
├── node/             # Full node implementation
├── miner/            # Mining client
└── wallet/           # Wallet with Terminal UI
```

Each component has its own detailed README:
- [**lib/**](./lib/README.md) - Core blockchain concepts and implementation
- [**node/**](./node/README.md) - Full node architecture and networking
- [**miner/**](./miner/README.md) - Mining process and Proof-of-Work
- [**wallet/**](./wallet/README.md) - Wallet functionality and user interface

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.70+ (install from [rustup.rs](https://rustup.rs))
- **Git**

### Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd custom-dlt-rs

# Build all components
cargo build --workspace --release

# Or build in debug mode (faster compilation)
cargo build --workspace
```

### Running the System

See [QUICKSTART.md](./QUICKSTART.md) for a complete step-by-step tutorial on running a local blockchain network.

#### Quick Test (3 terminals)

**Terminal 1 - Start a Node:**
```bash
cargo run --bin node -- --port 9000
```

**Terminal 2 - Start a Miner:**
```bash
# Generate a key pair first
cargo run --bin key_gen miner_key

# Start mining
cargo run --bin miner -- -a 127.0.0.1:9000 -p miner_key.pub.pem
```

**Terminal 3 - Use the Wallet:**
```bash
# Generate wallet config
cargo run --bin good-wallet -- generate-config -o wallet.toml

# Edit wallet.toml to add your keys, then run:
cargo run --bin good-wallet -- -c wallet.toml -n 127.0.0.1:9000
```

## 📖 Learning Path

If you're new to blockchain, we recommend reading in this order:

1. **[lib/README.md](./lib/README.md)** - Core concepts (UTXO, PoW, Merkle Trees)
2. **[node/README.md](./node/README.md)** - Network and consensus
3. **[miner/README.md](./miner/README.md)** - Mining mechanics
4. **[wallet/README.md](./wallet/README.md)** - User interaction
5. **[QUICKSTART.md](./QUICKSTART.md)** - Hands-on tutorial

## 🔑 Key Concepts Explained

### UTXO Model
Unlike account-based systems (like Ethereum), this blockchain uses **Unspent Transaction Outputs**. Each transaction consumes previous outputs and creates new ones. This provides better privacy and parallelizability.

### Proof-of-Work
Miners compete to find a nonce that makes the block hash meet a difficulty target. This makes the blockchain immutable - rewriting history requires redoing all the computational work.

### Difficulty Adjustment
The network automatically adjusts mining difficulty every 50 blocks to maintain a target block time of 10 seconds.

### Block Rewards
Miners earn rewards that halve every 210 blocks, creating a deflationary supply schedule similar to Bitcoin.

## 📊 Network Parameters

| Parameter | Value |
|-----------|-------|
| Block Time Target | 10 seconds |
| Difficulty Adjustment | Every 50 blocks |
| Block Size | 20 transactions max |
| Halving Interval | 210 blocks |
| Initial Reward | 50 coins (5,000,000,000 satoshis) |
| Hash Algorithm | SHA-256 |
| Signature Scheme | ECDSA (Secp256k1) |

## 🛠️ Utilities

The project includes several CLI tools:

```bash
# Generate key pairs
cargo run --bin key_gen <name>

# Create a transaction
cargo run --bin tx_gen <output_file>

# Print transaction details
cargo run --bin tx_print <tx_file>

# Generate a genesis block
cargo run --bin block_gen <output_file> [custom_target_hex]

# Print block details
cargo run --bin block_print <block_file>
```

## 🧪 Development

### Running Tests

```bash
cargo test --workspace
```

### Building Documentation

```bash
cargo doc --workspace --open
```

### Debug Mode

For faster iteration during development:
```bash
cargo build --workspace  # No --release flag
```

## 📁 Project Structure Details

### Core Library (`lib/`)
- **crypto.rs** - ECDSA signatures and key management
- **sha256.rs** - Hashing utilities
- **network.rs** - P2P message protocol
- **util.rs** - Merkle trees and serialization helpers
- **types/** - Core data structures

### Node (`node/`)
- **main.rs** - TCP server and initialization
- **handler.rs** - Message handling and blockchain operations
- **util.rs** - Blockchain sync and persistence

### Miner (`miner/`)
- **main.rs** - Mining loop and template management

### Wallet (`wallet/`)
- **core.rs** - UTXO management and transaction creation
- **ui.rs** - Terminal user interface
- **tasks.rs** - Background workers
- **util.rs** - Configuration and helpers

## 🎓 Educational Value

This project is excellent for learning:
- Blockchain fundamentals
- Cryptographic primitives
- Distributed systems
- Rust programming
- Network protocols
- Data structures (Merkle trees, hash maps)

## ⚠️ Production Readiness

**This is an educational project.** It demonstrates blockchain concepts but lacks:
- Fork resolution and chain reorganization
- Connection pooling and peer management
- Persistent database (uses in-memory + file serialization)
- Advanced DOS protection
- Comprehensive test coverage
- SPV (Simplified Payment Verification)

For production systems, consider established frameworks like:
- [Substrate](https://substrate.io/) (Rust)
- [Cosmos SDK](https://cosmos.network/) (Go)
- [Tendermint](https://tendermint.com/) (BFT consensus)

## 🤝 Contributing

This is an educational project. Feel free to:
- Add tests
- Improve documentation
- Fix bugs
- Add features for learning purposes

## 📚 Resources

- [Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf)
- [Mastering Bitcoin](https://github.com/bitcoinbook/bitcoinbook)
- [Learn Me a Bitcoin](https://learnmeabitcoin.com/)
- [Rust Book](https://doc.rust-lang.org/book/)

## 📄 License

[Specify your license here]

## 🙏 Acknowledgments

Based on the book **"Building Bitcoin in Rust"**.

---

**Ready to learn?** Start with the [QUICKSTART.md](./QUICKSTART.md) guide!

