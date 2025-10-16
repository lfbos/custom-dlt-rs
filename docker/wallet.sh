#!/bin/bash
# Run the interactive wallet in Docker

# Change to project root
cd "$(dirname "$0")/.."

echo "🔓 Starting blockchain wallet..."
echo ""

# Check if wallet.toml exists, create from example if not
if [ ! -f wallet.toml ]; then
    if [ -f wallet.toml.example ]; then
        echo "📝 No wallet.toml found, creating from template..."
        cp wallet.toml.example wallet.toml
        echo "✓ Created wallet.toml from template"
        echo ""
    else
        echo "❌ Error: wallet.toml.example not found!"
        echo "   Please create wallet.toml manually"
        exit 1
    fi
fi

echo "📝 To use miner keys with balance:"
echo "   Edit wallet.toml and uncomment the miner keys lines"
echo ""
echo "🎮 Wallet will connect to: node1:9000"
echo ""

# Run wallet interactively
docker-compose run --rm wallet

echo ""
echo "👋 Wallet closed"

