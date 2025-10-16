#!/bin/bash
# Clean up all Docker resources (containers, volumes, images)

set -e

# Change to project root
cd "$(dirname "$0")/.."

echo "⚠️  WARNING: This will remove ALL blockchain data!"
echo "This includes:"
echo "  - All Docker containers"
echo "  - All Docker volumes (blockchain data, keys)"
echo "  - Docker images"
echo ""
read -p "Are you sure? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "Cancelled."
    exit 0
fi

echo "🗑️  Stopping and removing containers..."
docker-compose down

echo "🗑️  Removing volumes..."
docker-compose down -v

echo "🗑️  Removing images..."
docker-compose down --rmi all

echo "✅ All Docker resources cleaned!"
echo ""
echo "To start fresh:"
echo "  ./docker/setup.sh"

