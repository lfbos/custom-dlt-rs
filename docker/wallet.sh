#!/bin/bash
# Run the interactive wallet in Docker

# Change to project root
cd "$(dirname "$0")/.."

echo "🔓 Starting blockchain wallet..."
echo ""
echo "📝 To use miner keys with balance:"
echo "   Edit wallet.toml and uncomment the miner keys lines"
echo ""
echo "🎮 Wallet will connect to: 172.25.0.10:9000 (node1)"
echo ""

# Run wallet interactively
docker-compose run --rm wallet

echo ""
echo "👋 Wallet closed"

