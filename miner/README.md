# Miner

The miner is a specialized client that performs Proof-of-Work (PoW) to create new blocks and earn block rewards. It continuously solves cryptographic puzzles to secure the blockchain network.

## 📚 Table of Contents

- [What is Mining?](#what-is-mining)
- [How Mining Works](#how-mining-works)
- [Architecture](#architecture)
- [Mining Process](#mining-process)
- [Running the Miner](#running-the-miner)
- [Performance Tuning](#performance-tuning)
- [Economics](#economics)
- [Troubleshooting](#troubleshooting)

## What is Mining?

**Mining** is the process of:
1. Collecting pending transactions
2. Creating a block with these transactions
3. Finding a **nonce** that makes the block hash meet difficulty requirements
4. Broadcasting the solved block to the network
5. Earning a **reward** for the work

### Why Mining Matters

Mining serves three critical functions:

1. **Security** 🔒
   - Makes blockchain immutable
   - Rewriting history requires redoing all work
   - Cost of attack >> potential profit

2. **Consensus** 🤝
   - Distributed agreement on transaction order
   - No central authority needed
   - Longest chain wins

3. **Coin Distribution** 💰
   - New coins created as block rewards
   - Fair distribution to those providing security
   - Incentivizes network participation

## How Mining Works

### The Core Challenge

Find a **nonce** such that:

```
SHA256(block_header) ≤ target

Where block_header contains:
- Previous block hash
- Merkle root of transactions
- Timestamp
- Target difficulty
- Nonce (the number we're searching for)
```

### Visual Example

```
Target: 0000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
        ^^^^
        Must start with 4 zeros (in hex)

Attempt 1:
Hash(nonce=0) = 1A3F... ❌ Too large

Attempt 2:
Hash(nonce=1) = 9B82... ❌ Too large

...

Attempt 42,857:
Hash(nonce=42857) = 0000A3B... ✅ Success!
```

### Difficulty

**Lower target = Harder difficulty**

```
Easy:   0x0FFFFF... (12 bits of zeros)
Medium: 0x0000FFFF... (16 bits of zeros)
Hard:   0x00000FFF... (20 bits of zeros)

On average, with difficulty N:
Expected attempts = 2^N
```

### Block Reward

Miners earn:
```
Block Reward = Base Reward + Transaction Fees

Base Reward:
  Start: 50 BTC (5,000,000,000 satoshis)
  Halves every 210 blocks
  
  Block 0-209:    50 BTC
  Block 210-419:  25 BTC
  Block 420-629:  12.5 BTC
  ...
```

## Architecture

### File Structure

```
miner/
├── Cargo.toml
└── src/
    └── main.rs     # Complete miner implementation
```

### Component Diagram

```
┌────────────────────────────────────────────┐
│               Miner Process                │
├────────────────────────────────────────────┤
│                                            │
│  ┌──────────────────────────────────────┐  │
│  │      Main Async Runtime (Tokio)      │  │
│  │                                      │  │
│  │  ┌────────────────────────────────┐  │  │
│  │  │  Template Fetcher              │  │  │
│  │  │  (every 5 seconds)             │  │  │
│  │  │  • Request block template      │  │  │
│  │  │  • Validate current template   │  │  │
│  │  └────────────────────────────────┘  │  │
│  │            ↓              ↑          │  │
│  │            ↓              ↑          │  │
│  │  ┌────────────────────────────────┐  │  │
│  │  │  Shared Template Storage       │  │  │
│  │  │  Arc<Mutex<Option<Block>>>     │  │  │
│  │  └────────────────────────────────┘  │  │
│  │            ↓              ↑          │  │
│  │            ↓              ↑          │  │
│  │  ┌────────────────────────────────┐  │  │
│  │  │  Mining Thread (CPU-bound)     │  │  │
│  │  │  • Clone template              │  │  │
│  │  │  • Increment nonce 2M times    │  │  │
│  │  │  • Check hash vs target        │  │  │
│  │  │  • Send if found               │  │  │
│  │  └────────────────────────────────┘  │  │
│  │            ↓                         │  │
│  │            ↓                         │  │
│  │  ┌────────────────────────────────┐  │  │
│  │  │  Block Submitter               │  │  │
│  │  │  • Receives mined blocks       │  │  │
│  │  │  • Submits to node             │  │  │
│  │  └────────────────────────────────┘  │  │
│  │                                      │  │
│  └──────────────────────────────────────┘  │
│                                            │
│  TCP Connection to Node                    │
│  └──→ 127.0.0.1:9000                       │
│                                            │
└────────────────────────────────────────────┘
```

### Key Components

```rust
struct Miner {
    // Your public key (for receiving rewards)
    public_key: PublicKey,
    
    // TCP connection to node
    stream: Mutex<TcpStream>,
    
    // Current block being mined
    current_template: Arc<Mutex<Option<Block>>>,
    
    // Mining on/off flag
    mining: Arc<AtomicBool>,
    
    // Channel to send mined blocks
    mined_block_sender: Sender<Block>,
    mined_block_receiver: Receiver<Block>,
}
```

## Mining Process

### Step-by-Step Flow

```
┌─────────────────────────────────────────┐
│ 1. Fetch Template from Node             │
│    • Request FetchTemplate(my_pubkey)   │
│    • Receive Template(block)            │
│    • Block includes:                    │
│      - Coinbase tx paying me            │
│      - Pending transactions             │
│      - Correct merkle root              │
│      - Current difficulty               │
└─────────────────────────────────────────┘
              ↓
┌─────────────────────────────────────────┐
│ 2. Start Mining                         │
│    • Set mining flag = true             │
│    • Mining thread wakes up             │
│    • Clone template                     │
└─────────────────────────────────────────┘
              ↓
┌─────────────────────────────────────────┐
│ 3. Hash Attempts                        │
│    for i in 0..2_000_000:               │
│      nonce++                            │
│      hash = SHA256(block_header)        │
│      if hash ≤ target:                  │
│        Found it! Send block             │
│        return                           │
└─────────────────────────────────────────┘
              ↓
┌─────────────────────────────────────────┐
│ 4. Periodic Validation (every 5 sec)    │
│    • Check if template still valid      │
│    • Someone else may have mined block  │
│    • If invalid:                        │
│      - Stop mining                      │
│      - Fetch new template               │
└─────────────────────────────────────────┘
              ↓
┌─────────────────────────────────────────┐
│ 5. Submit Mined Block                   │
│    • Send SubmitTemplate(block)         │
│    • Node validates                     │
│    • If valid:                          │
│      - Added to blockchain              │
│      - Reward earned! 🎉                │
│    • If invalid:                        │
│      - Someone else mined first         │
│      - Try again                        │
└─────────────────────────────────────────┘
              ↓
┌─────────────────────────────────────────┐
│ 6. Repeat                               │
│    • Fetch new template                 │
│    • Continue mining                    │
└─────────────────────────────────────────┘
```

### Code Walkthrough

#### 1. Template Fetching

```rust
async fn fetch_template(&self) -> Result<()> {
    println!("Fetching new template");
    
    // Request template with our pubkey for rewards
    let message = Message::FetchTemplate(self.public_key.clone());
    message.send_async(&mut *self.stream.lock().await).await?;
    
    // Receive template
    match Message::receive_async(&mut *self.stream.lock().await).await? {
        Message::Template(template) => {
            println!("Received template with target: {}", template.header.target);
            
            // Store template for mining thread
            *self.current_template.lock().unwrap() = Some(template);
            
            // Start mining
            self.mining.store(true, Ordering::Relaxed);
            Ok(())
        }
        _ => Err(anyhow!("Unexpected response"))
    }
}
```

#### 2. Mining Loop (Separate Thread)

```rust
fn spawn_mining_thread(&self) {
    let template = self.current_template.clone();
    let mining = self.mining.clone();
    let sender = self.mined_block_sender.clone();
    
    thread::spawn(move || {
        loop {
            // Only mine if flag is set
            if mining.load(Ordering::Relaxed) {
                if let Some(mut block) = template.lock().unwrap().clone() {
                    // Try 2 million nonces
                    if block.header.mine(2_000_000) {
                        println!("Block mined! {}", block.hash());
                        sender.send(block).unwrap();
                        mining.store(false, Ordering::Relaxed);
                    }
                }
            }
            thread::yield_now();
        }
    });
}
```

#### 3. Template Validation

```rust
async fn validate_template(&self) -> Result<()> {
    if let Some(template) = self.current_template.lock().unwrap().clone() {
        let message = Message::ValidateTemplate(template);
        message.send_async(&mut *self.stream.lock().await).await?;
        
        match Message::receive_async(&mut *self.stream.lock().await).await? {
            Message::TemplateValidity(valid) => {
                if !valid {
                    println!("Template invalid, stopping mining");
                    self.mining.store(false, Ordering::Relaxed);
                }
                Ok(())
            }
            _ => Err(anyhow!("Unexpected response"))
        }
    } else {
        Ok(())
    }
}
```

#### 4. Block Submission

```rust
async fn submit_block(&self, block: Block) -> Result<()> {
    println!("Submitting mined block");
    let message = Message::SubmitTemplate(block);
    message.send_async(&mut *self.stream.lock().await).await?;
    
    // Stop mining (will fetch new template on next interval)
    self.mining.store(false, Ordering::Relaxed);
    Ok(())
}
```

## Running the Miner

### Prerequisites

1. **Running Node**: A node must be running to provide templates
2. **Public Key**: Generate one for receiving rewards

### Generate Keys

```bash
# Generate a key pair
cargo run --bin key_gen miner_key

# This creates:
# - miner_key.pub.pem   (your public key)
# - miner_key.priv.cbor (your private key - keep secret!)
```

### Start Mining

```bash
# Connect to node at 127.0.0.1:9000
cargo run --bin miner -- \
  --address 127.0.0.1:9000 \
  --public-key-file miner_key.pub.pem
```

### Command-Line Arguments

```rust
-a, --address <ADDRESS>
    Node address to connect to
    Example: 127.0.0.1:9000

-p, --public-key-file <FILE>
    Path to your public key file
    Rewards will be sent to this key
```

### Example Output

```
Fetching new template
Received new template with target: 0x0000FFFFFFFFFFFF...
Mining block with target: 0x0000FFFFFFFFFFFF...
Current template is still valid
Mining block with target: 0x0000FFFFFFFFFFFF...
Block mined: 0x00009A3F2B...
Submitting mined block
Fetching new template
...
```

## Performance Tuning

### Hash Rate

**Hash Rate** = Number of hashes attempted per second

```
Typical rates:
- Debug build:   ~100,000 H/s
- Release build: ~10,000,000 H/s (100x faster!)

Always use release builds for mining:
cargo run --release --bin miner -- ...
```

### Tuning Parameters

#### Nonce Batch Size

Currently tries 2 million nonces before checking for new template:

```rust
// In spawn_mining_thread()
if block.header.mine(2_000_000) {
    // Found or exhausted attempts
}
```

**Trade-offs:**
- **Larger batch** = More efficient (fewer locks)
- **Smaller batch** = More responsive (updates faster)

Recommended: 1M - 10M

#### Template Refresh Rate

Currently checks every 5 seconds:

```rust
let mut template_interval = interval(Duration::from_secs(5));
```

**Trade-offs:**
- **More frequent** = Less wasted work on stale templates
- **Less frequent** = Less network overhead

Recommended: 3-10 seconds

### Multi-Core Mining

Current implementation uses a single thread. To use all cores:

```rust
// Spawn multiple mining threads
for core in 0..num_cpus::get() {
    spawn_mining_thread_with_offset(core);
}

// Each thread starts with different nonce offset
fn mine_with_offset(offset: u64, step: u64) {
    nonce = offset;
    loop {
        hash(nonce);
        nonce += step;  // Skip ahead by num_threads
    }
}
```

### GPU Mining

For serious mining, implement CUDA/OpenCL:
- SHA-256 is highly parallelizable
- GPUs can try millions of hashes simultaneously
- 100-1000x speedup vs CPU

## Economics

### Block Reward Calculation

```rust
// From lib/types/blockchain.rs
pub fn calculate_block_reward(&self) -> u64 {
    let block_height = self.block_height();
    let halvings = block_height / HALVING_INTERVAL;
    (INITIAL_REWARD * 10u64.pow(8)) >> halvings
}
```

**Reward Schedule:**

| Block Range | Reward (BTC) | Reward (Satoshis) |
|-------------|--------------|-------------------|
| 0 - 209     | 50           | 5,000,000,000     |
| 210 - 419   | 25           | 2,500,000,000     |
| 420 - 629   | 12.5         | 1,250,000,000     |
| 630 - 839   | 6.25         | 625,000,000       |

### Transaction Fees

Miners also earn fees from transactions:

```
Fee = Sum(inputs) - Sum(outputs)

Example:
  Input:  100 BTC
  Output: 95 BTC (to recipient) + 4 BTC (change)
  Fee:    1 BTC (goes to miner)
```

Transactions with higher fees get prioritized.

### Profitability

```
Revenue per block = Block Reward + Fees
Cost per block    = Electricity + Hardware + Time

Profitable if: Revenue > Cost

Factors:
- Your hash rate
- Network hash rate (competition)
- Current difficulty
- Electricity cost
- Block reward (decreases over time)
```

### Mining Pools

In real networks, solo mining is rarely profitable. Miners join **pools**:

```
Pool Mining:
1. Pool gives you partial work
2. You submit shares (near-misses)
3. Pool finds blocks collectively
4. Rewards split proportionally

Benefits:
- Steady income vs lottery
- Lower variance
- Smaller miners can participate
```

This implementation doesn't support pools (educational only).

## Troubleshooting

### No Blocks Found

```
Mining for hours, no success
```

**Possible causes:**
1. **Difficulty too high** - Adjust `MIN_TARGET` in `lib/lib.rs`
2. **Slow hash rate** - Use `--release` build
3. **Bad luck** - Mining is probabilistic
4. **Someone else mining faster** - They find blocks first

### Invalid Block Rejected

```
Submitting mined block
Error: block rejected
```

**Causes:**
1. **Stale template** - Someone else mined first
2. **Invalid transactions** - Transactions became invalid
3. **Timestamp too old** - Took too long to mine

**Solution:** Fetch new template and try again

### Connection Issues

```
Error: Connection refused
```

**Check:**
1. Node is running: `netstat -an | grep 9000`
2. Correct address: `--address 127.0.0.1:9000`
3. Firewall allows connections

### Low Hash Rate

```
Only 10,000 H/s
```

**Fixes:**
1. Use release mode: `cargo run --release`
2. Close other programs
3. Check CPU usage (should be 100%)
4. Update Rust: `rustup update`

### Nonce Overflow

```
Nonce reaches u64::MAX
```

The miner handles this by resetting and updating timestamp:

```rust
if let Some(new_nonce) = self.nonce.checked_add(1) {
    self.nonce = new_nonce;
} else {
    self.nonce = 0;
    self.timestamp = Utc::now();
}
```

## Mining Statistics

Track your mining performance:

```rust
// Add counters
let mut hashes_attempted = 0;
let start_time = Instant::now();

// In mining loop
hashes_attempted += 2_000_000;

// Every minute, print stats
if start_time.elapsed() > Duration::from_secs(60) {
    let hash_rate = hashes_attempted / 60;
    println!("Hash rate: {} H/s", hash_rate);
    hashes_attempted = 0;
    start_time = Instant::now();
}
```

## Best Practices

1. **Always use release builds** for actual mining
2. **Monitor your node** - ensure it's synced
3. **Keep keys secure** - backup `miner_key.priv.cbor`
4. **Check template validity** frequently
5. **Log your earnings** - track blocks found

## Further Reading

- [Bitcoin Mining](https://en.bitcoin.it/wiki/Mining)
- [Proof of Work](https://en.wikipedia.org/wiki/Proof_of_work)
- [SHA-256 Algorithm](https://en.wikipedia.org/wiki/SHA-2)
- [Mining Pools](https://en.bitcoin.it/wiki/Pooled_mining)

---

**Next:** Learn how to use the [Wallet](../wallet/README.md) or follow the [Quick Start Guide](../QUICKSTART.md)
