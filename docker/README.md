# Docker Deployment Guide

Run the entire blockchain network with a single command using Docker!

## 🐳 What's Included

This Docker setup creates a complete blockchain network:

- **3 Nodes** - Full blockchain nodes with P2P networking
- **2 Miners** - Competing miners producing blocks
- **Persistent Storage** - Data survives container restarts
- **Isolated Network** - Containers communicate via private network
- **Easy Management** - Simple scripts to control everything

## 📋 Prerequisites

- **Docker** 20.10+ ([Install Docker](https://docs.docker.com/get-docker/))
- **Docker Compose** 2.0+ (included with Docker Desktop)

Verify installation:
```bash
docker --version
docker-compose --version
```

## 🚀 Quick Start

### 1. Initial Setup (One-time)

```bash
# Run setup script
./docker/setup.sh
```

This will:
- Build Docker images for all components
- Generate cryptographic keys for miners
- Create Docker volumes for data storage

**First run takes 5-10 minutes** (compiling Rust in release mode).

### 2. Start the Network

```bash
# Start all nodes and miners
./docker/start.sh
```

This starts:
- `node1` on port 9000 (seed node)
- `node2` on port 9001 (peer)
- `node3` on port 9002 (peer)
- `miner1` (mining to node1)
- `miner2` (mining to node2)

### 3. View Logs

```bash
# Follow logs from all services
./docker/logs.sh

# Or view specific service
docker-compose logs -f node1
docker-compose logs -f miner1
```

### 4. Check Status

```bash
# See running containers and resource usage
./docker/status.sh
```

### 5. Stop the Network

```bash
# Stop all containers (data is preserved)
./docker/stop.sh
```

## 📊 Network Architecture

```
┌─────────────────────────────────────────────────┐
│          Host Machine (localhost)               │
├─────────────────────────────────────────────────┤
│                                                 │
│  Port 9000 ──┐                                  │
│  Port 9001 ──┼──> Docker Network (172.25.0.0)   │
│  Port 9002 ──┘                                  │
│                                                 │
│   ┌──────────────────────────────────────┐      │
│   │  Node 1 (172.25.0.10:9000)           │      │
│   │  - Seed node                         │      │
│   │  - Volume: node1-data                │      │
│   └────────┬─────────────────────────────┘      │
│            │                                    │
│   ┌────────┴─────────────┬──────────────┐       │
│   │                      │              │       │
│   ▼                      ▼              ▼       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────┐   │
│  │ Node 2       │  │ Node 3       │  │Miner1│   │
│  │ (172.25.0.11)│  │ (172.25.0.12)│  └──────┘   │
│  │ Port: 9001   │  │ Port: 9002   │             │
│  └──────┬───────┘  └──────────────┘             │
│         │                                       │
│         ▼                                       │
│     ┌──────┐                                    │
│     │Miner2│                                    │
│     └──────┘                                    │
│                                                 │
└─────────────────────────────────────────────────┘
```

## 🛠️ Available Scripts

All scripts are in the `docker/` directory:

| Script | Description |
|--------|-------------|
| `setup.sh` | Initial setup (build images, generate keys) |
| `start.sh` | Start the blockchain network |
| `stop.sh` | Stop all containers |
| `logs.sh` | View logs from all services |
| `status.sh` | Check network status and resource usage |
| `clean.sh` | Remove all data and images |
| `inspect.sh` | Interactive shell to inspect blockchain data |

## 🔧 Advanced Usage

### Connect Your Local Wallet

The nodes are accessible from your host machine:

```bash
# Option 1: Auto-generate wallet config (recommended)
cargo run --bin good-wallet -- generate-config -o alice_wallet.toml

# Generate your keys
cargo run --bin key_gen alice

# Edit alice_wallet.toml to add your keys
# Then run wallet (connects to Docker node)
cargo run --bin good-wallet -- -c alice_wallet.toml -n localhost:9000

# Option 2: Manual config creation
cat > alice_wallet.toml << EOF
my_keys = [
    { public_key_path = "alice.pub.pem", private_key_path = "alice.priv.cbor" }
]

default_node = "127.0.0.1:9000"

[[contacts]]
name = "Miner1"
key = "./miner1_public.pem"

[fee_config]
fee_type = "Fixed"
value = 1000
EOF

# Run wallet
cargo run --bin good-wallet -- -c alice_wallet.toml
```

### Scale the Network

Add more nodes by editing `docker-compose.yml`:

```yaml
node4:
  build:
    context: .
    target: node
  ports:
    - "9003:9003"
  command:
    - "--port"
    - "9003"
    - "172.25.0.10:9000"
    - "172.25.0.11:9001"
  networks:
    - blockchain-net
```

Then:
```bash
docker-compose up -d node4
```

### Inspect Blockchain Data

```bash
# Start interactive shell
./docker/inspect.sh

# Inside the container:
ls /data/node1/
block_print /data/node1/blockchain.cbor
```

### View Specific Service Logs

```bash
# Follow node1 only
docker-compose logs -f node1

# Last 100 lines from miner1
docker-compose logs --tail=100 miner1

# All logs since 10 minutes ago
docker-compose logs --since 10m
```

### Restart a Single Service

```bash
# Restart just miner1
docker-compose restart miner1

# Rebuild and restart node1
docker-compose up -d --build node1
```

## 🔍 Debugging

### Check if Services are Running

```bash
docker-compose ps

# Should show:
# NAME                  STATUS
# blockchain-node1      Up (healthy)
# blockchain-node2      Up
# blockchain-node3      Up
# blockchain-miner1     Up
# blockchain-miner2     Up
```

### Check Network Connectivity

```bash
# Execute command in node1
docker-compose exec node1 sh -c "netstat -an | grep 9000"

# Ping node2 from node1
docker-compose exec node1 ping -c 3 172.25.0.11
```

### View Resource Usage

```bash
# Real-time stats
docker stats

# Or use the status script
./docker/status.sh
```

### Access Container Shell

```bash
# Get shell in node1
docker-compose exec node1 /bin/bash

# Or use utils container
docker-compose run --rm utils /bin/bash
```

## 🐛 Troubleshooting

### Containers Won't Start

```bash
Error: port 9000 already in use
```

**Solution:**
```bash
# Check what's using the port
lsof -i :9000

# Stop local node if running
pkill -f "cargo run.*node"

# Or change port in docker-compose.yml
```

### Miners Can't Connect

```bash
Error: Connection refused
```

**Solution:**
```bash
# Wait for nodes to be healthy
docker-compose ps

# Check node logs
docker-compose logs node1

# Restart miners after nodes are ready
docker-compose restart miner1 miner2
```

### Build Fails

```bash
Error: failed to build
```

**Solution:**
```bash
# Clean build cache
docker-compose build --no-cache

# Or rebuild specific service
docker-compose build --no-cache node
```

### Out of Disk Space

```bash
# Check Docker disk usage
docker system df

# Clean up unused resources
docker system prune -a

# Or use clean script
./docker/clean.sh
```

### Containers Keep Restarting

```bash
# Check logs for errors
docker-compose logs --tail=50 node1

# Common issues:
# - Invalid blockchain data (delete volume)
# - Missing keys (run setup.sh again)
# - Port conflicts (change ports)
```

## 💾 Data Management

### Backup Blockchain Data

```bash
# Backup node1 data
docker run --rm \
  -v custom-dlt-rs_node1-data:/data \
  -v $(pwd)/backup:/backup \
  alpine tar czf /backup/node1-backup.tar.gz /data

# Restore
docker run --rm \
  -v custom-dlt-rs_node1-data:/data \
  -v $(pwd)/backup:/backup \
  alpine tar xzf /backup/node1-backup.tar.gz -C /
```

### Export Keys

```bash
# Copy miner keys to host
docker run --rm \
  -v custom-dlt-rs_miner1-keys:/keys \
  -v $(pwd):/backup \
  alpine cp -r /keys /backup/miner1-keys
```

### Reset Single Node

```bash
# Stop services
docker-compose stop node1

# Remove volume
docker volume rm custom-dlt-rs_node1-data

# Recreate
docker-compose up -d node1
```

## 🧪 Testing Different Scenarios

### Simulate Network Partition

```bash
# Disconnect node3 from network
docker network disconnect blockchain_blockchain-net blockchain-node3

# Wait a bit, then reconnect
docker network connect blockchain_blockchain-net blockchain-node3

# Watch it resync
docker-compose logs -f node3
```

### Test with Different Difficulty

Edit `lib/src/lib.rs` before building:
```rust
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x00FF_FFFF_FFFF_FFFF,  // Easier difficulty
]);
```

Then rebuild:
```bash
docker-compose build --no-cache
./docker/start.sh
```

### Run with Only 1 Miner

```bash
# Start without miner2
docker-compose up -d node1 node2 node3 miner1
```

## 📈 Monitoring

### Watch Block Production

```bash
# Follow miner logs to see blocks being found
docker-compose logs -f miner1 miner2 | grep "Block mined"
```

### Monitor Mempool

```bash
# Node logs show mempool activity
docker-compose logs -f node1 | grep mempool
```

### Check Blockchain Height

```bash
# Access node and check blockchain file
docker-compose exec node1 ls -lh /data/blockchain.cbor

# File size grows as blockchain grows
```

## 🔄 Update Workflow

When you modify code:

```bash
# 1. Stop running services
./docker/stop.sh

# 2. Rebuild images
docker-compose build

# 3. Start updated services
./docker/start.sh
```

Or rebuild and restart in one command:
```bash
docker-compose up -d --build
```

## 🌐 Access from Outside

### Connect External Miner

```bash
# From host machine or another computer
cargo run --bin miner -- \
  --address localhost:9000 \
  --public-key-file miner3.pub.pem

# Or from another Docker container
docker run -it --network custom-dlt-rs_blockchain-net \
  custom-blockchain-miner \
  --address 172.25.0.10:9000 \
  --public-key-file /keys/miner.pub.pem
```

### Connect External Wallet

```bash
# Your local wallet connects to dockerized node
cargo run --bin good-wallet -- \
  -c wallet.toml \
  -n localhost:9000
```

## 📚 Docker Commands Cheat Sheet

```bash
# Start everything
docker-compose up -d

# Stop everything
docker-compose down

# View logs (all)
docker-compose logs -f

# View logs (specific)
docker-compose logs -f node1

# Restart service
docker-compose restart miner1

# Rebuild service
docker-compose build --no-cache node

# Remove everything
docker-compose down -v --rmi all

# Check status
docker-compose ps

# Execute command in container
docker-compose exec node1 <command>

# Run one-off command
docker-compose run --rm utils <command>
```

## 🎯 Common Workflows

### Development Mode

```bash
# Hot reload: rebuild and restart on code changes
docker-compose up -d --build

# Watch logs during development
./docker/logs.sh
```

### Production Mode

```bash
# Start in background
./docker/start.sh

# Monitor with status checks
watch -n 5 ./docker/status.sh

# Backup data regularly
# (Set up cron job for backups)
```

### Demo Mode

```bash
# Start network
./docker/start.sh

# Open multiple terminals
# Terminal 1: docker-compose logs -f node1
# Terminal 2: docker-compose logs -f miner1
# Terminal 3: Your wallet
```

## 🛡️ Security Notes

**For production use (future):**

1. **Don't expose ports publicly**
   ```yaml
   # Instead of:
   ports:
     - "9000:9000"
   
   # Use:
   ports:
     - "127.0.0.1:9000:9000"  # Only localhost
   ```

2. **Add resource limits**
   ```yaml
   deploy:
     resources:
       limits:
         cpus: '2.0'
         memory: 2G
   ```

3. **Use secrets for keys**
   ```yaml
   secrets:
     - miner_private_key
   ```

4. **Enable TLS** (when implemented)

## 📖 Next Steps

- Modify `docker-compose.yml` to customize your network
- Add more miners or nodes as needed
- Connect your local wallet to the Docker network
- Experiment with different configurations

## 🆘 Getting Help

If you encounter issues:

1. Check container logs: `docker-compose logs <service>`
2. Verify containers are running: `docker-compose ps`
3. Check network connectivity: `docker network inspect custom-dlt-rs_blockchain-net`
4. Review Docker documentation: https://docs.docker.com/

---

**Happy containerizing!** 🐳

