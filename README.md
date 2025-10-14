# Custom Decentralized Ledger (Blockchain) in Rust

A Bitcoin-inspired blockchain implementation built from scratch in Rust for educational purposes. This project demonstrates core blockchain concepts including UTXO model, Proof-of-Work consensus, cryptographic signatures, and peer-to-peer networking.

> **Note:** This implementation is based on the book **"Building Bitcoin in Rust"**. It's an educational project designed to help others understand how blockchain technology works under the hood.

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

### Documentation Structure

Each component has its own detailed README:
- [**lib/**](./lib/README.md) - Core blockchain concepts and implementation
- [**node/**](./node/README.md) - Full node architecture and networking
- [**miner/**](./miner/README.md) - Mining process and Proof-of-Work
- [**wallet/**](./wallet/README.md) - Wallet functionality and user interface

Additional documentation:
- [**QUICKSTART.md**](./QUICKSTART.md) - Step-by-step tutorial
- [**CONFIGURATION.md**](./CONFIGURATION.md) - Environment variables and .env files
- [**DEPENDENCIES.md**](./DEPENDENCIES.md) - Explanation of all libraries used
- [**CREDITS.md**](./CREDITS.md) - Attribution and acknowledgments
- [**docker/README.md**](./docker/README.md) - Docker deployment guide
- [**LICENSE**](./LICENSE) - MIT License

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

## 🐳 Docker Quick Start (Recommended!)

The easiest way to run everything is with Docker:

```bash
# 1. One-time setup (builds images and generates keys)
./docker/setup.sh

# 2. Start the entire network (3 nodes + 2 miners)
./docker/start.sh

# 3. View logs
./docker/logs.sh

# 4. Check status
./docker/status.sh

# 5. Stop when done
./docker/stop.sh
```

**What you get:**
- ✅ 3 interconnected blockchain nodes
- ✅ 2 miners producing blocks
- ✅ Persistent data storage
- ✅ Isolated network
- ✅ No need to install Rust!

**First run:** Takes 5-10 minutes to build (compiling in release mode)

**Ports exposed:**
- Node 1: `localhost:9000`
- Node 2: `localhost:9001`
- Node 3: `localhost:9002`

**Connect your wallet:**
```bash
# Your local wallet can connect to the Docker network
cargo run --bin good-wallet -- -c wallet.toml -n localhost:9000
```

For detailed Docker documentation, see [docker/README.md](./docker/README.md)

**Pro Tip:** Use the `Makefile` for even easier commands:
```bash
make setup    # Same as ./docker/setup.sh
make start    # Same as ./docker/start.sh
make stop     # Same as ./docker/stop.sh
make logs     # Same as ./docker/logs.sh
make help     # See all available commands
```

## ⚙️ Configuration

The blockchain is fully configurable via environment variables!

### Quick Configuration

```bash
# 1. Copy example config
cp .env.example .env

# 2. Edit as needed
nano .env

# 3. Run (automatically loads .env)
cargo run --bin node
```

### Network Profiles

Pre-configured profiles for different use cases:

```bash
# Mainnet (default - standard speed)
cp .env.example .env

# Testnet (2x faster, easier mining)
cp .env.testnet.example .env

# Devnet (5x faster, instant mining)
cp .env.devnet.example .env
```

### Environment Variable Examples

```bash
# Change block time
IDEAL_BLOCK_TIME=5 cargo run --bin node

# Easier difficulty
MIN_TARGET_HEX=0x00FFFFFFFFFFFFFF... cargo run --bin node

# Custom port
NODE_PORT=9001 cargo run --bin node

# Debug logging
RUST_LOG=debug cargo run --bin miner
```

### What's Configurable?

- 🎯 **Network parameters**: Block time, difficulty, rewards
- 🌐 **Node settings**: Port, peers, save intervals
- ⛏️ **Mining config**: Batch size, update frequency
- 💰 **Wallet config**: Update intervals, node address
- 📊 **Logging**: Log level, backtraces

**See [CONFIGURATION.md](./CONFIGURATION.md) for complete reference**

## 📖 Learning Path

If you're new to blockchain, we recommend reading in this order:

1. **Start here:** [Key Concepts](#-key-concepts-explained-for-beginners) below - Fundamental blockchain concepts
2. **[DEPENDENCIES.md](./DEPENDENCIES.md)** - Understanding the libraries used (optional but helpful)
3. **[lib/README.md](./lib/README.md)** - Core concepts (UTXO, PoW, Merkle Trees) in depth
4. **[node/README.md](./node/README.md)** - Network and consensus
5. **[miner/README.md](./miner/README.md)** - Mining mechanics
6. **[wallet/README.md](./wallet/README.md)** - User interaction
7. **[QUICKSTART.md](./QUICKSTART.md)** - Hands-on tutorial to run everything

## 🔑 Key Concepts Explained (For Beginners)

### Hash Functions
**What is a hash?** A hash is like a digital fingerprint - it takes any data and produces a unique fixed-size code.

```
Input: "Hello World"  →  SHA-256  →  Output: "a591a6d4..."
Input: "Hello World!" →  SHA-256  →  Output: "7f83b165..." (completely different!)
```

**Properties:**
- **Deterministic**: Same input always gives same output
- **One-way**: Can't reverse it (fingerprint → original data)
- **Avalanche effect**: Tiny change in input = completely different output
- **Unique**: Nearly impossible to find two inputs with same output

**Used everywhere in blockchain:**
- Block IDs
- Transaction IDs  
- UTXO identifiers
- Proof-of-Work (finding hash that meets target)

---

### Public & Private Keys (Digital Wallets)
**Think of it like a mailbox:**

```
Private Key = Physical key to open your mailbox (KEEP SECRET!)
Public Key  = Your mailing address (SHARE FREELY!)
```

**How it works:**
1. You generate a **private key** (random 256-bit number)
2. Math derives your **public key** from private key (one-way!)
3. Public key becomes your "address" where people send coins
4. Only your private key can "unlock" (spend) coins sent to that address

**Security:**
- Private key is like your password × 1000
- If someone gets it, they steal ALL your coins
- Public key is safe to share (it's your "username")
- Can't calculate private key from public key (mathematically impossible)

---

### Digital Signatures (Proving Ownership)
**Problem:** How do you prove you own coins without revealing your private key?

**Solution:** Digital signatures!

```
1. You want to spend 10 BTC
2. Create message: "Transfer 10 BTC from UTXO #123 to Bob"
3. Sign message with your PRIVATE key → creates signature
4. Everyone can verify signature with your PUBLIC key
5. Signature proves you own private key WITHOUT revealing it!
```

**Real-world analogy:** Like signing a check
- Only you can create your signature (private key)
- Anyone can verify it's your signature (public key)
- Can't forge your signature without your pen (private key)

---

### Nonce (Number Used Once)
The magic number that makes Proof-of-Work work!

```
Block Header:
├─ Previous block hash: 0x1234...
├─ Transactions merkle root: 0x5678...
├─ Timestamp: 2025-10-12 14:30:00
├─ Target: 0x0000FFFF...
└─ Nonce: ??? ← The number we're searching for!

Mining Process:
Try nonce = 0 → Hash: 0x9876... (too big, doesn't meet target)
Try nonce = 1 → Hash: 0x8765... (too big, doesn't meet target)
Try nonce = 2 → Hash: 0x7654... (too big, doesn't meet target)
...
Try nonce = 482,573 → Hash: 0x0000A3B2... (SUCCESS! ✓)
```

**Why needed?**
Without the nonce, the hash would be fixed. The nonce gives us a way to keep trying different hashes until we find one that works.

---

### Coinbase Transaction (Block Reward)
**Special first transaction in every block** that creates new coins!

```
Normal Transaction:
Inputs: [Alice's 50 BTC]  → Outputs: [Bob gets 30 BTC, Alice gets 20 BTC change]
(Must reference existing coins)

Coinbase Transaction:
Inputs: [] (EMPTY - no previous coins!)  → Outputs: [Miner gets 50 BTC]
(Creates NEW coins from thin air!)
```

**Rules:**
1. Must be FIRST transaction in block
2. Has ZERO inputs
3. Output amount = Block reward + transaction fees
4. Only one per block
5. Name comes from "coin base" (base of new coins)

**Block Reward Schedule:**
```
Blocks 0-209:    50 BTC per block
Blocks 210-419:  25 BTC per block (halved!)
Blocks 420-629:  12.5 BTC per block (halved again!)
...
```

This is how new coins enter circulation!

---

### Satoshis vs BTC (Units)
Like dollars and cents:

```
1 BTC = 100,000,000 satoshis
1 satoshi = 0.00000001 BTC

Examples:
- 0.5 BTC = 50,000,000 satoshis
- 1,000 satoshis = 0.00001 BTC
- Transaction fee: 1,000 sats = 0.00001 BTC
```

**Why satoshis?**
- Bitcoin is divisible (like you can have $0.01)
- Allows micro-transactions
- Internally, everything is stored as satoshis (integers)
- No floating-point errors!

---

### Block Height vs Block Hash
**Two ways to identify a block:**

```
Block Height = Position in chain (0, 1, 2, 3...)
Block Hash   = Unique fingerprint (0x5d41402a...)

Example:
Block #0 (Genesis Block)
├─ Height: 0
├─ Hash: 0xABC123...
└─ Previous Hash: 0x000000... (none, it's first)

Block #1
├─ Height: 1
├─ Hash: 0xDEF456...
└─ Previous Hash: 0xABC123... (links to Block #0)
```

**Difference:**
- **Height**: Simple counter (easy for humans)
- **Hash**: Cryptographic proof (used by protocol)

**Why both?**
- Height for: "Get me block 100"
- Hash for: "Verify this exact block hasn't changed"

---

### Consensus (Agreement Without Trust)
**Problem:** 1000 computers need to agree on transaction order, but some might be malicious.

**Traditional solution:** Central authority (bank decides)
**Blockchain solution:** Proof-of-Work consensus

```
Scenario: Alice tries to spend same 10 BTC twice

Node A receives: Alice → Bob (10 BTC)
Node B receives: Alice → Charlie (10 BTC)
Nodes disagree! Who's right?

Solution:
1. Both transactions go to mempool
2. Miner includes ONLY ONE in their block
3. Miner solves Proof-of-Work first
4. All nodes accept that block
5. The other transaction gets rejected (double-spend prevented)

Consensus achieved! All nodes agree on same history.
```

**Why it works:**
- Longest chain (most work) wins
- Rewriting history requires redoing ALL the work
- 51% of mining power would need to collude
- Economically irrational (costs more than benefit)

---

### UTXO Model
Unlike account-based systems (like Ethereum), this blockchain uses **Unspent Transaction Outputs**. Each transaction consumes previous outputs and creates new ones. This provides better privacy and parallelizability.

[See detailed explanation in lib/README.md](./lib/README.md#1-utxo-model-unspent-transaction-outputs)

---

### Proof-of-Work
Miners compete to find a nonce that makes the block hash meet a difficulty target. This makes the blockchain immutable - rewriting history requires redoing all the computational work.

[See detailed explanation in lib/README.md](./lib/README.md#3-proof-of-work-pow)

---

### Difficulty Adjustment
The network automatically adjusts mining difficulty every 50 blocks to maintain a target block time of 10 seconds.

---

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

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### What does this mean?

You are free to:
- ✅ Use this code for learning
- ✅ Use it in your own projects (even commercial)
- ✅ Modify and adapt it
- ✅ Share it with others

You just need to:
- 📝 Include the original copyright notice
- 📝 Include the MIT License text

**TL;DR:** Do whatever you want with this code, just give credit! 🎉

## 🙏 Acknowledgments

### Primary Source
This project is **based on the book "Building Bitcoin in Rust"**. The core architecture, algorithms, and implementation approach follow the book's educational methodology. Special thanks to the book's author for providing an excellent learning resource for blockchain education.

### Additional Inspiration
- **Satoshi Nakamoto's Bitcoin Whitepaper** - The foundation of blockchain technology
- **RustCrypto Project** - High-quality cryptographic libraries
- **Rust Community** - Excellent documentation and ecosystem

### Educational Purpose
This implementation is intended for **educational purposes** to help others learn blockchain concepts. If you're learning blockchain development, consider getting the original book for comprehensive explanations and theory.

### Detailed Attribution
For comprehensive credits, source attribution, and contribution guidelines, see [CREDITS.md](./CREDITS.md).

---

**Ready to learn?** Start with the [QUICKSTART.md](./QUICKSTART.md) guide!

