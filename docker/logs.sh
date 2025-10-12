#!/bin/bash
# View logs from all services

# Follow logs from all services
# Press Ctrl+C to exit

docker-compose logs -f --tail=50

