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
