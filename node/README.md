# Full Node

A full node maintains the complete blockchain state, validates all transactions and blocks, and serves requests from wallets and miners. It's the backbone of the decentralized network.

## 📚 Table of Contents

- [What is a Full Node?](#what-is-a-full-node)
- [Responsibilities](#responsibilities)
- [Architecture](#architecture)
- [Network Protocol](#network-protocol)
- [Blockchain Synchronization](#blockchain-synchronization)
- [Running a Node](#running-a-node)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## What is a Full Node?

A **full node** is a program that:
1. **Validates** every transaction and block according to consensus rules
2. **Stores** the complete blockchain history
3. **Serves** data to wallets and miners
4. **Propagates** new transactions and blocks to other nodes
5. **Maintains** a mempool of unconfirmed transactions

Think of it as a bank teller that:
- ✅ Verifies all checks before accepting them
- 📚 Keeps complete records of all accounts
- 🤝 Shares information with other branches
- 🚫 Rejects fraudulent transactions

## Responsibilities

### 1. Block Validation

Every block received must pass these checks:

```rust
✅ Block hash meets difficulty target
✅ Previous block hash matches chain tip
✅ Merkle root is correctly calculated
✅ Timestamp is after previous block
✅ All transactions are valid
✅ Coinbase reward is correct
✅ No double-spending
```

**Why it matters:**
- Prevents invalid blocks from entering the chain
- Ensures consensus rules are followed
- Protects the network from attacks

### 2. Transaction Validation

Every transaction must:

```rust
✅ Reference existing, unspent UTXOs
✅ Have valid signatures
✅ Input sum ≥ Output sum
✅ Not double-spend (within block or mempool)
```

### 3. Mempool Management

The **mempool** (memory pool) holds unconfirmed transactions:

```rust
Mempool Features:
- Sorted by fee (highest first)
- Periodic cleanup (remove old transactions)
- Size-limited (prevents DoS)
- Tracks "marked" UTXOs (reserved for pending txs)
```

### 4. Blockchain Synchronization

When a new node joins:

```
1. Connect to known peers
2. Discover more peers through gossip
3. Find the longest chain
4. Download all blocks
5. Validate and add each block
6. Rebuild UTXO set
```

### 5. P2P Networking

Nodes communicate via TCP sockets:

```
Node A                         Node B
  |                              |
  |--- Connect to Node B ------→|
  |                              |
  |←-- Send NodeList -----------|
  |                              |
  |--- Request Block 42 -------→|
  |                              |
  |←-- Send Block 42 -----------|
```

## Architecture

### File Structure

```
node/
├── Cargo.toml          # Dependencies
└── src/
    ├── main.rs         # Entry point, TCP server
    ├── handler.rs      # Message handling logic
    └── util.rs         # Sync, persistence helpers
```

### Component Diagram

```
┌─────────────────────────────────────────┐
│              Node Process               │
├─────────────────────────────────────────┤
│                                         │
│  ┌───────────────────────────────────┐ │
│  │      TCP Listener (Port 9000)     │ │
│  └─────────────┬─────────────────────┘ │
│                │                        │
│                ↓                        │
│  ┌───────────────────────────────────┐ │
│  │   Connection Handler (per peer)   │ │
│  │   • Receive messages              │ │
│  │   • Validate requests             │ │
│  │   • Update blockchain state       │ │
│  │   • Send responses                │ │
│  └─────────────┬─────────────────────┘ │
│                │                        │
│                ↓                        │
│  ┌───────────────────────────────────┐ │
│  │     Global Blockchain State       │ │
│  │   (RwLock<Blockchain>)            │ │
│  │   • Blocks                        │ │
│  │   • UTXOs                         │ │
│  │   • Mempool                       │ │
│  │   • Target difficulty             │ │
│  └───────────────────────────────────┘ │
│                                         │
│  ┌───────────────────────────────────┐ │
│  │    Background Tasks               │ │
│  │   • Periodic mempool cleanup      │ │
│  │   • Periodic blockchain save      │ │
│  └───────────────────────────────────┘ │
│                                         │
└─────────────────────────────────────────┘
```

### Global State

The node maintains two global singletons:

```rust
// Blockchain state (thread-safe)
static BLOCKCHAIN: RwLock<Blockchain> = RwLock::new(Blockchain::new());

// Connected peers
static NODES: DashMap<String, TcpStream> = DashMap::new();
```

**Thread Safety:**
- `RwLock` allows multiple readers OR one writer
- `DashMap` is a concurrent HashMap
- Each connection gets its own task

### Message Flow

```
Incoming Message
    ↓
Parse Message Type
    ↓
┌───────────────────────────────────────┐
│ Match message:                        │
├───────────────────────────────────────┤
│ FetchUTXOs       → Query & respond    │
│ SubmitTransaction → Validate & add    │
│ FetchTemplate    → Build & send       │
│ SubmitTemplate   → Validate & store   │
│ NewBlock         → Validate & append  │
│ DiscoverNodes    → Send peer list     │
│ AskDifference    → Compare heights    │
│ FetchBlock       → Send specific block│
└───────────────────────────────────────┘
    ↓
Update Blockchain State
    ↓
Broadcast to Peers (if needed)
```

## Network Protocol

### Message Types

The node handles these message types (defined in `lib/network.rs`):

#### Wallet ↔ Node

```rust
// Wallet requests UTXOs for a public key
FetchUTXOs(PublicKey)
  ↓
UTXOs(Vec<(TransactionOutput, bool)>)

// Wallet submits a transaction
SubmitTransaction(Transaction)
  → Validated and added to mempool
  → Broadcast to peers as NewTransaction
```

#### Miner ↔ Node

```rust
// Miner requests a block template
FetchTemplate(PublicKey)  // Pubkey for coinbase
  ↓
Template(Block)  // Ready to mine

// Miner validates template is still good
ValidateTemplate(Block)
  ↓
TemplateValidity(bool)

// Miner submits mined block
SubmitTemplate(Block)
  → Validated and added to chain
  → Broadcast to peers as NewBlock
```

#### Node ↔ Node

```rust
// Discover other nodes
DiscoverNodes
  ↓
NodeList(Vec<String>)  // ["127.0.0.1:9001", ...]

// Compare blockchain heights
AskDifference(local_height)
  ↓
Difference(height_diff)  // Can be negative

// Download a specific block
FetchBlock(height)
  ↓
NewBlock(Block)

// Propagate new transaction
NewTransaction(Transaction)
  → Add to mempool

// Propagate new block
NewBlock(Block)
  → Validate and add to chain
```

### Connection Handling

Each incoming connection spawns an async task:

```rust
async fn handle_connection(mut socket: TcpStream) {
    loop {
        // Receive message
        let message = Message::receive_async(&mut socket).await?;
        
        // Process based on type
        match message {
            Message::FetchUTXOs(pubkey) => {
                // Lock blockchain (read)
                let blockchain = BLOCKCHAIN.read().await;
                
                // Filter UTXOs for this pubkey
                let utxos = blockchain.utxos()
                    .iter()
                    .filter(|(_, (_, output))| output.pubkey == pubkey)
                    .collect();
                
                // Send response
                Message::UTXOs(utxos).send_async(&mut socket).await?;
            }
            // ... other messages
        }
    }
}
```

## Blockchain Synchronization

When starting up, a node must sync with the network:

### Initial Sync Flow

```
1. Load from disk (if blockchain.cbor exists)
   ├─→ Deserialize blockchain
   ├─→ Rebuild UTXO set
   └─→ Adjust difficulty if needed

2. OR sync from network
   ├─→ Connect to seed nodes
   ├─→ Discover more peers (DiscoverNodes)
   ├─→ Find longest chain (AskDifference)
   ├─→ Download blocks (FetchBlock)
   └─→ Validate each block
```

### Code Example

```rust
// Check if blockchain file exists
if Path::new(&blockchain_file).exists() {
    // Load from disk
    load_blockchain(&blockchain_file).await?;
} else {
    // Sync from network
    populate_connections(&seed_nodes).await?;
    
    if !seed_nodes.is_empty() {
        // Find node with longest chain
        let (longest_node, height) = find_longest_chain_node().await?;
        
        // Download all blocks
        download_blockchain(&longest_node, height).await?;
        
        // Rebuild state
        let mut blockchain = BLOCKCHAIN.write().await;
        blockchain.rebuild_utxos();
        blockchain.try_adjust_target();
    }
}
```

### Persistence

The blockchain is periodically saved to disk:

```rust
// Background task runs every 15 seconds
async fn save(blockchain_file: String) {
    let mut interval = time::interval(Duration::from_secs(15));
    loop {
        interval.tick().await;
        let blockchain = BLOCKCHAIN.read().await;
        blockchain.save_to_file(&blockchain_file)?;
    }
}
```

**File Format:**
- CBOR (Concise Binary Object Representation)
- Compact and fast
- Human-unreadable (use `block_print` tool)

## Running a Node

### Basic Usage

```bash
# Start a node on default port (9000)
cargo run --bin node

# Specify custom port
cargo run --bin node -- --port 9001

# Custom blockchain file location
cargo run --bin node -- --blockchain-file ./my_chain.cbor

# Connect to existing network
cargo run --bin node -- --port 9002 127.0.0.1:9000 127.0.0.1:9001
```

### Command-Line Arguments

```rust
Options:
  --port <PORT>
      Port to listen on (default: 9000)
  
  --blockchain-file <FILE>
      Path to blockchain storage file (default: ./blockchain.cbor)
  
  <NODES>...
      Addresses of initial nodes to connect to
      Example: 127.0.0.1:9000 192.168.1.5:9000
```

### Examples

**Seed Node (first node):**
```bash
cargo run --bin node -- --port 9000
```

**Join existing network:**
```bash
# Connect to seed node at 127.0.0.1:9000
cargo run --bin node -- --port 9001 127.0.0.1:9000
```

**Multiple peers:**
```bash
cargo run --bin node -- --port 9002 127.0.0.1:9000 127.0.0.1:9001
```

## Configuration

### Constants (in `lib/lib.rs`)

```rust
// Block time target
IDEAL_BLOCK_TIME = 10 seconds

// Difficulty adjustment interval
DIFFICULTY_UPDATE_INTERVAL = 50 blocks

// Mempool transaction expiry
MAX_MEMPOOL_TRANSACTION_AGE = 600 seconds (10 minutes)

// Max transactions per block
BLOCK_TRANSACTION_CAP = 20 transactions
```

### Background Tasks

Two tasks run continuously:

1. **Mempool Cleanup** (every 30 seconds)
   - Removes transactions older than 10 minutes
   - Unmarks their UTXOs
   - Prevents memory bloat

2. **Blockchain Persistence** (every 15 seconds)
   - Saves blockchain to disk
   - Allows recovery after crash
   - Uses atomic write (temp file + rename)

## Node Lifecycle

### Startup

```
1. Parse command-line arguments
2. Load or sync blockchain
3. Start TCP listener on 0.0.0.0:<port>
4. Spawn background tasks
5. Accept connections (loop forever)
```

### Handling a Transaction

```
1. Receive SubmitTransaction message
2. Acquire write lock on blockchain
3. Validate transaction:
   ✓ All inputs exist
   ✓ Signatures are valid
   ✓ No double-spending
   ✓ Input sum ≥ Output sum
4. Add to mempool (sorted by fee)
5. Mark UTXOs as "in use"
6. Broadcast to peers
7. Release lock
```

### Handling a Block

```
1. Receive SubmitTemplate or NewBlock
2. Acquire write lock on blockchain
3. Validate block:
   ✓ Hash meets target
   ✓ Previous hash matches
   ✓ Merkle root correct
   ✓ All transactions valid
   ✓ Coinbase correct
4. Add to blockchain
5. Remove transactions from mempool
6. Rebuild UTXOs (consume inputs, create outputs)
7. Try adjust difficulty
8. Broadcast to peers
9. Release lock
```

## Troubleshooting

### Common Issues

**Node won't start:**
```bash
Error: Address already in use
```
→ Port is occupied. Use a different port: `--port 9001`

**Can't connect to peers:**
```bash
Error: Connection refused
```
→ Check peer is running and firewall allows connections

**Blockchain won't sync:**
```bash
Error: Invalid block
```
→ Peer has invalid chain. Try different seed nodes

**High memory usage:**
```bash
Mempool growing too large
```
→ Reduce `MAX_MEMPOOL_TRANSACTION_AGE` or `BLOCK_TRANSACTION_CAP`

### Debug Output

The node prints helpful messages:

```bash
Listening on 0.0.0.0:9000
blockchain file does not exist!
trying to connect to other nodes...
received new block
block looks good, broadcasting
cleaning the mempool from old transactions
saving blockchain to drive...
```

### Checking Node State

You can inspect the blockchain file:

```bash
# View block details
cargo run --bin block_print blockchain.cbor

# Check node is responding
# (Use wallet or miner to send messages)
```

### Performance Tips

1. **Reduce log verbosity** - Remove `println!` calls for production
2. **Increase save interval** - Less frequent disk writes
3. **Limit peer connections** - Fewer concurrent handlers
4. **Use release mode** - Much faster: `cargo run --release --bin node`

## Security Considerations

### Current Limitations

⚠️ **This is an educational implementation**. It lacks:

1. **No authentication** - Anyone can connect
2. **No rate limiting** - Vulnerable to spam
3. **No connection limits** - Can exhaust resources
4. **No peer reputation** - Trusts all peers equally
5. **Global state** - All threads access shared data

### Attack Vectors

**Eclipse Attack:**
- Attacker surrounds victim with malicious nodes
- Victim only sees attacker's chain
- **Mitigation:** Connect to diverse, trusted peers

**Sybil Attack:**
- Attacker creates many fake identities
- Can overwhelm network with spam
- **Mitigation:** Proof-of-Work for message sending, connection limits

**DoS (Denial of Service):**
- Attacker sends many invalid transactions/blocks
- Node wastes CPU validating
- **Mitigation:** Rate limiting, IP bans, challenge-response

## Monitoring

### Key Metrics to Track

```rust
- Number of connected peers
- Blockchain height
- Mempool size
- Blocks validated per second
- Average block time
- Current difficulty
- Disk usage
```

### Adding Logging

Replace `println!` with structured logging:

```rust
// Add to Cargo.toml
tracing = "0.1"
tracing-subscriber = "0.3"

// In code
use tracing::{info, warn, error};

info!("Block validated: height={}", height);
warn!("Invalid transaction received");
error!("Failed to sync: {}", err);
```

## Next Steps

- **Run a node** and observe the logs
- **Connect multiple nodes** on different ports
- **Start a miner** to produce blocks
- **Use the wallet** to create transactions
- **Read the code** in `handler.rs` to understand message processing

## Further Reading

- [Bitcoin P2P Network](https://developer.bitcoin.org/devguide/p2p_network.html)
- [Node Types](https://en.bitcoin.it/wiki/Full_node)
- [Network Protocol](https://en.bitcoin.it/wiki/Protocol_documentation)

---

**Next:** Learn about [Mining](../miner/README.md) or try the [Quick Start Guide](../QUICKSTART.md)

