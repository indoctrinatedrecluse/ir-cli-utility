#!/bin/bash
# Test: Simple chmod on Linux
echo "Building..."
cargo build --quiet
Executable="./target/debug/ir"

TestFile="temp_chmod_test.txt"

# Clean up
rm -f $TestFile

echo "initial content" > $TestFile

Result=0

echo "Applying chmod 444 (read-only)..."
$Executable chmod 444 $TestFile

# Check permissions
perm1=$(stat -c "%a" $TestFile)
if [ "$perm1" = "444" ]; then
    echo "✅ PASS: File has 444 permissions."
else
    echo "❌ FAIL: File has unexpected permissions: $perm1"
    Result=1
fi

echo "Applying chmod 755 (writeable/executable)..."
$Executable chmod 755 $TestFile

perm2=$(stat -c "%a" $TestFile)
if [ "$perm2" = "755" ]; then
    echo "✅ PASS: File has 755 permissions."
else
    echo "❌ FAIL: File has unexpected permissions: $perm2"
    Result=1
fi

# Clean up
rm -f $TestFile

exit $Result
