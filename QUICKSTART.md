# Quick Start Guide

This guide will walk you through setting up and running a complete local blockchain network with nodes, miners, and wallets. Perfect for learning how everything works together!

## 🎯 What You'll Build

By the end of this tutorial, you'll have:
- ✅ A running blockchain node
- ✅ A miner producing blocks
- ✅ Two wallets (Alice and Bob)
- ✅ Successfully sent transactions between them

**Time required:** 15-20 minutes

## 📋 Prerequisites

```bash
# Check Rust is installed
rustc --version
# Should show: rustc 1.70.0 or newer

# If not installed, get it from:
# https://rustup.rs
```

## 🚀 Step-by-Step Tutorial

### Step 1: Build Everything

First, compile all components:

```bash
# Navigate to project directory
cd custom-dlt-rs

# Build all binaries (debug mode for faster compilation)
cargo build --workspace

# This creates:
# - target/debug/node
# - target/debug/miner
# - target/debug/good-wallet
# - target/debug/key_gen
# - ... and other utilities
```

**Expected output:**
```
   Compiling btclib v0.1.0
   Compiling node v0.1.0
   Compiling miner v0.1.0
   Compiling good-wallet v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 45.2s
```

### Step 2: Create Keys

Generate key pairs for Alice, Bob, and the Miner:

```bash
# Create Alice's keys
cargo run --bin key_gen alice
# Creates: alice.pub.pem, alice.priv.cbor

# Create Bob's keys
cargo run --bin key_gen bob
# Creates: bob.pub.pem, bob.priv.cbor

# Create Miner's keys
cargo run --bin key_gen miner
# Creates: miner.pub.pem, miner.priv.cbor
```

**What just happened?**
- Generated 3 ECDSA key pairs (Secp256k1 curve)
- Public keys (.pub.pem): Shareable addresses
- Private keys (.priv.cbor): Secret, never share!

### Step 3: Setup Wallet Configs

#### Alice's Wallet

Create `alice_wallet.toml`:

```toml
# Alice's key
[[my_keys]]
public = "alice.pub.pem"
private = "alice.priv.cbor"

# Alice's contacts
[[contacts]]
name = "Bob"
key = "bob.pub.pem"

[[contacts]]
name = "Miner"
key = "miner.pub.pem"

# Node to connect to
default_node = "127.0.0.1:9000"

# Small fixed fee
[fee_config]
fee_type = "Fixed"
value = 1000  # 0.00001 BTC
```

#### Bob's Wallet

Create `bob_wallet.toml`:

```toml
# Bob's key
[[my_keys]]
public = "bob.pub.pem"
private = "bob.priv.cbor"

# Bob's contacts
[[contacts]]
name = "Alice"
key = "alice.pub.pem"

[[contacts]]
name = "Miner"
key = "miner.pub.pem"

# Node to connect to
default_node = "127.0.0.1:9000"

# Small percentage fee
[fee_config]
fee_type = "Percent"
value = 0.1  # 0.1%
```

**Quick method:**
```bash
# Generate template and edit
cargo run --bin good-wallet -- generate-config -o alice_wallet.toml
cargo run --bin good-wallet -- generate-config -o bob_wallet.toml

# Then edit both files as shown above
```

### Step 4: Start the Node

Open a new terminal (Terminal 1):

```bash
# Start node on port 9000
cargo run --bin node -- --port 9000

# Expected output:
# blockchain file does not exist!
# no initial nodes provided, starting as a seed node
# Listening on 0.0.0.0:9000
```

**What's happening?**
- Node starts with empty blockchain
- Listens for connections from miners and wallets
- Starts background tasks (mempool cleanup, periodic saves)

**Keep this terminal open!**

### Step 5: Start the Miner

Open a new terminal (Terminal 2):

```bash
# Start miner, send rewards to miner's key
cargo run --bin miner -- \
  --address 127.0.0.1:9000 \
  --public-key-file miner.pub.pem

# Expected output:
# Fetching new template
# Received new template with target: 0x0000FFFFFFFFFFFF...
# Mining block with target: 0x0000FFFFFFFFFFFF...
```

**What's happening?**
- Miner connects to node
- Requests block template
- Starts trying different nonces to find valid hash
- May take a minute to find first block (depends on difficulty)

**When a block is mined:**
```
Block mined: 0x00009A3F2B...
Submitting mined block
```

**Keep this terminal open!** The miner will continue producing blocks.

### Step 6: Wait for Initial Blocks

Let the miner produce at least 2-3 blocks before proceeding. This ensures:
- Miner has spendable rewards (coinbase maturity)
- Blockchain is initialized
- Wallets can fetch UTXOs

**Watch Terminal 1 (Node):**
```
received allegedly mined template
block looks good, broadcasting
saving blockchain to drive...
```

**This might take 1-5 minutes depending on your CPU.**

### Step 7: Open Alice's Wallet

Open a new terminal (Terminal 3):

```bash
# Start Alice's wallet
cargo run --bin good-wallet -- -c alice_wallet.toml
```

**Expected UI:**
```
╔═══════════════════════════════════════════════╗
║ [Send] [Quit]                                 ║
╠═══════════════════════════════════════════════╣
║                                               ║
║  ┌─────────── Balance ───────────┐            ║
║  │                                │           ║
║  │   ___    ___  _____  ___      │            ║
║  │  / _ \  / _ \/__  / / _ \     │            ║
║  │ | | | || | | | / /  | | | |   │            ║
║  │ | |_| || |_| |/ /__ | |_| |   │            ║
║  │  \___/  \___//____/ \___/     │            ║
║  │          BTC                   │           ║
║  └────────────────────────────────┘           ║
║                                               ║
║  Press Escape to select the top menu          ║
╚═══════════════════════════════════════════════╝
```

Alice's balance is 0 (she hasn't received any coins yet).

**Keep this terminal open!**

### Step 8: Send Coins from Miner to Alice

Since only the miner has coins (from block rewards), we need to get their private key into a wallet.

**Option A: Create a Miner Wallet**

Create `miner_wallet.toml`:

```toml
[[my_keys]]
public = "miner.pub.pem"
private = "miner.priv.cbor"

[[contacts]]
name = "Alice"
key = "alice.pub.pem"

[[contacts]]
name = "Bob"
key = "bob.pub.pem"

default_node = "127.0.0.1:9000"

[fee_config]
fee_type = "Fixed"
value = 1000
```

Open a new terminal (Terminal 4):

```bash
cargo run --bin good-wallet -- -c miner_wallet.toml
```

**Send to Alice:**
1. Press `Esc` to activate menu
2. Navigate to `[Send]` and press Enter
3. Fill in:
   - Recipient: `Alice`
   - Amount: `25`
   - Unit: `BTC` (default)
4. Click `[Send]`

**Expected:**
```
Transaction sent successfully
```

**Option B: Manual Transaction (Advanced)**

If you prefer to understand the low-level process, use CLI tools:

```bash
# This is more complex - use wallet method instead
# Included here for educational purposes

# 1. Create transaction file manually (requires coding)
# 2. Sign with miner's private key
# 3. Submit to node

# Not recommended for beginners
```

### Step 9: Wait for Transaction Confirmation

**Watch Terminal 1 (Node):**
```
submit tx
added transaction to mempool
transaction sent to friends
```

**Watch Terminal 2 (Miner):**
```
Fetching new template
Received new template with target: ...
Mining block with target: ...
```

The miner will include Alice's transaction in the next block.

**When block is mined:**
- Terminal 1: "received allegedly mined template, block looks good"
- Terminal 3 (Alice's wallet): Balance updates to ~25 BTC!

### Step 10: Alice Sends to Bob

Now Alice can send coins to Bob!

**In Terminal 3 (Alice's wallet):**

1. Press `Esc`
2. Select `[Send]`
3. Fill in:
   - Recipient: `Bob`
   - Amount: `10`
   - Unit: `BTC`
4. Click `[Send]`

**Expected:**
```
Transaction sent successfully
```

### Step 11: Open Bob's Wallet

Open a new terminal (Terminal 5):

```bash
cargo run --bin good-wallet -- -c bob_wallet.toml
```

**Initially:** Balance shows 0

**After next block is mined:**
- Bob's balance updates to ~10 BTC
- Alice's balance decreases (25 - 10 - fee ≈ 14.999 BTC)

### Step 12: Verification

Let's verify everything works:

**Check Node (Terminal 1):**
```bash
# Look for these messages:
# - "added transaction to mempool"
# - "block looks good, broadcasting"
# - "saving blockchain to drive..."
```

**Check blockchain file:**
```bash
ls -lh blockchain.cbor
# Should exist and grow over time
```

**Check all wallets:**
- Miner: Has rewards from multiple blocks
- Alice: Has ~15 BTC (received 25, sent 10 + fee)
- Bob: Has ~10 BTC (received from Alice)

## 🎉 Congratulations!

You've successfully:
- ✅ Set up a blockchain node
- ✅ Mined blocks with PoW
- ✅ Created wallets with key pairs
- ✅ Sent transactions between users
- ✅ Verified transaction confirmations

## 📊 Understanding What Happened

### Block 0 (Genesis)
```
Miner mines first block
Coinbase: 50 BTC → Miner
```

### Block 1-2
```
Miner continues mining
Coinbase: 50 BTC → Miner (each block)
```

### Block 3 (After Miner sends to Alice)
```
Transactions:
1. Coinbase: 50 BTC → Miner
2. Miner → Alice: 25 BTC
   - Input: 50 BTC UTXO from Block 0
   - Output 1: 25 BTC → Alice
   - Output 2: 24.999 BTC → Miner (change)
   - Fee: 0.001 BTC
```

### Block 4 (After Alice sends to Bob)
```
Transactions:
1. Coinbase: 50 BTC + fees → Miner
2. Alice → Bob: 10 BTC
   - Input: 25 BTC UTXO from Block 3
   - Output 1: 10 BTC → Bob
   - Output 2: 14.999 BTC → Alice (change)
   - Fee: 0.001 BTC
```

## 🔧 Troubleshooting

### Miner takes too long to find blocks

**Decrease difficulty:**

Edit `lib/src/lib.rs`:
```rust
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x00FF_FFFF_FFFF_FFFF,  // Changed: Easier difficulty
]);
```

Rebuild:
```bash
cargo build --workspace
```

### Wallet shows "Connection refused"

**Check node is running:**
```bash
# In another terminal
netstat -an | grep 9000

# Should show:
# tcp4  0  0  *.9000  *.*  LISTEN
```

If not, restart the node (Terminal 1).

### Transaction fails: "Insufficient funds"

**Possible causes:**
1. Wait for more blocks to be mined (coinbase maturity)
2. Amount + fee exceeds balance
3. UTXOs not yet fetched (wait 20 seconds)

### Wallet balance doesn't update

**Fixes:**
1. Wait 20 seconds (UTXO update interval)
2. Check node has processed the block
3. Restart wallet to force refresh

### "Invalid transaction" error

**Common causes:**
1. Trying to spend already-spent UTXO
2. Invalid signature (wrong private key?)
3. Amount exceeds available UTXOs

### Node crashes or freezes

**Recovery:**
```bash
# Delete blockchain file
rm blockchain.cbor

# Restart node
cargo run --bin node -- --port 9000

# Restart miner
# (Will start mining new blockchain)
```

## 🎯 Next Steps

### Experiment with the System

**Try these challenges:**

1. **Send coins back and forth**
   - Bob sends back to Alice
   - Observe UTXO changes

2. **Test fee calculation**
   - Try different fee types
   - Compare fixed vs percentage

3. **Mine with multiple miners**
   - Start second miner on same node
   - Watch them compete

4. **Run multiple nodes**
   ```bash
   # Terminal A: Node 1
   cargo run --bin node -- --port 9000
   
   # Terminal B: Node 2 (connects to Node 1)
   cargo run --bin node -- --port 9001 127.0.0.1:9000
   
   # Connect wallets/miners to either node
   ```

5. **Watch difficulty adjustment**
   - Mine 50 blocks
   - Observe target changes
   - Node logs will show new target

6. **Test block rewards halving**
   - Mine 210+ blocks
   - Watch reward drop from 50 → 25 BTC

### Explore the Code

**Start with:**
1. `lib/types/transaction.rs` - How transactions are structured
2. `lib/types/block.rs` - Block validation logic
3. `node/src/handler.rs` - Message handling
4. `miner/src/main.rs` - Mining loop
5. `wallet/src/core.rs` - Transaction creation

### Modify the System

**Ideas:**
1. Add a block explorer (read blockchain.cbor)
2. Implement transaction history in wallet
3. Create a web UI instead of TUI
4. Add more transaction types
5. Implement multi-signature

### Learn More

Read the detailed READMEs:
- [Core Library](./lib/README.md) - Blockchain concepts
- [Node](./node/README.md) - Networking and consensus
- [Miner](./miner/README.md) - Proof-of-Work details
- [Wallet](./wallet/README.md) - Transaction management

## 📚 Additional Resources

### Blockchain Concepts
- [Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf)
- [Mastering Bitcoin](https://github.com/bitcoinbook/bitcoinbook)
- [Learn Me a Bitcoin](https://learnmeabitcoin.com/)

### Rust Programming
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Rust](https://rust-lang.github.io/async-book/)

### Cryptography
- [Practical Cryptography](https://cryptopals.com/)
- [ECDSA Explained](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm)

## 🐛 Getting Help

If you encounter issues:

1. **Check logs:**
   - Node: stdout
   - Wallet: `logs/wallet.log`

2. **Verify setup:**
   - All binaries built?
   - Keys generated?
   - Configs correct?
   - Node running?

3. **Common fixes:**
   - Restart all components
   - Delete `blockchain.cbor`
   - Rebuild: `cargo clean && cargo build --workspace`

4. **Still stuck?**
   - Review error messages carefully
   - Check file paths in configs
   - Ensure ports aren't in use
   - Verify Rust version is current

## 📝 Summary

You've learned how to:
- ✅ Build a blockchain system from source
- ✅ Generate cryptographic key pairs
- ✅ Configure and run nodes
- ✅ Mine blocks with Proof-of-Work
- ✅ Manage wallets and UTXOs
- ✅ Create and broadcast transactions
- ✅ Verify transactions in blocks

**This is a complete, working blockchain!** 🎉

While it's educational and simplified, it demonstrates all the core concepts used in production cryptocurrency systems.

---

**Ready to dive deeper?** Explore the [detailed documentation](./README.md) for each component!

