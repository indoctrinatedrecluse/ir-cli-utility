#!/bin/bash
# Test: Simple ln (hard and soft links)
echo "Building..."
cargo build --quiet
Executable="./target/debug/ir"

SourceFile="temp_ln_source.txt"
HardLinkFile="temp_ln_hard.txt"
SymLinkFile="temp_ln_soft.txt"

# Clean up
rm -f $SourceFile $HardLinkFile $SymLinkFile

echo "original content" > $SourceFile

echo "Creating hard link..."
$Executable ln $SourceFile $HardLinkFile

echo "Creating symbolic link..."
$Executable ln -s $SourceFile $SymLinkFile

Result=0

# Check hard link exists
if [ -f "$HardLinkFile" ]; then
    echo "✅ PASS: Hard link exists."
    # Change content of source and check hard link
    echo "modified content" > $SourceFile
    hard_content=$(cat $HardLinkFile)
    if [ "$hard_content" = "modified content" ]; then
        echo "✅ PASS: Hard link reflects content change."
    else
        echo "❌ FAIL: Hard link did not reflect content change: $hard_content"
        Result=1
    fi
else
    echo "❌ FAIL: Hard link does not exist."
    Result=1
fi

# Check symbolic link exists
if [ -L "$SymLinkFile" ]; then
    echo "✅ PASS: Symbolic link exists."
else
    echo "❌ FAIL: Symbolic link does not exist."
    Result=1
fi

# Test overwriting symbolic link with -f (force)
echo "Recreating symlink with -f (force)..."
$Executable ln -sf $SourceFile $SymLinkFile

if [ -L "$SymLinkFile" ]; then
    echo "✅ PASS: Force recreate of symbolic link succeeded."
else
    echo "❌ FAIL: Force recreate failed."
    Result=1
fi

# Clean up
rm -f $SourceFile $HardLinkFile $SymLinkFile

exit $Result
