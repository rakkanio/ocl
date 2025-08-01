#!/bin/bash

# Render-optimized build script for Zklear Payment System
set -e  # Exit on any error

echo "🚀 Starting Zklear build for Render..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

# Install RISC Zero toolchain
echo "🔧 Installing RISC Zero toolchain..."
echo "Installing rzup..."
cargo install rzup --quiet
echo "Installing RISC Zero Rust toolchain..."
rzup install rust
echo "Adding RISC-V target..."
rustup target add riscv32imac-unknown-none-elf

# Verify toolchain installation
echo "✅ RISC Zero toolchain installed successfully"

# Build the Rust application
echo "🔨 Building Rust backend..."
cargo build --release --verbose

# Verify the binary was created
if [ ! -f "target/release/host" ]; then
    echo "❌ Error: Binary not found at target/release/host"
    exit 1
fi

echo "✅ Rust backend built successfully"

# Build the React frontend
echo "⚛️ Building React frontend..."
cd zklear-frontend

# Install dependencies
echo "Installing npm dependencies..."
npm ci --silent

# Build the application
echo "Building React app..."
npm run build

# Verify the build was created
if [ ! -d "build" ]; then
    echo "❌ Error: React build directory not found"
    exit 1
fi

cd ..

echo "✅ React frontend built successfully"

# Create necessary directories and files
echo "📁 Setting up runtime files..."
mkdir -p data

# Create initial files if they don't exist
if [ ! -f accounts.json ]; then
    echo '[]' > accounts.json
    echo "✅ Created initial accounts.json"
fi

if [ ! -f transactions.json ]; then
    echo '[]' > transactions.json
    echo "✅ Created initial transactions.json"
fi

# Final verification
echo "🔍 Final verification..."
ls -la target/release/host
ls -la zklear-frontend/build/
ls -la accounts.json transactions.json

echo "✅ Build completed successfully!"
echo "📦 Ready for Render deployment"
echo "📊 Build summary:"
echo "   - Rust binary: target/release/host"
echo "   - React build: zklear-frontend/build/"
echo "   - Data files: accounts.json, transactions.json" 