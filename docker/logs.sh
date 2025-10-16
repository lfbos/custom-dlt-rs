#!/bin/bash
# View logs from all services

# Change to project root
cd "$(dirname "$0")/.."

# Follow logs from all services
# Press Ctrl+C to exit

docker-compose logs -f --tail=50

