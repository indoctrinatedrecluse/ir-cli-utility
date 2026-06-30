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

