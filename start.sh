#!/bin/bash

# Render start script for Zklear Payment System

echo "🚀 Starting Zklear on Render..."

# Get port from environment variable (Render sets this)
PORT=${PORT:-8080}
echo "📡 Using port: $PORT"

# Start the Rust API server (serves both API and React frontend)
echo "🔧 Starting Zklear server..."
./target/release/host serve --port $PORT

echo "✅ Zklear is running!"
echo "🌐 Server: http://localhost:$PORT"
echo "🎨 React App: http://localhost:$PORT"
echo "📚 API Docs: http://localhost:$PORT/api-docs.html" 