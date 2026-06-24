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
```

### `list`
Lists files and directories with detailed information.

**Usage:** `ir list [switches]`

**Switches:**
*   `-a`: Shows all files, including hidden ones.
*   `-s`: Sorts the output by file size, from largest to smallest.
*   `-t`: Sorts the output by modification time, from newest to oldest.
*   `--filter <extension>`: Filters the list to only show files with the specified extension.

### `rename`
Renames a file or folder.

**Usage:** `ir rename [switches] <SOURCE> <DESTINATION>`

**Switches:**
*   `-f`, `--force`: Overwrites the destination if it already exists.
*   `-i`, `--interactive`: Prompts for confirmation before renaming.
*   `--force-links`: Allows the renaming of symbolic links themselves.

## Documentation

On Linux, a `man` page is available in the `docs/` directory. To install it:
```sh
sudo cp docs/ir.1 /usr/local/share/man/man1/
gzip /usr/local/share/man/man1/ir.1
```
Then view it with `man ir`.

**Output Columns (for `list` action):**

*   **Windows:** Permissions, Size, Created, Modified, Name
*   **Linux:** Permissions, Size, Owner, Group, Modified, Changed, Name
