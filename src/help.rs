pub fn print_general_help() {
    println!("ir - a cross-platform file system utility");
    println!("\nUSAGE:");
    println!("    ir <ACTION> [OPTIONS]");
    println!("\nACTIONS:");
    println!("    list      Lists files and directories with detailed information.");
    println!("    rename    Renames a file or folder.");
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
    println!("    ir rename [SWITCHES] <SOURCE> <DESTINATION>");
    println!("\nDESCRIPTION:");
    println!("    Renames a file or folder from SOURCE to DESTINATION.");
    println!("\nSWITCHES:");
    println!("    -f, --force          Overwrites the destination if it already exists.");
    println!("    -i, --interactive    Prompts for confirmation before renaming.");
    println!("        --force-links    Allows the renaming of symbolic links themselves.");
    println!("\nRULES:");
    println!("    - The '-f' and '-i' switches cannot be used together.");
    println!("    - By default, the command will not overwrite an existing file or folder.");
    println!("    - By default, the command will not rename a symbolic link.");
}
