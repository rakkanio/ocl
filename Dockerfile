# Multi-stage build for Zklear Payment System
FROM rust:1.82-slim as rust-builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install RISC Zero toolchain
RUN cargo install rzup
RUN rzup install rust
RUN rustup target add riscv32imac-unknown-none-elf

# Set working directory
WORKDIR /app

# Copy Rust project files
COPY Cargo.toml Cargo.lock ./
COPY methods/ ./methods/
COPY src/ ./src/
COPY build.rs ./

# Build the Rust application
RUN cargo build --release

# Node.js stage for React frontend
FROM node:18-alpine as node-builder

WORKDIR /app/frontend

# Copy React app files
COPY zklear-frontend/package*.json ./
RUN npm ci --only=production

COPY zklear-frontend/ ./
RUN npm run build

# Final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 appuser

# Set working directory
WORKDIR /app

# Copy built Rust binary
COPY --from=rust-builder /app/target/release/host /app/zklear-server

# Copy built React app
COPY --from=node-builder /app/frontend/build /app/frontend

# Create necessary directories and set permissions
RUN mkdir -p /app/data && chown -R appuser:appuser /app

# Create initial runtime files
RUN echo '[]' > /app/accounts.json && \
    echo '[]' > /app/transactions.json && \
    chown appuser:appuser /app/accounts.json /app/transactions.json

# Switch to non-root user
USER appuser

# Expose ports
EXPOSE 8081 3000

# Create startup script
RUN echo '#!/bin/bash\n\
# Start the Rust API server\n\
./zklear-server serve --port 8081 &\n\
\n\
# Start a simple HTTP server for the React app\n\
cd frontend && python3 -m http.server 3000 &\n\
\n\
# Wait for both processes\n\
wait' > /app/start.sh && chmod +x /app/start.sh

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8081/api/system-info || exit 1

# Default command
CMD ["/app/start.sh"] 