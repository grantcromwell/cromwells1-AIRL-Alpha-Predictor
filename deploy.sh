#!/bin/bash

# Deployment script for Financial Forecasting System

set -e

echo "=================================="
echo "Financial Forecasting Deployment"
echo "=================================="

# Check if Redis is running
echo "Checking Redis..."
if ! redis-cli ping > /dev/null 2>&1; then
    echo "Redis is not running. Starting Redis..."
    sudo systemctl start redis || sudo service redis-server start
    sleep 2
fi

echo "Redis is running ✓"

# Build Java backend
echo ""
echo "Building Java backend..."
cd java-backend
mvn clean package -DskipTests
cd ..
echo "Java backend built ✓"

# Build Rust models
echo ""
echo "Building Rust models..."
cd rust-model
cargo build --release
cd ..
echo "Rust models built ✓"

# Stop existing processes (if any)
echo ""
echo "Stopping existing processes..."
pkill -f "com.financial.backend.Main" || true
pkill -f "rust-model" || true
sleep 2

# Start Java backend in background
echo ""
echo "Starting Java backend..."
cd java-backend
nohup mvn exec:java -Dexec.mainClass="com.financial.backend.Main" > ../logs/backend.log 2>&1 &
JAVA_PID=$!
cd ..
echo "Java backend started (PID: $JAVA_PID)"

# Wait for backend to start
echo "Waiting for backend to initialize..."
sleep 5

# Check health endpoint
echo "Checking backend health..."
if curl -s http://localhost:8080/api/health > /dev/null; then
    echo "Backend is healthy ✓"
else
    echo "Warning: Backend health check failed. Check logs."
fi

echo ""
echo "=================================="
echo "Deployment Complete!"
echo "=================================="
echo ""
echo "Services running:"
echo "  - Java Backend: http://localhost:8080"
echo "  - Redis: localhost:6379"
echo ""
echo "API Endpoints:"
echo "  - GET /api/equity?symbol=NVDA&timestamp=..."
echo "  - GET /api/equity/symbol?symbol=NVDA&limit=50"
echo "  - GET /api/equity/all"
echo "  - GET /api/health"
echo ""
echo "To view the dashboard, open src/ui/dashboard.html in your browser"
echo ""
echo "Logs:"
echo "  - Backend: tail -f logs/backend.log"
echo ""
echo "To stop the services:"
echo "  kill $JAVA_PID"
echo "  sudo systemctl stop redis"
echo ""
