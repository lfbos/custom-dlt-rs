#!/bin/bash
# Quick network switching script

set -e

echo "🌐 Network Switcher"
echo "=================="
echo ""
echo "Available networks:"
echo "  1. mainnet (default - standard speed)"
echo "  2. testnet (2x faster, easier mining)"
echo "  3. devnet (5x faster, instant mining)"
echo ""
read -p "Select network (1-3): " choice

case $choice in
    1)
        cp .env.example .env
        echo "✅ Switched to mainnet"
        ;;
    2)
        cp .env.testnet.example .env
        echo "✅ Switched to testnet"
        ;;
    3)
        cp .env.devnet.example .env
        echo "✅ Switched to devnet"
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "Configuration loaded. Start with:"
echo "  make start    (Docker)"
echo "  cargo run --bin node    (Local)"
