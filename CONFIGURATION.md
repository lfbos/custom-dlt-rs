# Configuration Guide

Complete guide to configuring the blockchain using JSON config files and environment variables.

## 📚 Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Configuration Priority](#configuration-priority)
- [Network Profiles](#network-profiles)
- [Configuration File Reference](#configuration-file-reference)
- [Environment Variables Reference](#environment-variables-reference)
- [Examples](#examples)
- [Docker Configuration](#docker-configuration)
- [Best Practices](#best-practices)

## Overview

This blockchain supports flexible configuration through:
- 📋 **JSON config files** - Primary configuration method (discoverable, validated)
- 🔧 **Environment variables** - Override specific values
- 📄 **.env files** - Legacy support
- 🏭 **Hardcoded defaults** - Fallback when no config provided

### Why JSON Config Files?

**Benefits:**
- ✅ **Discoverable** - All settings visible in one file
- ✅ **Validated** - Parse errors detected at startup (no silent failures from typos)
- ✅ **Self-documenting** - See all available options with their values
- ✅ **Version controllable** - Ship templates with your project
- ✅ **Type-safe** - Invalid types caught immediately
- ✅ **Still flexible** - Environment variables can override any setting

### Migration from .env

The system still supports `.env` files for backward compatibility, but JSON configs are recommended. See [Migration Guide](#migration-from-environment-variables) below.

## Quick Start

### Generate Default Configuration

**Blockchain Configuration:**
```bash
# Generate config.default.json template
cargo run --bin config_gen

# Copy it to use as your config
cp config.default.json config.json

# Edit config.json to customize settings
nano config.json

# Run your application (automatically loads config.json)
cargo run --bin node
```

**Wallet Configuration:**
```bash
# Generate wallet.toml template
cargo run --bin good-wallet -- generate-config -o wallet.toml

# Edit wallet.toml to add your keys
nano wallet.toml

# Run the wallet
cargo run --bin good-wallet -- -c wallet.toml -n localhost:9000
```

### Use Pre-configured Network Profile

```bash
# Testnet (2x faster blocks, easier mining)
cp config.testnet.json config.json

# Devnet (5x faster blocks, instant mining)
cp config.devnet.json config.json

# Start node
cargo run --bin node
```

## Configuration Priority

Settings are applied in this order (highest to lowest):

```
1. Environment variables  (highest priority)
   └─ Example: NODE_PORT=9001 cargo run --bin node
   
2. JSON config file (config.json)
   └─ Example: {"node": {"port": 9000}}
   
3. .env file in current directory (legacy support)
   └─ Example: NODE_PORT=9001 in .env
   
4. Hardcoded defaults  (lowest priority)
   └─ Fallback if no config file exists
```

### Example

```bash
# config.json contains: "port": 9000
→ Uses 9000

# Override with environment variable
NODE_PORT=9001 cargo run --bin node
→ Uses 9001 (env var wins!)

# No config.json exists
→ Uses 9000 (built-in default)
```

## Configuration File Reference

### JSON Structure

The configuration file has four main sections:

```json
{
  "network": { /* Consensus rules - must match across all nodes */ },
  "node": { /* Node-specific settings */ },
  "mining": { /* Miner configuration */ },
  "wallet": { /* Wallet UI settings */ }
}
```

For detailed field-by-field documentation, see [CONFIG_README.md](./CONFIG_README.md).

### Sample config.json

```json
{
  "network": {
    "network_id": "mainnet",
    "initial_reward": 50,
    "halving_interval": 210,
    "ideal_block_time": 10,
    "difficulty_update_interval": 50,
    "max_mempool_transaction_age": 600,
    "block_transaction_cap": 20,
    "min_target_hex": "0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
  },
  "node": {
    "port": 9000,
    "blockchain_file": "./blockchain.cbor",
    "initial_peers": [],
    "mempool_cleanup_interval_secs": 30,
    "blockchain_save_interval_secs": 15,
    "max_peers": 50
  },
  "mining": {
    "mining_batch_size": 2000000,
    "template_fetch_interval_secs": 5,
    "node_address": "127.0.0.1:9000",
    "public_key_file": "miner.pub.pem"
  },
  "wallet": {
    "utxo_update_interval_secs": 20,
    "balance_display_update_interval_ms": 500,
    "node_address": "127.0.0.1:9000",
    "config_file": "wallet_config.toml"
  }
}
```

## Network Profiles

Pre-configured profiles for different use cases:

### Mainnet (Default)
**Purpose:** Standard network for regular operation

```bash
# Use default config or generate it
cargo run --bin config_gen
cp config.default.json config.json
```

**Parameters:**
- Block time: 10 seconds
- Halving: Every 210 blocks
- Difficulty adjustment: Every 50 blocks
- Block size: 20 transactions
- Port: 9000

### Testnet
**Purpose:** Faster network for testing without risk

```bash
cp config.testnet.json config.json
```

**Parameters:**
- Block time: 5 seconds (2x faster)
- Halving: Every 100 blocks
- Difficulty adjustment: Every 20 blocks  
- Block size: 10 transactions
- Easier difficulty
- Port: 19000

### Devnet
**Purpose:** Very fast network for development

```bash
cp config.devnet.json config.json
```

**Parameters:**
- Block time: 2 seconds (5x faster!)
- Halving: Every 50 blocks
- Difficulty adjustment: Every 10 blocks
- Block size: 5 transactions
- Instant mining (very easy difficulty)
- Port: 29000

## Environment Variables Reference

**Note:** Environment variables now serve as **overrides** to JSON config files. They are optional and should be used for:
- Quick testing/debugging
- CI/CD environments
- Docker deployments
- Overriding specific values without editing config files

For primary configuration, use JSON config files (see above).

### Network Consensus Parameters

These define the blockchain's consensus rules. **Changing these creates an incompatible network!**

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `NETWORK_ID` | String | `mainnet` | Network identifier (mainnet/testnet/devnet) |
| `INITIAL_REWARD` | u64 | `50` | Initial block reward in whole coins |
| `HALVING_INTERVAL` | u64 | `210` | Blocks between reward halvings |
| `IDEAL_BLOCK_TIME` | u64 | `10` | Target seconds per block |
| `DIFFICULTY_UPDATE_INTERVAL` | u64 | `50` | Blocks between difficulty adjustments |
| `MAX_MEMPOOL_TX_AGE` | u64 | `600` | Max mempool transaction age (seconds) |
| `BLOCK_TX_CAP` | usize | `20` | Maximum transactions per block |
| `MIN_TARGET_HEX` | String | `0x0000FF...` | Minimum difficulty target (hex) |

### Node Parameters

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `NODE_PORT` | u16 | `9000` | Port to listen on |
| `BLOCKCHAIN_FILE` | String | `./blockchain.cbor` | Blockchain data file path |
| `INITIAL_PEERS` | String | `""` | Comma-separated peer addresses |
| `MEMPOOL_CLEANUP_INTERVAL` | u64 | `30` | Mempool cleanup frequency (seconds) |
| `BLOCKCHAIN_SAVE_INTERVAL` | u64 | `15` | Blockchain save frequency (seconds) |
| `MAX_PEERS` | usize | `50` | Maximum peer connections |

### Miner Parameters

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `MINER_NODE_ADDRESS` | String | `127.0.0.1:9000` | Node address to connect to |
| `MINER_PUBLIC_KEY` | String | `miner.pub.pem` | Public key file for rewards |
| `MINING_BATCH_SIZE` | usize | `2000000` | Nonces per batch |
| `TEMPLATE_FETCH_INTERVAL` | u64 | `5` | Template update frequency (seconds) |

### Wallet Parameters

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `WALLET_NODE_ADDRESS` | String | `127.0.0.1:9000` | Node address to connect to |
| `WALLET_CONFIG_FILE` | String | `wallet_config.toml` | Wallet config file path |
| `UTXO_UPDATE_INTERVAL` | u64 | `20` | UTXO fetch frequency (seconds) |
| `BALANCE_UPDATE_INTERVAL_MS` | u64 | `500` | Balance display update (milliseconds) |

### Logging & Debug

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `RUST_LOG` | String | `info` | Log level (error/warn/info/debug/trace) |
| `RUST_BACKTRACE` | String | `1` | Enable backtraces on panic |

## Examples

### Example 1: Run Testnet Locally

```bash
# Copy testnet config
cp config.testnet.json config.json

# Start node
cargo run --bin node

# Start miner
cargo run --bin miner

# Observe: Faster blocks, easier mining!
```

### Example 2: Custom Development Setup

Create `config.json`:
```json
{
  "network": {
    "network_id": "custom-dev",
    "ideal_block_time": 1,
    "min_target_hex": "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
    "initial_reward": 50,
    "halving_interval": 50,
    "difficulty_update_interval": 10,
    "max_mempool_transaction_age": 120,
    "block_transaction_cap": 5
  },
  "node": {
    "port": 9000,
    "blockchain_file": "./blockchain.cbor",
    "initial_peers": [],
    "mempool_cleanup_interval_secs": 10,
    "blockchain_save_interval_secs": 5,
    "max_peers": 10
  },
  "mining": {
    "mining_batch_size": 1000,
    "template_fetch_interval_secs": 1,
    "node_address": "127.0.0.1:9000",
    "public_key_file": "miner.pub.pem"
  },
  "wallet": {
    "utxo_update_interval_secs": 5,
    "balance_display_update_interval_ms": 250,
    "node_address": "127.0.0.1:9000",
    "config_file": "wallet_config.toml"
  }
}
```

Run:
```bash
cargo run --bin node
cargo run --bin miner
# Blocks mine almost instantly!
```

### Example 3: Multi-Node Network

**Node 1** (node1/config.json):
```json
{
  "network": { /* ... same network config ... */ },
  "node": {
    "port": 9000,
    "blockchain_file": "./node1.cbor",
    "initial_peers": []
  }
}
```

**Node 2** (node2/config.json):
```json
{
  "network": { /* ... same network config ... */ },
  "node": {
    "port": 9001,
    "blockchain_file": "./node2.cbor",
    "initial_peers": ["127.0.0.1:9000"]
  }
}
```

**Node 3** (node3/config.json):
```json
{
  "network": { /* ... same network config ... */ },
  "node": {
    "port": 9002,
    "blockchain_file": "./node3.cbor",
    "initial_peers": ["127.0.0.1:9000", "127.0.0.1:9001"]
  }
}
```

### Example 4: Override Single Parameter with Environment Variable

```bash
# Use config.json but override block time for quick test
IDEAL_BLOCK_TIME=5 cargo run --bin node

# Use config.json but enable debug logging
RUST_LOG=debug cargo run --bin miner

# Combine multiple overrides
IDEAL_BLOCK_TIME=3 BLOCK_TX_CAP=10 cargo run --bin node
```

### Example 5: Production-Like Setup

`config.production.json`:
```json
{
  "network": {
    "network_id": "mainnet",
    "initial_reward": 50,
    "halving_interval": 210,
    "ideal_block_time": 10,
    "difficulty_update_interval": 50,
    "max_mempool_transaction_age": 600,
    "block_transaction_cap": 20,
    "min_target_hex": "0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
  },
  "node": {
    "port": 9000,
    "blockchain_file": "./blockchain.cbor",
    "initial_peers": [],
    "mempool_cleanup_interval_secs": 60,
    "blockchain_save_interval_secs": 60,
    "max_peers": 100
  },
  "mining": {
    "mining_batch_size": 2000000,
    "template_fetch_interval_secs": 5,
    "node_address": "127.0.0.1:9000",
    "public_key_file": "miner.pub.pem"
  },
  "wallet": {
    "utxo_update_interval_secs": 20,
    "balance_display_update_interval_ms": 500,
    "node_address": "127.0.0.1:9000",
    "config_file": "wallet_config.toml"
  }
}
```

## Docker Configuration

### Using JSON Config with Docker

Docker Compose can use JSON config files mounted as volumes:

```bash
# 1. Create config file
cargo run --bin config_gen
cp config.default.json config.json

# 2. Edit as needed
nano config.json

# 3. Start Docker (mounts config.json)
docker-compose up -d
```

### Network Profiles in Docker

**Mainnet:**
```bash
cp config.default.json config.json
docker-compose up -d
```

**Testnet:**
```bash
cp config.testnet.json config.json
docker-compose up -d
# Faster blocks, easier mining!
```

**Devnet:**
```bash
cp config.devnet.json config.json
docker-compose up -d
# Instant blocks for testing!
```

### Override with Environment Variables in Docker

You can still use environment variables to override config file values:

```bash
# Override specific values at runtime
NODE_PORT=9001 RUST_LOG=debug docker-compose up -d

# Or set in docker-compose.yml:
environment:
  - NETWORK_ID=testnet
  - IDEAL_BLOCK_TIME=7
  - RUST_LOG=debug
```

### Docker-Specific Variables

These control Docker deployment (in addition to blockchain params):

```bash
# Port mappings
NODE1_PORT=9000  # Host port for node1
NODE2_PORT=9001  # Host port for node2
NODE3_PORT=9002  # Host port for node3
```

## Best Practices

### 1. Use JSON Config Files for Persistent Settings

```bash
# ✅ Good: Settings in config.json
cp config.testnet.json config.json
cargo run --bin node

# ⚠️ Acceptable for quick tests: Env vars
IDEAL_BLOCK_TIME=5 cargo run --bin node

# ❌ Bad: Many env vars without config file
IDEAL_BLOCK_TIME=5 BLOCK_TX_CAP=10 NODE_PORT=9001 ... cargo run --bin node
```

### 2. Different Configs for Different Networks

```bash
project/
├── config.default.json    # Mainnet template (version controlled)
├── config.testnet.json    # Testnet template (version controlled)
├── config.devnet.json     # Devnet template (version controlled)
├── config.json           # Active config (gitignored)
├── config.production.json # Production settings (gitignored)
└── config.local.json     # Local development (gitignored)
```

Switch networks:
```bash
ln -sf config.testnet.json config.json  # Switch to testnet
cargo run --bin node

ln -sf config.default.json config.json  # Switch to mainnet
cargo run --bin node
```

### 3. Never Commit Active Config Files

```bash
# .gitignore should include:
config.json
config.local.json
config.production.json
*.local.json

# ✅ Always commit templates
config.default.json
config.testnet.json
config.devnet.json
```

### 4. Validate JSON Syntax

Always validate your JSON before deploying:

```bash
# Validate JSON syntax
jq . config.json

# Or let the application validate at startup
cargo run --bin node
# Look for: "✓ Loaded configuration from config.json"
```

### 5. Use Environment Variables for Overrides Only

```bash
# ✅ Good: Override specific values for testing
IDEAL_BLOCK_TIME=3 cargo run --bin node

# ✅ Good: CI/CD overrides
NODE_PORT=9001 cargo run --bin node

# ❌ Bad: All config via env vars (use JSON instead)
```

## Troubleshooting

### Config Not Loading

```bash
Problem: Changes to config.json don't apply

Solutions:
1. Check config.json is in current directory
   pwd
   ls -la config.json

2. Restart the application
   # Config is loaded on startup, not dynamically

3. Validate JSON syntax
   jq . config.json
   # Look for parse errors

4. Check application output
   cargo run --bin node
   # Should see: "✓ Loaded configuration from config.json"
```

### Invalid JSON

```bash
Error: Failed to parse config.json

Solutions:
1. Validate JSON syntax
   jq . config.json
   
2. Common JSON errors:
   - Trailing commas (not allowed)
   - Missing quotes around strings
   - Wrong types (string vs number)
   - Unescaped special characters

3. Use a template as reference
   cargo run --bin config_gen
   diff config.json config.default.json
```

### Invalid Values

```bash
Error: Type mismatch or invalid value

Solutions:
1. Check types match the specification
   "port": 9000      ✅ (number)
   "port": "9000"    ❌ (should be number, not string)

2. Check hex format
   "min_target_hex": "0xFF..."  ✅
   "min_target_hex": "FF..."    ❌ (needs 0x prefix)

3. Regenerate from defaults
   cargo run --bin config_gen
```

### Docker Not Using Config

```bash
Problem: docker-compose ignores config.json

Solutions:
1. Ensure config.json is mounted as volume in docker-compose.yml
   volumes:
     - ./config.json:/app/config.json

2. Rebuild containers
   docker-compose down
   docker-compose up --build -d

3. Check config.json exists in host directory
   ls -la config.json
```

## Advanced

### Load Config from Custom Path

You can load configuration from a custom path programmatically:

```rust
use btclib::config::BlockchainConfig;

// Load from custom path - returns Result
let config = BlockchainConfig::load_from_file("path/to/my_config.json")
    .expect("Failed to load configuration");

// Or handle the error gracefully
let config = match BlockchainConfig::load_from_file("path/to/my_config.json") {
    Ok(cfg) => cfg,
    Err(e) => {
        eprintln!("Failed to load config: {}", e);
        BlockchainConfig::default()
    }
};
```

### Conditional Configuration

```bash
# Different settings based on environment
if [ "$ENV" == "production" ]; then
    cp config.production.json config.json
elif [ "$ENV" == "staging" ]; then
    cp config.testnet.json config.json
else
    cp config.devnet.json config.json
fi

cargo run --bin node
```

### Generate Config Programmatically

```rust
use btclib::config::BlockchainConfig;

// Generate default config
let config = BlockchainConfig::default();

// Save to file
config.save_to_file("generated_config.json")?;
```

Or from command line:

```bash
#!/bin/bash
# generate-config.sh

cargo run --bin config_gen config.$NETWORK.json
```

## Configuration in CI/CD

### GitHub Actions

```yaml
# .github/workflows/test.yml
steps:
  - name: Generate test config
    run: |
      cargo run --bin config_gen
      cp config.devnet.json config.json
  
  - name: Run tests
    run: cargo test --workspace
    env:
      RUST_LOG: debug
      IDEAL_BLOCK_TIME: 1  # Override for faster tests
```

### Docker Compose in CI

```yaml
steps:
  - name: Create test config
    run: |
      cargo run --bin config_gen
      cp config.testnet.json config.json
  
  - name: Start network
    run: docker-compose up -d
  
  - name: Run integration tests
    run: ./test-integration.sh
```

## Security Considerations

### Sensitive Variables

Some variables may contain sensitive information:

```bash
# ⚠️  Never commit these
MINER_PUBLIC_KEY=/path/to/secret/key.pem
WALLET_CONFIG_FILE=/path/with/private/keys.toml

# ✅ Use environment-specific configs
MINER_PUBLIC_KEY=${HOME}/.blockchain/miner.pub.pem
```

### Docker Secrets (Future Enhancement)

For production deployments:

```yaml
# docker-compose.yml
services:
  node:
    secrets:
      - blockchain_key
    environment:
      - PRIVATE_KEY_FILE=/run/secrets/blockchain_key

secrets:
  blockchain_key:
    file: ./secrets/blockchain.key
```

## Migration from Environment Variables

If you're currently using `.env` files and environment variables, here's how to migrate to JSON configs:

### Step 1: Generate Base Config

```bash
# Generate config.json from current defaults
cargo run --bin config_gen
```

### Step 2: Port Your Settings

**Old `.env`:**
```bash
NETWORK_ID=testnet
IDEAL_BLOCK_TIME=7
NODE_PORT=9001
BLOCKCHAIN_FILE=./my_chain.cbor
INITIAL_PEERS=127.0.0.1:9000
```

**New `config.json`:**
```json
{
  "network": {
    "network_id": "testnet",
    "ideal_block_time": 7,
    ...
  },
  "node": {
    "port": 9001,
    "blockchain_file": "./my_chain.cbor",
    "initial_peers": ["127.0.0.1:9000"],
    ...
  }
}
```

### Step 3: Verify It Works

```bash
# Test the new config
cargo run --bin node

# Should see: "✓ Loaded configuration from config.json"
```

### Step 4: Clean Up (Optional)

```bash
# Keep .env for backward compatibility or remove it
rm .env

# Environment variables still work as overrides!
NODE_PORT=9002 cargo run --bin node
```

### Benefits After Migration

✅ **Discoverability** - See all settings in one place
✅ **Validation** - Typos caught at startup
✅ **Type safety** - Wrong types = immediate error
✅ **Still flexible** - Env vars still override when needed

## Reference

### Configuration Files

- [config.default.json](./config.default.json) - Mainnet configuration template
- [config.testnet.json](./config.testnet.json) - Testnet configuration
- [config.devnet.json](./config.devnet.json) - Development configuration
- [CONFIG_README.md](./CONFIG_README.md) - Detailed field-by-field documentation

### Tools

- `cargo run --bin config_gen` - Generate default config files
- `jq . config.json` - Validate JSON syntax

### Legacy Support

The system still supports `.env` files for backward compatibility:
- Environment variables work as overrides
- `.env` files are still loaded (lower priority than JSON)
- All previous env var names still work

---

**Ready to configure?** 

```bash
cargo run --bin config_gen
cp config.default.json config.json
# Edit config.json to customize
cargo run --bin node
```

