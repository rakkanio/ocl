#!/bin/bash

# Zklear Payment System Deployment Script
set -e

echo "🚀 Starting Zklear Payment System deployment..."

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if docker-compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Create necessary directories and files
echo "📁 Setting up directories and files..."

# Create data directory
mkdir -p data

# Create initial account and transaction files if they don't exist
if [ ! -f accounts.json ]; then
    echo '[]' > accounts.json
    echo "✅ Created initial accounts.json"
fi

if [ ! -f transactions.json ]; then
    echo '[]' > transactions.json
    echo "✅ Created initial transactions.json"
fi

# Stop any existing containers
echo "🛑 Stopping existing containers..."
docker-compose down --remove-orphans 2>/dev/null || true

# Build and start the application
echo "🔨 Building and starting Zklear..."
docker-compose up --build -d

# Wait for the application to start
echo "⏳ Waiting for application to start..."
sleep 10

# Check if the application is running
if curl -f http://localhost:8081/api/system-info > /dev/null 2>&1; then
    echo "✅ Zklear Payment System is running successfully!"
    echo ""
    echo "🌐 Access your application:"
    echo "   • API Server: http://localhost:8081"
    echo "   • React Frontend: http://localhost:3000"
    echo "   • API Documentation: http://localhost:3000/api-spec"
    echo ""
    echo "📋 Useful commands:"
    echo "   • View logs: docker-compose logs -f"
    echo "   • Stop application: docker-compose down"
    echo "   • Restart application: docker-compose restart"
    echo ""
    echo "🔧 API Endpoints:"
    echo "   • System Info: http://localhost:8081/api/system-info"
    echo "   • Create Account: http://localhost:8081/api/accounts"
    echo "   • Process Batch: http://localhost:8081/api/batch/process"
else
    echo "❌ Application failed to start. Check logs with: docker-compose logs"
    exit 1
fi 