#!/bin/bash
# Display status of the blockchain network

# Change to project root
cd "$(dirname "$0")/.."

echo "📊 Blockchain Network Status"
echo "=============================="
echo ""

echo "🖥️  Containers:"
docker-compose ps

echo ""
echo "💾 Volumes:"
docker volume ls | grep custom-dlt-rs || echo "No volumes found"

echo ""
echo "🌐 Networks:"
docker network ls | grep custom-dlt-rs || echo "No networks found"

echo ""
echo "📈 Resource Usage:"
docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}" \
  $(docker-compose ps -q) 2>/dev/null || echo "No containers running"

echo ""
echo "🔍 Quick checks:"
echo "  Node 1 listening: $(docker-compose exec -T node1 sh -c 'netstat -an | grep 9000' 2>/dev/null && echo '✅ Yes' || echo '❌ No')"
echo "  Node 2 listening: $(docker-compose exec -T node2 sh -c 'netstat -an | grep 9001' 2>/dev/null && echo '✅ Yes' || echo '❌ No')"
echo "  Node 3 listening: $(docker-compose exec -T node3 sh -c 'netstat -an | grep 9002' 2>/dev/null && echo '✅ Yes' || echo '❌ No')"

