# Project Dependencies

This document explains every external library used in this blockchain project, their purpose, and how they're used. Perfect for understanding the technology stack!

## 📚 Table of Contents

- [Core Library (`btclib`)](#core-library-btclib)
- [Node Dependencies](#node-dependencies)
- [Miner Dependencies](#miner-dependencies)
- [Wallet Dependencies](#wallet-dependencies)
- [Dependency Categories](#dependency-categories)

---

## Core Library (`btclib`)

The foundation of the blockchain. These dependencies provide cryptography, serialization, and networking.

### Cryptography & Security

#### `ecdsa = "0.16.9"`
**Purpose:** Elliptic Curve Digital Signature Algorithm

Digital signatures prove ownership of UTXOs without revealing private keys.

**What it does:**
- Signs transaction inputs with private keys
- Verifies signatures with public keys
- Uses the Secp256k1 curve (same as Bitcoin)

**Example:**
```rust
use ecdsa::{SigningKey, signature::Signer};

let private_key = SigningKey::random(&mut rng);
let message = b"Transfer 10 BTC to Alice";
let signature = private_key.sign(message);

// Signature proves you own the private key without revealing it
```

**Features used:**
- `signing` - Create signatures
- `verifying` - Validate signatures
- `serde` - Serialize/deserialize keys
- `pem` - PEM file format support

---

#### `k256 = "0.13.4"`
**Purpose:** Secp256k1 elliptic curve implementation

The specific curve used by Bitcoin and this blockchain.

**What it does:**
- Provides the mathematical curve for ECDSA
- Generates public keys from private keys
- Handles point multiplication on the curve

**Why Secp256k1?**
- Efficient on standard CPUs
- Well-tested (used by Bitcoin since 2009)
- 256-bit security level

**Example:**
```rust
use k256::SecretKey;

// Generate a random private key
let private_key = SecretKey::random(&mut rng);

// Derive public key from private key (one-way function)
let public_key = private_key.public_key();

// Public key can be shared, private key must stay secret
```

**Features used:**
- `serde` - Serialization support
- `pem` - PEM file format for public keys

---

#### `sha256 = "1.6.0"`
**Purpose:** SHA-256 cryptographic hash function

Used for hashing blocks, transactions, and creating addresses.

**What it does:**
- Generates unique 256-bit fingerprints of data
- One-way function (can't reverse)
- Deterministic (same input = same output)

**Example:**
```rust
use sha256::digest;

let data = "Alice sends 10 BTC to Bob";
let hash = digest(data);
// hash = "5d41402abc4b2a76b9719d911017c592"

// Change one character:
let data2 = "Alice sends 11 BTC to Bob";
let hash2 = digest(data2);
// hash2 = "completely different hash"
```

**Used for:**
- Block hashes (Proof-of-Work)
- Transaction IDs
- Merkle tree nodes
- UTXO identifiers

---

#### `spki = "0.7"`
**Purpose:** Subject Public Key Info (SPKI) encoding

Standard format for public keys.

**What it does:**
- Encodes/decodes public keys in industry-standard format
- Allows keys to be saved to files
- Ensures interoperability

**Example:**
```rust
use spki::EncodePublicKey;

public_key.to_public_key_pem()?;
// -----BEGIN PUBLIC KEY-----
// MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE...
// -----END PUBLIC KEY-----
```

---

### Serialization & Data Formats

#### `serde = "1.0.228"`
**Purpose:** Serialization framework

Converts Rust data structures to/from various formats.

**What it does:**
- Serialize structs to bytes/JSON/CBOR
- Deserialize bytes back to structs
- Works with many formats via "derives"

**Example:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

let tx = Transaction {
    from: "Alice".to_string(),
    to: "Bob".to_string(),
    amount: 100,
};

// Serialize to JSON
let json = serde_json::to_string(&tx)?;
// {"from":"Alice","to":"Bob","amount":100}
```

**Features used:**
- `derive` - Automatic implementation via macros

---

#### `ciborium = "0.2.2"`
**Purpose:** CBOR (Concise Binary Object Representation) codec

Binary serialization format used for blocks and transactions.

**What it does:**
- Serializes data to compact binary format
- Smaller than JSON (saves bandwidth)
- Fast to encode/decode
- Supports complex nested structures

**Example:**
```rust
use ciborium;

let data = MyStruct { /* ... */ };

// Save to file (binary format)
let file = File::create("block.cbor")?;
ciborium::into_writer(&data, file)?;

// Load from file
let file = File::open("block.cbor")?;
let loaded: MyStruct = ciborium::from_reader(file)?;
```

**Why CBOR over JSON?**
- 30-50% smaller file size
- Faster to parse
- Supports binary data natively
- Still human-readable with tools

**Used for:**
- Storing blocks to disk (`blockchain.cbor`)
- Saving transactions (`mytx.cbor`)
- Network message encoding

---

#### `uuid = "1.18.1"`
**Purpose:** Universally Unique Identifiers

Generates unique IDs for transaction outputs.

**What it does:**
- Creates statistically unique 128-bit identifiers
- No coordination needed (decentralized generation)
- Prevents UTXO collisions

**Example:**
```rust
use uuid::Uuid;

let id = Uuid::new_v4();
// e.g., "550e8400-e29b-41d4-a716-446655440000"

// Each UTXO gets a unique ID
let output = TransactionOutput {
    value: 100,
    pubkey: alice_key,
    unique_id: Uuid::new_v4(), // ← Ensures uniqueness
};
```

**Why needed?**
If two transactions create identical outputs (same amount, same recipient), we need UUIDs to tell them apart.

**Features used:**
- `v4` - Random UUID generation
- `serde` - Serialization support

---

### Numeric & Time Libraries

#### `bigdecimal = "0.4.8"`
**Purpose:** Arbitrary precision decimal arithmetic

Used for difficulty adjustment calculations.

**What it does:**
- Handles very large/small numbers precisely
- No floating-point rounding errors
- Critical for difficulty calculations

**Example:**
```rust
use bigdecimal::BigDecimal;

// Calculate: new_target = current_target × (actual_time / ideal_time)
let current_target = BigDecimal::from(1000000);
let ratio = BigDecimal::from(250) / BigDecimal::from(500); // 0.5
let new_target = current_target * ratio; // 500000 (exactly)

// Floating point would introduce errors:
// let ratio_f64 = 250.0 / 500.0; // might be 0.4999999999
```

**Why needed?**
Difficulty involves 256-bit numbers. Small rounding errors could break consensus.

---

#### `chrono = "0.4.42"`
**Purpose:** Date and time handling

Timestamps for blocks and transactions.

**What it does:**
- Current time with timezone support
- Time differences and arithmetic
- Serialization to standard formats

**Example:**
```rust
use chrono::{Utc, Duration};

// Block timestamp
let now = Utc::now();
println!("Block mined at: {}", now);
// "2025-10-12 14:30:45 UTC"

// Calculate time to mine last 50 blocks
let time_diff = end_time - start_time;
let seconds = time_diff.num_seconds();
```

**Features used:**
- `serde` - Serialize timestamps

---

#### `uint = "0.10.0"`
**Purpose:** Unsigned 256-bit integer (U256)

Used for block hashes and difficulty targets.

**What it does:**
- Represents numbers up to 2^256 - 1
- Essential for cryptographic operations
- Allows comparison of hashes vs targets

**Example:**
```rust
use uint::construct_uint;

construct_uint! {
    pub struct U256(4); // 4 × 64-bit words = 256 bits
}

// Difficulty target (very large number)
let target = U256::from_str_radix("0000FFFFFFFFFFFF...", 16)?;

// Check if block hash meets difficulty
if block_hash <= target {
    println!("Block mined successfully!");
}
```

**Why 256 bits?**
- SHA-256 produces 256-bit hashes
- Allows fine-grained difficulty adjustment
- Matches Bitcoin's design

---

### Utilities

#### `hex = "0.4.3"`
**Purpose:** Hexadecimal encoding/decoding

Converts binary data to human-readable hex strings.

**Example:**
```rust
use hex;

let hash_bytes = [0x5d, 0x41, 0x40, 0x2a];
let hex_string = hex::encode(hash_bytes);
// "5d41402a"

let decoded = hex::decode("5d41402a")?;
// [0x5d, 0x41, 0x40, 0x2a]
```

**Used for:**
- Displaying hashes
- Parsing difficulty targets from CLI
- Debug output

---

#### `thiserror = "2.0.17"`
**Purpose:** Ergonomic error handling

Simplifies creating custom error types.

**Example:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BtcError {
    #[error("Invalid transaction")]
    InvalidTransaction,
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Block hash doesn't meet target")]
    InvalidBlock,
}

// Usage:
return Err(BtcError::InvalidTransaction);
// Automatically provides nice error messages
```

**Why better than strings?**
- Type-safe error handling
- Automatic Display implementation
- Easy to match on specific errors

---

#### `rand = "0.8.5"`
**Purpose:** Random number generation

Used for creating private keys.

**Example:**
```rust
use rand::thread_rng;

let mut rng = thread_rng();
let private_key = SigningKey::random(&mut rng);
// Cryptographically secure random key
```

**Critical for security:**
- Keys must be truly random
- Predictable keys = loss of funds
- Uses OS entropy source

---

### Networking

#### `tokio = "1.47.1"`
**Purpose:** Asynchronous runtime

Handles async I/O, networking, and concurrency.

**What it does:**
- Async TCP server/client
- Spawns concurrent tasks
- Non-blocking I/O operations

**Example:**
```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    
    loop {
        let (socket, _) = listener.accept().await?;
        
        // Handle each connection concurrently
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}
```

**Features used:**
- `net` (lib) - TCP networking only
- `full` (node/miner/wallet) - All async features

**Why async?**
- Handle 100s of connections efficiently
- Non-blocking operations
- Better performance than threads

---

## Node Dependencies

Additional dependencies specific to the full node.

### `anyhow = "1.0.100"`
**Purpose:** Flexible error handling

Simplifies error propagation in applications.

**Example:**
```rust
use anyhow::{Result, Context};

fn load_blockchain(path: &str) -> Result<Blockchain> {
    let file = File::open(path)
        .context("Failed to open blockchain file")?;
    
    let blockchain = deserialize(file)
        .context("Failed to deserialize blockchain")?;
    
    Ok(blockchain)
}

// Error chain shows full context:
// Error: Failed to deserialize blockchain
// Caused by: Invalid CBOR format
```

**Difference from `thiserror`:**
- `anyhow` - Application errors (flexible)
- `thiserror` - Library errors (type-safe)

---

### `argh = "0.1.13"`
**Purpose:** Command-line argument parsing

Parse CLI arguments for the node.

**Example:**
```rust
use argh::FromArgs;

#[derive(FromArgs)]
/// A toy blockchain node
struct Args {
    #[argh(option, default = "9000")]
    /// port number
    port: u16,
    
    #[argh(positional)]
    /// addresses of initial nodes
    nodes: Vec<String>,
}

let args: Args = argh::from_env();
println!("Starting node on port {}", args.port);
```

**Usage:**
```bash
./node --port 9001 127.0.0.1:9000
```

---

### `dashmap = "6.1.0"`
**Purpose:** Concurrent HashMap

Thread-safe HashMap for storing peer connections.

**Example:**
```rust
use dashmap::DashMap;

static NODES: DashMap<String, TcpStream> = DashMap::new();

// Thread A
NODES.insert("127.0.0.1:9000", stream1);

// Thread B (concurrent access safe!)
NODES.insert("127.0.0.1:9001", stream2);

// Iterate safely
for node in NODES.iter() {
    send_block(&mut *node.value());
}
```

**Why not Mutex<HashMap>?**
- Better performance (fine-grained locking)
- Multiple readers can access simultaneously
- Perfect for peer management

---

### `static_init = "1.0.4"`
**Purpose:** Safe static initialization

Create complex global variables safely.

**Example:**
```rust
use static_init::dynamic;

#[dynamic]
static BLOCKCHAIN: RwLock<Blockchain> = RwLock::new(Blockchain::new());

// Can now use from any thread
let blockchain = BLOCKCHAIN.read().await;
```

**Why needed?**
- Rust doesn't allow complex statics by default
- Ensures proper initialization
- Thread-safe by design

---

## Miner Dependencies

Dependencies specific to the mining client.

### `clap = "4.5.48"`
**Purpose:** Command-line argument parsing (feature-rich)

More powerful than `argh`, used in miner and wallet.

**Example:**
```rust
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    /// Node address to connect to
    address: String,
    
    #[arg(short, long)]
    /// Public key file for rewards
    public_key_file: String,
}

let cli = Cli::parse();
```

**Features used:**
- `derive` - Derive macros for easy parsing

**Difference from `argh`:**
- `clap` - Full-featured (help, validation, subcommands)
- `argh` - Minimal, faster compilation

---

### `flume = "0.11.1"`
**Purpose:** Multi-producer, multi-consumer channels

Send mined blocks from mining thread to async task.

**Example:**
```rust
use flume;

let (tx, rx) = flume::unbounded();

// Mining thread (sync)
thread::spawn(move || {
    let block = mine_block();
    tx.send(block).unwrap();
});

// Async task
tokio::spawn(async move {
    while let Ok(block) = rx.recv_async().await {
        submit_block(block).await;
    }
});
```

**Why not tokio channels?**
- Works across sync/async boundary
- More flexible
- Unbounded for simplicity

---

## Wallet Dependencies

Dependencies specific to the TUI wallet.

### `crossbeam-skiplist = "0.1.3"`
**Purpose:** Concurrent skip-list map

Thread-safe ordered map for UTXO storage.

**Example:**
```rust
use crossbeam_skiplist::SkipMap;

let utxos = Arc::new(SkipMap::new());

// Thread A: Update UTXOs
utxos.insert(alice_key, vec![utxo1, utxo2]);

// Thread B: Read balance (concurrent!)
let balance: u64 = utxos.iter()
    .map(|entry| entry.value().iter().sum())
    .sum();
```

**Why skip-list?**
- Lock-free data structure
- Concurrent reads/writes
- Sorted by key (public keys)

---

### `cursive = "0.21.1"`
**Purpose:** Terminal User Interface (TUI) framework

Creates the interactive text-based UI.

**Example:**
```rust
use cursive::views::{Dialog, TextView};

let mut siv = cursive::default();

siv.add_layer(Dialog::around(TextView::new("Balance: 10 BTC"))
    .button("Send", |s| show_send_dialog(s))
    .button("Quit", |s| s.quit()));

siv.run();
```

**Provides:**
- Buttons, text inputs, menus
- Keyboard navigation
- Responsive layouts

---

### `futures = "0.3.31"`
**Purpose:** Async/await utilities

Additional async primitives and combinators.

**Example:**
```rust
use futures::future::join_all;

let tasks = vec![
    fetch_utxos(),
    update_balance(),
    handle_transactions(),
];

// Run all tasks concurrently
join_all(tasks).await;
```

---

### `kanal = "0.1.1"`
**Purpose:** Async channels

Send transactions from UI to network layer.

**Example:**
```rust
use kanal::bounded;

let (tx, rx) = bounded(10);

// UI thread
tx.send(transaction).unwrap();

// Network task
while let Ok(tx) = rx.recv().await {
    send_to_node(tx).await;
}
```

**Why another channel library?**
- Designed for async/await
- Bounded queues (backpressure)
- Simple API

---

### `text-to-ascii-art = "0.1.10"`
**Purpose:** Generate ASCII art text

Display large balance numbers in the UI.

**Example:**
```rust
use text_to_ascii_art::to_art;

let art = to_art("10.5 BTC", "standard", 0, 0, 0)?;
```

**Output:**
```
 __   ___    ____    ____  _____ _____  _____ 
/  \ /   \  |___ \  | __ )|_   _|_   _|/ ____|
\__  |  _  |   __) | |  _ \  | |   | | | |     
  / /| |_| |  / __/ _| |_) | | |   | | | |____ 
 /_/  \___/  |_____(_)____/  |_|   |_|  \_____|
```

Makes balance prominent in TUI!

---

### `toml = "0.9.8"`
**Purpose:** TOML configuration file parsing

Read wallet configuration.

**Example:**
```rust
use toml;

#[derive(Deserialize)]
struct Config {
    my_keys: Vec<Key>,
    contacts: Vec<Recipient>,
    default_node: String,
}

let config_str = fs::read_to_string("wallet.toml")?;
let config: Config = toml::from_str(&config_str)?;
```

**TOML format:**
```toml
default_node = "127.0.0.1:9000"

[[my_keys]]
public = "alice.pub.pem"
private = "alice.priv.cbor"

[[contacts]]
name = "Bob"
key = "bob.pub.pem"
```

---

### `tracing = "0.1.41"`
**Purpose:** Structured logging framework

Better than `println!` for applications.

**Example:**
```rust
use tracing::{info, error, debug};

info!("Wallet started");
debug!(balance = 100, "Fetched balance");
error!(err = %e, "Failed to send transaction");
```

**Benefits:**
- Structured (machine-parseable)
- Log levels (info, debug, error)
- Contextual information

---

### `tracing-appender = "0.2.3"`
**Purpose:** Log file rotation

Write logs to files with rotation.

**Example:**
```rust
use tracing_appender::rolling::{RollingFileAppender, Rotation};

let file_appender = RollingFileAppender::new(
    Rotation::DAILY,
    "logs",
    "wallet.log"
);

// Creates: logs/wallet.log.2025-10-12
```

---

### `tracing-subscriber = "0.3.20"`
**Purpose:** Tracing subscriber implementations

Configure how logs are processed and displayed.

**Example:**
```rust
use tracing_subscriber::fmt;

tracing_subscriber::fmt()
    .with_target(false)
    .with_level(true)
    .init();
```

**Features used:**
- `env-filter` - Filter logs by level
- `fmt` - Format log output

---

## Dependency Categories

### By Purpose

```
Cryptography & Security (5):
├─ ecdsa       - Digital signatures
├─ k256        - Secp256k1 curve
├─ sha256      - Hashing
├─ spki        - Key encoding
└─ rand        - Random generation

Data Structures (4):
├─ serde       - Serialization framework
├─ ciborium    - CBOR format
├─ uuid        - Unique identifiers
└─ hex         - Hex encoding

Numeric & Time (3):
├─ bigdecimal  - Arbitrary precision
├─ chrono      - Date/time handling
└─ uint        - 256-bit integers

Networking & Async (3):
├─ tokio       - Async runtime
├─ flume       - Channels (sync/async)
└─ kanal       - Async channels

Concurrency (3):
├─ dashmap     - Concurrent HashMap
├─ crossbeam-skiplist - Concurrent sorted map
└─ static_init - Safe statics

User Interface (2):
├─ cursive     - TUI framework
└─ text-to-ascii-art - ASCII art

Configuration & CLI (3):
├─ clap        - CLI parsing (full-featured)
├─ argh        - CLI parsing (minimal)
└─ toml        - Config files

Error Handling (2):
├─ anyhow      - Flexible errors
└─ thiserror   - Typed errors

Logging (3):
├─ tracing     - Structured logging
├─ tracing-appender - Log rotation
└─ tracing-subscriber - Log config

Utilities (1):
└─ futures     - Async utilities
```

### By Component

```
Core Library (11):
bigdecimal, chrono, ciborium, ecdsa, hex, k256,
rand, serde, sha256, spki, thiserror, tokio, uint, uuid

Node Only (4):
anyhow, argh, dashmap, static_init

Miner Only (2):
clap, flume

Wallet Only (9):
crossbeam-skiplist, cursive, futures, kanal,
text-to-ascii-art, toml, tracing, tracing-appender,
tracing-subscriber
```

---

## Choosing Dependencies

### Why These Specific Libraries?

**1. Industry Standard**
- `serde` - De-facto serialization in Rust
- `tokio` - Most popular async runtime
- `clap` - Standard CLI parsing

**2. Cryptographic Audits**
- `k256`, `ecdsa` - Part of RustCrypto (audited)
- `sha256` - Well-tested implementation

**3. Performance**
- `dashmap` - Faster than Mutex<HashMap>
- `ciborium` - Compact binary format
- `crossbeam` - Lock-free algorithms

**4. Educational Clarity**
- Simple APIs for learning
- Good documentation
- Active maintenance

---

## Version Pinning

Notice version numbers are specific (not `"*"`):

```toml
serde = "1.0.228"  # Exact version
NOT: serde = "*"   # Any version (dangerous!)
```

**Why?**
- Reproducible builds
- Avoid breaking changes
- Security (can audit specific version)

---

## Further Reading

**Official Documentation:**
- Rust crates: https://crates.io
- Docs.rs: https://docs.rs

**Specific Crates:**
- Tokio Guide: https://tokio.rs/tokio/tutorial
- Serde Documentation: https://serde.rs
- RustCrypto: https://github.com/RustCrypto

---

## Summary

This project uses **28 unique dependencies**:
- 📚 **11** in core library (blockchain primitives)
- 🌐 **4** additional for node (networking)
- ⛏️ **2** additional for miner (mining)
- 💰 **9** additional for wallet (user interface)

Each dependency solves a specific problem and contributes to creating a functional, educational blockchain implementation.

