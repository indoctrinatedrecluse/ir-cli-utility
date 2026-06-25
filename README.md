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
