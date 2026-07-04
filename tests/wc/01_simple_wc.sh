#!/bin/bash
# Test: Simple wc
echo "Building..."
cargo build --quiet
Executable="./target/debug/ir"

TempFile="temp_wc_test.txt"
printf "hello world\nline two\nline three" > $TempFile

# printf has 2 newlines (\n)
# Words: "hello", "world", "line", "two", "line", "three" -> 6 words
# Bytes: 11 + 1 + 8 + 1 + 10 = 31 bytes
BytesCount=$(wc -c < $TempFile | tr -d ' ')

echo "Running tests on temp file..."
output_full=$($Executable wc $TempFile)
echo "Output (default): $output_full"

output_lines=$($Executable wc -l $TempFile)
echo "Output (-l): $output_lines"

output_words=$($Executable wc -w $TempFile)
echo "Output (-w): $output_words"

output_bytes=$($Executable wc -c $TempFile)
echo "Output (-c): $output_bytes"

output_stdin=$(echo -n "hello world" | $Executable wc -w)
echo "Output (stdin -w): $output_stdin"

# Clean up
rm -f $TempFile

Result=0
if [[ "$output_lines" =~ 2[[:space:]]+temp_wc_test.txt ]]; then
    echo "✅ PASS: 'ir wc -l' returned 2 newlines."
else
    echo "❌ FAIL: 'ir wc -l' returned unexpected output."
    Result=1
fi

if [[ "$output_words" =~ 6[[:space:]]+temp_wc_test.txt ]]; then
    echo "✅ PASS: 'ir wc -w' returned 6 words."
else
    echo "❌ FAIL: 'ir wc -w' returned unexpected output: $output_words"
    Result=1
fi

if [[ "$output_bytes" =~ $BytesCount[[:space:]]+temp_wc_test.txt ]]; then
    echo "✅ PASS: 'ir wc -c' returned $BytesCount bytes."
else
    echo "❌ FAIL: 'ir wc -c' returned unexpected output: $output_bytes"
    Result=1
fi

if [[ "$output_stdin" =~ 2 ]]; then
    echo "✅ PASS: 'stdin | ir wc -w' returned 2 words."
else
    echo "❌ FAIL: 'stdin | ir wc -w' returned unexpected output."
    Result=1
fi

exit $Result
