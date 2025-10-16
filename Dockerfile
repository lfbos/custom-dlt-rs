# Multi-stage Dockerfile for Custom Blockchain
# Builds all components: node, miner, wallet, and utilities

# =============================================================================
# Stage 1: Builder - Compile all Rust binaries
# Use latest Rust for all features
# =============================================================================
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libc6 \
    libc6-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./
COPY lib/Cargo.toml ./lib/
COPY node/Cargo.toml ./node/
COPY miner/Cargo.toml ./miner/
COPY wallet/Cargo.toml ./wallet/

# Copy source code
COPY lib/ ./lib/
COPY node/ ./node/
COPY miner/ ./miner/
COPY wallet/ ./wallet/

# Build all binaries in release mode for optimal performance
RUN cargo build --release --workspace

# List built binaries for verification
RUN ls -lh /app/target/release/

# =============================================================================
# Stage 2: Node Runtime
# Match rust:latest base for GLIBC compatibility
# =============================================================================
FROM debian:sid-slim AS node

RUN apt-get update && apt-get install -y \
    ca-certificates \
    iproute2 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy node binary from builder
COPY --from=builder /app/target/release/node /usr/local/bin/node

# Expose node port
EXPOSE 9000

# Create volume for blockchain data
VOLUME ["/data"]

# No default command - let docker-compose or env vars configure it
ENTRYPOINT ["node"]

# =============================================================================
# Stage 3: Miner Runtime
# Match rust:latest base for GLIBC compatibility
# =============================================================================
FROM debian:sid-slim AS miner

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy miner binary from builder
COPY --from=builder /app/target/release/miner /usr/local/bin/miner
COPY --from=builder /app/target/release/key_gen /usr/local/bin/key_gen

# Create volume for keys
VOLUME ["/keys"]

# Default command (will be overridden in docker-compose)
ENTRYPOINT ["miner"]

# =============================================================================
# Stage 4: Wallet Runtime
# Use same base as builder to avoid GLIBC issues
# =============================================================================
FROM rust:latest AS wallet

WORKDIR /app

# Copy wallet binary and utilities from builder
COPY --from=builder /app/target/release/good-wallet /usr/local/bin/wallet
COPY --from=builder /app/target/release/key_gen /usr/local/bin/key_gen

# Create volumes
VOLUME ["/keys"]
VOLUME ["/config"]

# Wallet requires interactive terminal
ENTRYPOINT ["wallet"]

# =============================================================================
# Stage 5: Utilities (for key generation, etc.)
# Match rust:latest base for GLIBC compatibility
# =============================================================================
FROM debian:sid-slim AS utilities

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy all utility binaries from builder
COPY --from=builder /app/target/release/key_gen /usr/local/bin/key_gen
COPY --from=builder /app/target/release/block_gen /usr/local/bin/block_gen
COPY --from=builder /app/target/release/block_print /usr/local/bin/block_print
COPY --from=builder /app/target/release/tx_gen /usr/local/bin/tx_gen
COPY --from=builder /app/target/release/tx_print /usr/local/bin/tx_print

# Default to shell for running utilities
CMD ["/bin/bash"]

