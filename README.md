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
ir help find
ir help diff
ir help search
ir help which
```

### `list`
Lists files and directories with detailed information.

**Usage:** `ir list [switches]`

**Switches:**
*   `-a`: Shows all files, including hidden ones.
*   `-s`: Sorts the output by file size, from largest to smallest.
*   `-t`: Sorts the output by modification time, from newest to oldest.
*   `-f`: Lists only files (excludes directories).
*   `-l`: Lists only directories/folders (excludes files).
*   `--filter <ext>`: Filters by file extension.

### `rename`
Renames a file or folder.

**Usage:** `ir rename [switches] <SOURCE_PATH> <NEW_NAME>`

**Arguments:**
*   `<SOURCE_PATH>`: The full or relative path to the file/folder to rename.
*   `<NEW_NAME>`: The new name for the file/folder (not a path).

**Switches:**
*   `-f`, `--force`: Overwrites the destination if it already exists.
*   `-i`, `--interactive`: Prompts for confirmation before renaming.
*   `--force-links`: Allows the renaming of symbolic links themselves.

### `copy`
Copies files and folders.

**Usage:** `ir copy [switches] <SOURCE> <DESTINATION>`

**Arguments:**
*   `<SOURCE>`: The path to the file or folder to copy.
*   `<DESTINATION>`: The path to the destination folder.

**Switches:**
*   `--force`: Overwrites destination files if they already exist.
*   `-r`: (Default) Copies directories and their contents recursively.
*   `-f`: Copies only files from the source, not subdirectories.
*   `-l`: Copies only subdirectories from the source, not files.
*   `--rename <NAME>`: When copying a single file, saves it under a new name.

**Rules:**
*   The `-r` switch cannot be used with `-f` or `-l`.

### `remove`
Removes files and folders.

**Usage:** `ir remove [switches] <PATH...>`

**Arguments:**
*   `<PATH...>`: One or more paths to the files or folders to remove.

**Switches:**
*   `-f`, `--force`: Force removes files and directories without prompting.
*   `-i`, `--interactive`: Prompts for confirmation before every removal.
*   `-t`, `--trash`: Moves items to the system trash instead of permanently deleting.
*   `-v`, `--verbose`: Prints the name of each file as it is being removed.
*   `-y`: Skips the confirmation prompt for non-empty folders.

### `create`
Creates files and folders.

**Usage:** `ir create [switches] <PATH...>`

**Arguments:**
*   `<PATH...>`: One or more paths for the items to be created.

**Switches:**
*   `--create-file`: Forces the creation of a file, even if it has no extension.
*   `-p`, `--force-subdirs`: Creates parent directories as needed.

### `move`
Moves files and folders.

**Usage:** `ir move [switches] <SOURCE> <DESTINATION>`

**Arguments:**
*   `<SOURCE>`: The path to the file or folder to move.
*   `<DESTINATION>`: The path to the destination folder or new file path.

**Switches:**
*   `--force`: Overwrites destination files if they already exist.
*   `--rename <NAME>`: When moving a single file, saves it under a new name.

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

### `find`
Finds files and directories by name, type, depth, or emptiness.

**Usage:** `ir find [PATH...] [EXPRESSION]`

**Arguments:**
*   `[PATH...]`: Optional root paths to search. Defaults to the current directory. If paths are piped through stdin and no paths are specified, searches those paths.

**Expressions:**
*   `-name <PATTERN>`: Match a file or directory name using `*` and `?` wildcards.
*   `-iname <PATTERN>`: Like `-name`, but case-insensitive.
*   `-type f`: Match files only.
*   `-type d`: Match directories only.
*   `-maxdepth <N>`: Descend at most `N` levels below each root.
*   `-mindepth <N>`: Do not print entries shallower than `N` levels below each root.
*   `-empty`: Match empty files and empty directories.

**Examples:**
```
ir find . -name '*.rs'                     # Find Rust files under the current directory
ir find src -type d                        # Find directories under src
ir find . -maxdepth 1 -type f              # Find files directly under the current directory
echo src | ir find -name '*.rs'            # Search paths supplied through stdin
```

### `diff`
Compares two text files.

**Usage:** `ir diff [switches] <LEFT_FILE> <RIGHT_FILE>`

**Arguments:**
*   `<LEFT_FILE>`: The first file to compare.
*   `<RIGHT_FILE>`: The second file to compare.

**Switches:**
*   `-q`, `--brief`: Report only whether the files differ.
*   `-i`, `--ignore-case`: Ignore ASCII case differences.
*   `-u`, `--unified`: Print unified-style output.

**Examples:**
```
ir diff old.txt new.txt                    # Compare two files
ir diff -u old.txt new.txt                 # Show unified-style output
ir diff -q old.txt new.txt                 # Only report whether files differ
```

### `search`
Recursively searches file contents under one or more paths.

**Usage:** `ir search [switches] <PHRASE> [PATH...]`

**Arguments:**
*   `<PHRASE>`: The literal phrase to search for.
*   `[PATH...]`: Optional root paths to search. Defaults to the current directory. If paths are piped through stdin and no paths are specified, searches those paths.

**Switches:**
*   `-i`, `--ignore-case`: Perform case-insensitive matching.
*   `-n`, `--line-number`: Prefix matches with line numbers. Enabled by default.
*   `--no-line-number`: Do not print line numbers.
*   `-l`, `--files-with-matches`: Print file names with matches only.
*   `-c`, `--count`: Count matching lines per file.
*   `-name <PATTERN>`: Search only file names matching `*` and `?` wildcards.
*   `-iname <PATTERN>`: Like `-name`, but case-insensitive.
*   `-maxdepth <N>`: Descend at most `N` levels below each root.
*   `-mindepth <N>`: Do not search files shallower than `N` levels below each root.
*   `--include <EXT>`: Search only files with this extension. Can be repeated.
*   `--exclude <EXT>`: Skip files with this extension. Can be repeated.
*   `--all`: Include normally skipped file extensions.

Common binary, executable, archive, and document extensions are skipped by default.

**Examples:**
```
ir search TODO src                          # Search src recursively
ir search -i "error code" .                 # Case-insensitive phrase search
ir search TODO . --include rs               # Search only Rust files
echo src | ir search TODO                   # Search paths supplied through stdin
```

### `which`
Locates a command in `PATH`.

**Usage:** `ir which [switches] <COMMAND>`

**Arguments:**
*   `<COMMAND>`: The command name to locate.

**Switches:**
*   `-a`, `--all`: Print all matching commands in `PATH` order.

On Windows, `PATHEXT` is used to resolve executable extensions.

**Examples:**
```
ir which rustc                              # Locate rustc
ir which -a python                          # Print all python matches
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
