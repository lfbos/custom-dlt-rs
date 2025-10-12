#!/bin/bash
# Stop the blockchain network

echo "🛑 Stopping blockchain network..."

docker-compose down

echo "✅ Network stopped!"
echo ""
echo "💾 Data preserved in Docker volumes"
echo "To completely remove all data: ./docker/clean.sh"

