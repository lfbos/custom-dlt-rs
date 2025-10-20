# Configuration System

## Overview

This blockchain uses a JSON-based configuration system for maximum clarity and ease of use. All configuration parameters are documented and validated at startup.

## Quick Start

### 1. Generate Default Configuration

```bash
# Generate config.default.json template
cargo run --bin config_gen

# Copy it to use as your config
cp config.default.json config.json

# Edit config.json to customize
nano config.json
```

### 2. Use Pre-configured Network Profiles

```bash
# Mainnet (standard settings)
cp config.default.json config.json

# Testnet (2x faster, easier mining)
cp config.testnet.json config.json

# Devnet (5x faster, instant mining for development)
cp config.devnet.json config.json
```

### 3. Run Your Application

```bash
# Applications automatically load config.json
cargo run --bin node
cargo run --bin miner
cargo run --bin wallet
```

## Configuration Priority

Settings are applied in this order (highest priority first):

```
1. Environment Variables  ← Highest priority
2. config.json file
3. .env file (legacy)
4. Built-in defaults      ← Lowest priority
```

### Example

```bash
# config.json says: "port": 9000
# You can override it with environment variable:
NODE_PORT=9001 cargo run --bin node
# → Uses port 9001
```

## Configuration File Reference

### Network Configuration

These define blockchain consensus rules. **All nodes must use the same values to communicate!**

```json
{
  "network": {
    "network_id": "mainnet",              // Network identifier (string)
    "initial_reward": 50,                 // Initial block reward in whole coins (u64)
    "halving_interval": 210,              // Blocks between reward halvings (u64)
    "ideal_block_time": 10,               // Target seconds per block (u64)
    "difficulty_update_interval": 50,     // Blocks between difficulty adjustments (u64)
    "max_mempool_transaction_age": 600,   // Max mempool tx age in seconds (u64)
    "block_transaction_cap": 20,          // Max transactions per block (usize)
    "min_target_hex": "0x00FF..."         // Minimum difficulty (easiest, hex string)
  }
}
```

**Field Details:**

| Field | Type | Description | Example Values |
|-------|------|-------------|----------------|
| `network_id` | String | Network identifier | `"mainnet"`, `"testnet"`, `"devnet"` |
| `initial_reward` | u64 | Block reward in whole coins | `50` |
| `halving_interval` | u64 | Blocks between halvings | `210` (Bitcoin: 210,000) |
| `ideal_block_time` | u64 | Target block time in seconds | `10` (Bitcoin: 600) |
| `difficulty_update_interval` | u64 | Blocks between difficulty adjustments | `50` (Bitcoin: 2,016) |
| `max_mempool_transaction_age` | u64 | Max tx age in mempool (seconds) | `600` (10 minutes) |
| `block_transaction_cap` | usize | Max transactions per block | `20` |
| `min_target_hex` | String | Minimum difficulty target (hex) | See difficulty section below |

**Difficulty Target Format:**

The `min_target_hex` is a 256-bit number in hexadecimal format:

- **Easier (more `F`s)**: `0xFFFFFFFF...` = instant mining (devnet)
- **Medium**: `0x00FFFFFF...` = moderate difficulty (testnet)
- **Harder (fewer `F`s)**: `0x0000FFFF...` = production difficulty (mainnet)

### Node Configuration

Controls node operation and network behavior.

```json
{
  "node": {
    "port": 9000,                           // Port to listen on (u16)
    "blockchain_file": "./blockchain.cbor", // Blockchain storage file (string)
    "initial_peers": [],                    // Initial peer addresses (array of strings)
    "mempool_cleanup_interval_secs": 30,    // Mempool cleanup frequency (u64)
    "blockchain_save_interval_secs": 15,    // Blockchain save frequency (u64)
    "max_peers": 50                         // Maximum peer connections (usize)
  }
}
```

**Field Details:**

| Field | Type | Description | Example Values |
|-------|------|-------------|----------------|
| `port` | u16 | TCP port to listen on | `9000`, `19000`, `29000` |
| `blockchain_file` | String | Path to blockchain data file | `"./blockchain.cbor"` |
| `initial_peers` | Array | Peer addresses to connect to | `["127.0.0.1:9001"]` |
| `mempool_cleanup_interval_secs` | u64 | How often to clean mempool (seconds) | `30` |
| `blockchain_save_interval_secs` | u64 | How often to save blockchain (seconds) | `15` |
| `max_peers` | usize | Maximum number of peer connections | `50` |

### Mining Configuration

Controls miner behavior and performance.

```json
{
  "mining": {
    "mining_batch_size": 2000000,          // Nonces to try per batch (usize)
    "template_fetch_interval_secs": 5,     // Template update frequency (u64)
    "node_address": "127.0.0.1:9000",      // Node to connect to (string)
    "public_key_file": "miner.pub.pem"     // Public key for rewards (string)
  }
}
```

**Field Details:**

| Field | Type | Description | Example Values |
|-------|------|-------------|----------------|
| `mining_batch_size` | usize | Nonces per mining batch | `2000000` (higher = more CPU per batch) |
| `template_fetch_interval_secs` | u64 | Template update frequency | `5` seconds |
| `node_address` | String | Node address to connect to | `"127.0.0.1:9000"` |
| `public_key_file` | String | Public key file for rewards | `"miner.pub.pem"` |

### Wallet Configuration

Controls wallet UI and update behavior.

```json
{
  "wallet": {
    "utxo_update_interval_secs": 20,          // UTXO fetch frequency (u64)
    "balance_display_update_interval_ms": 500, // Balance display refresh (u64)
    "node_address": "127.0.0.1:9000",          // Node to connect to (string)
    "config_file": "wallet_config.toml"        // Wallet data file (string)
  }
}
```

**Field Details:**

| Field | Type | Description | Example Values |
|-------|------|-------------|----------------|
| `utxo_update_interval_secs` | u64 | How often to fetch UTXOs | `20` seconds |
| `balance_display_update_interval_ms` | u64 | Balance display refresh rate | `500` ms |
| `node_address` | String | Node address to connect to | `"127.0.0.1:9000"` |
| `config_file` | String | Wallet configuration file | `"wallet_config.toml"` |

## Network Profiles

### Mainnet (Production)

**Purpose:** Standard network for normal operation

```bash
cp config.default.json config.json
```

**Characteristics:**
- Block time: 10 seconds
- Moderate difficulty
- Halving every 210 blocks
- 20 transactions per block
- Port: 9000

**Use when:** Running a standard blockchain network

### Testnet (Testing)

**Purpose:** Faster network for testing without waiting

```bash
cp config.testnet.json config.json
```

**Characteristics:**
- Block time: 5 seconds (2x faster)
- Easier difficulty
- Halving every 100 blocks
- 10 transactions per block
- Port: 19000

**Use when:** Testing features, running experiments

### Devnet (Development)

**Purpose:** Ultra-fast network for development

```bash
cp config.devnet.json config.json
```

**Characteristics:**
- Block time: 2 seconds (5x faster!)
- Instant mining (easiest difficulty)
- Halving every 50 blocks
- 5 transactions per block
- Port: 29000

**Use when:** Developing, debugging, rapid iteration

## Environment Variable Overrides

You can override any configuration value with environment variables:

### Network Variables

```bash
export NETWORK_ID=testnet
export INITIAL_REWARD=100
export HALVING_INTERVAL=500
export IDEAL_BLOCK_TIME=15
export DIFFICULTY_UPDATE_INTERVAL=100
export MAX_MEMPOOL_TX_AGE=1200
export BLOCK_TX_CAP=30
export MIN_TARGET_HEX=0xFFFFFFFF...
```

### Node Variables

```bash
export NODE_PORT=9001
export BLOCKCHAIN_FILE=./my_blockchain.cbor
export INITIAL_PEERS="127.0.0.1:9000,127.0.0.1:9002"
export MEMPOOL_CLEANUP_INTERVAL=60
export BLOCKCHAIN_SAVE_INTERVAL=30
export MAX_PEERS=100
```

### Mining Variables

```bash
export MINING_BATCH_SIZE=5000000
export TEMPLATE_FETCH_INTERVAL=10
export MINER_NODE_ADDRESS=127.0.0.1:9001
export MINER_PUBLIC_KEY=./keys/miner.pub.pem
```

### Wallet Variables

```bash
export UTXO_UPDATE_INTERVAL=30
export BALANCE_UPDATE_INTERVAL_MS=1000
export WALLET_NODE_ADDRESS=127.0.0.1:9001
export WALLET_CONFIG_FILE=./my_wallet.toml
```

## Common Scenarios

### Scenario 1: Single Node Development

```bash
# Use devnet for instant mining
cp config.devnet.json config.json
cargo run --bin node &
cargo run --bin miner &
cargo run --bin wallet
```

### Scenario 2: Multi-Node Network

**Node 1** (config.json):
```json
{
  "node": {
    "port": 9000,
    "blockchain_file": "./node1.cbor",
    "initial_peers": []
  }
}
```

**Node 2** (config.json):
```json
{
  "node": {
    "port": 9001,
    "blockchain_file": "./node2.cbor",
    "initial_peers": ["127.0.0.1:9000"]
  }
}
```

### Scenario 3: Quick Testing with Override

```bash
# Use default config but change one parameter
IDEAL_BLOCK_TIME=3 cargo run --bin node
```

### Scenario 4: Production Deployment

```bash
# Create production config
cp config.default.json config.production.json

# Edit it carefully
nano config.production.json

# Use it
cp config.production.json config.json
cargo run --release --bin node
```

## Validation

The configuration system validates your settings at startup:

```bash
✓ Loaded configuration from config.json
  Network: testnet
  Port: 19000
  Block time: 5s
```

**Error handling:**

- **File not found:** Uses built-in defaults
- **Parse error:** Shows error and uses defaults
- **Invalid values:** Falls back to sensible defaults
- **Type mismatch:** Error message shows expected type

## Troubleshooting

### Config Not Loading

**Problem:** Changes to config.json don't apply

**Solutions:**
1. Ensure config.json is in the working directory
2. Restart the application (config loads at startup)
3. Check JSON syntax (use `jq . config.json` to validate)

### Invalid JSON

**Problem:** Parse error when loading config

**Solutions:**
```bash
# Validate JSON syntax
jq . config.json

# Common issues:
# - Trailing commas (not allowed in JSON)
# - Missing quotes around strings
# - Wrong types (string vs number)
```

### Environment Variables Not Working

**Problem:** Environment variables don't override config

**Solutions:**
```bash
# Check variable is set
echo $NODE_PORT

# Use export (not just assignment)
export NODE_PORT=9001  # ✓ Correct
NODE_PORT=9001         # ✗ Won't work in new shells

# Or inline:
NODE_PORT=9001 cargo run --bin node  # ✓ Correct
```

### Network Incompatibility

**Problem:** Nodes can't connect to each other

**Solutions:**
1. Ensure all nodes use the same network consensus parameters
2. Check `network_id`, `initial_reward`, `halving_interval`, etc. match
3. Verify ports and `initial_peers` are correct
4. Check firewall/network connectivity

## Migration from Environment Variables

If you're migrating from the old environment variable system:

### Step 1: Generate Config

```bash
# Generate config from current defaults
cargo run --bin config_gen
```

### Step 2: Port Your .env Settings

Old `.env`:
```bash
IDEAL_BLOCK_TIME=7
NODE_PORT=9001
```

New `config.json`:
```json
{
  "network": {
    "ideal_block_time": 7
  },
  "node": {
    "port": 9001
  }
}
```

### Step 3: Test

```bash
# Test that it works
cargo run --bin node

# Should see: "✓ Loaded configuration from config.json"
```

### Step 4: Keep Environment Overrides (Optional)

You can still use environment variables for specific overrides:

```bash
# Config file has defaults
# Override just the block time
IDEAL_BLOCK_TIME=3 cargo run --bin node
```

## Best Practices

### 1. Use Version Control

```bash
# Commit templates
git add config.default.json config.testnet.json config.devnet.json

# Don't commit actual config (may contain secrets/local paths)
echo "config.json" >> .gitignore
```

### 2. Document Custom Networks

If creating custom network configurations:

```json
{
  "network": {
    "network_id": "my-custom-network",
    "_comment": "Custom network for XYZ project - faster blocks, more txs"
  }
}
```

### 3. Separate Configs for Different Environments

```bash
workspace/
├── config.default.json      # Template
├── config.testnet.json      # Template
├── config.devnet.json       # Template
├── config.local.json        # Your local dev settings (gitignored)
├── config.staging.json      # Staging environment (gitignored)
└── config.production.json   # Production (gitignored, secured)
```

### 4. Validate Before Deployment

```bash
# Always validate JSON syntax
jq . config.json

# Test load the config
cargo run --bin node --help
```

## Advanced Usage

### Custom Config Path

You can load config from a custom path programmatically:

```rust
use btclib::config::BlockchainConfig;

// Load from custom path - returns Result
let config = BlockchainConfig::load_from_file("path/to/config.json")
    .expect("Failed to load configuration");

// Or handle the error gracefully
let config = match BlockchainConfig::load_from_file("path/to/config.json") {
    Ok(cfg) => cfg,
    Err(e) => {
        eprintln!("Failed to load config: {}", e);
        BlockchainConfig::default()
    }
};
```

### Programmatic Generation

Generate configs programmatically:

```rust
use btclib::config::BlockchainConfig;

let config = BlockchainConfig::default();
config.save_to_file("generated_config.json")?;
```

### Merging Configs

Load base config and override specific sections:

```bash
# Load base config
cp config.default.json config.json

# Override just network settings with env vars
export NETWORK_ID=testnet
export IDEAL_BLOCK_TIME=5
cargo run --bin node
```

## Further Reading

- [Full Configuration Guide](./CONFIGURATION.md) - Detailed guide with examples
- [Quick Start Guide](./QUICKSTART.md) - Get started quickly
- [API Documentation](./lib/src/config.rs) - Code-level documentation

---

**Questions?** Check the [main README](./README.md) or open an issue!

