#!/bin/bash

# Render start script for Zklear Payment System

echo "🚀 Starting Zklear on Render..."

# Get port from environment variable (Render sets this)
PORT=${PORT:-8080}
echo "📡 Using port: $PORT"

# Start the Rust API server in the background
echo "🔧 Starting API server..."
./target/release/host serve --port $PORT &
API_PID=$!

# Wait a moment for the API to start
sleep 3

# Start a simple HTTP server for the React app
echo "⚛️ Starting React frontend server..."
cd zklear-frontend/build
python3 -m http.server 3000 &
FRONTEND_PID=$!

echo "✅ Zklear is running!"
echo "🌐 API Server: http://localhost:$PORT"
echo "🎨 Frontend: http://localhost:3000"

# Wait for both processes
wait $API_PID $FRONTEND_PID 