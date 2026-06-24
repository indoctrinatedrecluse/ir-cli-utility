#!/bin/bash

# This script builds and runs the ir-cli-utility for testing.

# Exit immediately if a command exits with a non-zero status.
set -e

EXECUTABLE="./target/debug/ir"

# Remove the old executable if it exists
if [ -f "$EXECUTABLE" ]; then
    echo "Removing previous executable..."
    rm "$EXECUTABLE"
fi

echo "Building the project..."
# The 'set -e' command will cause the script to exit here if the build fails.
cargo build

echo "Build successful. Running the executable..."
"$EXECUTABLE" help
