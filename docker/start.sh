#!/bin/bash
# Start the blockchain network

set -e

echo "🚀 Starting blockchain network..."

# Start all services in detached mode
docker-compose up -d

echo ""
echo "✅ Network started!"
echo ""
echo "Services running:"
docker-compose ps

echo ""
echo "📊 Quick commands:"
echo "  View logs:           ./docker/logs.sh"
echo "  View node1 logs:     docker-compose logs -f node1"
echo "  View miner1 logs:    docker-compose logs -f miner1"
echo "  Stop network:        ./docker/stop.sh"
echo "  Check status:        docker-compose ps"
echo ""
echo "🌐 Node ports:"
echo "  Node 1: localhost:9000"
echo "  Node 2: localhost:9001"
echo "  Node 3: localhost:9002"

