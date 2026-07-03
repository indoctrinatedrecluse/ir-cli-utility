#!/bin/bash
# Test: Simple whoami
echo "Building..."
cargo build --quiet
Executable="./target/debug/ir"

echo "Running test: ir whoami"
output=$($Executable whoami)
echo "Output: $output"

if [ -n "$output" ]; then
    echo "✅ PASS: 'ir whoami' returned a non-empty identity."
    exit 0
else
    echo "❌ FAIL: 'ir whoami' returned empty identity."
    exit 1
fi
