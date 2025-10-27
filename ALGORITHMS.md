# Blockchain Algorithms & Actor Roles

## 📋 Table of Contents
1. [Core Algorithms](#core-algorithms)
2. [Actor Roles](#actor-roles)
3. [Data Structures](#data-structures)
4. [Network Protocol](#network-protocol)

---

## 🔧 Core Algorithms

### 1. **Proof of Work (PoW) Mining**

**Purpose**: Find a hash smaller than the target to create a valid block.

**Algorithm** (`block.rs:209-230`):
```rust
pub fn mine(&mut self, steps: usize) -> bool {
    for _ in 0..steps {
        self.nonce += 1;  // Try different nonce values
        if self.hash() < self.target {
            return true;  // Found valid hash!
        }
    }
    false
}
```

**How it works**:
1. Start with nonce = 0
2. Hash the block header
3. Check if hash < target
4. If not, increment nonce and repeat
5. When hash < target, block is valid! ✓

**Example**:
```
Target:   0xFFFF...
Hash:     0xA53B... (valid! A5 < FF)
Nonce:    42,365
```

---

### 2. **Difficulty Adjustment**

**Purpose**: Keep block time consistent (e.g., 10 seconds per block).

**Algorithm** (`blockchain.rs:299-367`):
```rust
new_target = current_target × (actual_time / target_time)
```

**Example**:
```
Target time: 500 seconds for 50 blocks
Actual time:  250 seconds (too fast!)
Adjustment:   new_target = current × 0.5 (harder)

Result: Blocks mined 2x faster, so difficulty doubles
```

**Safety Limits**:
- Max adjustment: 4x easier/harder per cycle
- Floor: MIN_TARGET (can't get easier than this)

---

### 3. **UTXO (Unspent Transaction Output) Model**

**Purpose**: Track who owns which coins without account balances.

**Concept**:
```
Coins = Unspent outputs from previous transactions

Alice's UTXO: [10 BTC from Transaction #123]
She can spend it in a new transaction
```

**Structure** (`blockchain.rs:18-19`):
```rust
utxos: HashMap<Hash, (bool, TransactionOutput)>
                          ↑     ↑
                     marked?   actual output
```

**Marking System**:
- `marked = true`:  UTXO reserved by mempool transaction
- `marked = false`: UTXO available to spend

---

### 4. **Transaction Validation**

**Purpose**: Ensure transactions are valid before adding to mempool.

**Checks** (`blockchain.rs:72-230`):
1. ✅ All inputs reference existing UTXOs
2. ✅ No duplicate inputs (internal double-spend)
3. ✅ Input value ≥ Output value (fee exists)
4. ✅ Signatures are valid (ECDSA)
5. ✅ Replace-By-Fee (RBF) handling

**Example**:
```
Transaction:
  Inputs:  [10 BTC, 5 BTC]  = 15 BTC
  Outputs: [12 BTC, 2.99 BTC] = 14.99 BTC
  Fee:     0.01 BTC (goes to miner)
  
Result: Valid! ✓
```

---

### 5. **Merkle Tree**

**Purpose**: Efficiently commit to all transactions in a block.

**Algorithm** (`util.rs:47-77`):
```text
For 4 transactions [A, B, C, D]:

      ROOT = H(H(AB) || H(CD))
       /                    \
   H(AB)                  H(CD)
    /   \                 /    \
  H(A)  H(B)           H(C)  H(D)
   |     |              |     |
   A     B              C     D
```

**Benefits**:
- Tamper-proof: Changing any tx changes root
- Efficient: Light clients need O(log n) hashes
- Compact: Single root commits to all transactions

---

### 6. **ECDSA Signatures**

**Purpose**: Prove ownership without revealing private key.

**Algorithm** (`crypto.rs:62-76`):
```rust
// Signing
signature = private_key.sign(tx_hash)

// Verification
valid = public_key.verify(tx_hash, signature)
```

**Security**:
- Private key: Secret (only owner knows)
- Public key: Public (address visible to all)
- Signature: Proves ownership without revealing secret

---

### 7. **Block Validation**

**Purpose**: Ensure a block is valid before adding to chain.

**Checks** (`block.rs:39-99`):
1. ✅ Non-empty transactions
2. ✅ Valid coinbase (block reward correct)
3. ✅ All transaction signatures valid
4. ✅ No double-spending within block
5. ✅ Merkle root correct
6. ✅ Block hash < target (PoW valid)
7. ✅ Timestamps in order
8. ✅ Previous block hash links correctly

---

## 👥 Actor Roles

### **1. Node**

**Role**: Maintains the blockchain, validates blocks/transactions.

**Responsibilities**:
- Store full blockchain
- Validate incoming blocks
- Maintain UTXO set
- Manage mempool
- Broadcast new blocks/transactions
- Serve templates to miners
- Serve UTXOs to wallets

**Algorithms Used**:
- Block validation
- Transaction validation
- UTXO management
- Mempool management
- Difficulty adjustment
- P2P networking

**Key Files**: `node/main.rs`, `node/handler.rs`

---

### **2. Miner**

**Role**: Solve PoW puzzles to create new blocks.

**Responsibilities**:
- Fetch block templates from node
- Mine new blocks (PoW)
- Submit valid blocks
- Receive block rewards
- Handle template updates (validation)

**Algorithms Used**:
- Proof of Work
- Template fetching/validation
- Block submission

**Key Files**: `miner/main.rs`

**Flow**:
```
1. Connect to node
2. Fetch template (transactions + target)
3. Mine block (try nonces until hash < target)
4. Submit block to node
5. Receive reward in coinbase transaction
```

---

### **3. Wallet**

**Role**: Manage user's keys, create transactions.

**Responsibilities**:
- Store private keys
- Track user's UTXOs
- Create transactions
- Sign transactions
- Display balance
- Send BTC to others

**Algorithms Used**:
- Key generation
- Transaction creation
- Transaction signing
- UTXO management
- Balance calculation

**Key Files**: `wallet/main.rs`, `wallet/core.rs`, `wallet/ui.rs`

**Flow**:
```
1. Connect to node
2. Fetch UTXOs for user's addresses
3. User sends BTC:
   - Select UTXOs to spend
   - Create transaction
   - Sign inputs
   - Send to node
4. Display updated balance
```

---

## 📊 Data Structures

### **Block**
```rust
pub struct Block {
    header: BlockHeader,        // Metadata
    transactions: Vec<Transaction>,  // Transactions included
}
```

### **BlockHeader**
```rust
pub struct BlockHeader {
    timestamp: DateTime<Utc>,   // When mined
    nonce: u64,                 // PoW solution
    prev_block_hash: Hash,      // Links to previous block
    merkle_root: MerkleRoot,    // All transactions
    target: U256,               // Difficulty
}
```

### **Transaction**
```rust
pub struct Transaction {
    inputs: Vec<TransactionInput>,   // UTXOs being spent
    outputs: Vec<TransactionOutput>, // New UTXOs created
}
```

### **TransactionInput**
```rust
pub struct TransactionInput {
    prev_transaction_output_hash: Hash,  // Which UTXO
    signature: Signature,                 // Proof of ownership
}
```

### **TransactionOutput**
```rust
pub struct TransactionOutput {
    value: u64,           // Amount in satoshis
    unique_id: Uuid,      // Unique identifier
    pubkey: PublicKey,    // Who receives it
}
```

---

## 🌐 Network Protocol

### **Message Types** (`network.rs`)
- `FetchBlock(height)` → Get specific block
- `FetchTemplate(key)` → Get mining template
- `SubmitTemplate(block)` → Submit mined block
- `NewTransaction(tx)` → Broadcast transaction
- `FetchUTXOs(key)` → Get user's UTXOs
- `ValidateTemplate(template)` → Check template validity

### **Serialization**
- Format: CBOR (Concise Binary Object Representation)
- Transport: TCP with length prefix
- All data sent as `[length][CBOR data]`

---

## 🔗 How They Work Together

```
┌─────────┐     Fetch Template     ┌─────────┐
│ Miner   │───────────────────────>│  Node   │
└─────────┘                        └─────────┘
     │                                   │
     │  Mine block                       │
     │  (PoW)                            │
     │                                   │
     ├──────────────── Submit Block ────>│
     │                                   │
     │                         Broadcast │
     │                                   ▼
┌─────────┐                         ┌─────────┐
│ Wallet  │<─────── New Block ──────│  Node   │
└─────────┘                         └─────────┘
     │                                   │
     │  Fetch UTXOs                      │
     ├──────────────────────────────────>│
     │<────────── UTXO List ─────────────┤
     │                                   │
     │  Create Transaction               │
     │                                   │
     ├───────── Submit TX ──────────────>│
     │                                   │
     │                         Add to    │
     │                         Mempool   │
```

---

## 🎯 Key Concepts Summary

| Concept | Purpose | Algorithm |
|---------|---------|-----------|
| **PoW** | Secure blockchain | Hash block header until < target |
| **UTXO** | Track ownership | HashMap of unspent outputs |
| **Merkle Tree** | Efficient verification | Hash pairs recursively |
| **ECDSA** | Digital signatures | Elliptic curve cryptography |
| **Difficulty Adjustment** | Consistent block time | `new = old × (actual / target)` |
| **RBF** | Replace transactions | Allow new tx if higher fee |
| **Mempool** | Transaction queue | Sorted by fee (highest first) |

---

## 📚 Further Reading

- Bitcoin Whitepaper: https://bitcoin.org/bitcoin.pdf
- "Building Bitcoin in Rust" (this implementation's basis)
- ECDSA: https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm
- Merkle Trees: https://en.wikipedia.org/wiki/Merkle_tree
- UTXO Model: https://en.bitcoin.it/wiki/UTXO

