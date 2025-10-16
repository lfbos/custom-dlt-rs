#!/bin/bash
# Inspect blockchain data using utilities container

# Change to project root
cd "$(dirname "$0")/.."

echo "🔍 Blockchain Inspector"
echo "======================="
echo ""
echo "Available commands:"
echo "  1. block_print <file>    - Display block details"
echo "  2. tx_print <file>       - Display transaction details"
echo "  3. ls /data/node1        - List node1 files"
echo "  4. ls /data/node2        - List node2 files"
echo "  5. ls /keys/miner1       - List miner1 keys"
echo ""

# Start interactive shell in utils container
docker-compose run --rm utils /bin/bash

