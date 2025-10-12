# Makefile for Custom Blockchain
# Provides convenient commands for Docker and local development

.PHONY: help setup start stop logs status clean build test docker-build docker-up docker-down

# Default target
help:
	@echo "Custom Blockchain - Available Commands"
	@echo "======================================"
	@echo ""
	@echo "🐳 Docker Commands:"
	@echo "  make docker-setup    - Initial setup (build images, generate keys)"
	@echo "  make docker-start    - Start the blockchain network"
	@echo "  make docker-stop     - Stop the network (preserve data)"
	@echo "  make docker-logs     - View logs from all services"
	@echo "  make docker-status   - Check network status"
	@echo "  make docker-clean    - Remove all data and containers"
	@echo "  make docker-restart  - Restart the entire network"
	@echo ""
	@echo "🔧 Development Commands:"
	@echo "  make build           - Build all binaries (local)"
	@echo "  make build-release   - Build optimized binaries"
	@echo "  make test            - Run tests (when implemented)"
	@echo "  make clean-local     - Clean Rust build artifacts"
	@echo "  make fmt             - Format code with rustfmt"
	@echo "  make clippy          - Run Clippy linter"
	@echo ""
	@echo "📚 Shortcuts:"
	@echo "  make setup           - Alias for docker-setup"
	@echo "  make start           - Alias for docker-start"
	@echo "  make stop            - Alias for docker-stop"
	@echo "  make logs            - Alias for docker-logs"

# =============================================================================
# Docker Commands
# =============================================================================

docker-setup:
	@./docker/setup.sh

docker-start:
	@./docker/start.sh

docker-stop:
	@./docker/stop.sh

docker-logs:
	@./docker/logs.sh

docker-status:
	@./docker/status.sh

docker-clean:
	@./docker/clean.sh

docker-restart: docker-stop docker-start

docker-inspect:
	@./docker/inspect.sh

# Shortcuts
setup: docker-setup
start: docker-start
stop: docker-stop
logs: docker-logs
status: docker-status
clean: docker-clean

# =============================================================================
# Local Development Commands
# =============================================================================

build:
	cargo build --workspace

build-release:
	cargo build --workspace --release

test:
	cargo test --workspace

clean-local:
	cargo clean

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

check:
	cargo check --workspace

# =============================================================================
# Utility Commands
# =============================================================================

# Generate keys locally
gen-keys:
	@echo "Generating keys..."
	@cargo run --bin key_gen alice
	@cargo run --bin key_gen bob
	@echo "Keys generated: alice.*, bob.*"

# Build and run specific components
run-node:
	cargo run --bin node -- --port 9000

run-miner:
	@if [ ! -f miner.pub.pem ]; then \
		echo "Generating miner keys..."; \
		cargo run --bin key_gen miner; \
	fi
	cargo run --bin miner -- -a 127.0.0.1:9000 -p miner.pub.pem

run-wallet:
	@if [ ! -f wallet_config.toml ]; then \
		echo "Generating wallet config..."; \
		cargo run --bin good-wallet -- generate-config; \
	fi
	cargo run --bin good-wallet

# =============================================================================
# Documentation
# =============================================================================

docs:
	cargo doc --workspace --open --no-deps

# =============================================================================
# Complete workflows
# =============================================================================

# Fresh start with Docker
fresh-start: docker-clean docker-setup docker-start
	@echo "✅ Fresh blockchain network started!"

# Local development workflow
dev: build run-node

# Production build
prod: build-release

# Pre-commit checks
check-all: fmt clippy test
	@echo "✅ All checks passed!"

