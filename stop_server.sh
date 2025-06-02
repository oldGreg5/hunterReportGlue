#!/bin/zsh

# Stop any running Node.js server processes
echo "Stopping Hunter Report Generator server..."

# Method 1: Using pkill
pkill -f "node testNode.js" 2>/dev/null

# Method 2: Find process by port and kill it
lsof -i :8089 | grep LISTEN | awk '{print $2}' | xargs kill -9 2>/dev/null

# Method 3: Killall node as a last resort
killall node 2>/dev/null

echo "Server stopped successfully." 