#!/bin/bash

# Render build script for Zklear Payment System

echo "🚀 Starting Zklear build for Render..."

# Build the Rust application
echo "🔨 Building Rust backend..."
cargo build --release

# Build the React frontend
echo "⚛️ Building React frontend..."
cd zklear-frontend
npm ci
npm run build
cd ..

# Create necessary directories
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

echo "✅ Build completed successfully!"
echo "📦 Ready for Render deployment" 