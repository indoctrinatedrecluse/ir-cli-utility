#!/bin/bash
# Test: Simple sockets
echo "Building..."
cargo build --quiet
Executable="./target/debug/ir"

echo "Running test: ir sockets -a"
output=$($Executable sockets -a)
echo "$output"

if echo "$output" | grep -q "Proto" && echo "$output" | grep -q "Local Address"; then
    echo "✅ PASS: 'ir sockets' output header verified."
    exit 0
else
    echo "❌ FAIL: 'ir sockets' output headers missing."
    exit 1
fi
