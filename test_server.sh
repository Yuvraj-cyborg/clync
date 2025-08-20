#!/bin/bash

# Test script for clync server functionality
set -e

echo "🧪 Testing clync server functionality..."

# Kill any existing server processes
pkill -f "clync.*server" 2>/dev/null || true
sleep 1

# Start the server in the background
echo "🚀 Starting server on port 8080..."
cargo run --release -- server 8080 &
SERVER_PID=$!

# Wait for server to start
echo "⏳ Waiting for server to start..."
sleep 3

# Function to cleanup on exit
cleanup() {
    echo "🧹 Cleaning up..."
    kill $SERVER_PID 2>/dev/null || true
    pkill -f "clync.*server" 2>/dev/null || true
}
trap cleanup EXIT

# Test 1: Check if server is responding
echo "📡 Test 1: Checking if server is responding..."
if curl -s -X GET http://localhost:8080/get > /dev/null; then
    echo "✅ Server is responding"
else
    echo "❌ Server is not responding"
    exit 1
fi

# Test 2: Test GET endpoint (should return 404 initially)
echo "📡 Test 2: Testing GET endpoint..."
RESPONSE=$(curl -s -w "%{http_code}" -X GET http://localhost:8080/get)
if [[ "$RESPONSE" == *"404"* ]]; then
    echo "✅ GET endpoint working (returned 404 as expected for empty clipboard)"
else
    echo "⚠️  GET endpoint returned: $RESPONSE"
fi

# Test 3: Test POST endpoint
echo "📡 Test 3: Testing POST endpoint..."
RESPONSE=$(curl -s -w "%{http_code}" -X POST http://localhost:8080/sync \
    -H "Content-Type: application/json" \
    -d '{"content": "Hello from test!"}')
if [[ "$RESPONSE" == *"200"* ]]; then
    echo "✅ POST endpoint working"
else
    echo "❌ POST endpoint failed: $RESPONSE"
    exit 1
fi

# Test 4: Test GET endpoint after POST
echo "📡 Test 4: Testing GET endpoint after POST..."
RESPONSE=$(curl -s -X GET http://localhost:8080/get)
if [[ "$RESPONSE" == *"Hello from test!"* ]]; then
    echo "✅ Data successfully stored and retrieved"
else
    echo "❌ Failed to retrieve stored data: $RESPONSE"
    exit 1
fi

echo ""
echo "🎉 All server tests passed!"
echo ""
echo "🔧 Manual testing instructions:"
echo "1. Keep the server running: cargo run --release -- server 8080"
echo "2. In another terminal, test the client: cargo run --release -- client http://localhost:8080"
echo "3. Copy something to your clipboard and watch it sync!"
echo ""
echo "📋 For Windows testing:"
echo "1. Transfer clync.exe to your Windows machine"
echo "2. Run: clync.exe server 8080 (on server machine)"
echo "3. Run: clync.exe client http://SERVER_IP:8080 (on client machine)"
