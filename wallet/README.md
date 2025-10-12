# Wallet

A terminal-based user interface (TUI) wallet for managing keys, checking balances, and sending transactions. Provides a user-friendly way to interact with the blockchain without dealing with low-level APIs.

## 📚 Table of Contents

- [What is a Wallet?](#what-is-a-wallet)
- [Features](#features)
- [Architecture](#architecture)
- [Setup](#setup)
- [Using the Wallet](#using-the-wallet)
- [Configuration](#configuration)
- [How It Works](#how-it-works)
- [Troubleshooting](#troubleshooting)

## What is a Wallet?

A **cryptocurrency wallet** is software that:
- 🔑 Stores your private keys (proof of ownership)
- 💰 Tracks your balances (UTXOs you can spend)
- 📤 Creates and signs transactions
- 📡 Communicates with blockchain nodes

**Important:** The wallet doesn't actually store coins. It stores **keys** that prove ownership of UTXOs on the blockchain.

Think of it like a bank card:
- 🔐 Private key = PIN code (secret)
- 🔓 Public key = Card number (shareable)
- 💵 Balance = Money in your account (on blockchain, not in wallet)

## Features

### Current Features

✅ **Balance Tracking**
- Real-time balance display in BTC
- Automatic updates every 20 seconds
- Large ASCII art display

✅ **Key Management**
- Support for multiple key pairs
- Secure private key storage (CBOR format)
- Contact list for frequent recipients

✅ **Transaction Sending**
- User-friendly TUI interface
- Support for BTC and Satoshi units
- Automatic change calculation
- Fee configuration (fixed or percentage)

✅ **UTXO Management**
- Automatic UTXO fetching
- Tracks spent/unspent outputs
- Prevents double-spending

✅ **Logging**
- All operations logged to `logs/wallet.log`
- Helpful for debugging
- Includes timestamps and severity levels

## Architecture

### File Structure

```
wallet/
├── Cargo.toml
└── src/
    ├── main.rs     # Entry point, CLI argument parsing
    ├── core.rs     # Core wallet logic (keys, UTXOs, transactions)
    ├── ui.rs       # Terminal user interface (Cursive)
    ├── tasks.rs    # Background async tasks
    └── util.rs     # Utilities (config, logging, formatting)
```

### Component Diagram

```
┌────────────────────────────────────────────────┐
│            Wallet Process                      │
├────────────────────────────────────────────────┤
│                                                │
│  ┌──────────────────────────────────────────┐  │
│  │         Terminal UI (Cursive)            │  │
│  │  ┌────────────┐  ┌────────────────────┐  │  │
│  │  │  Menu Bar  │  │  Balance Display   │  │  │
│  │  │  • Send    │  │  (ASCII Art)       │  │  │
│  │  │  • Quit    │  │  Updated every     │  │  │
│  │  └────────────┘  │  500ms             │  │  │
│  │                  └────────────────────┘  │  │
│  │  ┌────────────────────────────────────┐  │  │
│  │  │  Send Dialog (when triggered)      │  │  │
│  │  │  • Recipient input                 │  │  │
│  │  │  • Amount input                    │  │  │
│  │  │  • Unit selector (BTC/Sats)        │  │  │
│  │  │  • Send/Cancel buttons             │  │  │
│  │  └────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────┘  │
│                      ↕                         │
│  ┌──────────────────────────────────────────┐  │
│  │         Core Logic (Arc<Core>)           │  │
│  │  • Config (keys, contacts, fees)         │  │
│  │  • UTXO store (SkipMap)                  │  │
│  │  • TCP stream to node                    │  │
│  │  • Transaction sender channel            │  │
│  └──────────────────────────────────────────┘  │
│           ↕              ↕              ↕      │
│  ┌──────────────────────────────────────────┐  │
│  │      Background Tasks (Tokio)            │  │
│  │                                          │  │
│  │  ┌──────────────────────────────────┐    │  │
│  │  │  UTXO Updater (every 20s)        │    │  │
│  │  │  • Fetch UTXOs from node         │    │  │
│  │  │  • Update local cache            │    │  │
│  │  └──────────────────────────────────┘    │  │
│  │                                          │  │
│  │  ┌──────────────────────────────────┐    │  │
│  │  │  Transaction Handler             │    │  │
│  │  │  • Receive from channel          │    │  │
│  │  │  • Send to node                  │    │  │
│  │  └──────────────────────────────────┘    │  │
│  │                                          │  │
│  │  ┌──────────────────────────────────┐    │  │
│  │  │  Balance Updater (every 500ms)   │    │  │
│  │  │  • Calculate total balance       │    │  │
│  │  │  • Update UI display             │    │  │
│  │  └──────────────────────────────────┘    │  │
│  └──────────────────────────────────────────┘  │
│                      ↕                         │
│        TCP Connection to Node                  │
│        └──→ 127.0.0.1:9000                     │
│                                                │
└────────────────────────────────────────────────┘
```

### Key Data Structures

```rust
// Main wallet state
pub struct Core {
    pub config: Config,              // Settings
    utxos: UtxoStore,               // UTXO cache
    pub tx_sender: Sender<Transaction>,  // For async sends
    pub stream: Arc<Mutex<TcpStream>>,  // Node connection
}

// Configuration file (wallet.toml)
pub struct Config {
    pub my_keys: Vec<Key>,          // Your key pairs
    pub contacts: Vec<Recipient>,   // Address book
    pub default_node: String,       // Node to connect to
    pub fee_config: FeeConfig,      // Fee settings
}

// UTXO storage
struct UtxoStore {
    my_keys: Vec<LoadedKey>,        // Loaded private keys
    utxos: Arc<SkipMap<PublicKey, Vec<(bool, TransactionOutput)>>>,
}
```

## Setup

### Step 1: Generate Key Pair

```bash
# Generate a key pair for yourself
cargo run --bin key_gen alice

# Creates:
# - alice.pub.pem   (public key - share this)
# - alice.priv.cbor (private key - KEEP SECRET!)
```

### Step 2: Generate Config File

```bash
# Generate a template config
cargo run --bin good-wallet -- generate-config -o wallet.toml

# This creates wallet.toml with example structure
```

### Step 3: Edit Config

Open `wallet.toml` and customize:

```toml
# Your key pairs
[[my_keys]]
public = "alice.pub.pem"
private = "alice.priv.cbor"

# Can have multiple keys
# [[my_keys]]
# public = "alice2.pub.pem"
# private = "alice2.priv.cbor"

# Contacts (people you send money to)
[[contacts]]
name = "Bob"
key = "bob.pub.pem"

[[contacts]]
name = "Charlie"
key = "charlie.pub.pem"

# Node to connect to
default_node = "127.0.0.1:9000"

# Fee configuration
[fee_config]
fee_type = "Percent"  # or "Fixed"
value = 0.1           # 0.1% or 0.1 BTC depending on type
```

### Step 4: Run Wallet

```bash
# Using default config (wallet_config.toml)
cargo run --bin good-wallet

# Using custom config
cargo run --bin good-wallet -- -c wallet.toml

# Override default node
cargo run --bin good-wallet -- -c wallet.toml -n 127.0.0.1:9001
```

## Using the Wallet

### Main Interface

```
╔═══════════════════════════════════════════╗
║  [Send] [Quit]                            ║
╠═══════════════════════════════════════════╣
║                                           ║
║   ┌──────────── Balance ────────────┐     ║
║   │                                 │     ║
║   │    ___   __   __   ___  ___     │     ║
║   │   / _ \ / /  / /  / __\/ __\    │     ║
║   │  / /_\// /  / /  / /  / /       │     ║
║   │ / /  \/ /__/ /__/ /__/ /__      │     ║
║   │/_/  /____/____/____/____/       │     ║
║   │         BTC                     │     ║
║   │                                 │     ║
║   └─────────────────────────────────┘     ║
║                                           ║
║   ┌─── Your Keys ───┐  ┌── Contacts ───┐  ║
║   │ alice.priv.cbor │  │ Bob           │  ║
║   │                 │  │ Charlie       │  ║
║   └─────────────────┘  └───────────────┘  ║
║                                           ║
║   Press Escape to select the top menu     ║
╚═══════════════════════════════════════════╝
```

### Sending a Transaction

1. **Press Escape** to activate menu bar
2. **Navigate to "Send"** and press Enter
3. **Fill in the form:**
   - Recipient: Name from contacts (e.g., "Bob")
   - Amount: Number (e.g., 1.5)
   - Unit: BTC or Sats (click "Switch" to toggle)
4. **Click "Send"**

Example:
```
┌────── Send Transaction ──────┐
│ Recipient:                   │
│ Bob                          │
│                              │
│ Amount:                      │
│ 1.5                          │
│                              │
│ Unit: [BTC] [Switch]         │
│                              │
│      [Send]  [Cancel]        │
└──────────────────────────────┘
```

### Keyboard Shortcuts

```
Esc     - Activate menu bar
q       - Quit (from anywhere)
Tab     - Move between fields
Enter   - Activate button/submit
↑↓      - Navigate menu
```

### Reading the Display

**Balance**: Shows total spendable balance
- Automatically updated every 500ms
- Includes all UTXOs from all your keys
- Excludes UTXOs marked in mempool (pending spend)

**Your Keys**: Lists your private key files
- One line per key
- These are the keys wallet can spend from

**Contacts**: Lists recipients you can send to
- Add more in `wallet.toml`
- Just the name (key file stored in config)

## Configuration

### Config File Format

```toml
# wallet_config.toml

# Key pairs you control
[[my_keys]]
public = "path/to/public.pem"
private = "path/to/private.cbor"

# Recipients in your address book
[[contacts]]
name = "Friendly Name"
key = "path/to/their/public.pem"

# Node connection
default_node = "ip:port"

# Transaction fees
[fee_config]
fee_type = "Fixed"    # or "Percent"
value = 1000          # satoshis if Fixed, percentage if Percent
```

### Fee Configuration

**Fixed Fee:**
```toml
[fee_config]
fee_type = "Fixed"
value = 10000  # 0.0001 BTC (10,000 satoshis)
```

**Percentage Fee:**
```toml
[fee_config]
fee_type = "Percent"
value = 0.1  # 0.1% of transaction amount
```

**Fee Calculation:**
```rust
Fixed:   fee = value
Percent: fee = amount × (value / 100)

Total deducted = amount + fee
```

### Log Files

Logs are saved to `logs/wallet.log`:

```
2024-10-12 14:23:45 INFO Starting wallet application
2024-10-12 14:23:45 INFO Loading config from: wallet.toml
2024-10-12 14:23:46 INFO Starting background tasks
2024-10-12 14:23:46 INFO Running UI
2024-10-12 14:24:05 INFO Attempting to send transaction to Bob for 150000000 satoshis
```

**Log Rotation:**
- New file created daily
- Old files kept in `logs/`
- File naming: `wallet.log.YYYY-MM-DD`

## How It Works

### Transaction Creation Flow

```
User clicks "Send"
    ↓
1. Validate inputs
   • Recipient exists in contacts?
   • Amount is valid number?
    ↓
2. Calculate total needed
   total = amount + fee
    ↓
3. Select UTXOs (coin selection)
   • Sort by value
   • Pick smallest UTXOs that cover total
   • Stop when sum ≥ total
    ↓
4. Create transaction inputs
   For each selected UTXO:
     • Reference UTXO hash
     • Sign with private key
    ↓
5. Create transaction outputs
   • Output 1: amount → recipient
   • Output 2 (if change): (inputs - total) → self
    ↓
6. Send to node via channel
   • Async task picks it up
   • Sends to node
   • Node validates and broadcasts
    ↓
7. Show success dialog
```

### UTXO Tracking

**Problem:** Need to know what you can spend

**Solution:** Maintain local UTXO cache

```rust
// Background task (every 20 seconds)
async fn update_utxos(core: Arc<Core>) {
    loop {
        // For each of your keys
        for key in &core.my_keys {
            // Ask node for UTXOs
            Message::FetchUTXOs(key.public)
                .send_to_node()
                .await?;
            
            // Update local cache
            let utxos = receive_utxos().await?;
            core.utxos.insert(key.public, utxos);
        }
        
        sleep(20_seconds).await;
    }
}
```

**UTXO Format:**
```rust
Vec<(bool, TransactionOutput)>
     ↑
     Is this UTXO marked in mempool?
```

### Balance Calculation

```rust
pub fn get_balance(&self) -> u64 {
    self.utxos
        .iter()
        .map(|entry| {
            entry.value()
                .iter()
                .map(|(_, output)| output.value)
                .sum::<u64>()
        })
        .sum()
}
```

**Note:** This includes marked UTXOs. For spendable balance, filter out marked ones.

### Coin Selection Algorithm

Current: **Simple greedy**

```rust
1. Sort UTXOs by value (ascending)
2. Add UTXOs until sum ≥ target
3. Return selected UTXOs

Pros:
- Simple
- Minimizes number of inputs
- Reduces transaction size

Cons:
- May use more value than needed
- Creates dust (small UTXOs)
```

Better algorithms:
- **Branch and bound** (minimize change)
- **Random selection** (better privacy)
- **Knapsack** (optimal fit)

## Troubleshooting

### Can't Connect to Node

```
Error: Connection refused
```

**Solutions:**
1. Start node: `cargo run --bin node`
2. Check address in config: `default_node = "127.0.0.1:9000"`
3. Override at runtime: `-n 127.0.0.1:9001`

### Balance Shows Zero

**Possible causes:**
1. **No UTXOs** - You haven't received any funds yet
   - Ask a miner to send you coins
   - Or mine blocks yourself to a key you control

2. **Wrong keys** - Config points to different keys
   - Check `my_keys` in config
   - Verify public key matches what miners are sending to

3. **Node not synced** - Node doesn't have your transactions
   - Wait for node to sync
   - Check node has processed blocks

### Transaction Fails

```
Error: Insufficient funds
```

**Check:**
1. Balance is enough
2. Not forgetting fee
3. Amount + fee ≤ balance

```
Error: Recipient not found
```

**Check:**
1. Recipient name matches config exactly
2. Case-sensitive: "Bob" ≠ "bob"

```
Error: Failed to send transaction
```

**Check:**
1. Node is running
2. Transaction is valid
3. UTXOs not already spent

### UI is Frozen

**Causes:**
1. **Heavy computation** - Selecting from many UTXOs
2. **Network delay** - Waiting for node response
3. **UI thread blocked** - Bug in code

**Fix:**
- Press Ctrl+C to force quit
- Check `logs/wallet.log` for errors
- Reduce UTXO set (consolidate)

### Keys Not Found

```
Error reading public key: No such file or directory
```

**Fix:**
1. Check paths in config are correct
2. Paths are relative to where you run the command
3. Use absolute paths: `/full/path/to/alice.pub.pem`

## Advanced Usage

### Multiple Keys

Wallet supports multiple key pairs:

```toml
[[my_keys]]
public = "alice1.pub.pem"
private = "alice1.priv.cbor"

[[my_keys]]
public = "alice2.pub.pem"
private = "alice2.priv.cbor"
```

**Benefits:**
- Privacy: Different keys for different purposes
- Organization: Business vs personal
- Security: Separate keys for large amounts

**Coin Selection:**
The wallet automatically selects UTXOs from any of your keys to cover transactions.

### Custom Node

```bash
# Connect to remote node
cargo run --bin good-wallet -- -n 192.168.1.100:9000

# Connect to specific port
cargo run --bin good-wallet -- -n 127.0.0.1:9001
```

### Consolidating UTXOs

If you have many small UTXOs, consolidate them:

```rust
// Send entire balance to yourself
1. Check balance: 10 BTC across 100 UTXOs
2. Send 9.99 BTC to yourself (leave some for fee)
3. Results in 1 large UTXO

Benefits:
- Faster transaction creation
- Smaller transactions (lower fees)
- Better performance
```

## Security Best Practices

### Private Key Storage

⚠️ **CRITICAL:** Private keys are stored **unencrypted** in CBOR files.

**Recommendations:**
1. **Encrypt your disk** - Use full-disk encryption
2. **Backup keys** - Keep secure copies
3. **Limit access** - `chmod 600 *.priv.cbor`
4. **Never share** - Don't send private keys over network
5. **Use hardware wallets** - For large amounts (not supported yet)

### Transaction Verification

Before sending:
1. ✅ Double-check recipient
2. ✅ Verify amount
3. ✅ Confirm fee is reasonable
4. ✅ Ensure you have enough balance

Remember: **Blockchain transactions are irreversible!**

### Network Security

This wallet connects to nodes over **unencrypted TCP**.

**Risks:**
- Man-in-the-middle attacks
- Eavesdropping
- Transaction tampering

**Mitigations:**
- Use trusted nodes only
- Run your own node
- Connect over VPN/SSH tunnel
- Future: Add TLS support

## Future Enhancements

Potential improvements:
- [ ] HD wallets (hierarchical deterministic)
- [ ] Multi-signature support
- [ ] Transaction history view
- [ ] QR code generation/scanning
- [ ] Encrypted private key storage
- [ ] Hardware wallet integration
- [ ] Better coin selection
- [ ] RBF (Replace-By-Fee)
- [ ] CPFP (Child-Pays-For-Parent)
- [ ] GUI version

## Further Reading

- [Bitcoin Wallets](https://en.bitcoin.it/wiki/Wallet)
- [Coin Selection](https://bitcoin.stackexchange.com/questions/1077/what-is-the-coin-selection-algorithm)
- [HD Wallets (BIP32)](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [Hierarchical Deterministic Wallets](https://en.bitcoin.it/wiki/Deterministic_wallet)

---

**Ready to use your wallet?** Follow the [Quick Start Guide](../QUICKSTART.md) for a complete tutorial!

