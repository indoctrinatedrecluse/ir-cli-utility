# 🛠️ ir-cli-utility

`ir` is a cross-platform command-line utility for file system operations, built with a focus on performance and minimal dependencies by making direct syscalls to the underlying operating system.

> [!NOTE]
> The name `ir` is short for **"indoctrinatedrecluse"**, the alias of the project's author.

---

## 📥 Installation

You can install `ir` automatically using our installer scripts, or manually.

### ⚡ Automated Installation (Recommended)

#### Windows (PowerShell):
Run the following command in PowerShell to download, extract to `%APPDATA%\ir`, and add it to your user `PATH`:
```powershell
iwr -useb https://raw.githubusercontent.com/indoctrinatedrecluse/ir-cli-utility/main/install.ps1 | iex
```

#### Linux/macOS (curl/wget):
Run the following command in your terminal to download, extract to `/usr/local/bin` (or `~/.local/bin`), and update your shell profile `PATH`:
```bash
curl -fsSL https://raw.githubusercontent.com/indoctrinatedrecluse/ir-cli-utility/main/install.sh | sh
```

---

### 🛠️ Manual Installation

#### Windows (PowerShell):
If you prefer to perform the steps manually:
```powershell
# Create target directory in AppData
New-Item -ItemType Directory -Path "$env:APPDATA\ir" -Force

# Download latest archive
Invoke-WebRequest -Uri "https://github.com/indoctrinatedrecluse/ir-cli-utility/releases/latest/download/ir-windows.zip" -OutFile "$env:TEMP\ir.zip"

# Extract to AppData
Expand-Archive -Path "$env:TEMP\ir.zip" -DestinationPath "$env:APPDATA\ir" -Force
Remove-Item "$env:TEMP\ir.zip"

# Add to User PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -split ';' -notcontains "$env:APPDATA\ir") {
    [Environment]::SetEnvironmentVariable("Path", $userPath + ";$env:APPDATA\ir", "User")
}
```

#### Linux (Command Line):
```bash
# Download latest archive
curl -LO https://github.com/indoctrinatedrecluse/ir-cli-utility/releases/latest/download/ir-linux.tar.gz

# Extract to /usr/local/bin
sudo tar -xzf ir-linux.tar.gz -C /usr/local/bin

# Make executable
sudo chmod +x /usr/local/bin/ir
rm ir-linux.tar.gz
```

---

### 📦 From Source (Cross-platform)
If you have [Rust and Cargo](https://rustup.rs/) installed, you can build and install `ir` directly from the repository:

```bash
cargo install --git https://github.com/indoctrinatedrecluse/ir-cli-utility.git
```

This compiles the utility optimized for your local CPU and installs it to your Cargo bin directory (`~/.cargo/bin`), which is typically in your system `PATH`.

---

## 🚀 Usage

```bash
ir <ACTION> [OPTIONS]
```

---

## 📂 Actions

### 📘 `help`
Displays general help or help for a specific action.

**Usage:**
```bash
ir help [action]
```

**Examples:**
```bash
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

---

### 📋 `list`
Lists files and directories with detailed information.

**Usage:**
```bash
ir list [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-a` | Shows all files, including hidden ones. |
| `-s` | Sorts the output by file size, from largest to smallest. |
| `-t` | Sorts the output by modification time, from newest to oldest. |
| `-f` | Lists only files (excludes directories). |
| `-l` | Lists only directories/folders (excludes files). |
| `--filter <ext>` | Filters by file extension. |

---

### ✏️ `rename`
Renames a file or folder.

**Usage:**
```bash
ir rename [switches] <SOURCE_PATH> <NEW_NAME>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<SOURCE_PATH>` | The full or relative path to the file/folder to rename. |
| `<NEW_NAME>` | The new name for the file/folder (not a path). |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-f`, `--force` | Overwrites the destination if it already exists. |
| `-i`, `--interactive` | Prompts for confirmation before renaming. |
| `--force-links` | Allows the renaming of symbolic links themselves. |

---

### 💾 `copy`
Copies files and folders.

**Usage:**
```bash
ir copy [switches] <SOURCE> <DESTINATION>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<SOURCE>` | The path to the file or folder to copy. |
| `<DESTINATION>` | The path to the destination folder. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `--force` | Overwrites destination files if they already exist. |
| `-r` | (Default) Copies directories and their contents recursively. |
| `-f` | Copies only files from the source, not subdirectories. |
| `-l` | Copies only subdirectories from the source, not files. |
| `--rename <NAME>` | When copying a single file, saves it under a new name. |

> [!IMPORTANT]
> The `-r` switch cannot be used with `-f` or `-l`.

---

### 🗑️ `remove`
Removes files and folders.

**Usage:**
```bash
ir remove [switches] <PATH...>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<PATH...>` | One or more paths to the files or folders to remove. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-f`, `--force` | Force removes files and directories without prompting. |
| `-i`, `--interactive` | Prompts for confirmation before every removal. |
| `-t`, `--trash` | Moves items to the system trash instead of permanently deleting. |
| `-v`, `--verbose` | Prints the name of each file as it is being removed. |
| `-y` | Skips the confirmation prompt for non-empty folders. |

---

### ➕ `create`
Creates files and folders.

**Usage:**
```bash
ir create [switches] <PATH...>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<PATH...>` | One or more paths for the items to be created. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `--create-file` | Forces the creation of a file, even if it has no extension. |
| `-p`, `--force-subdirs` | Creates parent directories as needed. |

---

### 📦 `move`
Moves files and folders.

**Usage:**
```bash
ir move [switches] <SOURCE> <DESTINATION>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<SOURCE>` | The path to the file or folder to move. |
| `<DESTINATION>` | The path to the destination folder or new file path. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `--force` | Overwrites destination files if they already exist. |
| `--rename <NAME>` | When moving a single file, saves it under a new name. |

---

### 🗜️ `archive`
Creates, extracts, or tests archives.

**Usage:**
```bash
ir archive [switches] <PATH>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<PATH>` | The path to the source file/folder or the archive to be processed. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `--dest <PATH>` | Specify a destination path for the output. |
| `--arc` | (Default) Creates an archive from the source path. |
| `--unarc` | Extracts the contents of the archive specified in `<PATH>`. |
| `--test` | Tests the integrity of the specified archive. |
| `--format <FORMAT>` | Specifies the archive format (e.g., zip, tar.gz). |
| `--force` | Overwrites the destination archive if it already exists. |
| `--verbose` | Prints the name of each file as it is being processed. |

---

### 🐱 `cat`
Prints file contents to standard output.

**Usage:**
```bash
ir cat [switches] <PATH>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<PATH>` | The path to the file to print. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-n`, `--line-numbers` | Prefix each output line with its source line number. |
| `--head <N>` | Prints the first `N` lines. |
| `--tail <N>` | Prints the last `N` lines. |
| `--range <START:END>` | Prints a 1-based inclusive line range. |
| `--binary` | Prints a hexadecimal preview of the file bytes. |
| `--encoding <ENC>` | Decodes text as `utf-8`, `utf-16`, or `ascii`. |

> [!IMPORTANT]
> * `--head`, `--tail`, and `--range` cannot be used together.
> * `--binary` cannot be used with text formatting switches.

---

### 🔍 `grep`
Searches for patterns in files or stdin (for piping).

**Usage:**
```bash
ir grep [switches] <PATTERN> [FILE...]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<PATTERN>` | The pattern to search for. |
| `[FILE...]` | Optional file paths to search. If omitted, reads from stdin for piping. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-i`, `--ignore-case` | Perform case-insensitive matching. |
| `-n`, `--line-number` | Prefix each output line with its line number. |
| `-c`, `--count` | Count matching lines instead of displaying them. |
| `-l`, `--files-with-matches` | Print file names with matches only (no content). |
| `-v`, `--invert-match` | Select lines that do NOT match the pattern. |
| `-x`, `--line-regexp` | Match the entire line only. |
| `-F`, `--fixed-strings` | Treat pattern as a literal string, not regex. |
| `-E`, `--extended-regexp` | Use extended regular expression syntax. |

**Examples:**
```bash
ir grep 'error' file.txt                    # Search for 'error' in a file
dir | ir grep 'README'                      # Search piped output from dir command
ir list | ir grep -i '.txt'                 # Pipe from another ir command
ir grep -n 'warning' app.log                # Show line numbers with matches
ir grep -c 'TODO' src/main.rs               # Count matching lines
```

---

### 🗺️ `find`
Finds files and directories by name, type, depth, or emptiness.

**Usage:**
```bash
ir find [PATH...] [EXPRESSION]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[PATH...]` | Optional root paths to search. Defaults to the current directory. If paths are piped through stdin and no paths are specified, searches those paths. |

**Expressions:**
| Expression | Description |
| :--- | :--- |
| `-name <PATTERN>` | Match a file or directory name using `*` and `?` wildcards. |
| `-iname <PATTERN>` | Like `-name`, but case-insensitive. |
| `-type f` | Match files only. |
| `-type d` | Match directories only. |
| `-maxdepth <N>` | Descend at most `N` levels below each root. |
| `-mindepth <N>` | Do not print entries shallower than `N` levels below each root. |
| `-empty` | Match empty files and empty directories. |

**Examples:**
```bash
ir find . -name '*.rs'                     # Find Rust files under the current directory
ir find src -type d                        # Find directories under src
ir find . -maxdepth 1 -type f              # Find files directly under the current directory
echo src | ir find -name '*.rs'            # Search paths supplied through stdin
```

---

### ⚖️ `diff`
Compares two text files.

**Usage:**
```bash
ir diff [switches] <LEFT_FILE> <RIGHT_FILE>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<LEFT_FILE>` | The first file to compare. |
| `<RIGHT_FILE>` | The second file to compare. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-q`, `--brief` | Report only whether the files differ. |
| `-i`, `--ignore-case` | Ignore ASCII case differences. |
| `-u`, `--unified` | Print unified-style output. |

**Examples:**
```bash
ir diff old.txt new.txt                    # Compare two files
ir diff -u old.txt new.txt                 # Show unified-style output
ir diff -q old.txt new.txt                 # Only report whether files differ
```

---

### 🕵️ `search`
Recursively searches file contents under one or more paths.

**Usage:**
```bash
ir search [switches] <PHRASE> [PATH...]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<PHRASE>` | The literal phrase to search for. |
| `[PATH...]` | Optional root paths to search. Defaults to the current directory. If paths are piped through stdin and no paths are specified, searches those paths. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-i`, `--ignore-case` | Perform case-insensitive matching. |
| `-n`, `--line-number` | Prefix matches with line numbers. Enabled by default. |
| `--no-line-number` | Do not print line numbers. |
| `-l`, `--files-with-matches` | Print file names with matches only. |
| `-c`, `--count` | Count matching lines per file. |
| `-name <PATTERN>` | Search only file names matching `*` and `?` wildcards. |
| `-iname <PATTERN>` | Like `-name`, but case-insensitive. |
| `-maxdepth <N>` | Descend at most `N` levels below each root. |
| `-mindepth <N>` | Do not search files shallower than `N` levels below each root. |
| `--include <EXT>` | Search only files with this extension. Can be repeated. |
| `--exclude <EXT>` | Skip files with this extension. Can be repeated. |
| `--all` | Include normally skipped file extensions. |

> [!NOTE]
> Common binary, executable, archive, and document extensions are skipped by default.

**Examples:**
```bash
ir search TODO src                          # Search src recursively
ir search -i "error code" .                 # Case-insensitive phrase search
ir search TODO . --include rs               # Search only Rust files
echo src | ir search TODO                   # Search paths supplied through stdin
```

---

### 🌳 `tree`
Displays a directory tree representation of the filesystem.

**Usage:**
```bash
ir tree [switches] [PATH]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[PATH]` | The root path of the directory tree. Defaults to the current directory. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-a` | Shows all files, including hidden ones. |
| `-d` | List directories only. |
| `-L <depth>` | Max display depth of the directory tree. |
| `-f` | Print the full path prefix for each file. |
| `-i` | Makes tree not print the indentation lines. |
| `-s` | Print the size of each file in bytes. |
| `-h` | Print the size in a more human-readable format. |
| `-p` | Print file permissions. |
| `--noreport` | Omits printing of the file and directory report at the end. |

> [!NOTE]
> * Concatenated switches like `-adps` are fully supported.
> * If `-h` (human-readable size) is specified, it takes precedence over raw sizes from `-s`.

**Examples:**
```bash
ir tree                                  # Show the tree structure of the current directory
ir tree -L 2 src                         # Show the src directory tree up to depth 2
ir tree -adps -h                         # Show all files, permissions, human-readable sizes
```

---

### 📊 `du`
Estimates file space usage recursively.

**Usage:**
```bash
ir du [switches] [PATH...]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[PATH...]` | One or more paths to estimate. Defaults to the current directory. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-a` | Write counts for all files, not just directories. |
| `-c` | Produce a grand total at the end. |
| `-h` | Print sizes in human-readable format. |
| `-s` | Display only a total for each argument (equivalent to `-d 0`). |
| `-d <depth>, --max-depth <depth>` | Print the total for a directory only if it is at or below this depth. |
| `-k` | Print sizes in kilobytes (1024-byte blocks) [Default]. |
| `-m` | Print sizes in megabytes (1024*1024-byte blocks). |

> [!IMPORTANT]
> **Rules:**
> * `-h`, `-k`, and `-m` are mutually exclusive size formatting switches.
> * `-s` (summarize) and `-d` (max-depth > 0) cannot be combined.

**Examples:**
```bash
ir du                                    # Show disk usage of all directories
ir du -sh *                              # Summarize disk usage of all items in human-readable format
ir du -ah -d 1                           # Show human-readable usage of all files up to depth 1
```

---

### ⚡ `fastfetch`
Displays system information side-by-side with a stylized ASCII logo.

**Usage:**
```bash
ir fastfetch
```

**Examples:**
```bash
ir fastfetch                             # Show system info
```

---

### 🖥️ `monitor`
Invokes the bundled `term-sys-monitor` utility in a separate shell window.

**Usage:**
```bash
ir monitor
```

**Examples:**
```bash
ir monitor                               # Launch system monitor
```

---

### 🔑 `hash`
Generates or verifies cryptographic file checksums (MD5, SHA-1, SHA-256, SHA-512).

**Usage:**
```bash
ir hash [switches] <PATH>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<PATH>` | The file to hash, or the checksum file to verify. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-a <algorithm>` | Hash algorithm: `md5`, `sha1`, `sha256`, `sha512` (Default: `sha256`). |
| `-v <hash>` | Compare computed hash against this expected hash string. |
| `-c` | Read checksums and paths from the checksum file and verify them. |

**Examples:**
```bash
ir hash file.txt                         # Compute SHA-256 hash of file.txt
ir hash -a md5 file.txt                  # Compute MD5 hash of file.txt
ir hash -a sha256 -v <HASH> file.txt     # Verify file.txt matches expected SHA-256 hash
ir hash -c checksums.txt                 # Verify all files listed in checksums.txt
```

---

### 📊 `ps`
Displays information about active processes (PID, CPU time, Memory working set, and command/name).

**Usage:**
```bash
ir ps [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-s <field>` | Sort by `pid`, `name`, `cpu`, or `mem` (Default: `pid`). |
| `-f <filter>` | Filter processes by name (case-insensitive). |
| `-n <limit>` | Limit output to the first N processes. |

**Examples:**
```bash
ir ps                                    # List all processes sorted by PID
ir ps -s cpu                             # List all processes sorted by CPU time
ir ps -f chrome -s mem                   # List all chrome processes sorted by memory usage
```

---

### 🛑 `kill`
Terminates one or more processes by PID or process name.

**Usage:**
```bash
ir kill [switches] <TARGET>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<TARGET>` | The process ID (PID) or process name to terminate. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-f` | Force termination (send SIGKILL on Unix, or forcefully terminate on Windows). |
| `-a` | Kill all processes matching the process name (required if name matches multiple processes). |

**Examples:**
```bash
ir kill 1234                             # Terminate process with PID 1234
ir kill chrome -a                        # Terminate all processes named 'chrome'
ir kill -f 5678                          # Forcefully terminate process with PID 5678
```

---

### 🌐 `fetch`
Downloads content from a URL or queries an HTTP/HTTPS endpoint.

**Usage:**
```bash
ir fetch [switches] <URL>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<URL>` | The target HTTP or HTTPS URL. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-X <method>` | HTTP request method: `GET`, `POST`, `PUT`, `DELETE` etc. (Default: `GET`). |
| `-H <header>` | Custom request header in `'Name: Value'` format (can be specified multiple times). |
| `-d <data>` | Request body / POST data string. |
| `-o <file>` | Write response body to a file instead of standard output. |
| `-i` | Include response HTTP status line and headers in the output. |

**Examples:**
```bash
ir fetch https://api.ipify.org           # Fetch public IP address
ir fetch -i https://httpbin.org/get      # Fetch URL and print headers and body
ir fetch -X POST -d '{"id":1}' URL       # Send a POST request with JSON payload
```

---

### 🔍 `env`
Lists, searches, or formats environment variables.

**Usage:**
```bash
ir env [switches] [VARIABLE_NAME]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[VARIABLE_NAME]` | Optionally retrieve a single variable. PATH variables are auto-formatted line-by-line. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-s <query>` | Filter results to variables containing the search query. |

**Examples:**
```bash
ir env                                   # List all variables sorted alphabetically
ir env -s path                           # Search for variables containing 'path'
ir env PATH                              # Format and print PATH directories line-by-line
```

---

### 💾 `hex`
Displays a hexadecimal dump of a file alongside its ASCII translation.

**Usage:**
```bash
ir hex [switches] <PATH>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<PATH>` | The file to display. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-n <bytes>` | Limit display to the first N bytes of the file. |
| `-c <cols>` | Number of columns of bytes to display (Default: 16). |

**Examples:**
```bash
ir hex file.bin                          # Hex dump of file.bin
ir hex -n 128 file.bin                   # Dump only the first 128 bytes
ir hex -c 8 file.bin                     # Display dump in 8 columns
```

---

### 📡 `ping`
Sends ICMP Echo requests to verify connectivity to a network host.

**Usage:**
```bash
ir ping [switches] <HOST>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<HOST>` | The hostname or IP address to ping. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-c <count>` | Number of requests to send (Default: 4). |
| `-t <ms>` | Timeout in milliseconds to wait for each reply (Default: 1000). |

**Examples:**
```bash
ir ping google.com                       # Ping google.com 4 times
ir ping -c 10 127.0.0.1                  # Ping localhost 10 times
```

---

### 🔑 `base64`
Encodes or decodes text or files using Base64 format.

**Usage:**
```bash
ir base64 [switches] [PATH]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[PATH]` | Optionally read from a file. If omitted, reads from standard input. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-d` | Decode instead of encode. |
| `-u` | Use URL-safe alphabet (replaces `+` and `/` with `-` and `_`). |
| `-n` | Do not append padding characters (`=`) (when encoding). |
| `-o <file>` | Write output directly to a file instead of standard output. |

**Examples:**
```bash
echo "hello" | ir base64                 # Encode 'hello' to Base64
ir base64 -d encoded.txt                 # Decode a base64 encoded file
ir base64 -u -n -o out.b64 input.bin     # URL-safe unpadded encoding to a file
```

---

### 🆔 `uuid`
Generates RFC-compliant UUIDv4 (random) and UUIDv7 (time-ordered) identifiers.

**Usage:**
```bash
ir uuid [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-v <version>` | UUID version to generate: `4` or `7` (Default: `4`). |
| `-c <count>` | Number of UUIDs to generate (Default: `1`). |
| `-u` | Output in uppercase letters. |
| `-n` | Remove hyphen separators (compact form). |

**Examples:**
```bash
ir uuid                                  # Generate one UUIDv4
ir uuid -v 7 -c 5                        # Generate five time-ordered UUIDv7s
ir uuid -n -u                            # Generate uppercase compact UUIDv4
```

---

### 🌐 `ip`
Displays local network adapter configuration and queries public IP details.

**Usage:**
```bash
ir ip [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-p` | Query public IP, location, and ISP details using public geolocator APIs. |
| `-a` | Display all local network interfaces (including inactive/disconnected ones). |

**Examples:**
```bash
ir ip                                    # List all active local network adapters
ir ip -a                                 # List all local network adapters
ir ip -p                                 # Query public IP and location details
```

---

### 🗣️ `echo`
Prints text to standard output or redirects/appends to a file.

**Usage:**
```bash
ir echo [switches] [TEXT] [> / >> FILE]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-n` | Do not print the trailing newline. |
| `-e` | Enable interpretation of backslash escapes (e.g. `\n`, `\t`, `\r`, `\\`, `\xHH`). |

**Redirections:**
| Operator | Description |
| :--- | :--- |
| `>` | Write output to the specified file (overwrites existing content). Creates the file if it does not exist. |
| `>>` | Append output to the specified file. Creates the file if it does not exist. |

**Examples:**
```bash
ir echo hello world                      # Print 'hello world'
ir echo -e "line1\nline2\x41"            # Print multiline text with parsed escapes
ir echo "some text" '>' out.txt          # Write 'some text' to out.txt
ir echo "more text" '>>' out.txt         # Append 'more text' to out.txt
```

---

### 📍 `which`
Locates a command in `PATH`.

**Usage:**
```bash
ir which [switches] <COMMAND>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<COMMAND>` | The command name to locate. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-a`, `--all` | Print all matching commands in `PATH` order. |

> [!NOTE]
> On Windows, `PATHEXT` is used to resolve executable extensions.

**Examples:**
```bash
ir which rustc                              # Locate rustc
ir which -a python                          # Print all python matches
```

---

## 📖 Documentation

On Linux, a `man` page is available in the `docs/` directory. To install it:

```sh
sudo cp docs/ir.1 /usr/local/share/man/man1/
gzip /usr/local/share/man/man1/ir.1
```

Then view it with:
```sh
man ir
```

---

## 🧪 Testing

The project includes a set of integration tests located in the `/tests` directory. These tests are written as shell scripts (`.sh` for Linux, `.ps1` for Windows) and are organized into subdirectories for each action.

Each test script is self-contained. It will:
1. **Build** the `ir` executable.
2. **Create** a temporary environment (files and folders).
3. **Run** the specific `ir` command being tested.
4. **Verify** that the outcome is correct.
5. **Clean up** the temporary environment.

To run a test, navigate to the appropriate directory and execute the script.

### On Linux:
```sh
cd tests/rename
./01_simple_file_rename.sh
```

### On Windows (PowerShell):
```powershell
cd tests/rename
.\01_simple_file_rename.ps1
```
