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
    println!("    encode    Encodes text or files into base64, hex, url, base32, or rot13.");
    println!("    decode    Decodes base64, hex, url, base32, or rot13 text or files.");
    println!("    json      Pretty-prints, minifies, validates, or queries JSON data.");
    println!("    plot      Plots numerical data in the terminal using ASCII graphics.");
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
    println!("    sort      Sorts lines from text files or standard input.");
    println!("    wc        Counts lines, words, characters, and bytes.");
    println!("    ln        Creates hard links or symbolic links.");
    println!("    chmod     Changes file permissions.");
    println!("    pmon      Displays a live graphical process monitor.");
    println!("    watch     Runs a command periodically, showing its output fullscreen.");
    println!("    nettop    Displays a live graphical network traffic monitor.");
    println!("    dua       Launches an interactive disk usage analyzer.");
    println!("    browse    Launches an interactive terminal file browser.");
    println!("    edit      Opens a file in the inline terminal text editor.");
    println!("    scrape    Downloads files from a URL matching given extension(s).");
    println!("    help      Prints general help or help for a specific action.");
    println!("\nALIASES:");
    println!("    ls        Alias for 'list'");
    println!("    touch     Alias for 'create'");
    println!("    tar       Alias for 'archive'");
    println!("    mv        Alias for 'move'");
    println!("    cp        Alias for 'copy'");
    println!("    rm        Alias for 'remove'");
    println!("    ff        Alias for 'fastfetch'");
    println!("    ptop      Alias for 'pmon'");
    println!("    smon      Alias for 'monitor'");
    println!("    ntop      Alias for 'nettop'");
    println!("    ncdu      Alias for 'dua'");
    println!("    fm        Alias for 'browse'");
    println!("    ed        Alias for 'edit'");
    println!("    dl        Alias for 'scrape'");
    println!("\nRun 'ir help <ACTION>' for more information on a specific action.");
}

pub fn print_list_help() {
    println!("ir-list");
    println!("\nUSAGE:");
    println!("    ir list [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Lists files and directories with detailed information.");
    println!("    Reparse points/symbolic links display target paths in '-> TARGET' notation.");
    println!("\nSWITCHES:");
    println!("    -a        Shows all files, including hidden ones.");
    println!("    -s        Sorts the output by file size, from largest to smallest.");
    println!("    -t        Sorts the output by modification time, from newest to oldest.");
    println!("    -f        Lists only files (excludes directories).");
    println!("    -l        Lists only directories/folders (excludes files).");
    println!("    -h, --human, --human-readable");
    println!("              Displays file sizes using KiB, MiB, GiB suffixes (IEC standard).");
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
    println!("    -s, --squeeze-blank      Collapse consecutive empty lines into a single empty line.");
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
    println!("    -A, --after-context <N>        Print N lines of trailing context after matching lines.");
    println!("    -B, --before-context <N>       Print N lines of leading context before matching lines.");
    println!("    -C, --context <N>              Print N lines of leading and trailing context.");
    println!("\nEXAMPLES:");
    println!("    ir grep 'error' file.txt                   Search for 'error' in a file");
    println!("    dir | ir grep 'README'                     Search piped output from dir command");
    println!("    ir list | ir grep -i '.txt'                Pipe from another ir command");
    println!("    ir grep -n 'warning' app.log               Show line numbers with matches");
    println!("    ir grep -c 'TODO' src/main.rs              Count matching lines");
    println!("    ir grep -C 3 'panic' src/main.rs           Show matches with 3 lines of context");
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
    println!("    -min-size, --min-size <SIZE>");
    println!("                        Match files at least this large. Suffixes K, M, G supported.");
    println!("    -max-size, --max-size <SIZE>");
    println!("                        Match files at most this large. Suffixes K, M, G supported.");
    println!("    -newer, --newer <FILE>");
    println!("                        Match files modified more recently than the modification time of FILE.");
    println!("    -older, --older <FILE>");
    println!("                        Match files modified less recently than the modification time of FILE.");
    println!("\nEXAMPLES:");
    println!("    ir find . -name '*.rs'                    Find Rust files under the current directory");
    println!("    ir find src -type d                       Find directories under src");
    println!("    ir find . -maxdepth 1 -type f             Find files directly under the current directory");
    println!("    echo src | ir find -name '*.rs'           Search paths supplied through stdin");
    println!("    ir find . -type f --min-size 10M          Find files larger than 10MB");
    println!("    ir find . -type f --newer README.md       Find files modified after README.md");
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
    println!("    -p, --progress                  Show download progress bar (only with --output).");
    println!("        --timeout <SECS>            Set request timeout in seconds.");
    println!("        --no-follow-redirects       Do not follow redirects.");
    println!("\nEXAMPLES:");
    println!("    ir fetch https://httpbin.org/get          Fetch URL content");
    println!("    ir fetch -i https://httpbin.org/get       Fetch URL and print status and headers");
    println!("    ir fetch -X POST -d 'name=value' URL      Send a POST request with data");
    println!("    ir fetch -o file.zip --progress URL       Download file showing progress");
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

pub fn print_encode_help() {
    println!("ir-encode");
    println!("\nUSAGE:");
    println!("    ir encode [SWITCHES] [PATH]");
    println!("\nDESCRIPTION:");
    println!("    Encodes text or files into various formats.");
    println!("\nARGUMENTS:");
    println!("    [PATH]          Optionally read from a file. If omitted, reads from standard input.");
    println!("\nSWITCHES:");
    println!("    -f, --format <fmt>  Encoding format: base64, base64url, hex, url, base32, rot13 [Default: base64]");
    println!("    -o, --output <file> Write output to a file instead of standard output");
    println!("    -n              Do not append padding characters (=) (for base64, base64url, base32)");
    println!("    --upper         Output hex in uppercase (for hex)");
    println!("    --separator <c> Insert character/string between hex bytes (for hex)");
    println!("    --all           Percent-encode all characters (for url format)");
    println!("\nEXAMPLES:");
    println!("    echo 'hello' | ir encode -f hex          Encode text to lowercase hex");
    println!("    ir encode -f base32 -n input.bin         Base32 encode a binary file without padding");
    println!("    echo 'hello' | ir encode -f url --all    Percent-encode all chars in 'hello'");
    println!("    echo 'hello' | ir encode -f hex --separator ':' --upper");
}

pub fn print_decode_help() {
    println!("ir-decode");
    println!("\nUSAGE:");
    println!("    ir decode [SWITCHES] [PATH]");
    println!("\nDESCRIPTION:");
    println!("    Decodes encoded text or files into original form.");
    println!("\nARGUMENTS:");
    println!("    [PATH]          Optionally read from a file. If omitted, reads from standard input.");
    println!("\nSWITCHES:");
    println!("    -f, --format <fmt>  Decoding format: base64, base64url, hex, url, base32, rot13 [Default: base64]");
    println!("    -o, --output <file> Write output to a file instead of standard output");
    println!("    -n              No padding expected (for base64, base64url, base32)");
    println!("    --separator <c> Expected character/string between hex bytes to ignore (for hex)");
    println!("\nEXAMPLES:");
    println!("    echo '48656c6c6f' | ir decode -f hex     Decode hex back to 'Hello'");
    println!("    ir decode -f base32 -o out.bin file.b32  Decode a Base32 file to a binary file");
    println!("    echo 'hello%20world' | ir decode -f url  Decode url encoding to 'hello world'");
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

pub fn print_wc_help() {
    println!("ir-wc");
    println!("\nUSAGE:");
    println!("    ir wc [SWITCHES] [PATH...]");
    println!("\nDESCRIPTION:");
    println!("    Counts lines, words, characters, and bytes for files or standard input.");
    println!("\nARGUMENTS:");
    println!("    [PATH...] The paths of files to process. If omitted or '-', reads standard input.");
    println!("\nSWITCHES:");
    println!("    -l, --lines      Count newlines.");
    println!("    -w, --words      Count words.");
    println!("    -c, --bytes      Count bytes.");
    println!("    -m, --chars      Count characters.");
    println!("\nRULES:");
    println!("    - -c and -m are mutually exclusive size metric switches.");
    println!("\nEXAMPLES:");
    println!("    ir wc file.txt                           Count lines, words, and bytes of file.txt");
    println!("    ir wc -l file.txt                        Count only lines of file.txt");
    println!("    ir wc -lw file1.txt file2.txt            Count lines and words for both files");
    println!("    cat file.txt | ir wc                     Count lines, words, and bytes from stdin");
}

pub fn print_ln_help() {
    println!("ir-ln");
    println!("\nUSAGE:");
    println!("    ir ln [SWITCHES] <TARGET> <LINK_NAME>");
    println!("\nDESCRIPTION:");
    println!("    Creates a link pointing to the TARGET file or directory.");
    println!("\nARGUMENTS:");
    println!("    <TARGET>    The existing file or directory to link to.");
    println!("    <LINK_NAME> The name/path of the link to be created.");
    println!("\nSWITCHES:");
    println!("    -s, --symbolic   Create a symbolic (soft) link instead of a hard link.");
    println!("    -f, --force      Remove/overwrite existing destination file/link.");
    println!("\nEXAMPLES:");
    println!("    ir ln target.txt hardlink.txt            Create a hard link");
    println!("    ir ln -s target.txt symlink.txt          Create a symbolic link");
    println!("    ir ln -sf target.txt existing.txt        Forcefully overwrite existing link with new symlink");
}

pub fn print_chmod_help() {
    println!("ir-chmod");
    println!("\nUSAGE:");
    println!("    ir chmod [SWITCHES] <MODE> <PATH...>");
    println!("\nDESCRIPTION:");
    println!("    Changes file mode bits (permissions) of files or directories.");
    println!("\nARGUMENTS:");
    println!("    <MODE>    Octal mode (e.g. 755, 644, 444).");
    println!("    <PATH...> One or more paths to modify.");
    println!("\nSWITCHES:");
    println!("    -R, --recursive  Recursively apply mode changes to directories and their contents.");
    println!("\nWINDOWS SUPPORT:");
    println!("    - Octal modes containing owner write bit (e.g. 200, 600, 755) remove the read-only attribute.");
    println!("    - Octal modes without owner write bit (e.g. 400, 444, 555) set the read-only attribute.");
    println!("\nEXAMPLES:");
    println!("    ir chmod 755 script.sh                   Make file executable/writeable");
    println!("    ir chmod 444 document.txt                Make file read-only");
    println!("    ir chmod -R 755 src                      Recursively make src/ writeable");
}

pub fn print_pmon_help() {
    println!("ir-pmon");
    println!("\nUSAGE:");
    println!("    ir pmon [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Displays a live graphical process monitor in the terminal.");
    println!("\nSWITCHES:");
    println!("    -d, --delay <VAL>  Update delay (e.g. 1.5s, 500ms, default: 1s).");
    println!("\nCONTROLS (interactive):");
    println!("    q                  Quit");
    println!("    c                  Sort by CPU % (descending)");
    println!("    m                  Sort by Memory usage (descending)");
    println!("    n                  Sort by Process Name");
    println!("    p                  Sort by PID");
    println!("    k                  Kill a process by prompting for its PID");
}

pub fn print_watch_help() {
    println!("ir-watch");
    println!("\nUSAGE:");
    println!("    ir watch [SWITCHES] <COMMAND>");
    println!("\nDESCRIPTION:");
    println!("    Runs a command periodically, displaying its output fullscreen and optional diff-highlighting.");
    println!("\nSWITCHES:");
    println!("    -n, --interval <VAL>  Update interval (e.g. 1.5s, 500ms, default: 2s).");
    println!("    --diff                Highlight changes between consecutive runs in reverse video.");
    println!("\nCONTROLS (interactive):");
    println!("    q, Esc, Ctrl+C        Quit");
}

pub fn print_nettop_help() {
    println!("ir-nettop");
    println!("\nUSAGE:");
    println!("    ir nettop [SWITCHES]");
    println!("\nDESCRIPTION:");
    println!("    Displays a live graphical network traffic monitor in the terminal.");
    println!("\nSWITCHES:");
    println!("    -d, --delay <VAL>  Update delay (e.g. 1.5s, 500ms, default: 1s).");
    println!("\nCONTROLS (interactive):");
    println!("    q, Esc                Quit");
    println!("    i                     Cycle to next network interface");
}

pub fn print_dua_help() {
    println!("ir-dua");
    println!("\nUSAGE:");
    println!("    ir dua [PATH]");
    println!("\nDESCRIPTION:");
    println!("    Launches an interactive disk usage analyzer (TUI) for the specified path.");
    println!("\nCONTROLS (interactive):");
    println!("    q                     Quit");
    println!("    ↑/↓ (or j/k)          Navigate directory contents");
    println!("    Enter (or l)          Enter the selected directory");
    println!("    Backspace (or h)      Go up to the parent directory");
    println!("    d                     Delete the selected file or directory");
}

pub fn print_browse_help() {
    println!("ir-browse");
    println!("\nUSAGE:");
    println!("    ir browse [PATH]");
    println!("\nDESCRIPTION:");
    println!("    Launches an interactive terminal file browser (TUI) for the specified path.");
    println!("\nCONTROLS (interactive):");
    println!("    q                     Quit");
    println!("    ↑/↓ (or j/k)          Navigate files and folders");
    println!("    Enter (or l)          Enter the selected directory");
    println!("    Backspace (or h)      Go up to the parent directory");
    println!("    c                     Copy the selected file or directory");
    println!("    m                     Move the selected file or directory");
    println!("    r                     Rename the selected file or directory");
    println!("    d                     Delete the selected file or directory");
}

pub fn print_edit_help() {
    println!("ir-edit");
    println!("\nUSAGE:");
    println!("    ir edit <FILE>");
    println!("    ir ed   <FILE>      (alias)");
    println!("\nDESCRIPTION:");
    println!("    Opens a file in a minimalist inline terminal text editor.");
    println!("    If FILE does not exist, it is created on first save.");
    println!("    Line numbers are shown in the left gutter.");
    println!("    Available key bindings are displayed at the bottom of the screen.");
    println!("\nNAVIGATION:");
    println!("    Arrow keys            Move cursor");
    println!("    Ctrl+Left / Right     Jump by word");
    println!("    Home                  Smart home (first non-whitespace, then col 0)");
    println!("    End                   Jump to end of line");
    println!("    Ctrl+Home / End       Jump to start / end of file");
    println!("    Page Up / Page Down   Scroll one screenful");
    println!("\nEDITING:");
    println!("    Enter                 Insert new line (inherits indentation)");
    println!("    Backspace             Delete character before cursor");
    println!("    Delete                Delete character at cursor");
    println!("    Ctrl+Backspace        Delete word before cursor");
    println!("    Ctrl+Delete           Delete word after cursor");
    println!("    Tab                   Insert 4 spaces (or indent selected lines)");
    println!("    Shift+Tab             Dedent current / selected lines");
    println!("\nSELECTION:");
    println!("    Shift+Arrow           Extend selection");
    println!("    Ctrl+A                Select all");
    println!("    Ctrl+C                Copy selection (or current line if none)");
    println!("    Ctrl+X                Cut  selection (or current line if none)");
    println!("    Ctrl+V                Paste");
    println!("\nSEARCH & GOTO:");
    println!("    Ctrl+F                Open search bar (type to filter, ↑↓ prev/next)");
    println!("    Ctrl+G                Open go-to-line bar");
    println!("\nUNDO / REDO:");
    println!("    Ctrl+Z                Undo last change (up to 100 levels)");
    println!("    Ctrl+Y                Redo");
    println!("\nFILE:");
    println!("    Ctrl+S                Save the file");
    println!("    Ctrl+Q / Esc          Quit (prompts to save if unsaved changes exist)");
    println!("\nERROR HANDLING:");
    println!("    Passing a directory as FILE exits immediately with an error.");
    println!("    Binary files show a warning in the status bar.");
    println!("    Save errors (permissions, disk full, locked) appear in the status bar.");
}

pub fn print_scrape_help() {
    println!("ir-scrape");
    println!("\nUSAGE:");
    println!("    ir scrape <URL> --format <EXT>[,EXT,...] [OPTIONS]");
    println!("    ir dl     <URL> --format <EXT>[,EXT,...] [OPTIONS]   (alias)");
    println!("\nDESCRIPTION:");
    println!("    Visits a URL, finds all linked files whose extension matches --format,");
    println!("    and downloads them to the destination directory.");
    println!("    When --depth > 1 the scraper crawls linked HTML pages up to that depth.");
    println!("    A realistic browser User-Agent and matching headers are used by default.");
    println!("\nREQUIRED:");
    println!("    --format <EXT[,EXT,...]>  File extension(s) to download.");
    println!("                              Comma-separated list or repeat the flag.");
    println!("                              Format group aliases are also accepted:");
    println!("                                documents  pdf doc docx rtf odt ppt pptx xls xlsx");
    println!("                                images     jpg jpeg png gif svg webp ico bmp tiff");
    println!("                                data       json csv xml yaml yml toml");
    println!("                                web        html htm css js wasm");
    println!("                                archives   zip tar gz bz2 xz 7z rar tgz");
    println!("                                text       txt md rst log conf cfg ini");
    println!("                                audio      mp3 flac wav ogg aac m4a opus wma");
    println!("                                video      mp4 webm avi mkv mov flv wmv m4v ts");
    println!("\nDESTINATION:");
    println!("    --dest <DIR>              Output directory (default: ./output).");
    println!("                              Created automatically if it does not exist.");
    println!("                              Must be writable.");
    println!("\nCONTENT CONTROL:");
    println!("    --include-video           Allow video file downloads (blocked by default).");
    println!("    --include-audio           Allow audio file downloads (blocked by default).");
    println!("    --no-images               Skip image files even if the extension matches.");
    println!("\nCRAWL / SAFETY LIMITS:");
    println!("    --depth <N>               Max crawl depth (default: 1 = start page only).");
    println!("    --max-pages <N>           Max HTML pages fetched during crawl (default: 10).");
    println!("    --max-size <N>[K|M|G]     Max total downloaded data (default: 50M).");
    println!("                              Accepts suffixes K, M, G or a raw byte count.");
    println!("    --max-links <N>           Max links followed per page (default: 100).");
    println!("    --timeout <SECS>          Per-request timeout in seconds (default: 30).");
    println!("    --rate-limit <MS>         Sleep N milliseconds between HTTP requests (default: 0).");
    println!("\nBEHAVIOUR:");
    println!("    --same-domain             Only follow links within the start URL's domain.");
    println!("    --ignore-robots           Ignore robots.txt restrictions.");
    println!("    --user-agent <UA>         Override the User-Agent header.");
    println!("    --dry-run                 Print what would be downloaded; write nothing.");
    println!("    --overwrite               Overwrite existing files (default: rename).");
    println!("    --verbose / -v            Print per-URL decisions.");
    println!("\nERROR HANDLING:");
    println!("    Non-http/https URLs are rejected immediately.");
    println!("    A missing or non-writable --dest exits with an error before fetching.");
    println!("    Per-file network errors are printed and skipped; the crawl continues.");
    println!("    Files that would exceed --max-size are skipped, not truncated.");
    println!("    robots.txt is fetched from the start domain; unreachable → unrestricted.");
    println!("\nEXAMPLES:");
    println!("    ir scrape https://example.com --format pdf");
    println!("    ir scrape https://example.com --format pdf,docx --dest ~/downloads");
    println!("    ir scrape https://example.com --format documents --depth 2");
    println!("    ir scrape https://example.com --format images --max-size 100M --same-domain");
    println!("    ir scrape https://example.com --format mp3 --include-audio --dry-run");
    println!("    ir dl     https://example.com --format data --verbose");
}

pub fn print_sort_help() {
    println!("ir-sort");
    println!("\nUSAGE:");
    println!("    ir sort [SWITCHES] [FILE...]");
    println!("\nDESCRIPTION:");
    println!("    Sorts lines from FILEs or standard input and prints the result.");
    println!("\nARGUMENTS:");
    println!("    [FILE...]  Optional path(s) to files to read and sort. If omitted, reads from stdin.");
    println!("\nSWITCHES:");
    println!("    -r, --reverse         Reverse the result of comparisons.");
    println!("    -n, --numeric         Compare according to string numerical value (supports decimals).");
    println!("    -u, --unique          Output only the first of an equal run (remove duplicates).");
    println!("    -f, --ignore-case     Fold lower and upper case characters together.");
    println!("    -c, --check           Check whether input is sorted; do not sort. Exit code 1 if unsorted.");
    println!("        --field <N>       Sort by Nth whitespace-delimited field (1-based index).");
    println!("        --separator <C>   Use character C as field separator instead of whitespace.");
    println!("\nEXAMPLES:");
    println!("    ir sort lines.txt                          Sort lines alphabetically");
    println!("    ir sort -n -r scores.txt                   Sort numbers in descending order");
    println!("    ir sort -u users.log                       Sort lines and remove duplicate entries");
    println!("    ir sort --field 2 --separator ',' data.csv  Sort CSV file by the second field");
}

pub fn print_json_help() {
    println!("ir-json");
    println!("\nUSAGE:");
    println!("    ir json [SWITCHES] [PATH]");
    println!("\nDESCRIPTION:");
    println!("    Pretty-prints, minifies, validates, or queries JSON data from PATH or stdin.");
    println!("\nARGUMENTS:");
    println!("    [PATH]                  Optional path to JSON file. If omitted, reads from standard input.");
    println!("\nSWITCHES:");
    println!("    -q, --query <selector>  Evaluate selector path query (e.g. '.dependencies.chrono' or '.users[0].name').");
    println!("    -m                      Minify JSON (one line, compact spacing).");
    println!("    -p                      Pretty-print JSON (default behavior).");
    println!("    --indent <spaces>       Number of indentation spaces (default: 4).");
    println!("    -o, --output <file>     Write output directly to a file instead of standard output.");
    println!("\nEXAMPLES:");
    println!("    ir json package.json                            Format and pretty-print JSON file");
    println!("    echo '{{\"a\":1}}' | ir json -m                     Minify JSON string: {{\"a\":1}}");
    println!("    ir json package.json -q .dependencies           Query dependencies object");
    println!("    ir json data.json -q '.users[0].name'           Query nested field in array");
    println!("    ir json input.json --indent 2 -o formatted.json Format and save with 2-space indents");
}

pub fn print_plot_help() {
    println!("ir-plot");
    println!("\nUSAGE:");
    println!("    ir plot [SWITCHES] [PATH]");
    println!("\nDESCRIPTION:");
    println!("    Plots numerical data in the terminal using ASCII graphics.");
    println!("\nARGUMENTS:");
    println!("    [PATH]                  Optional path to numeric data file. If omitted, reads from standard input.");
    println!("\nSWITCHES:");
    println!("    -t, --type <type>       Chart type: 'line', 'bar', 'scatter' (default: 'line').");
    println!("        --title <text>      Add a title text to the top of the plot.");
    println!("    -w, --width <cols>      Plot width in terminal characters (default: 60).");
    println!("    -g, --height <lines>    Plot height in terminal lines (default: 15).");
    println!("        --source <format>   Source format: 'txt', 'csv', 'json' (default: 'txt').");
    println!("        --csv-col <index>   0-based column index to plot (default: 0, for csv format).");
    println!("        --csv-headers       Skip the first row in CSV format (header row).");
    println!("\nEXAMPLES:");
    println!("    echo \"1 5 3 8 4 10\" | ir plot                   Line chart from standard input");
    println!("    ir plot --type bar --title \"Monthly Sales\" data  Bar chart from file data");
    println!("    ir plot --source csv --csv-col 1 data.csv       Plot the second column from a CSV file");
    println!("    ir plot --source json list.json                 Plot a flat JSON array of numbers");
}

