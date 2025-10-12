# Core Blockchain Library (`btclib`)

This is the heart of the blockchain implementation. It contains all the fundamental data structures, cryptographic primitives, and validation logic needed to build a Bitcoin-like cryptocurrency.

## 📚 Table of Contents

- [Overview](#overview)
- [Key Concepts](#key-concepts)
- [Module Structure](#module-structure)
- [Data Structures](#data-structures)
- [Algorithms](#algorithms)
- [API Examples](#api-examples)

## Overview

The `btclib` library provides:
- **Data Structures**: Blocks, transactions, blockchain state
- **Cryptography**: ECDSA signatures, SHA-256 hashing
- **Validation**: Transaction and block verification logic
- **Networking**: P2P message protocol
- **Utilities**: Merkle trees, serialization

## Key Concepts

### 1. UTXO Model (Unspent Transaction Outputs)

**What is UTXO?**

Unlike traditional databases with account balances, Bitcoin uses **outputs** that can be spent exactly once. Think of them as physical cash:

#### Real-Life Analogy: Cash at a Coffee Shop

Imagine you want to buy a $3 coffee, but you only have a $5 bill:

```
Your Wallet Before:
├─ $5 bill (this is like a UTXO)

At the Coffee Shop:
├─ You give: $5 bill
├─ Coffee costs: $3
└─ You receive: $2 bill (change)

Your Wallet After:
├─ $2 bill (new UTXO)
└─ [The $5 bill is gone forever - it's been "spent"]
```

**Key insight:** You can't tear the $5 bill in half! You must spend the entire bill and get change back.

#### How UTXO Works in Blockchain

```
Alice's Wallet:
├─ UTXO #1: 50 BTC (from mining block 0)

Alice Sends 30 BTC to Bob:

Transaction {
  Inputs: [
    ├─ UTXO #1 (50 BTC) ← Must consume the ENTIRE amount
    └─ Signed with Alice's private key
  ]
  Outputs: [
    ├─ 30 BTC → Bob's address (payment)
    ├─ 19.99 BTC → Alice's address (change to herself)
    └─ 0.01 BTC → (implicit fee for miner)
  ]
}

After Transaction:
├─ UTXO #1 is DESTROYED (marked as spent)
├─ New UTXO #2: 30 BTC → Bob (Bob can spend this)
└─ New UTXO #3: 19.99 BTC → Alice (Alice can spend this)
```

#### More Complex Example: Multiple UTXOs

Alice has received payments from different sources:

```
Alice's Wallet:
├─ UTXO #1: 10 BTC (from Bob)
├─ UTXO #2: 15 BTC (from Charlie)
└─ UTXO #3: 8 BTC (from mining)

Total Balance: 33 BTC

Alice wants to send 25 BTC to David:

Transaction {
  Inputs: [
    ├─ UTXO #1 (10 BTC) ← Not enough alone
    ├─ UTXO #2 (15 BTC) ← Need to combine multiple
    └─ Both signed by Alice
  ]
  Outputs: [
    ├─ 25 BTC → David (payment)
    └─ 0 BTC → Alice (no change needed, perfect amount!)
  ]
}

Alice's Wallet After:
├─ UTXO #3: 8 BTC (untouched, wasn't needed)
└─ Total Balance: 8 BTC
```

#### Why Not Just Update an Account Balance?

**Traditional Bank Account Model:**
```
Database Table: accounts
├─ Alice: balance = 100
└─ Bob: balance = 50

Transfer $30 from Alice to Bob:
├─ UPDATE accounts SET balance = 70 WHERE name = 'Alice'
└─ UPDATE accounts SET balance = 80 WHERE name = 'Bob'
```

**Problems with Account Model in Blockchain:**
1. ❌ Parallel transactions are hard (what if two transactions try to update Alice's balance simultaneously?)
2. ❌ Complete transaction history requires separate records
3. ❌ Difficult to verify you have funds without scanning all history

**Benefits of UTXO Model:**
- ✅ **Better Privacy**: Each payment creates new addresses/outputs
- ✅ **Parallel Processing**: Different UTXOs are independent (can validate simultaneously)
- ✅ **Simpler Double-Spend Prevention**: Just check if UTXO exists in unspent set
- ✅ **Clear Ownership**: Each UTXO has exactly one owner at a time
- ✅ **Explicit History**: See exactly which coins came from where

#### UTXO Lifecycle

```
1. CREATION (Born)
   └─ New UTXO created as transaction output
   
2. UNSPENT (Available)
   └─ Sits in the "UTXO set"
   └─ Owner can spend it anytime
   
3. SPENT (Dead)
   └─ Consumed as transaction input
   └─ Removed from UTXO set forever
   └─ Cannot be spent again (double-spend prevention)
```

#### Real Blockchain Example

```
Block 0: (Genesis/Coinbase)
Transaction {
  Inputs: [] (no inputs - new coins created)
  Outputs: [50 BTC → Miner's address]
}
UTXO Set After Block 0:
└─ UTXO(hash=0xABC...): 50 BTC → Miner

Block 1:
Transaction {
  Inputs: [UTXO(hash=0xABC...)] ← Spend the miner's reward
  Outputs: [
    ├─ 25 BTC → Alice
    └─ 25 BTC → Bob
  ]
}
UTXO Set After Block 1:
├─ UTXO(hash=0xDEF...): 25 BTC → Alice
└─ UTXO(hash=0x123...): 25 BTC → Bob
(The original UTXO is gone)

Block 2:
Transaction {
  Inputs: [UTXO(hash=0xDEF...)] ← Alice spends hers
  Outputs: [
    ├─ 10 BTC → Charlie
    └─ 14.99 BTC → Alice (change)
  ]
}
UTXO Set After Block 2:
├─ UTXO(hash=0x123...): 25 BTC → Bob (still unspent)
├─ UTXO(hash=0x456...): 10 BTC → Charlie
└─ UTXO(hash=0x789...): 14.99 BTC → Alice
```

**Implementation:** See `types/transaction.rs`

### 2. Mempool (Memory Pool)

**What is a Mempool?**

The mempool is a **waiting room** for unconfirmed transactions. When someone creates a transaction, it doesn't immediately go into a block. Instead, it sits in the mempool until a miner includes it in a block.

#### Real-Life Analogy: Airport Security Queue

Think of the mempool like the security line at an airport:

```
                    ┌─────────────────────────┐
Passengers Arrive → │  Security Queue         │ → Board Plane
(Transactions)      │  (Mempool)              │   (Block)
                    │                         │
                    │  ┌────────────────────┐ │
                    │  │ Priority Pass      │ │
                    │  │ (Higher Fee)       │ │
                    │  └────────────────────┘ │
                    │  ┌────────────────────┐ │
                    │  │ Regular Passenger  │ │
                    │  │ (Lower Fee)        │ │
                    │  └────────────────────┘ │
                    └─────────────────────────┘
```

- **Passengers** = Transactions
- **Security Queue** = Mempool
- **Priority Pass** = Higher transaction fees
- **Boarding the plane** = Getting included in a block

#### How the Mempool Works

```
1. User creates transaction
   └─→ "Send 5 BTC to Bob"

2. Transaction enters mempool
   └─→ Validated: signatures OK, UTXOs exist, no double-spend
   └─→ Added to waiting queue

3. Miners select transactions
   └─→ Pick highest fee transactions first
   └─→ Limited space (20 transactions per block in our system)

4. Transaction included in block
   └─→ Removed from mempool
   └─→ Now confirmed on blockchain

5. Block propagates to network
   └─→ Other nodes remove same transactions from their mempools
```

#### Mempool States

```
Transaction Lifecycle:

┌──────────────┐
│   Created    │  User signs transaction
└──────┬───────┘
       │
       ↓
┌──────────────┐
│  Broadcast   │  Sent to nodes
└──────┬───────┘
       │
       ↓
┌──────────────┐
│   Mempool    │  ◄─── We are here (unconfirmed)
│  (Waiting)   │       • Validated but not in blockchain
│              │       • Can be replaced (Replace-By-Fee)
└──────┬───────┘       • Might expire if too old
       │
       ↓
┌──────────────┐
│   In Block   │  Miner includes in block
└──────┬───────┘
       │
       ↓
┌──────────────┐
│  Confirmed   │  Block accepted by network
└──────────────┘  Transaction is final
```

#### Why Mempools Exist

**Problem without mempool:**
```
Miner mines block immediately upon receiving transaction
  ↓
Only 1 transaction per block (wasteful)
  ↓
Very slow network (10 seconds per transaction)
```

**Solution with mempool:**
```
Collect many transactions (up to 20 in our system)
  ↓
Mine one block with all of them
  ↓
Efficient: 20 transactions per 10 seconds = 2 tx/second
```

#### Fee-Based Prioritization

Transactions in the mempool are **sorted by fee**:

```
Mempool (sorted highest fee first):
┌───────────────────────────────────┐
│ 1. Alice→Bob  (0.5 BTC fee)  ⭐⭐⭐ │ ← Miner picks this first
│ 2. Carol→Dave (0.1 BTC fee)  ⭐⭐  │
│ 3. Eve→Frank  (0.01 BTC fee) ⭐   │
│ 4. Greg→Helen (0.001 BTC fee) ▪   │ ← Might wait a long time
└───────────────────────────────────┘

When miner creates block:
- Takes top 20 transactions (highest fees)
- Lower fee transactions wait for next block
- Very low fee transactions might never confirm
```

#### Real Example Flow

```
Time: 0s
  User sends transaction: "Alice → Bob: 10 BTC (0.1 BTC fee)"
  
Time: 1s
  Transaction validated and enters mempool
  Node broadcasts to peers
  
Time: 2s
  10 other transactions arrive (various fees)
  Mempool now has 11 transactions total
  
Time: 5s
  Miner requests block template
  Node creates template with highest-fee transactions
  Alice's transaction included (fee is competitive)
  
Time: 8s
  Miner finds valid nonce
  Block is broadcast
  
Time: 9s
  All nodes receive block
  Remove Alice's transaction from mempool
  Transaction is now CONFIRMED ✓
```

#### Mempool Properties in This Implementation

```rust
Mempool Data Structure:
Vec<(DateTime<Utc>, Transaction)>
     ↑                    ↑
     Timestamp          The transaction
     (for cleanup)

Properties:
- Sorted by fee (highest first)
- Transactions older than 10 minutes are removed
- Maximum of 20 transactions included per block
- UTXOs used in mempool transactions are "marked"
```

#### UTXO Marking System

To prevent double-spending within the mempool, we mark UTXOs:

```
Example:
1. Alice has UTXO: 50 BTC
2. Alice creates Transaction A: spend 50 BTC → Bob
3. Transaction A enters mempool
4. UTXO is marked: (true, 50 BTC)
5. Alice tries to create Transaction B: spend same 50 BTC → Charlie
6. Transaction B is rejected (UTXO is marked)

Alternative (Replace-By-Fee):
5. Alice creates Transaction B with HIGHER fee
6. Transaction A is removed from mempool
7. Transaction B replaces it
8. UTXO is re-marked for Transaction B
```

#### Mempool Cleanup

Transactions don't stay in mempool forever:

```
Cleanup Process (every 30 seconds):
1. Check timestamp of each transaction
2. If older than 10 minutes (MAX_MEMPOOL_TRANSACTION_AGE):
   - Remove from mempool
   - Unmark its UTXOs
   - User can try again with higher fee

Why cleanup?
- Prevents memory bloat
- Removes stuck low-fee transactions
- Allows users to retry with higher fees
```

#### Common Mempool Issues

**Issue 1: Transaction Stuck**
```
Problem: Low fee transaction waiting days
Solution: 
  - Replace-By-Fee (send again with higher fee)
  - Wait for mempool to clear
  - After 10 minutes, transaction expires and can be resent
```

**Issue 2: Mempool Full**
```
Problem: Mempool has 100 transactions, all higher fee than yours
Solution:
  - Increase your fee
  - Wait for blocks to clear mempool
  - Try during low-traffic time
```

**Issue 3: Double-Spend Attempt**
```
Problem: Try to spend same UTXO twice
Result:
  - First transaction: Accepted ✓
  - Second transaction: Rejected ✗ (UTXO marked)
Solution:
  - Use RBF if you want to replace first transaction
```

#### Mempool vs Confirmed Transactions

```
┌─────────────────┬──────────────┬────────────────┐
│                 │   Mempool    │   Confirmed    │
├─────────────────┼──────────────┼────────────────┤
│ Reversible?     │ YES          │ NO             │
│ Can be replaced?│ YES (RBF)    │ NO             │
│ Guaranteed?     │ NO           │ YES            │
│ How long?       │ Seconds-mins │ Forever        │
│ Visible to all? │ Most nodes   │ ALL nodes      │
│ Trust level     │ LOW          │ HIGH           │
└─────────────────┴──────────────┴────────────────┘
```

**Best Practice:**
- For small amounts: 1 confirmation is usually enough
- For large amounts: Wait for 6+ confirmations
- Never trust 0-confirmation (mempool only) for valuable transactions

#### Code References

**Adding to mempool:** See `types/blockchain.rs` → `add_to_mempool()`
**Mempool cleanup:** See `types/blockchain.rs` → `cleanup_mempool()`
**Fetching for mining:** See `node/src/handler.rs` → `FetchTemplate`

**Implementation:** See `types/blockchain.rs`

### 3. Proof-of-Work (PoW)

**What is PoW?**

A mechanism to achieve consensus in a distributed network without trust. Miners compete to find a number (nonce) that makes the block hash meet a difficulty requirement.

**How it works:**

```rust
// The block hash must be less than the target
hash(block_header) ≤ target

// Lower target = harder difficulty
// Example target: 0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFF...
//                  ^^^^
//                  Must start with zeros
```

**The Mining Process:**

```
1. Collect transactions from mempool
2. Create block header with:
   - Previous block hash
   - Merkle root of transactions
   - Timestamp
   - Target difficulty
   - Nonce = 0
3. Loop:
   - Hash the block header
   - If hash ≤ target: Success! Broadcast block
   - Else: nonce++, try again
```

**Why it matters:**
- 🔒 Makes blockchain immutable (rewriting requires redoing all work)
- ⚖️ Provides fairness (more computing power = more chance to mine)
- 🎯 Self-regulating (difficulty adjusts to maintain block time)

**Implementation:** See `types/block.rs` → `BlockHeader::mine()`

### 3. Merkle Trees

**What is a Merkle Tree?**

A cryptographic data structure that allows efficient verification of large datasets. Used to commit to all transactions in a block with a single hash.

**Structure:**

```
         Root Hash (Merkle Root)
          /                \
       Hash(A,B)          Hash(C,D)
       /      \           /      \
    Hash(A) Hash(B)  Hash(C)  Hash(D)
      |       |        |        |
     Tx A    Tx B     Tx C     Tx D
```

**Benefits:**
- ✅ Verify any transaction is in the block with O(log n) proof
- ✅ Light clients don't need all transactions
- ✅ Changes to any transaction change the root

**Implementation:** See `util.rs` → `MerkleRoot::calculate()`

### 4. Digital Signatures

**What are Digital Signatures?**

Proof that a transaction was authorized by the owner of the private key. Uses ECDSA (Elliptic Curve Digital Signature Algorithm) with the Secp256k1 curve.

**How it works:**

```
Private Key → [Sign message] → Signature
Public Key + Signature + Message → [Verify] → Valid/Invalid
```

**Process:**
1. User has private key (secret) and public key (shared)
2. To spend UTXO, sign its hash with private key
3. Network verifies signature using public key
4. Only correct private key can create valid signature

**Security:**
- 🔐 Private key never leaves user's device
- 🔓 Public key can be shared freely
- ✍️ Signature proves ownership without revealing private key

**Implementation:** See `crypto.rs`

### 5. Difficulty Adjustment

**What is Difficulty Adjustment?**

The network automatically adjusts mining difficulty to maintain a consistent block time, regardless of total mining power.

**Algorithm:**

```rust
// Every 50 blocks:
actual_time = time_to_mine_last_50_blocks
target_time = 50 blocks × 10 seconds = 500 seconds

new_target = current_target × (actual_time / target_time)

// Clamped to prevent extreme changes:
new_target = clamp(new_target, current_target / 4, current_target × 4)
```

**Examples:**
- Blocks mined too fast → Target decreases (harder)
- Blocks mined too slow → Target increases (easier)

**Implementation:** See `types/blockchain.rs` → `try_adjust_target()`

### 6. Block Structure

**Components:**

```rust
Block {
    header: BlockHeader {
        timestamp: DateTime,      // When block was created
        nonce: u64,              // Number used once (for PoW)
        prev_block_hash: Hash,   // Links to previous block
        merkle_root: MerkleRoot, // Commits to all transactions
        target: U256,            // Difficulty target
    },
    transactions: Vec<Transaction>
}
```

**Block Validation Checks:**
1. ✅ Hash meets difficulty target
2. ✅ Previous block hash matches
3. ✅ Merkle root is correct
4. ✅ Timestamp is after previous block
5. ✅ All transactions are valid
6. ✅ Coinbase transaction is correct

**Implementation:** See `types/block.rs`

### 7. Transaction Structure

**Components:**

```rust
Transaction {
    inputs: Vec<TransactionInput> {
        prev_transaction_output_hash: Hash,  // Which UTXO to spend
        signature: Signature,                // Proof of ownership
    },
    outputs: Vec<TransactionOutput> {
        value: u64,              // Amount in satoshis
        unique_id: Uuid,         // Unique identifier
        pubkey: PublicKey,       // Who can spend this
    }
}
```

**Transaction Validation:**
1. ✅ All input UTXOs exist and are unspent
2. ✅ All signatures are valid
3. ✅ Sum of inputs ≥ Sum of outputs (difference = fee)
4. ✅ No double-spending within block

**Special Case - Coinbase Transaction:**
- First transaction in every block
- Has no inputs (creates new coins)
- Outputs = Block reward + Transaction fees
- Pays the miner for their work

**Implementation:** See `types/transaction.rs`

## Module Structure

```
lib/
├── src/
│   ├── lib.rs          # Module exports and constants
│   ├── crypto.rs       # ECDSA signatures, key management
│   ├── sha256.rs       # SHA-256 hashing wrapper
│   ├── network.rs      # P2P message protocol
│   ├── util.rs         # Merkle trees, serialization
│   ├── error.rs        # Error types
│   └── bin/            # CLI utilities
│       ├── key_gen.rs      # Generate key pairs
│       ├── tx_gen.rs       # Create transactions
│       ├── tx_print.rs     # Display transactions
│       ├── block_gen.rs    # Create blocks
│       └── block_print.rs  # Display blocks
└── types/
    ├── mod.rs          # Type exports
    ├── transaction.rs  # Transaction structures
    ├── block.rs        # Block structures and validation
    └── blockchain.rs   # Blockchain state management
```

## Data Structures

### Core Constants

```rust
// Initial block reward (50 BTC = 5,000,000,000 satoshis)
pub const INITIAL_REWARD: u64 = 50;

// Reward halves every 210 blocks
pub const HALVING_INTERVAL: u64 = 210;

// Target: 10 seconds per block
pub const IDEAL_BLOCK_TIME: u64 = 10;

// Adjust difficulty every 50 blocks
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;

// Max 20 transactions per block
pub const BLOCK_TRANSACTION_CAP: usize = 20;

// Mempool transactions expire after 10 minutes
pub const MAX_MEMPOOL_TRANSACTION_AGE: u64 = 600;

// Easiest possible difficulty (256-bit number)
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);
```

### Hash Type

```rust
pub struct Hash(U256);

impl Hash {
    // Hash any serializable data
    pub fn hash<T: Serialize>(data: &T) -> Self;
    
    // Check if hash meets PoW target
    pub fn matches_target(&self, target: U256) -> bool;
    
    // Zero hash (for genesis block)
    pub fn zero() -> Self;
}
```

### Transaction Types

```rust
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

pub struct TransactionInput {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}

pub struct TransactionOutput {
    pub value: u64,        // Amount in satoshis (1 BTC = 100,000,000 satoshis)
    pub unique_id: Uuid,   // Prevents identical outputs from colliding
    pub pubkey: PublicKey, // Owner's public key
}
```

### Block Types

```rust
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

pub struct BlockHeader {
    pub timestamp: DateTime<Utc>,
    pub nonce: u64,
    pub prev_block_hash: Hash,
    pub merkle_root: MerkleRoot,
    pub target: U256,
}
```

### Blockchain State

```rust
pub struct Blockchain {
    // All unspent transaction outputs
    // Hash -> (is_marked_in_mempool, output)
    utxos: HashMap<Hash, (bool, TransactionOutput)>,
    
    // Current difficulty target
    target: U256,
    
    // All validated blocks
    blocks: Vec<Block>,
    
    // Pending transactions (timestamp, transaction)
    mempool: Vec<(DateTime<Utc>, Transaction)>,
}
```

## Algorithms

### Mining Algorithm

```rust
fn mine(&mut self, steps: usize) -> bool {
    for _ in 0..steps {
        // Increment nonce
        if let Some(new_nonce) = self.nonce.checked_add(1) {
            self.nonce = new_nonce;
        } else {
            // Nonce overflow: reset and update timestamp
            self.nonce = 0;
            self.timestamp = Utc::now();
        }
        
        // Check if we found a valid hash
        if self.hash().matches_target(self.target) {
            return true;  // Block mined!
        }
    }
    false  // Keep trying
}
```

### Transaction Validation

```rust
// Simplified version
fn verify_transaction(
    tx: &Transaction,
    utxos: &HashMap<Hash, TransactionOutput>
) -> Result<()> {
    let mut input_sum = 0;
    let mut output_sum = 0;
    
    // Verify inputs
    for input in &tx.inputs {
        // Check UTXO exists
        let utxo = utxos.get(&input.prev_transaction_output_hash)
            .ok_or(Error::InvalidTransaction)?;
        
        // Verify signature
        if !input.signature.verify(&input.prev_transaction_output_hash, &utxo.pubkey) {
            return Err(Error::InvalidSignature);
        }
        
        input_sum += utxo.value;
    }
    
    // Sum outputs
    for output in &tx.outputs {
        output_sum += output.value;
    }
    
    // Inputs must cover outputs (difference is fee)
    if input_sum < output_sum {
        return Err(Error::InsufficientFunds);
    }
    
    Ok(())
}
```

### Merkle Root Calculation

```rust
fn calculate_merkle_root(transactions: &[Transaction]) -> MerkleRoot {
    // Start with transaction hashes
    let mut layer: Vec<Hash> = transactions
        .iter()
        .map(|tx| tx.hash())
        .collect();
    
    // Build tree bottom-up
    while layer.len() > 1 {
        let mut new_layer = vec![];
        
        for pair in layer.chunks(2) {
            let left = pair[0];
            let right = pair.get(1).unwrap_or(&pair[0]); // Duplicate if odd
            new_layer.push(Hash::hash(&[left, *right]));
        }
        
        layer = new_layer;
    }
    
    MerkleRoot(layer[0])
}
```

## API Examples

### Creating a Transaction

```rust
use btclib::{
    crypto::{PrivateKey, Signature},
    types::{Transaction, TransactionInput, TransactionOutput},
};

// Create a transaction
let mut private_key = PrivateKey::new_key();
let public_key = private_key.public_key();

let transaction = Transaction::new(
    vec![TransactionInput {
        prev_transaction_output_hash: previous_utxo_hash,
        signature: Signature::sign_output(&previous_utxo_hash, &mut private_key),
    }],
    vec![TransactionOutput {
        value: 1_000_000_000, // 10 BTC in satoshis
        unique_id: Uuid::new_v4(),
        pubkey: recipient_pubkey,
    }],
);
```

### Building and Mining a Block

```rust
use btclib::{
    types::{Block, BlockHeader},
    util::MerkleRoot,
    sha256::Hash,
};
use chrono::Utc;

// Create block header
let transactions = vec![/* ... */];
let merkle_root = MerkleRoot::calculate(&transactions);

let mut header = BlockHeader::new(
    Utc::now(),
    0, // nonce starts at 0
    prev_block_hash,
    merkle_root,
    target,
);

// Mine the block
let found = header.mine(1_000_000); // Try 1M nonces
if found {
    let block = Block::new(header, transactions);
    println!("Mined block: {}", block.hash());
}
```

### Managing Blockchain State

```rust
use btclib::types::Blockchain;

// Create new blockchain
let mut blockchain = Blockchain::new();

// Add genesis block
blockchain.add_block(genesis_block)?;

// Add transaction to mempool
blockchain.add_to_mempool(transaction)?;

// Get UTXOs for an address
let utxos = blockchain.utxos()
    .iter()
    .filter(|(_, (_, output))| output.pubkey == my_pubkey)
    .collect();

// Check current difficulty
let target = blockchain.target();

// Calculate block reward
let reward = blockchain.calculate_block_reward();
```

### Using Cryptography

```rust
use btclib::crypto::{PrivateKey, PublicKey, Signature};
use btclib::util::Saveable;

// Generate new key pair
let private_key = PrivateKey::new_key();
let public_key = private_key.public_key();

// Save keys to files
private_key.save_to_file("my_key.priv")?;
public_key.save_to_file("my_key.pub")?;

// Load keys
let loaded_private = PrivateKey::load_from_file("my_key.priv")?;
let loaded_public = PublicKey::load_from_file("my_key.pub")?;

// Sign and verify
let message_hash = Hash::hash(&"Hello, blockchain!");
let signature = Signature::sign_output(&message_hash, &mut private_key);
let is_valid = signature.verify(&message_hash, &public_key);
```

## Network Protocol

See `network.rs` for the complete P2P message protocol:

```rust
pub enum Message {
    // Wallet <-> Node
    FetchUTXOs(PublicKey),
    UTXOs(Vec<(TransactionOutput, bool)>),
    SubmitTransaction(Transaction),
    
    // Miner <-> Node
    FetchTemplate(PublicKey),
    Template(Block),
    ValidateTemplate(Block),
    TemplateValidity(bool),
    SubmitTemplate(Block),
    
    // Node <-> Node
    NewTransaction(Transaction),
    NewBlock(Block),
    DiscoverNodes,
    NodeList(Vec<String>),
    AskDifference(u32),
    Difference(i32),
    FetchBlock(usize),
}
```

## Testing

```bash
# Run all tests
cargo test

# Test specific module
cargo test --lib types

# Test with output
cargo test -- --nocapture
```

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Hash calculation | O(n) | n = data size |
| Signature verification | O(1) | Constant time ECDSA |
| UTXO lookup | O(1) | HashMap |
| Transaction validation | O(m) | m = inputs + outputs |
| Block validation | O(t×m) | t = transactions |
| Merkle root | O(n log n) | n = transactions |
| Mining | O(∞) | Probabilistic |

## Further Reading

- **Bitcoin Whitepaper**: https://bitcoin.org/bitcoin.pdf
- **Mastering Bitcoin**: https://github.com/bitcoinbook/bitcoinbook
- **ECDSA**: https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm
- **Merkle Trees**: https://en.wikipedia.org/wiki/Merkle_tree
- **Proof-of-Work**: https://en.bitcoin.it/wiki/Proof_of_work

---

**Next Steps:**
- Explore the [Node README](../node/README.md) to learn about networking
- Try the [Miner README](../miner/README.md) to understand mining
- Use the [Wallet README](../wallet/README.md) to interact with the blockchain

