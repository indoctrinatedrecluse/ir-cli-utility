pub fn print_general_help() {
    println!("ir - a cross-platform file system utility");
    println!("\nUSAGE:");
    println!("    ir <ACTION> [OPTIONS]");
    println!("\nACTIONS:");
    println!("    list      Lists files and directories with detailed information.");
    println!("    rename    Renames a file or folder.");
    println!("    copy      Copies files and folders.");
    println!("    remove    Removes files and folders.");
    println!("    create    Creates files and folders.");
    println!("    move      Moves files and folders.");
    println!("    archive   Creates or extracts archives.");
    println!("    cat       Prints file contents.");
    println!("    grep      Searches for patterns in files or stdin.");
    println!("    find      Finds files and directories by name, type, depth, or emptiness.");
    println!("    diff      Compares two text files.");
    println!("    search    Recursively searches file contents under one or more paths.");
    println!("    which     Locates a command in PATH.");
    println!("    tree      Displays a directory tree representation of the filesystem.");
    println!("    du        Estimates file space usage.");
    println!("    fastfetch Displays system information and a fancy logo.");
    println!("    monitor   Invokes the bundled term-sys-monitor utility.");
    println!("    hash      Generates or verifies file checksums.");
    println!("    ps        Displays information about active processes.");
    println!("    kill      Terminates one or more processes.");
    println!("    fetch     Downloads a URL or queries an HTTP endpoint.");
    println!("    env       Views, searches, or formats environment variables.");
    println!("    hex       Displays a hexadecimal dump of a file.");
    println!("    ping      Sends ICMP Echo requests to a network host.");
    println!("    base64    Encodes or decodes data using Base64.");
    println!("    uuid      Generates UUIDv4 and UUIDv7 identifiers.");
    println!("    ip        Displays local network adapter and public IP information.");
    println!("    echo      Prints text, parses escapes, and redirects to files.");
    println!("    clip      Copies standard input to clipboard or prints clipboard contents.");
    println!("    math      Evaluates a mathematical expression.");
    println!("    sleep     Suspends execution for a specified duration.");
    println!("    time      Measures and displays command execution duration.");
    println!("    dns       Queries DNS records for a given host.");
    println!("    path      Views, adds, or removes directories from system PATH.");
    println!("    df        Estimates file space usage of all mounted file systems or drives.");
    println!("    whoami    Displays the current user name and domain.");
    println!("    sockets   Lists active TCP and UDP sockets with owning process.");
    println!("    help      Prints general help or help for a specific action.");
    println!("\nRun 'ir help <ACTION>' for more information on a specific action.");
}

pub fn print_list_help() {
    println!("ir-list");
    println!("\nUSAGE:");
    println!("    ir list [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Lists files and directories with detailed information.");
    println!("\nSWITCHES:");
    println!("    -a        Shows all files, including hidden ones.");
    println!("    -s        Sorts the output by file size, from largest to smallest.");
    println!("    -t        Sorts the output by modification time, from newest to oldest.");
    println!("    -f        Lists only files (excludes directories).");
    println!("    -l        Lists only directories/folders (excludes files).");
    println!("    --filter <ext> Filters by file extension.");
}

pub fn print_rename_help() {
    println!("ir-rename");
    println!("\nUSAGE:");
    println!("    ir rename [SWITCHES] <SOURCE_PATH> <NEW_NAME>");
    println!("\nDESCRIPTION:");
    println!("    Renames a file or folder at SOURCE_PATH to NEW_NAME in the same directory.");
    println!("\nARGUMENTS:");
    println!("    <SOURCE_PATH>    The full or relative path to the file/folder to rename.");
    println!("    <NEW_NAME>       The new name for the file/folder (not a path).");
    println!("\nSWITCHES:");
    println!("    -f, --force          Overwrites the destination if it already exists.");
    println!("    -i, --interactive    Prompts for confirmation before renaming.");
    println!("        --force-links    Allows the renaming of symbolic links themselves.");
}

pub fn print_copy_help() {
    println!("ir-copy");
    println!("\nUSAGE:");
    println!("    ir copy [SWITCHES] <SOURCE> <DESTINATION>");
    println!("\nDESCRIPTION:");
    println!("    Copies a file or folder from SOURCE to the DESTINATION directory.");
    println!("\nARGUMENTS:");
    println!("    <SOURCE>         The path to the file or folder to copy.");
    println!("    <DESTINATION>    The path to the destination folder.");
    println!("\nSWITCHES:");
    println!("        --force          Overwrites destination files if they already exist.");
    println!("    -r                   (Default) Copies directories and their contents recursively.");
    println!("    -f                   Copies only files from the source, not subdirectories.");
    println!("    -l                   Copies only subdirectories from the source, not files.");
    println!("        --rename <NAME>  When copying a single file, saves it under a new name.");
    println!("\nRULES:");
    println!("    - The '-r' switch cannot be used with '-f' or '-l'.");
}

pub fn print_remove_help() {
    println!("ir-remove");
    println!("\nUSAGE:");
    println!("    ir remove [SWITCHES] <PATH...>");
    println!("\nDESCRIPTION:");
    println!("    Removes the specified files or folders.");
    println!("\nARGUMENTS:");
    println!("    <PATH...>    One or more paths to the files or folders to remove.");
    println!("\nSWITCHES:");
    println!("    -f, --force          Force removes files and directories without prompting.");
    println!("    -i, --interactive    Prompts for confirmation before every removal.");
    println!("    -t, --trash          Moves items to the system trash instead of permanently deleting.");
    println!("    -v, --verbose        Prints the name of each file as it is being removed.");
    println!("    -y                   Skips the confirmation prompt for non-empty folders.");
}

pub fn print_create_help() {
    println!("ir-create");
    println!("\nUSAGE:");
    println!("    ir create [SWITCHES] <PATH...>");
    println!("\nDESCRIPTION:");
    println!("    Creates files or folders at the specified paths.");
    println!("\nARGUMENTS:");
    println!("    <PATH...>    One or more paths for the items to be created.");
    println!("\nSWITCHES:");
    println!("        --create-file    Forces the creation of a file, even if it has no extension.");
    println!("    -p, --force-subdirs  Creates parent directories as needed.");
}

pub fn print_move_help() {
    println!("ir-move");
    println!("\nUSAGE:");
    println!("    ir move [SWITCHES] <SOURCE> <DESTINATION>");
    println!("\nDESCRIPTION:");
    println!("    Moves a file or folder from SOURCE to DESTINATION.");
    println!("\nARGUMENTS:");
    println!("    <SOURCE>         The path to the file or folder to move.");
    println!("    <DESTINATION>    The path to the destination folder or new file path.");
    println!("\nSWITCHES:");
    println!("        --force          Overwrites destination files if they already exist.");
    println!("        --rename <NAME>  When moving a single file, saves it under a new name.");
}

pub fn print_archive_help() {
    println!("ir-archive");
    println!("\nUSAGE:");
    println!("    ir archive [SWITCHES] <PATH>");
    println!("\nDESCRIPTION:");
    println!("    Creates, extracts, or tests an archive.");
    println!("\nARGUMENTS:");
    println!("    <PATH>    The path to the source file/folder or the archive to be processed.");
    println!("\nSWITCHES:");
    println!("        --dest <PATH>      Specify a destination path for the output.");
    println!("        --arc              (Default) Creates an archive from the source path.");
    println!("        --unarc            Extracts the contents of the archive specified in <PATH>.");
    println!("        --test             Tests the integrity of the specified archive.");
    println!("        --format <FORMAT>  Specifies the archive format (e.g., zip, tar.gz).");
    println!("        --force            Overwrites the destination archive if it already exists.");
    println!("        --verbose          Prints the name of each file as it is being processed.");
}

pub fn print_cat_help() {
    println!("ir-cat");
    println!("\nUSAGE:");
    println!("    ir cat [SWITCHES] <PATH>");
    println!("\nDESCRIPTION:");
    println!("    Prints file contents to standard output.");
    println!("\nARGUMENTS:");
    println!("    <PATH>    The path to the file to print.");
    println!("\nSWITCHES:");
    println!("    -n, --line-numbers       Prefix each output line with its source line number.");
    println!("        --head <N>           Prints the first N lines.");
    println!("        --tail <N>           Prints the last N lines.");
    println!("        --range <START:END>  Prints a 1-based inclusive line range.");
    println!("        --binary             Prints a hexadecimal preview of the file bytes.");
    println!("        --encoding <ENC>     Decodes text as utf-8, utf-16, or ascii.");
    println!("\nRULES:");
    println!("    - --head, --tail, and --range cannot be used together.");
    println!("    - --binary cannot be used with text formatting switches.");
}

pub fn print_grep_help() {
    println!("ir-grep");
    println!("\nUSAGE:");
    println!("    ir grep [SWITCHES] <PATTERN> [FILE...]");
    println!("\nDESCRIPTION:");
    println!("    Searches for lines matching a pattern in files or stdin (for piping).");
    println!("    If no files are specified, reads from standard input.");
    println!("\nARGUMENTS:");
    println!("    <PATTERN>  The pattern to search for.");
    println!("    [FILE...]  Optional file paths to search. If omitted, reads from stdin.");
    println!("\nSWITCHES:");
    println!("    -i, --ignore-case              Perform case-insensitive matching.");
    println!("    -n, --line-number              Prefix each output line with its line number.");
    println!("    -c, --count                    Count matching lines instead of displaying them.");
    println!("    -l, --files-with-matches       Print file names with matches only (no content).");
    println!("    -v, --invert-match             Select lines that do NOT match the pattern.");
    println!("    -x, --line-regexp              Match the entire line only.");
    println!("    -F, --fixed-strings            Treat pattern as a literal string, not regex.");
    println!("    -E, --extended-regexp          Use extended regular expression syntax.");
    println!("\nEXAMPLES:");
    println!("    ir grep 'error' file.txt                   Search for 'error' in a file");
    println!("    dir | ir grep 'README'                     Search piped output from dir command");
    println!("    ir list | ir grep -i '.txt'                Pipe from another ir command");
    println!("    ir grep -n 'warning' app.log               Show line numbers with matches");
    println!("    ir grep -c 'TODO' src/main.rs              Count matching lines");
}

pub fn print_find_help() {
    println!("ir-find");
    println!("\nUSAGE:");
    println!("    ir find [PATH...] [EXPRESSION]");
    println!("\nDESCRIPTION:");
    println!("    Finds files and directories under one or more paths.");
    println!("    If no paths are specified, searches the current directory.");
    println!("    If paths are piped through stdin and no paths are specified, searches those paths.");
    println!("\nARGUMENTS:");
    println!("    [PATH...]  Optional root paths to search. Defaults to the current directory.");
    println!("\nEXPRESSIONS:");
    println!("    -name <PATTERN>     Match a file or directory name using '*' and '?' wildcards.");
    println!("    -iname <PATTERN>    Like -name, but case-insensitive.");
    println!("    -type f             Match files only.");
    println!("    -type d             Match directories only.");
    println!("    -maxdepth <N>       Descend at most N levels below each root.");
    println!("    -mindepth <N>       Do not print entries shallower than N levels below each root.");
    println!("    -empty              Match empty files and empty directories.");
    println!("\nEXAMPLES:");
    println!("    ir find . -name '*.rs'                    Find Rust files under the current directory");
    println!("    ir find src -type d                       Find directories under src");
    println!("    ir find . -maxdepth 1 -type f             Find files directly under the current directory");
    println!("    echo src | ir find -name '*.rs'           Search paths supplied through stdin");
}

pub fn print_diff_help() {
    println!("ir-diff");
    println!("\nUSAGE:");
    println!("    ir diff [SWITCHES] <LEFT_FILE> <RIGHT_FILE>");
    println!("\nDESCRIPTION:");
    println!("    Compares two text files and prints their line differences.");
    println!("\nARGUMENTS:");
    println!("    <LEFT_FILE>   The first file to compare.");
    println!("    <RIGHT_FILE>  The second file to compare.");
    println!("\nSWITCHES:");
    println!("    -q, --brief        Report only whether the files differ.");
    println!("    -i, --ignore-case  Ignore ASCII case differences.");
    println!("    -u, --unified      Print unified-style output.");
    println!("\nEXAMPLES:");
    println!("    ir diff old.txt new.txt                  Compare two files");
    println!("    ir diff -u old.txt new.txt               Show unified-style output");
    println!("    ir diff -q old.txt new.txt               Only report whether files differ");
}

pub fn print_search_help() {
    println!("ir-search");
    println!("\nUSAGE:");
    println!("    ir search [SWITCHES] <PHRASE> [PATH...]");
    println!("\nDESCRIPTION:");
    println!("    Recursively searches file contents under one or more paths.");
    println!("    If no paths are specified, searches the current directory.");
    println!("    If paths are piped through stdin and no paths are specified, searches those paths.");
    println!("    Common binary, executable, archive, and document extensions are skipped by default.");
    println!("\nARGUMENTS:");
    println!("    <PHRASE>   The literal phrase to search for.");
    println!("    [PATH...]  Optional root paths to search. Defaults to the current directory.");
    println!("\nSWITCHES:");
    println!("    -i, --ignore-case          Perform case-insensitive matching.");
    println!("    -n, --line-number          Prefix matches with line numbers. Enabled by default.");
    println!("        --no-line-number       Do not print line numbers.");
    println!("    -l, --files-with-matches   Print file names with matches only.");
    println!("    -c, --count                Count matching lines per file.");
    println!("    -name <PATTERN>            Search only file names matching '*' and '?' wildcards.");
    println!("    -iname <PATTERN>           Like -name, but case-insensitive.");
    println!("    -maxdepth <N>              Descend at most N levels below each root.");
    println!("    -mindepth <N>              Do not search files shallower than N levels below each root.");
    println!("        --include <EXT>        Search only files with this extension. Can be repeated.");
    println!("        --exclude <EXT>        Skip files with this extension. Can be repeated.");
    println!("        --all                  Include normally skipped file extensions.");
    println!("\nEXAMPLES:");
    println!("    ir search TODO src                     Search src recursively");
    println!("    ir search -i \"error code\" .           Case-insensitive phrase search");
    println!("    ir search TODO . --include rs          Search only Rust files");
    println!("    echo src | ir search TODO              Search paths supplied through stdin");
}

pub fn print_which_help() {
    println!("ir-which");
    println!("\nUSAGE:");
    println!("    ir which [SWITCHES] <COMMAND>");
    println!("\nDESCRIPTION:");
    println!("    Locates a command by searching the directories listed in PATH.");
    println!("    On Windows, PATHEXT is used to resolve executable extensions.");
    println!("\nARGUMENTS:");
    println!("    <COMMAND>  The command name to locate.");
    println!("\nSWITCHES:");
    println!("    -a, --all  Print all matching commands in PATH order.");
    println!("\nEXAMPLES:");
    println!("    ir which rustc                         Locate rustc");
    println!("    ir which -a python                     Print all python matches");
}

pub fn print_tree_help() {
    println!("ir-tree");
    println!("\nUSAGE:");
    println!("    ir tree [SWITCHES] [PATH]");
    println!("\nDESCRIPTION:");
    println!("    Displays a directory tree representation of the filesystem.");
    println!("\nARGUMENTS:");
    println!("    [PATH]    The root path of the directory tree. Defaults to the current directory.");
    println!("\nSWITCHES:");
    println!("    -a        Shows all files, including hidden ones.");
    println!("    -d        List directories only.");
    println!("    -L <depth> Max display depth of the directory tree.");
    println!("    -f        Print the full path prefix for each file.");
    println!("    -i        Makes tree not print the indentation lines.");
    println!("    -s        Print the size of each file in bytes.");
    println!("    -h        Print the size in a more human-readable format.");
    println!("    -p        Print file permissions.");
    println!("        --noreport Omits printing of the file and directory report at the end.");
    println!("\nEXAMPLES:");
    println!("    ir tree                                  Show the tree structure of the current directory");
    println!("    ir tree -L 2 src                         Show the src directory tree up to depth 2");
    println!("    ir tree -adps -h                         Show all files, permissions, sizes human-readably, including hidden files");
}

pub fn print_du_help() {
    println!("ir-du");
    println!("\nUSAGE:");
    println!("    ir du [SWITCHES] [PATH...]");
    println!("\nDESCRIPTION:");
    println!("    Estimates file space usage recursively.");
    println!("\nARGUMENTS:");
    println!("    [PATH...]  One or more paths to estimate. Defaults to the current directory.");
    println!("\nSWITCHES:");
    println!("    -a        Write counts for all files, not just directories.");
    println!("    -c        Produce a grand total.");
    println!("    -h        Print sizes in human-readable format.");
    println!("    -s        Display only a total for each argument (equivalent to -d 0).");
    println!("    -d <depth>, --max-depth <depth>  Print the total for a directory only if it is at or below this depth.");
    println!("    -k        Print sizes in kilobytes (1024-byte blocks) [Default].");
    println!("    -m        Print sizes in megabytes (1024*1024-byte blocks).");
    println!("\nRULES:");
    println!("    - -h, -k, and -m are mutually exclusive size formatting switches.");
    println!("    - -s (summarize) and -d (max-depth > 0) cannot be combined.");
    println!("\nEXAMPLES:");
    println!("    ir du                                    Show disk usage of all directories");
    println!("    ir du -sh *                              Summarize disk usage of all items in human-readable format");
    println!("    ir du -ah -d 1                           Show human-readable usage of all files up to depth 1");
}

pub fn print_fastfetch_help() {
    println!("ir-fastfetch");
    println!("\nUSAGE:");
    println!("    ir fastfetch");
    println!("\nDESCRIPTION:");
    println!("    Displays system information and a fancy logo.");
    println!("\nEXAMPLES:");
    println!("    ir fastfetch                             Show system info");
}

pub fn print_monitor_help() {
    println!("ir-monitor");
    println!("\nUSAGE:");
    println!("    ir monitor");
    println!("\nDESCRIPTION:");
    println!("    Spawns the bundled term-sys-monitor utility in a separate shell window.");
    println!("\nEXAMPLES:");
    println!("    ir monitor                               Launch system monitor");
}

pub fn print_hash_help() {
    println!("ir-hash");
    println!("\nUSAGE:");
    println!("    ir hash [SWITCHES] <PATH>");
    println!("\nDESCRIPTION:");
    println!("    Generates or verifies cryptographic file checksums.");
    println!("\nARGUMENTS:");
    println!("    <PATH>    The file to hash, or the checksum file to verify.");
    println!("\nSWITCHES:");
    println!("    -a <algorithm>, --algorithm <algorithm>  Hash algorithm: md5, sha1, sha256, sha512 [Default: sha256]");
    println!("    -v <hash>, --verify <hash>               Compare computed hash against this expected hash string");
    println!("    -c        Read checksums and paths from the checksum file and verify them");
    println!("\nRULES:");
    println!("    - -v (verify) and -c (checksum file) are mutually exclusive switches.");
    println!("\nEXAMPLES:");
    println!("    ir hash file.txt                         Compute SHA-256 hash of file.txt");
    println!("    ir hash -a md5 file.txt                  Compute MD5 hash of file.txt");
    println!("    ir hash -a sha256 -v <HASH> file.txt     Verify file.txt matches expected SHA-256 hash");
    println!("    ir hash -c checksums.txt                 Verify all files listed in checksums.txt");
}

pub fn print_ps_help() {
    println!("ir-ps");
    println!("\nUSAGE:");
    println!("    ir ps [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Displays information about active processes.");
    println!("\nSWITCHES:");
    println!("    -s <field>, --sort <field>  Sort by 'pid', 'name', 'cpu', or 'mem' [Default: pid]");
    println!("    -f <filter>, --filter <filter> Filter processes by name (case-insensitive)");
    println!("    -n <limit>, --limit <limit>   Limit output to the first N processes");
    println!("\nEXAMPLES:");
    println!("    ir ps                                    List all processes sorted by PID");
    println!("    ir ps -s cpu                             List all processes sorted by CPU time");
    println!("    ir ps -f chrome -s mem                   List all chrome processes sorted by memory usage");
}

pub fn print_kill_help() {
    println!("ir-kill");
    println!("\nUSAGE:");
    println!("    ir kill [SWITCHES] <TARGET>");
    println!("\nDESCRIPTION:");
    println!("    Terminates one or more processes.");
    println!("\nARGUMENTS:");
    println!("    <TARGET>  The process ID (PID) or process name to terminate.");
    println!("\nSWITCHES:");
    println!("    -f        Force termination (send SIGKILL on Unix, or forcefully terminate on Windows)");
    println!("    -a        Kill all processes matching the process name (required if name matches multiple processes)");
    println!("\nEXAMPLES:");
    println!("    ir kill 1234                             Terminate process with PID 1234");
    println!("    ir kill chrome -a                        Terminate all processes named 'chrome'");
    println!("    ir kill -f 5678                          Forcefully terminate process with PID 5678");
}

pub fn print_fetch_help() {
    println!("ir-fetch");
    println!("\nUSAGE:");
    println!("    ir fetch [SWITCHES] <URL>");
    println!("\nDESCRIPTION:");
    println!("    Downloads content from a URL or queries an HTTP endpoint.");
    println!("\nARGUMENTS:");
    println!("    <URL>     The target HTTP/HTTPS URL.");
    println!("\nSWITCHES:");
    println!("    -X <method>, --method <method>  HTTP request method (GET, POST, etc.) [Default: GET]");
    println!("    -H <header>, --header <header>  Custom header in 'Name: Value' format (can be specified multiple times)");
    println!("    -d <data>, --data <data>        Request body string (useful for POST/PUT)");
    println!("    -o <file>, --output <file>      Write response body to file instead of stdout");
    println!("    -i                              Include HTTP status line and response headers in output");
    println!("\nEXAMPLES:");
    println!("    ir fetch https://httpbin.org/get          Fetch URL content");
    println!("    ir fetch -i https://httpbin.org/get       Fetch URL and print status and headers");
    println!("    ir fetch -X POST -d 'name=value' URL      Send a POST request with data");
}

pub fn print_env_help() {
    println!("ir-env");
    println!("\nUSAGE:");
    println!("    ir env [SWITCHES] [VARIABLE_NAME]");
    println!("\nDESCRIPTION:");
    println!("    Lists, searches, or formats environment variables.");
    println!("\nARGUMENTS:");
    println!("    [VARIABLE_NAME]  Optionally retrieve a single variable. PATH variables are auto-formatted line-by-line.");
    println!("\nSWITCHES:");
    println!("    -s <query>, --search <query>  Search for variables whose name or value contains the query");
    println!("\nEXAMPLES:");
    println!("    ir env                                   List all environment variables sorted alphabetically");
    println!("    ir env -s path                           Search for environment variables containing 'path'");
    println!("    ir env PATH                              Retrieve PATH variable, formatted cleanly line-by-line");
}

pub fn print_hex_help() {
    println!("ir-hex");
    println!("\nUSAGE:");
    println!("    ir hex [SWITCHES] <PATH>");
    println!("\nDESCRIPTION:");
    println!("    Displays a hexadecimal dump of the target file alongside ASCII representation.");
    println!("\nARGUMENTS:");
    println!("    <PATH>    The file to dump.");
    println!("\nSWITCHES:");
    println!("    -n <bytes>, --limit <bytes>  Limit display to first N bytes");
    println!("    -c <cols>, --cols <cols>    Number of columns of bytes to display [Default: 16]");
    println!("\nEXAMPLES:");
    println!("    ir hex file.bin                          Hex dump of file.bin");
    println!("    ir hex -n 128 file.bin                   Hex dump of the first 128 bytes of file.bin");
    println!("    ir hex -c 8 file.bin                     Hex dump displaying 8 columns of bytes per row");
}

pub fn print_ping_help() {
    println!("ir-ping");
    println!("\nUSAGE:");
    println!("    ir ping [SWITCHES] <HOST>");
    println!("\nDESCRIPTION:");
    println!("    Sends ICMP Echo requests to verify connection to a network host.");
    println!("\nARGUMENTS:");
    println!("    <HOST>    The hostname or IP address to ping.");
    println!("\nSWITCHES:");
    println!("    -c <count>, --count <count>     Number of requests to send [Default: 4]");
    println!("    -t <timeout>, --timeout <ms>    Timeout in milliseconds to wait for each reply [Default: 1000]");
    println!("\nEXAMPLES:");
    println!("    ir ping google.com                       Ping google.com with 4 requests");
    println!("    ir ping -c 10 127.0.0.1                  Ping localhost 10 times");
    println!("    ir ping -t 500 google.com                Ping google.com with a 500ms timeout limit");
}

pub fn print_base64_help() {
    println!("ir-base64");
    println!("\nUSAGE:");
    println!("    ir base64 [SWITCHES] [PATH]");
    println!("\nDESCRIPTION:");
    println!("    Encodes or decodes text or files using Base64.");
    println!("\nARGUMENTS:");
    println!("    [PATH]    Optionally read from a file. If omitted, reads from standard input.");
    println!("\nSWITCHES:");
    println!("    -d        Decode instead of encode");
    println!("    -u        Use URL-safe alphabet (replace + and / with - and _)");
    println!("    -n        Do not append padding characters (=) (when encoding)");
    println!("    -o <file> Write output to a file instead of standard output");
    println!("\nEXAMPLES:");
    println!("    echo 'hello' | ir base64                 Encode text 'hello'");
    println!("    ir base64 -d encoded.txt                 Decode base64 file");
    println!("    ir base64 -u -n -o out.b64 input.bin     URL-safe unpadded encoding to a file");
}

pub fn print_uuid_help() {
    println!("ir-uuid");
    println!("\nUSAGE:");
    println!("    ir uuid [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Generates UUIDv4 (random) and UUIDv7 (time-ordered) identifiers.");
    println!("\nSWITCHES:");
    println!("    -v <version>  UUID version to generate: 4 or 7 [Default: 4]");
    println!("    -c <count>    Number of UUIDs to generate [Default: 1]");
    println!("    -u            Output in uppercase letters");
    println!("    -n            Remove hyphen separators (compact form)");
    println!("\nEXAMPLES:");
    println!("    ir uuid                                  Generate one UUIDv4");
    println!("    ir uuid -v 7 -c 5                        Generate five time-ordered UUIDv7s");
    println!("    ir uuid -n -u                            Generate uppercase UUIDv4 without hyphens");
}

pub fn print_ip_help() {
    println!("ir-ip");
    println!("\nUSAGE:");
    println!("    ir ip [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Displays local network adapter and public IP information.");
    println!("\nSWITCHES:");
    println!("    -p            Query public IP, location, and ISP details");
    println!("    -a            Display all network adapters, including inactive/down ones");
    println!("\nEXAMPLES:");
    println!("    ir ip                                    List all active local network adapters");
    println!("    ir ip -a                                 List all network adapters (even disconnected)");
    println!("    ir ip -p                                 Query public IP and location information");
}

pub fn print_echo_help() {
    println!("ir-echo");
    println!("\nUSAGE:");
    println!("    ir echo [SWITCHES] [TEXT] [> / >> FILE]");
    println!("\nDESCRIPTION:");
    println!("    Prints text to standard output or redirects to a file.");
    println!("\nSWITCHES:");
    println!("    -n            Do not print the trailing newline");
    println!("    -e            Enable interpretation of backslash escapes:");
    println!("                  \\n  Newline");
    println!("                  \\t  Horizontal tab");
    println!("                  \\r  Carriage return");
    println!("                  \\\\  Backslash");
    println!("                  \\xHH Hexadecimal byte value (HH)");
    println!("\nREDIRECTIONS:");
    println!("    >  FILE       Write output to FILE (overwrites existing content)");
    println!("    >> FILE       Append output to FILE");
    println!("\nEXAMPLES:");
    println!("    ir echo hello world                      Print 'hello world'");
    println!("    ir echo -e 'line1\\nline2'                Print text with newline escape interpreted");
    println!("    ir echo 'some text' > out.txt            Write 'some text' to out.txt");
    println!("    ir echo 'more text' >> out.txt           Append 'more text' to out.txt");
}

pub fn print_clip_help() {
    println!("ir-clip");
    println!("\nUSAGE:");
    println!("    ir clip [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Copies standard input to clipboard or prints clipboard contents.");
    println!("    If stdin is redirected (piped), copies stdin contents to the clipboard.");
    println!("    Otherwise, prints the current clipboard text contents to standard output.");
    println!("\nSWITCHES:");
    println!("    -c, --clear   Clear the clipboard contents");
    println!("\nEXAMPLES:");
    println!("    echo 'hello' | ir clip                   Copy 'hello' to the clipboard");
    println!("    ir clip                                  Print current clipboard contents");
    println!("    ir clip -c                               Clear the clipboard");
}

pub fn print_math_help() {
    println!("ir-math");
    println!("\nUSAGE:");
    println!("    ir math <EXPRESSION>");
    println!("\nDESCRIPTION:");
    println!("    Evaluates a mathematical expression and prints the result.");
    println!("    Supports operators: +, -, *, /, % (modulo), ^ (power).");
    println!("    Supports parentheses () and negative numbers.");
    println!("\nARGUMENTS:");
    println!("    <EXPRESSION>  The mathematical expression to evaluate (should be quoted)");
    println!("\nEXAMPLES:");
    println!("    ir math '2 * (3.5 + 4)'                  Evaluate and print 15");
    println!("    ir math '10 % 3'                         Evaluate modulo (prints 1)");
    println!("    ir math '2^3^2'                          Right-associative power (prints 512)");
}

pub fn print_sleep_help() {
    println!("ir-sleep");
    println!("\nUSAGE:");
    println!("    ir sleep <DURATION>");
    println!("\nDESCRIPTION:");
    println!("    Suspends execution for a specified duration.");
    println!("    Supported suffixes: ms (milliseconds), s (seconds), m (minutes), h (hours).");
    println!("    If no suffix is provided, the value defaults to seconds.");
    println!("\nARGUMENTS:");
    println!("    <DURATION>    The delay time, e.g. 5, 2.5s, 500ms, 1m");
    println!("\nEXAMPLES:");
    println!("    ir sleep 5                               Sleep for 5 seconds");
    println!("    ir sleep 500ms                           Sleep for 500 milliseconds");
    println!("    ir sleep 1.5m                            Sleep for 1.5 minutes");
}

pub fn print_time_help() {
    println!("ir-time");
    println!("\nUSAGE:");
    println!("    ir time <COMMAND> [ARGS...]");
    println!("\nDESCRIPTION:");
    println!("    Measures and displays the execution time of a command.");
    println!("\nARGUMENTS:");
    println!("    <COMMAND>     The command to execute and measure");
    println!("    [ARGS...]     Arguments passed to the command");
    println!("\nEXAMPLES:");
    println!("    ir time cargo build                      Measure execution of 'cargo build'");
    println!("    ir time ir ping -c 5 google.com          Measure execution of 'ir ping'");
}

pub fn print_dns_help() {
    println!("ir-dns");
    println!("\nUSAGE:");
    println!("    ir dns <HOST>");
    println!("\nDESCRIPTION:");
    println!("    Queries DNS records (A, AAAA, CNAME, MX, TXT) for a host.");
    println!("\nARGUMENTS:");
    println!("    <HOST>        The hostname to resolve, e.g. google.com");
    println!("\nEXAMPLES:");
    println!("    ir dns google.com                        Resolve records for google.com");
}

pub fn print_path_help() {
    println!("ir-path");
    println!("\nUSAGE:");
    println!("    ir path [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Views, adds, or removes directories from user environment PATH.");
    println!("\nSWITCHES:");
    println!("    -a, --add <dir>      Add a directory permanently to user PATH");
    println!("    -r, --remove <dir>   Remove a directory permanently from user PATH");
    println!("\nEXAMPLES:");
    println!("    ir path                                  List directories in PATH");
    println!("    ir path -a C:\\bin                        Add C:\\bin to user PATH");
    println!("    ir path -r C:\\bin                        Remove C:\\bin from user PATH");
}

pub fn print_df_help() {
    println!("ir-df");
    println!("\nUSAGE:");
    println!("    ir df [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Estimates and prints disk space usage of all mounted file systems or drives.");
    println!("\nSWITCHES:");
    println!("    -a, --all            Include pseudo, duplicate, and virtual filesystems.");
    println!("    -h, --human-readable Print sizes in human-readable format.");
    println!("\nEXAMPLES:");
    println!("    ir df                                    Show disk usage for all physical volumes");
    println!("    ir df -h                                 Show human-readable disk usage");
    println!("    ir df -ah                                Show all mounted drives human-readable");
}

pub fn print_whoami_help() {
    println!("ir-whoami");
    println!("\nUSAGE:");
    println!("    ir whoami");
    println!("\nDESCRIPTION:");
    println!("    Displays the current user name and domain.");
    println!("\nEXAMPLES:");
    println!("    ir whoami                                Show current user and domain");
}

pub fn print_sockets_help() {
    println!("ir-sockets");
    println!("\nUSAGE:");
    println!("    ir sockets [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Lists active TCP and UDP sockets along with their owning processes (PID and name).");
    println!("\nSWITCHES:");
    println!("    -a, --all            Show both listening and connected/active sockets.");
    println!("    -t, --tcp            Show TCP sockets only.");
    println!("    -u, --udp            Show UDP sockets only.");
    println!("    -l, --listening      Show listening sockets only (implies TCP LISTEN and UDP).");
    println!("\nEXAMPLES:");
    println!("    ir sockets                               Show only active/connected TCP connections");
    println!("    ir sockets -a                            Show all connections (listening & active)");
    println!("    ir sockets -at                           Show all TCP connections");
    println!("    ir sockets -l                            Show listening sockets only");
}
