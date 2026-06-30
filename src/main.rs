use std::env;
use ir_cli_utility::{help, ListOptions, RenameOptions, CopyOptions, RemoveOptions, CreateOptions, MoveOptions, ArchiveOptions, CatOptions, GrepOptions, FindOptions, FindItemType, DiffOptions, SearchOptions, WhichOptions, TreeOptions, DuOptions};

fn is_path(s: &str) -> bool {
    s.contains('/') || s.contains('\\')
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help::print_general_help();
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "list" => {
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
                            'f' => options.files_only = true,
                            'l' => options.folders_only = true,
                            _ => { eprintln!("Error: Unknown switch '-{}'", char); valid = false; break; }
                        }
                    }
                    if !valid { break; }
                } else { eprintln!("Error: Invalid argument '{}'", arg); valid = false; break; }
            }

            if options.files_only && options.folders_only {
                eprintln!("Error: '-f' (files only) and '-l' (folders only) cannot be used together.");
                valid = false;
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
                eprintln!("Error: 'rename' requires exactly two arguments: a source path and a new name.");
                valid = false;
            } else {
                if is_path(&positionals[1]) {
                    eprintln!("Error: The destination argument ('{}') must be a new name, not a path.", positionals[1]);
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::rename(&positionals[0], &positionals[1], options);
            } else {
                help::print_rename_help();
            }
        }
        "copy" => {
            let mut options = CopyOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "--force" {
                    options.force = true;
                } else if arg == "--rename" {
                    if let Some(new_name) = args_iter.next() {
                        if is_path(new_name) || new_name.starts_with('-') {
                            eprintln!("Error: --rename requires a valid filename, not a path or a switch.");
                            valid = false; break;
                        }
                        options.rename = Some(new_name.clone());
                    } else {
                        eprintln!("Error: --rename switch requires a filename argument.");
                        valid = false; break;
                    }
                } else if arg.starts_with('-') {
                    for char in arg.chars().skip(1) {
                        match char {
                            'r' => options.recursive = true,
                            'f' => options.files_only = true,
                            'l' => options.folders_only = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for copy.", char);
                                valid = false; break;
                            }
                        }
                    }
                    if !valid { break; }
                } else {
                    positionals.push(arg.clone());
                }
            }

            if options.recursive && (options.files_only || options.folders_only) {
                eprintln!("Error: The '-r' switch cannot be used with '-f' or '-l'.");
                valid = false;
            }
            if options.files_only && options.folders_only {
                options.recursive = true;
                options.files_only = false;
                options.folders_only = false;
            }
            if !options.files_only && !options.folders_only {
                options.recursive = true;
            }

            if positionals.len() != 2 {
                eprintln!("Error: 'copy' requires exactly two arguments: a source and a destination.");
                valid = false;
            }

            if valid {
                ir_cli_utility::copy(&positionals[0], &positionals[1], options);
            } else {
                help::print_copy_help();
            }
        }
        "remove" => {
            let mut options = RemoveOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;

            for arg in &args[2..] {
                if arg == "--force" {
                    options.force = true;
                } else if arg == "--trash" {
                    options.trash = true;
                } else if arg == "--interactive" {
                    options.interactive = true;
                } else if arg == "--verbose" {
                    options.verbose = true;
                } else if arg.starts_with('-') {
                    for char in arg.chars().skip(1) {
                        match char {
                            'f' => options.force = true,
                            't' => options.trash = true,
                            'i' => options.interactive = true,
                            'y' => options.yes = true,
                            'v' => options.verbose = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for remove.", char);
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

            if positionals.is_empty() {
                eprintln!("Error: 'remove' requires at least one path argument.");
                valid = false;
            }

            if options.force {
                options.interactive = false;
                options.yes = true;
            }

            if valid {
                for path in positionals {
                    ir_cli_utility::remove(&path, &options);
                }
            } else {
                help::print_remove_help();
            }
        }
        "create" => {
            let mut options = CreateOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;

            for arg in &args[2..] {
                if arg == "--create-file" {
                    options.create_file = true;
                } else if arg == "--force-subdirs" {
                    options.force_subdirs = true;
                } else if arg.starts_with('-') {
                    for char in arg.chars().skip(1) {
                        match char {
                            'p' => options.force_subdirs = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for create.", char);
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

            if positionals.is_empty() {
                eprintln!("Error: 'create' requires a path argument.");
                valid = false;
            }

            if valid {
                for path in positionals {
                    ir_cli_utility::create(&path, options.clone());
                }
            } else {
                help::print_create_help();
            }
        }
        "move" => {
            let mut options = MoveOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "--force" {
                    options.force = true;
                } else if arg == "--rename" {
                    if let Some(new_name) = args_iter.next() {
                        if is_path(new_name) || new_name.starts_with('-') {
                            eprintln!("Error: --rename requires a valid filename, not a path or a switch.");
                            valid = false; break;
                        }
                        options.rename = Some(new_name.clone());
                    } else {
                        eprintln!("Error: --rename switch requires a filename argument.");
                        valid = false; break;
                    }
                } else {
                    positionals.push(arg.clone());
                }
            }

            if positionals.len() != 2 {
                eprintln!("Error: 'move' requires exactly two arguments: a source and a destination.");
                valid = false;
            }

            if valid {
                ir_cli_utility::move_item(&positionals[0], &positionals[1], options);
            } else {
                help::print_move_help();
            }
        }
        "archive" => {
            let mut options = ArchiveOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "--dest" {
                    if let Some(dest) = args_iter.next() {
                        if !std::path::Path::new(dest).is_dir() {
                            eprintln!("Error: Destination path '{}' is not a valid directory.", dest);
                            valid = false; break;
                        }
                        options.dest = Some(dest.clone());
                    } else {
                        eprintln!("Error: --dest switch requires a path argument.");
                        valid = false; break;
                    }
                } else if arg == "--arc" {
                    options.arc = true;
                } else if arg == "--unarc" {
                    options.unarc = true;
                } else if arg == "--format" {
                    if let Some(format) = args_iter.next() {
                        options.format = Some(format.clone());
                    } else {
                        eprintln!("Error: --format switch requires a format string.");
                        valid = false; break;
                    }
                } else if arg == "--test" {
                    options.test = true;
                } else if arg == "--force" {
                    options.force = true;
                } else if arg == "--verbose" {
                    options.verbose = true;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if options.arc && options.unarc {
                eprintln!("Error: --arc and --unarc cannot be used together.");
                valid = false;
            }
            if !options.arc && !options.unarc {
                options.arc = true; // Default action
            }

            if positionals.len() != 1 {
                eprintln!("Error: 'archive' requires exactly one path argument.");
                valid = false;
            }

            if valid {
                ir_cli_utility::archive(&positionals[0], options);
            } else {
                help::print_archive_help();
            }
        }
        "cat" => {
            let mut options = CatOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-n" || arg == "--line-numbers" {
                    options.line_numbers = true;
                } else if arg == "--head" {
                    match args_iter.next().and_then(|value| value.parse::<usize>().ok()) {
                        Some(count) => options.head = Some(count),
                        None => {
                            eprintln!("Error: --head requires a non-negative line count.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--tail" {
                    match args_iter.next().and_then(|value| value.parse::<usize>().ok()) {
                        Some(count) => options.tail = Some(count),
                        None => {
                            eprintln!("Error: --tail requires a non-negative line count.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--range" {
                    if let Some(range) = args_iter.next() {
                        match parse_line_range(range) {
                            Some(parsed) => options.range = Some(parsed),
                            None => {
                                eprintln!("Error: --range requires a START:END line range.");
                                valid = false;
                                break;
                            }
                        }
                    } else {
                        eprintln!("Error: --range requires a START:END line range.");
                        valid = false;
                        break;
                    }
                } else if arg == "--binary" {
                    options.binary = true;
                } else if arg == "--encoding" {
                    if let Some(encoding) = args_iter.next() {
                        if encoding.starts_with('-') {
                            eprintln!("Error: --encoding requires an encoding name.");
                            valid = false;
                            break;
                        }
                        options.encoding = Some(encoding.clone());
                    } else {
                        eprintln!("Error: --encoding requires an encoding name.");
                        valid = false;
                        break;
                    }
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for cat.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            let selector_count = usize::from(options.head.is_some())
                + usize::from(options.tail.is_some())
                + usize::from(options.range.is_some());
            if selector_count > 1 {
                eprintln!("Error: --head, --tail, and --range cannot be used together.");
                valid = false;
            }

            if options.binary && (options.line_numbers || selector_count > 0 || options.encoding.is_some()) {
                eprintln!("Error: --binary cannot be used with text formatting switches.");
                valid = false;
            }

            if positionals.len() != 1 {
                eprintln!("Error: 'cat' requires exactly one path argument.");
                valid = false;
            }

            if valid {
                ir_cli_utility::cat(&positionals[0], options);
            } else {
                help::print_cat_help();
            }
        }
        "grep" => {
            let mut options = GrepOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-i" || arg == "--ignore-case" {
                    options.case_insensitive = true;
                } else if arg == "-n" || arg == "--line-number" {
                    options.line_numbers = true;
                } else if arg == "-c" || arg == "--count" {
                    options.count = true;
                } else if arg == "-l" || arg == "--files-with-matches" {
                    options.list = true;
                } else if arg == "-v" || arg == "--invert-match" {
                    options.invert_match = true;
                } else if arg == "-x" || arg == "--line-regexp" {
                    options.entire_line = true;
                } else if arg == "-F" || arg == "--fixed-strings" {
                    options.fixed_string = true;
                } else if arg == "-E" || arg == "--extended-regexp" {
                    options.extended_regex = true;
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for grep.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if positionals.is_empty() {
                eprintln!("Error: 'grep' requires at least a pattern argument.");
                valid = false;
            }

            if options.fixed_string && options.extended_regex {
                eprintln!("Error: '-F' (--fixed-strings) and '-E' (--extended-regexp) cannot be used together.");
                valid = false;
            }

            if valid {
                let pattern = positionals.remove(0);
                ir_cli_utility::grep(&pattern, positionals, options);
            } else {
                help::print_grep_help();
            }
        }
        "find" => {
            let mut options = FindOptions::default();
            let mut paths: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-name" {
                    match args_iter.next() {
                        Some(pattern) if !pattern.starts_with('-') => options.name = Some(pattern.clone()),
                        _ => {
                            eprintln!("Error: -name requires a pattern.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-iname" {
                    match args_iter.next() {
                        Some(pattern) if !pattern.starts_with('-') => options.case_insensitive_name = Some(pattern.clone()),
                        _ => {
                            eprintln!("Error: -iname requires a pattern.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-type" {
                    match args_iter.next().map(String::as_str) {
                        Some("f") => options.item_type = Some(FindItemType::File),
                        Some("d") => options.item_type = Some(FindItemType::Directory),
                        Some(other) => {
                            eprintln!("Error: Unsupported -type '{}'. Use 'f' or 'd'.", other);
                            valid = false;
                            break;
                        }
                        None => {
                            eprintln!("Error: -type requires 'f' or 'd'.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-maxdepth" {
                    match args_iter.next().and_then(|value| value.parse::<usize>().ok()) {
                        Some(depth) => options.max_depth = Some(depth),
                        None => {
                            eprintln!("Error: -maxdepth requires a non-negative number.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-mindepth" {
                    match args_iter.next().and_then(|value| value.parse::<usize>().ok()) {
                        Some(depth) => options.min_depth = depth,
                        None => {
                            eprintln!("Error: -mindepth requires a non-negative number.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-empty" {
                    options.empty = true;
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for find.", arg);
                    valid = false;
                    break;
                } else {
                    paths.push(arg.clone());
                }
            }

            if let Some(max_depth) = options.max_depth {
                if options.min_depth > max_depth {
                    eprintln!("Error: -mindepth cannot be greater than -maxdepth.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::find(paths, options);
            } else {
                help::print_find_help();
            }
        }
        "diff" => {
            let mut options = DiffOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;

            for arg in &args[2..] {
                if arg == "-q" || arg == "--brief" {
                    options.brief = true;
                } else if arg == "-i" || arg == "--ignore-case" {
                    options.ignore_case = true;
                } else if arg == "-u" || arg == "--unified" {
                    options.unified = true;
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for diff.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if positionals.len() != 2 {
                eprintln!("Error: 'diff' requires exactly two file paths.");
                valid = false;
            }

            if valid {
                ir_cli_utility::diff(&positionals[0], &positionals[1], options);
            } else {
                help::print_diff_help();
            }
        }
        "search" => {
            let mut options = SearchOptions::default();
            options.line_numbers = true;
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-i" || arg == "--ignore-case" {
                    options.case_insensitive = true;
                } else if arg == "-n" || arg == "--line-number" {
                    options.line_numbers = true;
                } else if arg == "--no-line-number" {
                    options.line_numbers = false;
                } else if arg == "-l" || arg == "--files-with-matches" {
                    options.files_with_matches = true;
                } else if arg == "-c" || arg == "--count" {
                    options.count = true;
                } else if arg == "-name" {
                    match args_iter.next() {
                        Some(pattern) if !pattern.starts_with('-') => options.name = Some(pattern.clone()),
                        _ => {
                            eprintln!("Error: -name requires a pattern.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-iname" {
                    match args_iter.next() {
                        Some(pattern) if !pattern.starts_with('-') => options.case_insensitive_name = Some(pattern.clone()),
                        _ => {
                            eprintln!("Error: -iname requires a pattern.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-maxdepth" {
                    match args_iter.next().and_then(|value| value.parse::<usize>().ok()) {
                        Some(depth) => options.max_depth = Some(depth),
                        None => {
                            eprintln!("Error: -maxdepth requires a non-negative number.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-mindepth" {
                    match args_iter.next().and_then(|value| value.parse::<usize>().ok()) {
                        Some(depth) => options.min_depth = depth,
                        None => {
                            eprintln!("Error: -mindepth requires a non-negative number.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--include" {
                    match args_iter.next() {
                        Some(ext) if !ext.starts_with('-') => options.include_extensions.push(normalize_extension(ext)),
                        _ => {
                            eprintln!("Error: --include requires a file extension.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--exclude" {
                    match args_iter.next() {
                        Some(ext) if !ext.starts_with('-') => options.exclude_extensions.push(normalize_extension(ext)),
                        _ => {
                            eprintln!("Error: --exclude requires a file extension.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--all" {
                    options.include_skipped = true;
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for search.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if options.files_with_matches && options.count {
                eprintln!("Error: '-l' (--files-with-matches) and '-c' (--count) cannot be used together.");
                valid = false;
            }

            if let Some(max_depth) = options.max_depth {
                if options.min_depth > max_depth {
                    eprintln!("Error: -mindepth cannot be greater than -maxdepth.");
                    valid = false;
                }
            }

            if positionals.is_empty() {
                eprintln!("Error: 'search' requires a phrase argument.");
                valid = false;
            }

            if valid {
                let phrase = positionals.remove(0);
                ir_cli_utility::search(&phrase, positionals, options);
            } else {
                help::print_search_help();
            }
        }
        "which" => {
            let mut options = WhichOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;

            for arg in &args[2..] {
                if arg == "-a" || arg == "--all" {
                    options.all = true;
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for which.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if positionals.len() != 1 {
                eprintln!("Error: 'which' requires exactly one command name.");
                valid = false;
            }

            if valid {
                ir_cli_utility::which(&positionals[0], options);
            } else {
                help::print_which_help();
            }
        }
        "tree" => {
            let mut options = TreeOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-L" {
                    match args_iter.next().and_then(|value| value.parse::<usize>().ok()) {
                        Some(depth) => options.max_depth = Some(depth),
                        None => {
                            eprintln!("Error: -L requires a non-negative depth limit.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--noreport" {
                    options.no_report = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'a' => options.show_all = true,
                            'd' => options.dirs_only = true,
                            'f' => options.full_path = true,
                            'i' => options.no_indent = true,
                            's' => options.show_size = true,
                            'h' => options.human_readable = true,
                            'p' => options.show_perms = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for tree.", char);
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

            if positionals.len() > 1 {
                eprintln!("Error: 'tree' accepts at most one path argument.");
                valid = false;
            }

            if valid {
                let path = if positionals.is_empty() { "." } else { &positionals[0] };
                ir_cli_utility::tree(path, options);
            } else {
                help::print_tree_help();
            }
        }
        "du" => {
            let mut options = DuOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-d" || arg == "--max-depth" {
                    match args_iter.next().and_then(|value| value.parse::<usize>().ok()) {
                        Some(depth) => options.max_depth = Some(depth),
                        None => {
                            eprintln!("Error: --max-depth requires a non-negative depth limit.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'a' => options.show_all = true,
                            'c' => options.total = true,
                            'h' => options.human_readable = true,
                            's' => options.summarize = true,
                            'k' => options.kilobytes = true,
                            'm' => options.megabytes = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for du.", char);
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

            if valid {
                if options.summarize && options.max_depth.is_some() && options.max_depth.unwrap() > 0 {
                    eprintln!("Error: Cannot combine -s (summarize) with -d (max-depth > 0).");
                    valid = false;
                }
                
                let format_count = usize::from(options.human_readable) + usize::from(options.kilobytes) + usize::from(options.megabytes);
                if format_count > 1 {
                    eprintln!("Error: -h, -k, and -m are mutually exclusive size formatting switches.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::du(positionals, options);
            } else {
                help::print_du_help();
            }
        }
        "help" => {
            if args.len() > 2 {
                match args[2].as_str() {
                    "list" => help::print_list_help(),
                    "rename" => help::print_rename_help(),
                    "copy" => help::print_copy_help(),
                    "remove" => help::print_remove_help(),
                    "create" => help::print_create_help(),
                    "move" => help::print_move_help(),
                    "archive" => help::print_archive_help(),
                    "cat" => help::print_cat_help(),
                    "grep" => help::print_grep_help(),
                    "find" => help::print_find_help(),
                    "diff" => help::print_diff_help(),
                    "search" => help::print_search_help(),
                    "which" => help::print_which_help(),
                    "tree" => help::print_tree_help(),
                    "du" => help::print_du_help(),
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

fn normalize_extension(ext: &str) -> String {
    ext.trim_start_matches('.').to_ascii_lowercase()
}

fn parse_line_range(range: &str) -> Option<(usize, usize)> {
    let (start, end) = range.split_once(':')?;
    let start = start.parse::<usize>().ok()?;
    let end = end.parse::<usize>().ok()?;

    if start == 0 || end == 0 || start > end {
        None
    } else {
        Some((start, end))
    }
}
