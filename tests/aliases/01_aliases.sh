#!/bin/bash
# Test: Command Aliases on Linux
echo "Building..."
cargo build --quiet
Executable="./target/debug/ir"

TestFile="temp_alias_touch.txt"
TempDir="temp_alias_dir"
MovedFile="temp_alias_moved.txt"

# Clean up
rm -f $TestFile $MovedFile
rm -rf $TempDir

Result=0

echo "Testing alias 'touch' -> maps to 'create'..."
$Executable touch $TestFile

if [ -f "$TestFile" ]; then
    echo "✅ PASS: 'ir touch' created the file successfully."
else
    echo "❌ FAIL: 'ir touch' did not create the file."
    Result=1
fi

echo "Testing alias 'ls' -> maps to 'list'..."
ls_out=$($Executable ls)
if echo "$ls_out" | grep -q "temp_alias_touch"; then
    echo "✅ PASS: 'ir ls' executed successfully and listed files."
else
    echo "❌ FAIL: 'ir ls' did not return expected list."
    Result=1
fi

echo "Testing alias 'tar' -> maps to 'archive'..."
tar_out=$($Executable tar 2>&1)
if echo "$tar_out" | grep -qE "ir-archive|Destination"; then
    echo "✅ PASS: 'ir tar' successfully routed to archive."
else
    echo "❌ FAIL: 'ir tar' did not route to archive: $tar_out"
    Result=1
fi

echo "Testing alias 'cp' -> maps to 'copy'..."
mkdir -p $TempDir
$Executable cp $TestFile $TempDir
if [ -f "$TempDir/$TestFile" ]; then
    echo "✅ PASS: 'ir cp' copied file successfully."
else
    echo "❌ FAIL: 'ir cp' did not copy file."
    Result=1
fi

echo "Testing alias 'mv' -> maps to 'move'..."
$Executable mv "$TempDir/$TestFile" $MovedFile
if [ -f "$MovedFile" ] && [ ! -f "$TempDir/$TestFile" ]; then
    echo "✅ PASS: 'ir mv' moved file successfully."
else
    echo "❌ FAIL: 'ir mv' did not move file."
    Result=1
fi

echo "Testing alias 'rm' -> maps to 'remove'..."
$Executable rm $MovedFile
if [ ! -f "$MovedFile" ]; then
    echo "✅ PASS: 'ir rm' removed files successfully."
else
    echo "❌ FAIL: 'ir rm' did not remove files."
    Result=1
fi

echo "Testing alias 'ff' -> maps to 'fastfetch'..."
ff_out=$($Executable ff)
if echo "$ff_out" | grep -qE "OS|Host"; then
    echo "✅ PASS: 'ir ff' successfully ran fastfetch."
else
    echo "❌ FAIL: 'ir ff' did not run fastfetch properly."
    Result=1
fi

# Clean up
rm -f $TestFile $MovedFile
rm -rf $TempDir

exit $Result
