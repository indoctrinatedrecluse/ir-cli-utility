use std::env;
use ir_cli_utility::{help, ListOptions, RenameOptions};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help::print_general_help();
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "list" => {
            // ... (list argument parsing remains the same)
            let mut options = ListOptions::default();
            let mut list_args = args[2..].iter().peekable();
            let mut valid = true;

            while let Some(arg) = list_args.next() {
                if arg == "--filter" {
                    if options.filter.is_some() { eprintln!("Error: --filter can only be used once."); valid = false; break; }
                    if let Some(ext) = list_args.next() {
                        if ext.starts_with('-') { eprintln!("Error: --filter requires a file extension argument."); valid = false; break; }
                        options.filter = Some(ext.to_string());
                    } else { eprintln!("Error: --filter requires a file extension argument."); valid = false; break; }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'a' => options.show_all = true,
                            's' => options.sort_by_size = true,
                            't' => options.sort_by_time = true,
                            _ => { eprintln!("Error: Unknown switch '-{}'", char); valid = false; break; }
                        }
                    }
                    if !valid { break; }
                } else { eprintln!("Error: Invalid argument '{}'", arg); valid = false; break; }
            }

            if valid { ir_cli_utility::list(options); } else { help::print_list_help(); }
        }
        "rename" => {
            let mut options = RenameOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;

            for arg in &args[2..] {
                if arg == "--force" {
                    options.force = true;
                } else if arg == "--interactive" {
                    options.interactive = true;
                } else if arg == "--force-links" {
                    options.force_links = true;
                } else if arg.starts_with('-') {
                    for char in arg.chars().skip(1) {
                        match char {
                            'f' => options.force = true,
                            'i' => options.interactive = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for rename.", char);
                                valid = false;
                                break;
                            }
                        }
                    }
                    if !valid { break; }
                } else {
                    positionals.push(arg.clone());
                }
            }

            if options.force && options.interactive {
                eprintln!("Error: The '-f' (--force) and '-i' (--interactive) switches cannot be used together.");
                valid = false;
            }

            if positionals.len() != 2 {
                eprintln!("Error: 'rename' requires exactly two arguments: a source and a destination.");
                valid = false;
            }

            if valid {
                ir_cli_utility::rename(&positionals[0], &positionals[1], options);
            } else {
                help::print_rename_help();
            }
        }
        "help" => {
            if args.len() > 2 {
                match args[2].as_str() {
                    "list" => help::print_list_help(),
                    "rename" => help::print_rename_help(),
                    _ => {
                        eprintln!("Error: Unknown action '{}'", args[2]);
                        help::print_general_help();
                    }
                }
            } else {
                help::print_general_help();
            }
        }
        _ => {
            eprintln!("Error: Unknown action '{}'", action);
            help::print_general_help();
        }
    }
}
