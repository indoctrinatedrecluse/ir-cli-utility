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
ir help watch
ir help nettop
ir help dua
ir help browse
ir help edit
```

---

### 📋 `list`
Lists files and directories with detailed information. Reparse points and symbolic links display target paths in `-> TARGET` format.

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
| `-h`, `--human` | Displays file sizes using KiB, MiB, GiB suffixes (IEC standard). |
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
Prints file contents to standard output, or redirects/appends to a file or system clipboard.

**Usage:**
```bash
ir cat [switches] <PATH> [> / >> REDIRECTION_TARGET]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<PATH>` | The path to the file to print. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-n`, `--line-numbers` | Prefix each output line with its source line number. |
| `-s`, `--squeeze-blank` | Collapse consecutive empty lines into a single empty line. |
| `--head <N>` | Prints the first `N` lines. |
| `--tail <N>` | Prints the last `N` lines. |
| `--range <START:END>` | Prints a 1-based inclusive line range. |
| `--binary` | Prints a hexadecimal preview of the file bytes. |
| `--encoding <ENC>` | Decodes text as `utf-8`, `utf-16`, or `ascii`. |

**Redirections:**
| Operator | Description |
| :--- | :--- |
| `>` | Write output to specified file or `"clip"` clipboard keyword (overwrites existing content). |
| `>>` | Append output to specified file or `"clip"` clipboard keyword. |

**Examples:**
```bash
ir cat file.txt                          # Print file.txt
ir cat -s file.txt                       # Squeeze consecutive empty lines
ir cat file.txt > out.txt                # Copy file.txt contents to out.txt
ir cat file.txt > clip                   # Copy file.txt contents to system clipboard
ir cat file.txt >> clip                  # Append file.txt contents to system clipboard
```


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
| `-A <N>`, `--after-context <N>` | Print `N` lines of trailing context after matching lines. |
| `-B <N>`, `--before-context <N>` | Print `N` lines of leading context before matching lines. |
| `-C <N>`, `--context <N>` | Print `N` lines of leading and trailing context. |

**Examples:**
```bash
ir grep 'error' file.txt                    # Search for 'error' in a file
dir | ir grep 'README'                      # Search piped output from dir command
ir list | ir grep -i '.txt'                 # Pipe from another ir command
ir grep -n 'warning' app.log                # Show line numbers with matches
ir grep -C 3 'panic' main.rs                # Show matches with 3 lines of context
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
| `-min-size <SIZE>` \| `--min-size <SIZE>` | Match files at least this large. Suffixes K, M, G supported. |
| `-max-size <SIZE>` \| `--max-size <SIZE>` | Match files at most this large. Suffixes K, M, G supported. |
| `-newer <FILE>` \| `--newer <FILE>` | Match files modified more recently than the modification time of FILE. |
| `-older <FILE>` \| `--older <FILE>` | Match files modified less recently than the modification time of FILE. |

**Examples:**
```bash
ir find . -name '*.rs'                     # Find Rust files under the current directory
ir find src -type d                        # Find directories under src
ir find . -maxdepth 1 -type f              # Find files directly under the current directory
echo src | ir find -name '*.rs'            # Search paths supplied through stdin
ir find . -type f --min-size 10M           # Find files larger than 10MB
ir find . -type f --newer README.md        # Find files modified after README.md
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

### 📊 `df`
Estimates and prints disk space usage of all mounted file systems or drives.

**Usage:**
```bash
ir df [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-a`, `--all` | Include pseudo, duplicate, and virtual filesystems. |
| `-h`, `--human-readable` | Print sizes in human-readable format. |

**Examples:**
```bash
ir df                                    # Show disk usage for all physical volumes
ir df -h                                 # Show human-readable disk usage
ir df -ah                                # Show all mounted drives human-readable
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
| `-p`, `--progress` | Show download progress bar (only with `-o`/`--output`). |
| `--timeout <SECS>` | Set request timeout in seconds. |
| `--no-follow-redirects` | Disable automatic redirect following. |

**Examples:**
```bash
ir fetch https://api.ipify.org           # Fetch public IP address
ir fetch -i https://httpbin.org/get      # Fetch URL and print headers and body
ir fetch -X POST -d '{"id":1}' URL       # Send a POST request with JSON payload
ir fetch -o file.zip --progress URL      # Download a file with progress bar
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

### 🔒 `encode`
Encodes text or files into standard formats (base64, base64url, hex, url, base32, rot13).

**Usage:**
```bash
ir encode [switches] [PATH]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[PATH]` | Optionally read from a file. If omitted, reads from standard input. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-f, --format <fmt>` | Format to encode to: `base64`, `base64url`, `hex`, `url`, `base32`, `rot13` [Default: `base64`]. |
| `-o, --output <file>` | Write output directly to a file instead of standard output. |
| `-n` | Do not append padding characters (`=`) (for `base64`, `base64url`, `base32`). |
| `--upper` | Output hex in uppercase (for `hex` format). |
| `--separator <sep>` | Insert character/string between hex bytes (for `hex` format). |
| `--all` | Percent-encode all characters, not just reserved/non-ascii ones (for `url` format). |

**Examples:**
```bash
echo "hello" | ir encode -f hex                  # Encode to hex: 68656c6c6f
ir encode -f base32 -n input.bin                 # Encode binary file to unpadded Base32
echo "hello" | ir encode -f url --all            # Percent-encode all characters: %68%65%6C%6C%6F
echo "hello" | ir encode -f hex --separator ":" --upper # Encode with custom separator: 68:65:6C:6C:6F
```

---

### 🔓 `decode`
Decodes encoded text or files back to their original form.

**Usage:**
```bash
ir decode [switches] [PATH]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[PATH]` | Optionally read from a file. If omitted, reads from standard input. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-f, --format <fmt>` | Format to decode from: `base64`, `base64url`, `hex`, `url`, `base32`, `rot13` [Default: `base64`]. |
| `-o, --output <file>` | Write output directly to a file instead of standard output. |
| `-n` | No padding expected (for `base64`, `base64url`, `base32`). |
| `--separator <sep>` | Expected separator character/string between hex bytes to ignore (for `hex` format). |

**Examples:**
```bash
echo "68656c6c6f" | ir decode -f hex            # Decode hex back to text
ir decode -f base32 -o out.bin file.b32          # Decode Base32 file to a binary file
echo "hello%20world%21" | ir decode -f url       # Decode percent-encoding to 'hello world!'
```

---

### 🗃️ `json`
Pretty-prints, minifies, validates, or queries JSON data.

**Usage:**
```bash
ir json [switches] [PATH]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[PATH]` | Optionally read from a file. If omitted, reads from standard input. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-q, --query <selector>` | Evaluate selector path query (e.g. `.dependencies.chrono` or `.users[0].name`). |
| `-m` | Minify JSON into a single line with compact spacing. |
| `-p` | Pretty-print JSON (default behavior). |
| `--indent <spaces>` | Number of indentation spaces (default: 4). |
| `-o, --output <file>` | Write output directly to a file instead of standard output. |

**Examples:**
```bash
ir json package.json                            # Format and pretty-print JSON file
echo '{"a":1}' | ir json -m                     # Minify JSON string: {"a":1}
ir json package.json -q .dependencies           # Query dependencies object
ir json data.json -q '.users[0].name'           # Query nested field in array
ir json input.json --indent 2 -o formatted.json # Format and save with 2-space indents
```

---

### 📊 `plot`
Plots numerical data in the terminal using ASCII/Unicode block graphics.

**Usage:**
```bash
ir plot [switches] [PATH]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[PATH]` | Optionally read from a file. If omitted, reads from standard input. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-t, --type <type>` | Chart type: `line`, `bar`, `scatter` [Default: `line`]. |
| `--title <text>` | Add a title to the top of the plot. |
| `-w, --width <cols>` | Plot width in terminal characters (default: auto-fit terminal width). |
| `-g, --height <lines>` | Plot height in terminal lines (default: auto-fit terminal height). |
| `--source <format>` | Source format: `txt`, `csv`, `json` [Default: `txt`]. |
| `--csv-col <index>` | 0-based column index to plot (default: 0, for CSV format). |
| `--csv-headers` | Skip the first row in CSV format (header row). |
| `--smooth` | Render high-resolution charts using Unicode Braille characters. |
| `--log` | Apply base-10 logarithmic scaling to data points. |
| `--json-key <path>` | JSON query path (e.g. `.info.value`) to extract numbers from an array of JSON objects. |
| `-H, --horizontal` | Render bar chart horizontally from left to right. |

**Examples:**
```bash
echo "1 5 3 8 4 10" | ir plot                   # Line chart from standard input
ir plot --type bar --title "Monthly Sales" data  # Bar chart from file data
ir plot --source csv --csv-col 1 data.csv       # Plot second column from CSV
ir plot --source json list.json                 # Plot a flat JSON array of numbers
ir plot --smooth data.txt                       # Render line chart with Unicode Braille
ir plot --source json --json-key .val list.json # Query and plot nested values from JSON array
ir plot -H data.txt                             # Render bar chart horizontally
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
| `>` | Write output to specified file or `"clip"` clipboard keyword (overwrites existing content). |
| `>>` | Append output to specified file or `"clip"` clipboard keyword. |

**Examples:**
```bash
ir echo hello world                      # Print 'hello world'
ir echo -e "line1\nline2\x41"            # Print multiline text with parsed escapes
ir echo "some text" '>' out.txt          # Write 'some text' to out.txt
ir echo "more text" '>>' out.txt         # Append 'more text' to out.txt
ir echo "copy to clipboard" '>' clip     # Copy text to system clipboard
ir echo "append to clipboard" '>>' clip  # Append text to system clipboard
```

---

### 📋 `clip`
Clipboard manager to copy standard input to the clipboard or print clipboard text. Also supports clearing.

**Usage:**
```bash
ir clip [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-c`, `--clear` | Empty the system clipboard. |

**Examples:**
```bash
echo "hello clipboard" | ir clip          # Copy text to clipboard
ir clip                                  # Print current clipboard content
ir clip -c                               # Clear the clipboard
```

---

### 🧮 `math`
Evaluates a mathematical expression and prints the result, or launches an interactive REPL calculator.

**Usage:**
```bash
ir math [EXPRESSION]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[EXPRESSION]` | Optional mathematical expression string to evaluate (e.g. `'sqrt(144) + sin(pi/2)'`). If omitted, starts interactive REPL. |

**REPL Commands:**
* `vars`: Lists all custom defined variables.
* `clear`: Clears all custom variables.
* `exit` / `quit`: Exits the interactive shell.

**Examples:**
```bash
ir math "2 * (3.5 + 4)"                  # Evaluate basic math (prints 15)
ir math "sqrt(144) + sin(pi/2)"          # Evaluate functions (prints 13)
ir math "x = 4.5 * 2"                    # Define variable 'x' (prints 9)
ir math                                  # Launches the interactive REPL calculator
```

---

### ⏰ `clock`
Launches a fullscreen visual digital clock, stopwatch, and countdown timer.

**Usage:**
```bash
ir clock [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-t, --timer <duration>` | Set initial countdown timer duration (e.g. `5m30s`, `10m`, `300`). Auto-starts in Timer mode. |
| `-m, --mode <mode>` | Initial mode: `clock`, `stopwatch`, or `timer` [Default: `clock`]. |

**Hotkeys:**
* `Tab` / `C` — Cycle modes (Clock → Stopwatch → Timer).
* `Space` — Toggle play/pause (Stopwatch / Timer).
* `Enter` / `L` — Record lap split time (Stopwatch).
* `R` — Reset stopwatch/timer.
* `1`, `2`, `3` — Direct mode navigation.
* `Esc` / `Q` — Quit.

**Examples:**
```bash
ir clock                                 # Open clock in default Clock mode
ir clock -m stopwatch                    # Open stopwatch directly
ir clock --timer 5m30s                   # Open and start countdown timer
```

---

### 📝 `text`
Formats and case-converts strings or files. If no file path is specified, reads from standard input.

**Usage:**
```bash
ir text [switches] [PATH]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-c, --case <format>` | Case format: `camel`, `snake`, `pascal`, `kebab`, `upper`, `lower`, `title`, `sentence`, `slug`. |
| `-w, --width <cols>` | Target column width for alignment and truncation (Default: `80`). |
| `--align <align>` | Align text: `left`, `right`, or `center`. |
| `--truncate` | Truncate lines exceeding `--width`. |
| `--ellipsis <str>` | Custom truncation ellipsis suffix (Default: `...`). |
| `--strip-ansi` | Remove terminal color/ANSI escape sequences. |
| `--strip-non-alphanumeric` | Strip symbols, keeping only letters, numbers, and whitespace. |
| `-o, --output <file>` | Write formatted output to a file instead of standard output. |

**Examples:**
```bash
echo "hello_world" | ir text -c title    # Prints "Hello World"
ir text --strip-ansi logs.txt            # Strip color escape codes from logs
ir text --align center -w 60 data.txt    # Center lines of a file within 60 characters
```

---

### 🌍 `globe`
Launches an interactive fullscreen 3D terminal world globe and flat map viewer.

**Usage:**
```bash
ir globe [switches] [PATH]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-m, --mode <mode>` | Initial view mode: `globe` (3D spherical) or `map` (2D flat) [Default: `globe`]. |
| `-c, --center <lat,lon>` | Initial center coordinates (e.g. `40.7,-74.0`). |
| `--day-night` | Enable UTC day/night terminator shading overlay. |

**Hotkeys:**
* `Arrows` — Pan or rotate the globe/map view.
* `Tab` / `M` — Toggle between 3D Globe and 2D Flat Map views.
* `D` — Toggle day/night terminator shading.
* `+` / `-` — Zoom in and out on the globe projection.
* `Esc` / `Q` — Quit the interactive screen.

**Examples:**
```bash
ir globe                                 # Start interactive globe at 0,0 center
ir globe -c 40.7,-74.0 --day-night       # Start globe centered at New York with day/night shading
ir globe ips.csv                         # Plot coordinates from CSV as blinking dots
```

---

### 💤 `sleep`
Suspends execution for a specified duration. Supports suffixes: `ms` (milliseconds), `s` (seconds), `m` (minutes), `h` (hours). If no suffix is provided, defaults to seconds.

**Usage:**
```bash
ir sleep <DURATION>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<DURATION>` | The delay duration string (e.g. `5`, `2.5s`, `500ms`, `1m`). |

**Examples:**
```bash
ir sleep 5                               # Sleep for 5 seconds
ir sleep 500ms                           # Sleep for 500 milliseconds
ir sleep 1.5m                            # Sleep for 1.5 minutes
```

---

### 🔌 `sockets`
Lists active TCP and UDP sockets along with their owning processes (PID and name).

**Usage:**
```bash
ir sockets [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-a`, `--all` | Show both listening and connected/active sockets (default shows only active/established connections). |
| `-t`, `--tcp` | Show TCP sockets only. |
| `-u`, `--udp` | Show UDP sockets only. |
| `-l`, `--listening` | Show listening sockets only (implies TCP LISTEN and UDP). |

**Examples:**
```bash
ir sockets                               # Show only active/connected TCP connections
ir sockets -a                            # Show all connections (listening & active)
ir sockets -at                           # Show all TCP connections
ir sockets -l                            # Show listening sockets only
```

---

### ⏱️ `time`
Measures and displays the exact wall-clock execution duration of a specified command. Passes inputs and outputs directly through to/from the process, and returns the process exit status code.

**Usage:**
```bash
ir time <COMMAND> [ARGS...]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<COMMAND>` | The command to run and measure. |
| `[ARGS...]` | Arguments to pass to the command. |

**Examples:**
```bash
ir time cargo build                      # Measure cargo build execution
ir time ir ping -c 5 google.com          # Measure ping command execution
```

---

### 🌐 `dns`
A self-contained native DNS query tool resolving IP addresses (A/AAAA), mail exchange (MX), text records (TXT), and canonical names (CNAME) of a host. Queries system active adapters DNS addresses, falling back to public servers.

**Usage:**
```bash
ir dns <HOST>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<HOST>` | The hostname to query records for (e.g. `google.com`). |

**Examples:**
```bash
ir dns google.com                        # Query and display DNS records
```

---

### 🛣️ `path`
Views, adds, or removes directories from user system PATH environment variables permanently. On Windows, modifies HKCU Registry and broadcasts environment changes. On Linux, modifies standard shell profiles (`.bashrc` / `.zshrc` / `.profile`).

**Usage:**
```bash
ir path [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-a`, `--add <dir>` | Add a directory permanently to user PATH environment. |
| `-r`, `--remove <dir>` | Remove a directory permanently from user PATH environment. |

**Examples:**
```bash
ir path                                  # List PATH directories
ir path -a C:\bin                        # Add directory to PATH
ir path -r C:\bin                        # Remove directory from PATH
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

### 👤 `whoami`
Displays the current user name and domain.

**Usage:**
```bash
ir whoami
```

**Examples:**
```bash
ir whoami                                # Show current user and domain
```

---

### 🧮 `wc`
Counts lines, words, characters, and bytes for files or standard input.

**Usage:**
```bash
ir wc [switches] [PATH...]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[PATH...]` | File paths to count. If omitted or `-`, reads standard input (`stdin`). |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-l`, `--lines` | Count newlines. |
| `-w`, `--words` | Count words. |
| `-c`, `--bytes` | Count bytes. |
| `-m`, `--chars` | Count characters (UTF-8). |

> [!NOTE]
> - `-c` (bytes) and `-m` (chars) are mutually exclusive switches.
> - If no switches are provided, the default is to count lines, words, and bytes (`-l`, `-w`, `-c`).
> - Switches can be concatenated (e.g. `-lw`).

**Examples:**
```bash
ir wc file.txt                           # Count lines, words, and bytes
ir wc -l file.txt                        # Count only lines
ir wc -lw file1.txt file2.txt            # Count lines and words for both files
cat file.txt | ir wc                     # Count lines, words, and bytes from stdin
```

---

### 🔗 `ln`
Creates a hard link or symbolic/soft link pointing to a target file or directory.

**Usage:**
```bash
ir ln [switches] <TARGET> <LINK_NAME>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<TARGET>` | The existing file or directory to link to. |
| `<LINK_NAME>` | The name/path of the link to be created. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-s`, `--symbolic` | Create a symbolic (soft) link instead of a hard link. |
| `-f`, `--force` | Forcefully remove/overwrite an existing destination file/link. |

> [!NOTE]
> - On Windows, symbolic link creation requires Developer Mode enabled or running from an elevated prompt.
> - Single-character switches can be concatenated (e.g. `-sf` or `-fs`).

**Examples:**
```bash
ir ln target.txt hardlink.txt            # Create a hard link
ir ln -s target.txt symlink.txt          # Create a symbolic link
ir ln -sf target.txt existing.txt        # Force overwrite with a new symlink
```

---

### 🔒 `chmod`
Changes file mode bits (permissions) of files or directories.

**Usage:**
```bash
ir chmod [switches] <MODE> <PATH...>
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `<MODE>` | Octal mode string (e.g. `755`, `644`, `444`). |
| `<PATH...>` | One or more file or directory paths to modify. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-R`, `--recursive` | Recursively apply mode changes to directories and their contents. |

> [!NOTE]
> **Windows Support:**
> - Octal modes containing the owner write bit (e.g. `200`, `600`, `755`) remove the read-only attribute.
> - Octal modes without the owner write bit (e.g. `400`, `444`, `555`) set the read-only attribute.

**Examples:**
```bash
ir chmod 755 script.sh                   # Make script executable/writeable
ir chmod 444 document.txt                # Make document read-only
ir chmod -R 755 src                      # Recursively make directory writeable
```

---

### 📊 `pmon`
Displays a live graphical process monitor with real-time statistics (CPU/Memory gauges) and sorting options.

**Usage:**
```bash
ir pmon [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-d`, `--delay <VAL>` | Update delay (e.g. `1.5s`, `500ms`, default: `1s`). |

**Interactive Controls:**
* `q`: Quit.
* `c`: Sort by CPU %.
* `m`: Sort by Memory (RSS) usage.
* `n`: Sort by Process Name.
* `p`: Sort by PID.
* `k`: Kill a process (prompts for PID in raw mode).

**Examples:**
```bash
ir pmon                                  # Launch process monitor with 1s refresh delay
ir pmon -d 500ms                         # Launch process monitor with 500ms refresh delay
```

---

### ⏱️ `watch`
Runs a command periodically, displaying its output fullscreen and optional diff-highlighting.

**Usage:**
```bash
ir watch [switches] <COMMAND>
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-n`, `--interval <VAL>` | Update interval (e.g. `1.5s`, `500ms`, default: `2s`). |
| `--diff` | Highlight changes between consecutive runs in reverse video. |

**Interactive Controls:**
* `q` / `Esc` / `Ctrl+C`: Quit the watch mode.

**Examples:**
```bash
ir watch "ir ff"                         # Watch system info update every 2s
ir watch -n 500ms --diff "ir sockets"    # Watch sockets every 500ms highlighting changes
```

---

### 🌐 `nettop`
Displays a live graphical network traffic monitor in the terminal with ASCII speed graphs.

**Usage:**
```bash
ir nettop [switches]
```

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-d`, `--delay <VAL>` | Update delay (e.g. `1.5s`, `500ms`, default: `1s`). |

**Interactive Controls:**
* `q` / `Esc`: Quit the network monitor.
* `i`: Cycle to next active network interface.

**Examples:**
```bash
ir nettop                                # Launch network monitor with 1s update delay
ir nettop -d 500ms                       # Launch network monitor with 500ms update delay
```

---

### 📊 `dua`
Launches an interactive disk usage analyzer (TUI) for the specified path, listing folders and files sorted by size with proportional ASCII bars.

**Usage:**
```bash
ir dua [PATH]
```

**Interactive Controls:**
* `q`: Quit.
* `↑` / `↓` (or `j` / `k`): Navigate directory contents.
* `Enter` (or `l`): Enter the selected directory.
* `Backspace` (or `h`): Go up to the parent directory.
* `d`: Delete the selected file or directory (prompts for confirmation).

**Examples:**
```bash
ir dua                                   # Scan and analyze current directory
ir dua /var/log                          # Scan and analyze /var/log directory
```

---

### 🗂️ `browse`
Launches an interactive dual-pane terminal file browser (TUI) for the specified path.

**Usage:**
```bash
ir browse [PATH]
```

**Interactive Controls:**
* `q`: Quit.
* `↑` / `↓` (or `j` / `k`): Navigate files and folders.
* `Enter` (or `l`): Enter the selected directory.
* `Backspace` (or `h`): Go up to the parent directory.
* `c`: Copy the selected file or directory (prompts for destination path).
* `m`: Move the selected file or directory (prompts for destination path).
* `r`: Rename the selected file or directory (prompts for new name).
* `d`: Delete the selected file or directory (prompts for confirmation).

**Examples:**
```bash
ir browse                                # Browse the current directory
ir browse /home/user/projects            # Browse a specific directory
```

---

### ✏️ `edit`
Opens a file in a minimalist inline terminal text editor. Features line numbers, undo/redo, incremental search, selection, copy/cut/paste, word navigation, auto-indent, and a persistent controls bar at the bottom of the screen. Binary files are detected and warned about before editing.

**Usage:**
```bash
ir edit <FILE>
ir ed   <FILE>   # alias
```

**Navigation:**
| Key | Action |
| :--- | :--- |
| `↑` / `↓` / `←` / `→` | Move cursor |
| `Ctrl+←` / `Ctrl+→` | Jump by word |
| `Home` | Smart home (first non-whitespace, then col 0) |
| `End` | Jump to end of line |
| `Ctrl+Home` / `Ctrl+End` | Jump to start / end of file |
| `Page Up` / `Page Down` | Scroll one screenful |

**Editing:**
| Key | Action |
| :--- | :--- |
| `Enter` | Insert new line (inherits current line's indentation) |
| `Backspace` | Delete character before cursor |
| `Delete` | Delete character at cursor |
| `Ctrl+Backspace` | Delete word before cursor |
| `Ctrl+Delete` | Delete word after cursor |
| `Tab` | Insert 4 spaces (or indent selected lines) |
| `Shift+Tab` | Dedent current / selected lines |

**Selection, Copy & Paste:**
| Key | Action |
| :--- | :--- |
| `Shift+Arrow` | Extend selection |
| `Ctrl+A` | Select all |
| `Ctrl+C` | Copy selection (or current line if nothing selected) |
| `Ctrl+X` | Cut selection (or current line if nothing selected) |
| `Ctrl+V` | Paste |

**Search & Navigation:**
| Key | Action |
| :--- | :--- |
| `Ctrl+F` | Open search bar — type to filter, `↑`/`↓` for prev/next match |
| `Ctrl+G` | Open go-to-line bar — type a number then `Enter` |

**Undo / Redo:**
| Key | Action |
| :--- | :--- |
| `Ctrl+Z` | Undo (up to 100 levels) |
| `Ctrl+Y` | Redo |

**File:**
| Key | Action |
| :--- | :--- |
| `Ctrl+S` | Save the file |
| `Ctrl+Q` / `Esc` | Quit (prompts to save if there are unsaved changes) |

**Error handling:**
* Passing a **directory** as the filename exits immediately with an error.
* Opening an **unreadable file** shows the OS error in the status bar.
* Saving to a **read-only, locked, or inaccessible** file shows the error in the status bar without crashing.
* Opening a **binary file** shows a warning in the status bar.

**Examples:**
```bash
ir edit notes.txt        # Open (or create) notes.txt
ir edit src/main.rs      # Edit a source file
ir ed README.md          # Alias form
```


### 🔀 `sort`
Sorts lines from text files or standard input and prints the result to standard output.

**Usage:**
```bash
ir sort [switches] [FILE...]
```

**Arguments:**
| Argument | Description |
| :--- | :--- |
| `[FILE...]` | Optional file paths to read and sort. If omitted, reads from standard input. |

**Switches:**
| Switch | Description |
| :--- | :--- |
| `-r`, `--reverse` | Reverse the result of comparisons (sort descending). |
| `-n`, `--numeric`, `--numeric-sort` | Compare according to string numerical value (supports decimals). |
| `-u`, `--unique` | Output only the first of an equal run (remove duplicates). |
| `-f`, `--ignore-case` | Fold lower and upper case characters together. |
| `-c`, `--check` | Check whether input is sorted; do not sort. Exit code 1 if unsorted. |
| `--field <N>` | Sort by Nth whitespace-delimited field (1-based index). |
| `--separator <C>` | Use character C as field separator instead of whitespace. |

**Examples:**
```bash
ir sort lines.txt                          # Sort lines alphabetically
ir sort -n -r scores.txt                   # Sort numbers in descending order
ir sort -u users.log                       # Sort lines and remove duplicates
ir sort --field 2 --separator ',' data.csv  # Sort CSV file by the second field
```

---

### 🌐 `scrape`
Visits a URL, finds all linked files whose extension matches `--format`, and downloads them to the destination directory. When `--depth > 1` the scraper crawls linked HTML pages breadth-first. A realistic browser User-Agent (Chrome/Firefox/Edge pool) and matching headers are sent automatically to avoid bot-detection.

**Usage:**
```bash
ir scrape <URL> --format <EXT>[,EXT,...] [OPTIONS]
ir dl     <URL> --format <EXT>[,EXT,...] [OPTIONS]   # alias
```

**Format groups** (aliases that expand to multiple extensions):
| Group | Expands to |
| :--- | :--- |
| `documents` | pdf doc docx rtf odt ppt pptx xls xlsx |
| `images` | jpg jpeg png gif svg webp ico bmp tiff |
| `data` | json csv xml yaml yml toml |
| `web` | html htm css js wasm |
| `archives` | zip tar gz bz2 xz 7z rar tgz |
| `text` | txt md rst log conf cfg ini |
| `audio` | mp3 flac wav ogg aac m4a opus wma |
| `video` | mp4 webm avi mkv mov flv wmv m4v ts |

**Required:**
| Switch | Description |
| :--- | :--- |
| `--format <EXT[,EXT,...]>` | File extension(s) to download. Comma-separated or repeated. |

**Destination:**
| Switch | Description |
| :--- | :--- |
| `--dest <DIR>` | Output directory (default: `./output`). Created if absent; must be writable. |

**Content control:**
| Switch | Description |
| :--- | :--- |
| `--include-video` | Allow video file downloads (blocked by default). |
| `--include-audio` | Allow audio file downloads (blocked by default). |
| `--no-images` | Skip image files even if the extension matches. |

**Crawl / safety limits:**
| Switch | Default | Description |
| :--- | :--- | :--- |
| `--depth <N>` | 1 | Max crawl depth (1 = start page only). |
| `--max-pages <N>` | 10 | Max HTML pages fetched during the crawl. |
| `--max-size <N>[K\|M\|G]` | 50M | Max total downloaded data; suffixes K, M, G supported. |
| `--max-links <N>` | 100 | Max links followed per page. |
| `--timeout <SECS>` | 30 | Per-request timeout in seconds. |
| `--rate-limit <MS>` | 0 | Sleep N milliseconds between HTTP requests. |


**Behaviour:**
| Switch | Description |
| :--- | :--- |
| `--same-domain` | Only follow links within the start URL's domain. |
| `--ignore-robots` | Ignore robots.txt restrictions. |
| `--user-agent <UA>` | Override the User-Agent header. |
| `--dry-run` | Print what would be downloaded without writing files. |
| `--overwrite` | Overwrite existing files (default: rename with counter). |
| `--verbose` / `-v` | Print per-URL decisions. |

**Error handling:**
* Non-`http://`/`https://` URLs are rejected before any request is made.
* A non-writable or missing `--dest` exits immediately with an error.
* Per-file network errors are printed and skipped; the crawl continues.
* Files that would exceed `--max-size` are **skipped**, never truncated.
* `robots.txt` is fetched from the start domain; if unreachable, all paths are assumed allowed.
* Video and audio downloads are **blocked by default** — use `--include-video` / `--include-audio` to allow them.

**Examples:**
```bash
ir scrape https://example.com --format pdf
ir scrape https://example.com --format pdf,docx --dest ~/downloads
ir scrape https://example.com --format documents --depth 2
ir scrape https://example.com --format images --max-size 100M --same-domain
ir scrape https://example.com --format mp3 --include-audio --dry-run
ir dl     https://example.com --format data --verbose
```

---

### 🔄 Command Aliases
For convenience and familiar muscle memory, several common commands are aliased in-binary to map directly to their counterparts:

| Alias | Target Action | Description |
| :--- | :--- | :--- |
| `ls` | `list` | Lists files and directories with detailed information. |
| `touch` | `create` | Creates empty files (and folders). |
| `tar` | `archive` | Creates or extracts archives. |
| `mv` | `move` | Moves files and folders. |
| `cp` | `copy` | Copies files and folders. |
| `rm` | `remove` | Removes files and folders. |
| `ff` | `fastfetch` | Displays system information and a fancy logo. |
| `ptop` | `pmon` | Displays a live graphical process monitor. |
| `smon` | `monitor` | Launches the default system monitor. |
| `ntop` | `nettop` | Displays a live graphical network traffic monitor. |
| `ncdu` | `dua` | Launches an interactive disk usage analyzer. |
| `fm` | `browse` | Launches an interactive terminal file browser. |
| `ed` | `edit` | Opens a file in the inline terminal text editor. |
| `dl` | `scrape` | Downloads files from a URL matching given extension(s). |

You can use these interchangeable pairings interchangeably (e.g. `ir ls` works exactly like `ir list`, and `ir help ls` shows the list help screen).

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
