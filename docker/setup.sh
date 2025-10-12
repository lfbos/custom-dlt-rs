#!/bin/bash
# Setup script for blockchain Docker environment
# Generates keys for miners before starting the network

set -e  # Exit on error

echo "🔧 Setting up blockchain Docker environment..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}Step 1: Building Docker images...${NC}"
docker-compose build

echo -e "${BLUE}Step 2: Generating keys for miners...${NC}"

# Start utils container to generate keys
docker-compose run --rm utils bash -c "
    echo 'Generating miner1 keys...'
    key_gen miner1
    mv miner1.pub.pem /keys/miner1/
    mv miner1.priv.cbor /keys/miner1/
    
    echo 'Generating miner2 keys...'
    key_gen miner2
    mv miner2.pub.pem /keys/miner2/
    mv miner2.priv.cbor /keys/miner2/
    
    echo 'Keys generated successfully!'
    ls -la /keys/miner1/
    ls -la /keys/miner2/
"

echo -e "${GREEN}✅ Setup complete!${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Start the network:    ./docker/start.sh"
echo "  2. View logs:            ./docker/logs.sh"
echo "  3. Stop the network:     ./docker/stop.sh"
echo ""
echo "For more info: docker-compose --help"

