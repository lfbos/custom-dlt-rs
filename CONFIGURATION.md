# Configuration Guide

Complete guide to configuring the blockchain using environment variables and .env files.

## 📚 Table of Contents

- [Overview](#overview)
- [Configuration Priority](#configuration-priority)
- [Network Profiles](#network-profiles)
- [Environment Variables Reference](#environment-variables-reference)
- [Examples](#examples)
- [Docker Configuration](#docker-configuration)
- [Best Practices](#best-practices)

## Overview

This blockchain supports flexible configuration through:
- 🔧 **Environment variables** - Quick overrides
- 📄 **.env files** - Persistent configuration
- ⚙️ **Command-line arguments** - Runtime overrides
- 🏭 **Hardcoded defaults** - Educational baseline

### Why Environment Variables?

**Benefits:**
- ✅ No recompilation needed
- ✅ Easy testing with different parameters
- ✅ Docker-friendly
- ✅ Multiple network support (mainnet/testnet/devnet)
- ✅ Secure (secrets not in code)

## Configuration Priority

Settings are applied in this order (highest to lowest):

```
1. Command-line arguments  (highest priority)
   └─ Example: --port 9001
   
2. Environment variables
   └─ Example: NODE_PORT=9001
   
3. .env file in current directory
   └─ Example: NODE_PORT=9001 in .env
   
4. Hardcoded defaults  (lowest priority)
   └─ Example: DEFAULT_PORT = 9000
```

### Example

```bash
# Hardcoded default
NODE_PORT = 9000

# Override with .env file
# .env contains: NODE_PORT=9001
→ Uses 9001

# Override with environment variable
NODE_PORT=9002 cargo run --bin node
→ Uses 9002

# Override with CLI argument
NODE_PORT=9002 cargo run --bin node -- --port 9003
→ Uses 9003 (CLI wins!)
```

## Network Profiles

Pre-configured profiles for different use cases:

### Mainnet (Default)
**Purpose:** Standard network for regular operation

```bash
cp .env.example .env
# Or
export NETWORK_ID=mainnet
```

**Parameters:**
- Block time: 10 seconds
- Halving: Every 210 blocks
- Difficulty adjustment: Every 50 blocks
- Block size: 20 transactions
- Ports: 9000-9002

### Testnet
**Purpose:** Faster network for testing without risk

```bash
cp .env.testnet.example .env
# Or
export NETWORK_ID=testnet
```

**Parameters:**
- Block time: 5 seconds (2x faster)
- Halving: Every 100 blocks
- Difficulty adjustment: Every 20 blocks  
- Block size: 10 transactions
- Easier difficulty
- Ports: 19000-19002

### Devnet
**Purpose:** Very fast network for development

```bash
cp .env.devnet.example .env
# Or
export NETWORK_ID=devnet
```

**Parameters:**
- Block time: 2 seconds (5x faster!)
- Halving: Every 50 blocks
- Difficulty adjustment: Every 10 blocks
- Block size: 5 transactions
- Instant mining (very easy difficulty)
- Ports: 29000-29002
- Verbose logging

## Environment Variables Reference

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
cp .env.testnet.example .env

# Start node
cargo run --bin node

# Start miner
cargo run --bin miner

# Observe: Faster blocks, easier mining!
```

### Example 2: Custom Development Setup

Create `.env`:
```bash
# Super fast for development
NETWORK_ID=custom-dev
IDEAL_BLOCK_TIME=1
MIN_TARGET_HEX=0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
MINING_BATCH_SIZE=1000
RUST_LOG=debug
```

Run:
```bash
cargo run --bin node
cargo run --bin miner
# Blocks mine almost instantly!
```

### Example 3: Multi-Node Network

**Node 1** (.env):
```bash
NODE_PORT=9000
BLOCKCHAIN_FILE=./node1.cbor
```

**Node 2** (.env):
```bash
NODE_PORT=9001
BLOCKCHAIN_FILE=./node2.cbor
INITIAL_PEERS=127.0.0.1:9000
```

**Node 3** (.env):
```bash
NODE_PORT=9002
BLOCKCHAIN_FILE=./node3.cbor
INITIAL_PEERS=127.0.0.1:9000,127.0.0.1:9001
```

### Example 4: Override Single Parameter

```bash
# Use defaults but change block time
IDEAL_BLOCK_TIME=5 cargo run --bin node

# Use defaults but enable debug logging
RUST_LOG=debug cargo run --bin miner

# Combine multiple overrides
IDEAL_BLOCK_TIME=3 BLOCK_TX_CAP=10 RUST_LOG=trace cargo run --bin node
```

### Example 5: Production-Like Setup

`.env`:
```bash
NETWORK_ID=mainnet

# Conservative settings
IDEAL_BLOCK_TIME=10
DIFFICULTY_UPDATE_INTERVAL=50
MAX_MEMPOOL_TX_AGE=600

# Security
RUST_LOG=warn  # Less verbose
MAX_PEERS=100

# Persistence
BLOCKCHAIN_SAVE_INTERVAL=60  # Save less often (better performance)
```

## Docker Configuration

### Using .env with Docker

Docker Compose automatically loads `.env` from the project root:

```bash
# 1. Create .env file
cp .env.example .env

# 2. Edit as needed
nano .env

# 3. Start Docker (automatically uses .env)
docker-compose up -d
```

### Network Profiles in Docker

**Mainnet:**
```bash
cp .env.example .env
docker-compose up -d
```

**Testnet:**
```bash
cp .env.testnet.example .env
docker-compose up -d
# Faster blocks, easier mining!
```

**Devnet:**
```bash
cp .env.devnet.example .env
docker-compose up -d
# Instant blocks for testing!
```

### Override Docker Variables

```bash
# Override at runtime
NETWORK_ID=testnet RUST_LOG=debug docker-compose up -d

# Or create custom .env
cat > .env << EOF
NETWORK_ID=custom
IDEAL_BLOCK_TIME=7
RUST_LOG=debug
EOF

docker-compose up -d
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

### 1. Use .env for Persistent Settings

```bash
# ✅ Good: Settings in .env
cp .env.testnet.example .env
cargo run --bin node

# ❌ Bad: Typing env vars every time
IDEAL_BLOCK_TIME=5 BLOCK_TX_CAP=10 cargo run --bin node
```

### 2. Different .env for Different Networks

```bash
project/
├── .env.mainnet    # Production settings
├── .env.testnet    # Testing settings
├── .env.devnet     # Development settings
└── .env -> .env.testnet  # Symlink to active config
```

Switch networks:
```bash
ln -sf .env.testnet .env  # Switch to testnet
cargo run --bin node

ln -sf .env.mainnet .env  # Switch to mainnet
cargo run --bin node
```

### 3. Never Commit .env Files

```bash
# .gitignore already includes:
.env
.env.local
.env.*.local

# ✅ Always use .example files in repo
.env.example
.env.testnet.example
.env.devnet.example
```

### 4. Document Custom Variables

If you add new variables:

```bash
# Add to .env.example with comment
# My new feature setting
MY_NEW_FEATURE=true
```

### 5. Validate Configuration

```bash
# Check what config is loaded
RUST_LOG=debug cargo run --bin node
# Look for: "Network: testnet" in output

# Or add to code:
println!("Loaded config: {:?}", BlockchainConfig::global());
```

## Troubleshooting

### Config Not Loading

```bash
Problem: Changes to .env don't apply

Solutions:
1. Check .env is in current directory
   pwd
   ls -la .env

2. Restart the application
   # .env is loaded on startup, not dynamically

3. Check for typos
   # Variable names are case-sensitive
   NODE_PORT=9000  ✅
   node_port=9000  ❌
```

### Invalid Values

```bash
Error: Failed to parse config

Solutions:
1. Check types match
   NODE_PORT=9000      ✅ (number)
   NODE_PORT="9000"    ✅ (quotes ok)
   NODE_PORT=abc       ❌ (not a number!)

2. Check hex format
   MIN_TARGET_HEX=0xFF...  ✅
   MIN_TARGET_HEX=FF...    ❌ (needs 0x prefix)

3. Use .env.example as template
   cp .env.example .env
```

### Docker Not Using .env

```bash
Problem: docker-compose ignores .env

Solutions:
1. Ensure .env is in same dir as docker-compose.yml
   ls -la .env docker-compose.yml

2. Rebuild containers
   docker-compose down
   docker-compose up --build -d

3. Check .env format (no spaces around =)
   NODE_PORT=9000  ✅
   NODE_PORT = 9000  ❌
```

## Advanced

### Multiple .env Files

```bash
# Load multiple .env files
set -a  # Auto-export variables
source .env.base
source .env.custom
set +a

cargo run --bin node
```

### Conditional Configuration

```bash
# Different settings based on environment
if [ "$ENV" == "production" ]; then
    cp .env.mainnet .env
else
    cp .env.devnet .env
fi

docker-compose up -d
```

### Generate .env Programmatically

```bash
#!/bin/bash
# generate-env.sh

cat > .env << EOF
NETWORK_ID=${NETWORK:-testnet}
IDEAL_BLOCK_TIME=${BLOCK_TIME:-5}
NODE_PORT=${PORT:-9000}
RUST_LOG=${LOG_LEVEL:-info}
EOF

echo "Generated .env for $NETWORK network"
```

## Configuration in CI/CD

### GitHub Actions

```yaml
# .github/workflows/test.yml
env:
  NETWORK_ID: testnet
  IDEAL_BLOCK_TIME: 2
  RUST_LOG: debug

steps:
  - name: Run tests
    run: cargo test --workspace
```

### Docker Compose in CI

```yaml
steps:
  - name: Create .env
    run: |
      cat > .env << EOF
      NETWORK_ID=testnet
      IDEAL_BLOCK_TIME=2
      MIN_TARGET_HEX=0xFFFF...
      EOF
  
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

## Reference

### Complete .env Example

See [.env.example](./.env.example) for all available variables with documentation.

### Network-Specific Examples

- [.env.testnet.example](./.env.testnet.example) - Testnet configuration
- [.env.devnet.example](./.env.devnet.example) - Development configuration

### Component-Specific

- [node/.env.example](./node/.env.example) - Node-only variables
- [miner/.env.example](./miner/.env.example) - Miner-only variables
- [wallet/.env.example](./wallet/.env.example) - Wallet-only variables
- [docker/.env.example](./docker/.env.example) - Docker deployment variables

---

**Ready to configure?** Copy `.env.example` to `.env` and start customizing!

