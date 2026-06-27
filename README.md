# ir-cli-utility

`ir` is a cross-platform command-line utility for file system operations, built with a focus on performance and minimal dependencies by making direct syscalls to the underlying operating system.

The name `ir` is short for "indoctrinatedrecluse," the alias of the project's author.

## Usage
```
ir <ACTION> [OPTIONS]
```

## Actions

### `help`
Displays general help or help for a specific action.
```
ir help
ir help list
ir help rename
ir help copy
ir help remove
ir help create
ir help move
ir help archive
ir help cat
ir help grep
```

### `list`
Lists files and directories with detailed information. See `ir help list` for more.

### `rename`
Renames a file or folder. See `ir help rename` for more.

### `copy`
Copies files and folders. See `ir help copy` for more.

### `remove`
Removes files and folders. See `ir help remove` for more.

### `create`
Creates files and folders. See `ir help create` for more.

### `move`
Moves files and folders. See `ir help move` for more.

### `archive`
Creates, extracts, or tests archives.

**Usage:** `ir archive [switches] <PATH>`

**Arguments:**
*   `<PATH>`: The path to the source file/folder or the archive to be processed.

**Switches:**
*   `--dest <PATH>`: Specify a destination path for the output.
*   `--arc`: (Default) Creates an archive from the source path.
*   `--unarc`: Extracts the contents of the archive specified in `<PATH>`.
*   `--test`: Tests the integrity of the specified archive.
*   `--format <FORMAT>`: Specifies the archive format (e.g., zip, tar.gz).
*   `--force`: Overwrites the destination archive if it already exists.
*   `--verbose`: Prints the name of each file as it is being processed.

### `cat`
Prints file contents to standard output.

**Usage:** `ir cat [switches] <PATH>`

**Arguments:**
*   `<PATH>`: The path to the file to print.

**Switches:**
*   `-n`, `--line-numbers`: Prefix each output line with its source line number.
*   `--head <N>`: Prints the first `N` lines.
*   `--tail <N>`: Prints the last `N` lines.
*   `--range <START:END>`: Prints a 1-based inclusive line range.
*   `--binary`: Prints a hexadecimal preview of the file bytes.
*   `--encoding <ENC>`: Decodes text as `utf-8`, `utf-16`, or `ascii`.

**Rules:**
*   `--head`, `--tail`, and `--range` cannot be used together.
*   `--binary` cannot be used with text formatting switches.

### `grep`
Searches for patterns in files or stdin (for piping).

**Usage:** `ir grep [switches] <PATTERN> [FILE...]`

**Arguments:**
*   `<PATTERN>`: The pattern to search for.
*   `[FILE...]`: Optional file paths to search. If omitted, reads from stdin for piping.

**Switches:**
*   `-i`, `--ignore-case`: Perform case-insensitive matching.
*   `-n`, `--line-number`: Prefix each output line with its line number.
*   `-c`, `--count`: Count matching lines instead of displaying them.
*   `-l`, `--files-with-matches`: Print file names with matches only (no content).
*   `-v`, `--invert-match`: Select lines that do NOT match the pattern.
*   `-x`, `--line-regexp`: Match the entire line only.
*   `-F`, `--fixed-strings`: Treat pattern as a literal string, not regex.
*   `-E`, `--extended-regexp`: Use extended regular expression syntax.

**Examples:**
```
ir grep 'error' file.txt                    # Search for 'error' in a file
dir | ir grep 'README'                      # Search piped output from dir command
ir list | ir grep -i '.txt'                 # Pipe from another ir command
ir grep -n 'warning' app.log                # Show line numbers with matches
ir grep -c 'TODO' src/main.rs               # Count matching lines
```

## Documentation

On Linux, a `man` page is available in the `docs/` directory. To install it:
```sh
sudo cp docs/ir.1 /usr/local/share/man/man1/
gzip /usr/local/share/man/man1/ir.1
```
Then view it with `man ir`.

## Testing

The project includes a set of integration tests located in the `/tests` directory. These tests are written as shell scripts (`.sh` for Linux, `.ps1` for Windows) and are organized into subdirectories for each action.

Each test script is self-contained. It will:
1. Build the `ir` executable.
2. Create a temporary environment (files and folders).
3. Run the specific `ir` command being tested.
4. Verify that the outcome is correct.
5. Clean up the temporary environment.

To run a test, navigate to the appropriate directory and execute the script. For example:

**On Linux:**
```sh
cd tests/rename
./01_simple_file_rename.sh
```

**On Windows (PowerShell):**
```powershell
cd tests/rename
.\01_simple_file_rename.ps1
```
