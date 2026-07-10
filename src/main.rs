use std::env;
use ir_cli_utility::{help, ListOptions, RenameOptions, CopyOptions, RemoveOptions, CreateOptions, MoveOptions, ArchiveOptions, CatOptions, GrepOptions, FindOptions, FindItemType, DiffOptions, SearchOptions, WhichOptions, TreeOptions, DuOptions, HashOptions, PsOptions, KillOptions, FetchOptions, HexOptions, PingOptions, Base64Options, EncodeOptions, DecodeOptions, UuidOptions, IpOptions, EchoOptions, ClipOptions, PathOptions, DfOptions, WhoamiOptions, SocketsOptions, WcOptions, LnOptions, ChmodOptions, ScrapeOptions, SortOptions, JsonOptions, PlotOptions, DnsOptions, PortscanOptions, MacOptions, ServeOptions, MatrixOptions, GitInfoOptions, DbViewOptions, RequestOptions, HexViewOptions, SysInfoOptions};
use ir_cli_utility::scrape::parse_size as scrape_parse_size;
use ir_cli_utility::find::parse_size as find_parse_size;

fn is_path(s: &str) -> bool {
    s.contains('/') || s.contains('\\')
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help::print_general_help();
        return;
    }

    let action_raw = &args[1];
    let action = match action_raw.as_str() {
        "ls" => "list",
        "touch" => "create",
        "tar" => "archive",
        "mv" => "move",
        "cp" => "copy",
        "rm" => "remove",
        "ff" => "fastfetch",
        "ptop" => "pmon",
        "smon" => "monitor",
        "ntop" => "nettop",
        "ncdu" => "dua",
        "ed" => "edit",
        "dl" => "scrape",
        "gin" => "gitinfo",
        "dbv" => "dbview",
        "req" => "request",
        "hexv" => "hexview",
        "sys" => "sysinfo",
        other => other,
    };

    match action {
        "list" => {
            let mut options = ListOptions::default();
            let mut list_args = args[2..].iter().peekable();
            let mut valid = true;

            while let Some(arg) = list_args.next() {
                if arg == "--human" || arg == "--human-readable" {
                    options.human_readable = true;
                } else if arg == "--filter" {
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
                            'h' => options.human_readable = true,
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
        "cat_old_dup" => {
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
                } else if arg == "-A" || arg == "--after-context" {
                    if let Some(val) = args_iter.next() {
                        match val.parse::<usize>() {
                            Ok(n) => options.after_context = n,
                            Err(_) => { eprintln!("Error: -A/--after-context requires a non-negative integer."); valid = false; break; }
                        }
                    } else { eprintln!("Error: -A/--after-context requires a value."); valid = false; break; }
                } else if arg.starts_with("-A") && arg.len() > 2 {
                    match arg[2..].parse::<usize>() {
                        Ok(n) => options.after_context = n,
                        Err(_) => { eprintln!("Error: Unknown switch '{}'", arg); valid = false; break; }
                    }
                } else if arg == "-B" || arg == "--before-context" {
                    if let Some(val) = args_iter.next() {
                        match val.parse::<usize>() {
                            Ok(n) => options.before_context = n,
                            Err(_) => { eprintln!("Error: -B/--before-context requires a non-negative integer."); valid = false; break; }
                        }
                    } else { eprintln!("Error: -B/--before-context requires a value."); valid = false; break; }
                } else if arg.starts_with("-B") && arg.len() > 2 {
                    match arg[2..].parse::<usize>() {
                        Ok(n) => options.before_context = n,
                        Err(_) => { eprintln!("Error: Unknown switch '{}'", arg); valid = false; break; }
                    }
                } else if arg == "-C" || arg == "--context" {
                    if let Some(val) = args_iter.next() {
                        match val.parse::<usize>() {
                            Ok(n) => { options.before_context = n; options.after_context = n; }
                            Err(_) => { eprintln!("Error: -C/--context requires a non-negative integer."); valid = false; break; }
                        }
                    } else { eprintln!("Error: -C/--context requires a value."); valid = false; break; }
                } else if arg.starts_with("-C") && arg.len() > 2 {
                    match arg[2..].parse::<usize>() {
                        Ok(n) => { options.before_context = n; options.after_context = n; }
                        Err(_) => { eprintln!("Error: Unknown switch '{}'", arg); valid = false; break; }
                    }
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
                } else if arg == "-min-size" || arg == "--min-size" {
                    if let Some(val) = args_iter.next() {
                        match find_parse_size(val) {
                            Some(n) => options.min_size = Some(n),
                            None => {
                                eprintln!("Error: --min-size requires a size like '50M', '1G', '500K', or a byte count.");
                                valid = false; break;
                            }
                        }
                    } else {
                        eprintln!("Error: --min-size requires a value.");
                        valid = false; break;
                    }
                } else if arg == "-max-size" || arg == "--max-size" {
                    if let Some(val) = args_iter.next() {
                        match find_parse_size(val) {
                            Some(n) => options.max_size = Some(n),
                            None => {
                                eprintln!("Error: --max-size requires a size like '50M', '1G', '500K', or a byte count.");
                                valid = false; break;
                            }
                        }
                    } else {
                        eprintln!("Error: --max-size requires a value.");
                        valid = false; break;
                    }
                } else if arg == "-newer" || arg == "--newer" {
                    if let Some(val) = args_iter.next() {
                        if val.starts_with('-') {
                            eprintln!("Error: --newer requires a file path.");
                            valid = false; break;
                        }
                        options.newer = Some(val.clone());
                    } else {
                        eprintln!("Error: --newer requires a file path.");
                        valid = false; break;
                    }
                } else if arg == "-older" || arg == "--older" {
                    if let Some(val) = args_iter.next() {
                        if val.starts_with('-') {
                            eprintln!("Error: --older requires a file path.");
                            valid = false; break;
                        }
                        options.older = Some(val.clone());
                    } else {
                        eprintln!("Error: --older requires a file path.");
                        valid = false; break;
                    }
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
        "cat" => {
            let mut options = CatOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut output_file: Option<String> = None;
            let mut append_mode = false;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-n" || arg == "--line-numbers" {
                    options.line_numbers = true;
                } else if arg == "-s" || arg == "--squeeze-blank" {
                    options.squeeze_blank = true;
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
                } else if arg == ">" {
                    match args_iter.next() {
                        Some(file) => {
                            output_file = Some(file.clone());
                            append_mode = false;
                            break;
                        }
                        None => {
                            eprintln!("Error: Redirection '>' requires a destination file path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == ">>" {
                    match args_iter.next() {
                        Some(file) => {
                            output_file = Some(file.clone());
                            append_mode = true;
                            break;
                        }
                        None => {
                            eprintln!("Error: Redirection '>>' requires a destination file path.");
                            valid = false;
                            break;
                        }
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
                if let Some(ref path) = output_file {
                    if path.to_lowercase() == "clip" {
                        let mut buf = Vec::new();
                        if let Err(err) = ir_cli_utility::cat_to_writer(&positionals[0], options, &mut buf) {
                            eprintln!("{}", err);
                            std::process::exit(1);
                        }
                        let text = String::from_utf8_lossy(&buf).into_owned();
                        let text_to_copy = if append_mode {
                            match ir_cli_utility::read_from_clipboard() {
                                Ok(current) => {
                                    let mut temp = current;
                                    temp.push_str(&text);
                                    temp
                                }
                                Err(_) => text,
                            }
                        } else {
                            text
                        };
                        if let Err(e) = ir_cli_utility::copy_to_clipboard(&text_to_copy) {
                            eprintln!("Error: Failed to copy to clipboard: {}", e);
                            std::process::exit(1);
                        }
                    } else {
                        let mut file = match std::fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(!append_mode)
                            .append(append_mode)
                            .open(path)
                        {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("Error: Failed to open or create file '{}': {}", path, e);
                                std::process::exit(1);
                            }
                        };
                        if let Err(err) = ir_cli_utility::cat_to_writer(&positionals[0], options, &mut file) {
                            eprintln!("{}", err);
                            std::process::exit(1);
                        }
                    }
                } else {
                    ir_cli_utility::cat(&positionals[0], options);
                }
            } else {
                help::print_cat_help();
                std::process::exit(1);
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
        "fastfetch" => {
            ir_cli_utility::fastfetch();
        }
        "monitor" => {
            ir_cli_utility::monitor();
        }
        "hash" => {
            let mut options = HashOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-a" || arg == "--algorithm" {
                    match args_iter.next() {
                        Some(algo) => options.algorithm = algo.clone(),
                        None => {
                            eprintln!("Error: --algorithm requires an algorithm name (e.g. md5, sha256).");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-v" || arg == "--verify" {
                    match args_iter.next() {
                        Some(expected) => options.verify = Some(expected.clone()),
                        None => {
                            eprintln!("Error: --verify requires the expected hash string.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'c' => options.checksum_file = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for hash.", char);
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
                if options.verify.is_some() && options.checksum_file {
                    eprintln!("Error: Cannot combine -v (verify) and -c (checksum-file).");
                    valid = false;
                }
                if positionals.len() != 1 {
                    eprintln!("Error: 'hash' action requires exactly one file path argument.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::hash(&positionals[0], options);
            } else {
                help::print_hash_help();
            }
        }
        "ps" => {
            let mut options = PsOptions::default();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-s" || arg == "--sort" {
                    match args_iter.next() {
                        Some(field) => {
                            let field_lower = field.to_lowercase();
                            if field_lower == "pid" || field_lower == "name" || field_lower == "cpu" || field_lower == "mem" {
                                options.sort_by = field_lower;
                            } else {
                                eprintln!("Error: Sort field must be one of 'pid', 'name', 'cpu', or 'mem'.");
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --sort requires a field name ('pid', 'name', 'cpu', 'mem').");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-f" || arg == "--filter" {
                    match args_iter.next() {
                        Some(filter) => options.filter = Some(filter.clone()),
                        None => {
                            eprintln!("Error: --filter requires a filter string.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-n" || arg == "--limit" {
                    match args_iter.next().and_then(|value| value.parse::<usize>().ok()) {
                        Some(limit) => options.limit = Some(limit),
                        None => {
                            eprintln!("Error: --limit requires a positive number.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for ps.", arg);
                    valid = false;
                    break;
                } else {
                    eprintln!("Error: 'ps' action does not accept positional arguments.");
                    valid = false;
                    break;
                }
            }

            if valid {
                ir_cli_utility::ps(options);
            } else {
                help::print_ps_help();
            }
        }
        "kill" => {
            let mut options = KillOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'f' => options.force = true,
                            'a' => options.all = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for kill.", char);
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
                if positionals.len() != 1 {
                    eprintln!("Error: 'kill' action requires exactly one process ID (PID) or process name argument.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::kill(&positionals[0], options);
            } else {
                help::print_kill_help();
            }
        }
        "fetch" => {
            let mut options = FetchOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-H" || arg == "--header" {
                    match args_iter.next() {
                        Some(hdr) => options.headers.push(hdr.clone()),
                        None => {
                            eprintln!("Error: --header requires a header string (e.g. 'Content-Type: application/json').");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-X" || arg == "--method" {
                    match args_iter.next() {
                        Some(m) => options.method = m.to_uppercase(),
                        None => {
                            eprintln!("Error: --method requires a method name (e.g. GET, POST).");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-d" || arg == "--data" {
                    match args_iter.next() {
                        Some(d) => options.data = Some(d.clone()),
                        None => {
                            eprintln!("Error: --data requires a request body string.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-o" || arg == "--output" {
                    match args_iter.next() {
                        Some(out) => options.output = Some(out.clone()),
                        None => {
                            eprintln!("Error: --output requires a file path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--timeout" {
                    match args_iter.next().and_then(|value| value.parse::<u64>().ok()) {
                        Some(secs) => options.timeout_secs = secs,
                        None => {
                            eprintln!("Error: --timeout requires a non-negative integer (seconds).");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--no-follow-redirects" {
                    options.no_follow_redirects = true;
                } else if arg == "-p" || arg == "--progress" {
                    options.progress = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'i' => options.include_headers = true,
                            'p' => options.progress = true,
                            _ => {
                                  eprintln!("Error: Unknown switch '-{}' for fetch.", char);
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
                if positionals.len() != 1 {
                    eprintln!("Error: 'fetch' action requires exactly one URL argument.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::fetch(&positionals[0], options);
            } else {
                help::print_fetch_help();
            }
        }

        "hex" => {
            let mut options = HexOptions::default();
            options.cols = 16; // default
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-n" || arg == "--limit" {
                    match args_iter.next().and_then(|v| v.parse::<usize>().ok()) {
                        Some(limit) => options.limit = Some(limit),
                        None => {
                            eprintln!("Error: --limit requires a positive number of bytes.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-c" || arg == "--cols" {
                    match args_iter.next().and_then(|v| v.parse::<usize>().ok()) {
                        Some(cols) => {
                            if cols > 0 {
                                options.cols = cols;
                            } else {
                                eprintln!("Error: --cols requires a column count greater than zero.");
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --cols requires a positive column count.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for hex.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() != 1 {
                    eprintln!("Error: 'hex' action requires exactly one file path argument.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::hex(&positionals[0], options);
            } else {
                help::print_hex_help();
            }
        }
        "ping" => {
            let mut options = PingOptions::default();
            options.count = 4; // default
            options.timeout_ms = 1000; // default
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-c" || arg == "--count" {
                    match args_iter.next().and_then(|v| v.parse::<usize>().ok()) {
                        Some(count) => {
                            if count > 0 {
                                options.count = count;
                            } else {
                                eprintln!("Error: --count must be greater than zero.");
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --count requires a positive count.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-t" || arg == "--timeout" {
                    match args_iter.next().and_then(|v| v.parse::<u64>().ok()) {
                        Some(t) => {
                            if t > 0 {
                                options.timeout_ms = t;
                            } else {
                                eprintln!("Error: --timeout must be greater than zero.");
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --timeout requires a positive timeout (ms).");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for ping.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() != 1 {
                    eprintln!("Error: 'ping' action requires exactly one host/IP argument.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::ping(&positionals[0], options);
            } else {
                help::print_ping_help();
            }
        }
        "base64" => {
            let mut options = Base64Options::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-o" || arg == "--output" {
                    match args_iter.next() {
                        Some(out) => options.output = Some(out.clone()),
                        None => {
                            eprintln!("Error: --output requires a file path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'd' => options.decode = true,
                            'u' => options.url = true,
                            'n' => options.no_padding = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for base64.", char);
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
                if positionals.len() > 1 {
                    eprintln!("Error: 'base64' action accepts at most one file path argument.");
                    valid = false;
                }
            }

            if valid {
                let input_path = positionals.get(0).map(|s| s.as_str());
                ir_cli_utility::base64(input_path, options);
            } else {
                help::print_base64_help();
                std::process::exit(1);
            }
        }
        "encode" => {
            let mut options = EncodeOptions::default();
            options.format = "base64".to_string(); // default
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-o" || arg == "--output" {
                    match args_iter.next() {
                        Some(out) => options.output = Some(out.clone()),
                        None => {
                            eprintln!("Error: --output requires a file path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-f" || arg == "--format" {
                    match args_iter.next() {
                        Some(f) => {
                            let fmt = f.to_lowercase();
                            if fmt == "base64" || fmt == "base64url" || fmt == "hex" || fmt == "base16" || fmt == "url" || fmt == "base32" || fmt == "rot13" {
                                options.format = fmt;
                            } else {
                                eprintln!("Error: Invalid encode format '{}'. Supported: base64, base64url, hex, url, base32, rot13", f);
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --format requires a format value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--separator" {
                    match args_iter.next() {
                        Some(sep) => options.hex_separator = Some(sep.clone()),
                        None => {
                            eprintln!("Error: --separator requires a separator character.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--upper" {
                    options.hex_upper = true;
                } else if arg == "--all" {
                    options.url_encode_all = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'n' => options.no_padding = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for encode.", char);
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
                if positionals.len() > 1 {
                    eprintln!("Error: 'encode' action accepts at most one file path argument.");
                    valid = false;
                }
            }

            if valid {
                let input_path = positionals.get(0).map(|s| s.as_str());
                ir_cli_utility::encode(input_path, options);
            } else {
                help::print_encode_help();
                std::process::exit(1);
            }
        }
        "decode" => {
            let mut options = DecodeOptions::default();
            options.format = "base64".to_string(); // default
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-o" || arg == "--output" {
                    match args_iter.next() {
                        Some(out) => options.output = Some(out.clone()),
                        None => {
                            eprintln!("Error: --output requires a file path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-f" || arg == "--format" {
                    match args_iter.next() {
                        Some(f) => {
                            let fmt = f.to_lowercase();
                            if fmt == "base64" || fmt == "base64url" || fmt == "hex" || fmt == "base16" || fmt == "url" || fmt == "base32" || fmt == "rot13" {
                                options.format = fmt;
                            } else {
                                eprintln!("Error: Invalid decode format '{}'. Supported: base64, base64url, hex, url, base32, rot13", f);
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --format requires a format value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--separator" {
                    match args_iter.next() {
                        Some(sep) => options.hex_separator = Some(sep.clone()),
                        None => {
                            eprintln!("Error: --separator requires a separator character.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'n' => options.no_padding = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for decode.", char);
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
                if positionals.len() > 1 {
                    eprintln!("Error: 'decode' action accepts at most one file path argument.");
                    valid = false;
                }
            }

            if valid {
                let input_path = positionals.get(0).map(|s| s.as_str());
                ir_cli_utility::decode(input_path, options);
            } else {
                help::print_decode_help();
                std::process::exit(1);
            }
        }
        "json" => {
            let mut options = JsonOptions::default();
            options.indent = 4; // default
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-q" || arg == "--query" {
                    match args_iter.next() {
                        Some(q) => options.query = Some(q.clone()),
                        None => {
                            eprintln!("Error: --query requires a selector path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-o" || arg == "--output" {
                    match args_iter.next() {
                        Some(out) => options.output = Some(out.clone()),
                        None => {
                            eprintln!("Error: --output requires a file path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--indent" {
                    match args_iter.next().and_then(|i| i.parse::<usize>().ok()) {
                        Some(ind) => options.indent = ind,
                        None => {
                            eprintln!("Error: --indent requires a valid positive integer.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'm' => options.minify = true,
                            'p' => options.pretty = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for json.", char);
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
                if positionals.len() > 1 {
                    eprintln!("Error: 'json' action accepts at most one file path argument.");
                    valid = false;
                }
            }

            if valid {
                let input_path = positionals.get(0).map(|s| s.as_str());
                ir_cli_utility::json(input_path, options);
            } else {
                help::print_json_help();
                std::process::exit(1);
            }
        }
        "plot" => {
            let mut options = PlotOptions::default();
            options.chart_type = "line".to_string(); // default
            options.source_format = "txt".to_string(); // default
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-t" || arg == "--type" {
                    match args_iter.next() {
                        Some(t) => {
                            let t_lower = t.to_lowercase();
                            if t_lower == "line" || t_lower == "bar" || t_lower == "scatter" {
                                options.chart_type = t_lower;
                            } else {
                                eprintln!("Error: Invalid chart type '{}'. Supported: line, bar, scatter", t);
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --type requires a chart type value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--title" {
                    match args_iter.next() {
                        Some(title) => options.title = Some(title.clone()),
                        None => {
                            eprintln!("Error: --title requires a text title.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-w" || arg == "--width" {
                    match args_iter.next().and_then(|w| w.parse::<usize>().ok()) {
                        Some(w) => options.width = Some(w),
                        None => {
                            eprintln!("Error: --width requires a valid positive integer.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-g" || arg == "--height" {
                    match args_iter.next().and_then(|h| h.parse::<usize>().ok()) {
                        Some(h) => options.height = Some(h),
                        None => {
                            eprintln!("Error: --height requires a valid positive integer.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--csv-col" {
                    match args_iter.next().and_then(|c| c.parse::<usize>().ok()) {
                        Some(c) => options.csv_col = Some(c),
                        None => {
                            eprintln!("Error: --csv-col requires a valid 0-indexed column integer.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--csv-headers" {
                    options.csv_headers = true;
                } else if arg == "--source" {
                    match args_iter.next() {
                        Some(s) => {
                            let s_lower = s.to_lowercase();
                            if s_lower == "txt" || s_lower == "csv" || s_lower == "json" {
                                options.source_format = s_lower;
                            } else {
                                eprintln!("Error: Invalid source format '{}'. Supported: txt, csv, json", s);
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --source requires a format value (txt, csv, json).");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--smooth" {
                    options.smooth = true;
                } else if arg == "--log" {
                    options.log_scale = true;
                } else if arg == "--json-key" {
                    match args_iter.next() {
                        Some(k) => options.json_key = Some(k.clone()),
                        None => {
                            eprintln!("Error: --json-key requires a field selector.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-H" || arg == "--horizontal" {
                    options.horizontal = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for plot.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() > 1 {
                    eprintln!("Error: 'plot' action accepts at most one file path argument.");
                    valid = false;
                }
            }

            if valid {
                let input_path = positionals.get(0).map(|s| s.as_str());
                ir_cli_utility::plot(input_path, options);
            } else {
                help::print_plot_help();
                std::process::exit(1);
            }
        }
        "text" => {
            let mut options = ir_cli_utility::TextOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-c" || arg == "--case" {
                    match args_iter.next() {
                        Some(c) => {
                            let c_lower = c.to_lowercase();
                            if ["camel", "snake", "pascal", "kebab", "upper", "lower", "title", "sentence", "slug"].contains(&c_lower.as_str()) {
                                options.case = Some(c_lower);
                            } else {
                                eprintln!("Error: Invalid case format '{}'. Supported: camel, snake, pascal, kebab, upper, lower, title, sentence, slug", c);
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --case requires a case format value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-w" || arg == "--width" {
                    match args_iter.next().and_then(|w| w.parse::<usize>().ok()) {
                        Some(w) if w > 0 => options.width = Some(w),
                        _ => {
                            eprintln!("Error: --width requires a valid positive integer.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--align" {
                    match args_iter.next() {
                        Some(a) => {
                            let a_lower = a.to_lowercase();
                            if a_lower == "left" || a_lower == "right" || a_lower == "center" {
                                options.align = Some(a_lower);
                            } else {
                                eprintln!("Error: Invalid align option '{}'. Supported: left, right, center", a);
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --align requires an alignment value (left, right, center).");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--truncate" {
                    options.truncate = true;
                } else if arg == "--ellipsis" {
                    match args_iter.next() {
                        Some(e) => options.ellipsis = Some(e.clone()),
                        None => {
                            eprintln!("Error: --ellipsis requires a string value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--strip-ansi" {
                    options.strip_ansi = true;
                } else if arg == "--strip-non-alphanumeric" {
                    options.strip_non_alphanumeric = true;
                } else if arg == "-o" || arg == "--output" {
                    match args_iter.next() {
                        Some(out) => options.output = Some(out.clone()),
                        None => {
                            eprintln!("Error: --output requires a file path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for text.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() > 1 {
                    eprintln!("Error: 'text' action accepts at most one file path argument.");
                    valid = false;
                }
            }

            if valid {
                let input_path = positionals.get(0).map(|s| s.as_str());
                ir_cli_utility::text(input_path, options);
            } else {
                help::print_text_help();
                std::process::exit(1);
            }
        }
        "globe" => {
            let mut options = ir_cli_utility::GlobeOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-m" || arg == "--mode" {
                    match args_iter.next() {
                        Some(m) => {
                            let m_lower = m.to_lowercase();
                            if m_lower == "globe" || m_lower == "map" {
                                options.mode = Some(m_lower);
                            } else {
                                eprintln!("Error: Invalid mode '{}'. Supported: globe, map", m);
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --mode requires a projection mode value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-c" || arg == "--center" {
                    match args_iter.next() {
                        Some(c) => {
                            if c.contains(',') {
                                options.center = Some(c.clone());
                            } else {
                                eprintln!("Error: --center must be formatted as 'lat,lon' (e.g. 40.7,-74.0).");
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --center requires a coordinate value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--day-night" {
                    options.day_night = true;
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for globe.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() > 1 {
                    eprintln!("Error: 'globe' action accepts at most one file path argument.");
                    valid = false;
                }
            }

            if valid {
                let input_path = positionals.get(0).map(|s| s.as_str());
                ir_cli_utility::globe(input_path, options);
            } else {
                help::print_globe_help();
                std::process::exit(1);
            }
        }
        "log" => {
            let mut options = ir_cli_utility::LogOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-f" || arg == "--format" {
                    match args_iter.next() {
                        Some(f) => {
                            let f_lower = f.to_lowercase();
                            if ["common", "combined", "json", "csv", "auto"].contains(&f_lower.as_str()) {
                                options.format = Some(f_lower);
                            } else {
                                  eprintln!("Error: Invalid log format '{}'. Supported: common, combined, json, csv, auto", f);
                                  valid = false;
                                  break;
                            }
                        }
                        None => {
                            eprintln!("Error: --format requires a log format value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-q" || arg == "--query" {
                    match args_iter.next() {
                        Some(q) => options.query = Some(q.clone()),
                        None => {
                            eprintln!("Error: --query requires a query filter expression.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-s" || arg == "--stats" {
                    options.stats = true;
                } else if arg == "-n" || arg == "--limit" {
                    match args_iter.next().and_then(|limit| limit.parse::<usize>().ok()) {
                        Some(limit) if limit > 0 => options.limit = Some(limit),
                        _ => {
                            eprintln!("Error: --limit requires a valid positive integer limit.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-o" || arg == "--output" {
                    match args_iter.next() {
                        Some(out) => options.output = Some(out.clone()),
                        None => {
                            eprintln!("Error: --output requires an output file path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for log action.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() > 1 {
                    eprintln!("Error: 'log' action accepts at most one file path argument.");
                    valid = false;
                }
            }

            if valid {
                let input_path = positionals.get(0).map(|s| s.as_str());
                ir_cli_utility::log_action(input_path, options);
            } else {
                help::print_log_help();
                std::process::exit(1);
            }
        }
        "life" => {
            let mut options = ir_cli_utility::LifeOptions::default();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-f" || arg == "--fps" {
                    match args_iter.next().and_then(|fps| fps.parse::<u32>().ok()) {
                        Some(fps) if fps >= 1 && fps <= 30 => options.fps = Some(fps),
                        _ => {
                            eprintln!("Error: --fps must be a valid integer between 1 and 30.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-p" || arg == "--preset" {
                    match args_iter.next() {
                        Some(p) => {
                            let p_lower = p.to_lowercase();
                            if ["random", "glider-gun", "pulsar"].contains(&p_lower.as_str()) {
                                options.preset = Some(p_lower);
                            } else {
                                  eprintln!("Error: Invalid preset '{}'. Supported: random, glider-gun, pulsar", p);
                                  valid = false;
                                  break;
                            }
                        }
                        None => {
                            eprintln!("Error: --preset requires a preset pattern value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for life simulator.", arg);
                    valid = false;
                    break;
                } else {
                    eprintln!("Error: 'life' action does not accept positional arguments.");
                    valid = false;
                    break;
                }
            }

            if valid {
                ir_cli_utility::life_action(options);
            } else {
                help::print_life_help();
                std::process::exit(1);
            }
        }
        "uuid" => {
            let mut options = UuidOptions::default();
            options.version = 4; // default
            options.count = 1; // default
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-v" || arg == "--version" {
                    match args_iter.next().and_then(|v| v.parse::<usize>().ok()) {
                        Some(v) => {
                            if v == 4 || v == 7 {
                                options.version = v;
                            } else {
                                eprintln!("Error: UUID version must be 4 or 7.");
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --version requires a version number (4 or 7).");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-c" || arg == "--count" {
                    match args_iter.next().and_then(|v| v.parse::<usize>().ok()) {
                        Some(count) => {
                            if count > 0 {
                                options.count = count;
                            } else {
                                eprintln!("Error: --count must be greater than zero.");
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --count requires a positive number.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'u' => options.uppercase = true,
                            'n' => options.no_hyphens = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for uuid.", char);
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
                if !positionals.is_empty() {
                    eprintln!("Error: 'uuid' action does not accept positional arguments.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::uuid(options);
            } else {
                help::print_uuid_help();
                std::process::exit(1);
            }
        }
        "ip" => {
            let mut options = IpOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'p' => options.public = true,
                            'a' => options.all = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for ip.", char);
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
                if !positionals.is_empty() {
                    eprintln!("Error: 'ip' action does not accept positional arguments.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::ip(options);
            } else {
                help::print_ip_help();
                std::process::exit(1);
            }
        }
        "echo" => {
            let mut options = EchoOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") {
                    for char in arg.chars().skip(1) {
                        match char {
                            'n' => options.no_newline = true,
                            'e' => options.escapes = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for echo.", char);
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
                ir_cli_utility::echo(positionals, options);
            } else {
                help::print_echo_help();
                std::process::exit(1);
            }
        }
        "clip" => {
            let mut options = ClipOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-c" || arg == "--clear" {
                    options.clear = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'c' => options.clear = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for clip.", char);
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
                if !positionals.is_empty() {
                    eprintln!("Error: 'clip' action does not accept positional arguments.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::clip(options);
            } else {
                help::print_clip_help();
                std::process::exit(1);
            }
        }
        "math" => {
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for math.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() > 1 {
                    eprintln!("Error: 'math' action accepts at most one mathematical expression string argument.");
                    valid = false;
                }
            }

            if valid {
                let expr_opt = positionals.get(0).map(|s| s.as_str());
                ir_cli_utility::math(expr_opt);
            } else {
                help::print_math_help();
                std::process::exit(1);
            }
        }
        "sleep" => {
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for sleep.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() != 1 {
                    eprintln!("Error: 'sleep' action requires exactly one duration string argument.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::sleep(&positionals[0]);
            } else {
                help::print_sleep_help();
                std::process::exit(1);
            }
        }
        "time" => {
            let cmd_args = args[2..].to_vec();
            if cmd_args.is_empty() {
                help::print_time_help();
                std::process::exit(1);
            }
            ir_cli_utility::time(cmd_args);
        }
        "dns" => {
            let mut options = DnsOptions::default();
            options.record_type = "A".to_string(); // default
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-t" || arg == "--type" {
                    if let Some(val) = args_iter.next() {
                        let val_upper = val.to_uppercase();
                        if ["A", "AAAA", "MX", "TXT", "CNAME", "NS", "SOA", "ANY"].contains(&val_upper.as_str()) {
                            options.record_type = val_upper;
                        } else {
                            eprintln!("Error: Invalid DNS record type '{}'. Supported types: A, AAAA, MX, TXT, CNAME, NS, SOA, ANY.", val);
                            valid = false;
                            break;
                        }
                    } else {
                        eprintln!("Error: -t/--type requires a record type argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "-s" || arg == "--server" {
                    if let Some(val) = args_iter.next() {
                        options.server = Some(val.clone());
                    } else {
                        eprintln!("Error: -s/--server requires a resolver address argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "-x" || arg == "--reverse" {
                    options.reverse = true;
                } else if arg == "--short" {
                    options.short = true;
                } else if arg == "--trace" {
                    options.trace = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    let chars: Vec<char> = arg.chars().skip(1).collect();
                    let mut is_valid_combo = true;
                    for &c in &chars {
                        if c == 'x' {
                            options.reverse = true;
                        } else {
                            is_valid_combo = false;
                            break;
                        }
                    }
                    if !is_valid_combo {
                        eprintln!("Error: Unknown switch '{}' for dns.", arg);
                        valid = false;
                        break;
                    }
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() != 1 {
                    eprintln!("Error: 'dns' action requires exactly one host/IP argument.");
                    valid = false;
                }
            }

            if valid {
                options.host = positionals[0].clone();
                if options.trace && (options.short || options.server.is_some()) {
                    eprintln!("Error: --trace is incompatible with --short or -s/--server options.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::dns(options);
            } else {
                help::print_dns_help();
                std::process::exit(1);
            }
        }
        "portscan" => {
            let mut options = PortscanOptions::default();
            options.ports = "top100".to_string(); // default
            options.timeout_ms = 500; // default
            options.concurrency = 100; // default
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-p" || arg == "--ports" {
                    if let Some(val) = args_iter.next() {
                        options.ports = val.clone();
                    } else {
                        eprintln!("Error: -p/--ports requires a ports range argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "-t" || arg == "--timeout" {
                    if let Some(val) = args_iter.next() {
                        if let Ok(ms) = val.parse::<u64>() {
                            options.timeout_ms = ms;
                        } else {
                            eprintln!("Error: Invalid timeout '{}'. Must be a positive integer.", val);
                            valid = false;
                            break;
                        }
                    } else {
                        eprintln!("Error: -t/--timeout requires a milliseconds argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "-c" || arg == "--concurrency" {
                    if let Some(val) = args_iter.next() {
                        if let Ok(threads) = val.parse::<usize>() {
                            options.concurrency = threads;
                        } else {
                            eprintln!("Error: Invalid concurrency '{}'. Must be a positive integer.", val);
                            valid = false;
                            break;
                        }
                    } else {
                        eprintln!("Error: -c/--concurrency requires a number of threads argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "--ping-first" {
                    options.ping_first = true;
                } else if arg == "--json" {
                    options.json = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for portscan.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() != 1 {
                    eprintln!("Error: 'portscan' action requires exactly one host/IP argument.");
                    valid = false;
                }
            }

            if valid {
                options.host = positionals[0].clone();
                ir_cli_utility::portscan(options);
            } else {
                help::print_portscan_help();
                std::process::exit(1);
            }
        }
        "mac" => {
            let mut options = MacOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-q" || arg == "--query" {
                    if let Some(val) = args_iter.next() {
                        options.query = Some(val.clone());
                    } else {
                        eprintln!("Error: -q/--query requires a MAC address argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "-l" || arg == "--local" {
                    options.local = true;
                } else if arg == "--update" {
                    options.update = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    let chars: Vec<char> = arg.chars().skip(1).collect();
                    let mut is_valid_combo = true;
                    for &c in &chars {
                        if c == 'l' {
                            options.local = true;
                        } else {
                            is_valid_combo = false;
                            break;
                        }
                    }
                    if !is_valid_combo {
                        eprintln!("Error: Unknown switch '{}' for mac.", arg);
                        valid = false;
                        break;
                    }
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if !positionals.is_empty() {
                    if options.query.is_none() {
                        options.query = Some(positionals[0].clone());
                    } else {
                        eprintln!("Error: Multiple MAC addresses specified.");
                        valid = false;
                    }
                }
            }

            if valid {
                let mut modes = 0;
                if options.query.is_some() { modes += 1; }
                if options.local { modes += 1; }
                if options.update { modes += 1; }
                
                if modes == 0 {
                    options.local = true;
                } else if modes > 1 {
                    eprintln!("Error: Switches --query, --local, and --update are mutually exclusive.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::mac(options);
            } else {
                help::print_mac_help();
                std::process::exit(1);
            }
        }
        "serve" => {
            let mut options = ServeOptions::default();
            options.directory = ".".to_string();
            options.port = 8080;
            options.bind = "127.0.0.1".to_string();
            options.cache_seconds = 0;

            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-p" || arg == "--port" {
                    if let Some(val) = args_iter.next() {
                        match val.parse::<u16>() {
                            Ok(p) => options.port = p,
                            Err(_) => {
                                eprintln!("Error: Invalid port number '{}'.", val);
                                valid = false;
                                break;
                            }
                        }
                    } else {
                        eprintln!("Error: -p/--port requires a port number.");
                        valid = false;
                        break;
                    }
                } else if arg == "-b" || arg == "--bind" {
                    if let Some(val) = args_iter.next() {
                        options.bind = val.clone();
                    } else {
                        eprintln!("Error: -b/--bind requires an IP address.");
                        valid = false;
                        break;
                    }
                } else if arg == "-c" || arg == "--cache" {
                    if let Some(val) = args_iter.next() {
                        match val.parse::<u64>() {
                            Ok(sec) => options.cache_seconds = sec,
                            Err(_) => {
                                eprintln!("Error: Invalid cache duration '{}'.", val);
                                valid = false;
                                break;
                            }
                        }
                    } else {
                        eprintln!("Error: -c/--cache requires a duration in seconds.");
                        valid = false;
                        break;
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for serve.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() > 1 {
                    eprintln!("Error: Multiple directory paths specified.");
                    valid = false;
                } else if positionals.len() == 1 {
                    options.directory = positionals[0].clone();
                }
            }

            if valid {
                ir_cli_utility::serve(options);
            } else {
                help::print_serve_help();
                std::process::exit(1);
            }
        }
        "matrix" => {
            let mut options = MatrixOptions::default();
            options.mode = "matrix".to_string();
            options.fps = 15;

            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-m" || arg == "--mode" {
                    if let Some(val) = args_iter.next() {
                        options.mode = val.to_lowercase();
                    } else {
                        eprintln!("Error: -m/--mode requires a mode name ('matrix' or 'fire').");
                        valid = false;
                        break;
                    }
                } else if arg == "-f" || arg == "--fps" {
                    if let Some(val) = args_iter.next() {
                        match val.parse::<u32>() {
                            Ok(f) if f >= 1 && f <= 60 => options.fps = f,
                            _ => {
                                eprintln!("Error: Invalid FPS '{}'. Must be between 1 and 60.", val);
                                valid = false;
                                break;
                            }
                        }
                    } else {
                        eprintln!("Error: -f/--fps requires an integer FPS value.");
                        valid = false;
                        break;
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for matrix.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() > 1 {
                    eprintln!("Error: Multiple modes specified.");
                    valid = false;
                } else if positionals.len() == 1 {
                    options.mode = positionals[0].to_lowercase();
                }
            }

            if valid {
                if options.mode != "matrix" && options.mode != "fire" {
                    eprintln!("Error: Unsupported mode '{}'. Use 'matrix' or 'fire'.", options.mode);
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::matrix(options);
            } else {
                help::print_matrix_help();
                std::process::exit(1);
            }
        }
        "gitinfo" => {
            let mut options = GitInfoOptions::default();
            options.source = ".".to_string();

            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "--source" {
                    if let Some(val) = args_iter.next() {
                        options.source = val.clone();
                    } else {
                        eprintln!("Error: --source requires a directory path.");
                        valid = false;
                        break;
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for gitinfo.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if positionals.len() > 1 {
                    eprintln!("Error: Multiple repository paths specified.");
                    valid = false;
                } else if positionals.len() == 1 {
                    options.source = positionals[0].clone();
                }
            }

            if valid {
                ir_cli_utility::gitinfo(options);
            } else {
                help::print_gitinfo_help();
                std::process::exit(1);
            }
        }
        "dbview" => {
            let mut options = DbViewOptions::default();
            let mut positionals = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();
            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for dbview.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }
            if valid {
                if positionals.len() != 1 {
                    eprintln!("Error: dbview requires exactly one file path argument.");
                    valid = false;
                } else {
                    options.file_path = positionals[0].clone();
                }
            }
            if valid {
                ir_cli_utility::dbview(options);
            } else {
                help::print_dbview_help();
                std::process::exit(1);
            }
        }
        "request" => {
            let mut options = RequestOptions::default();
            let mut positionals = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();
            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for request.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }
            if valid {
                if positionals.len() > 1 {
                    eprintln!("Error: request accepts at most one URL argument.");
                    valid = false;
                } else if positionals.len() == 1 {
                    options.url = Some(positionals[0].clone());
                }
            }
            if valid {
                ir_cli_utility::request(options);
            } else {
                help::print_request_help();
                std::process::exit(1);
            }
        }
        "hexview" => {
            let mut options = HexViewOptions::default();
            let mut positionals = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();
            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for hexview.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }
            if valid {
                if positionals.len() != 1 {
                    eprintln!("Error: hexview requires exactly one file path argument.");
                    valid = false;
                } else {
                    options.file_path = positionals[0].clone();
                }
            }
            if valid {
                ir_cli_utility::hexview(options);
            } else {
                help::print_hexview_help();
                std::process::exit(1);
            }
        }
        "sysinfo" => {
            let options = SysInfoOptions::default();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();
            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for sysinfo.", arg);
                    valid = false;
                    break;
                }
            }
            if valid {
                ir_cli_utility::sysinfo(options);
            } else {
                help::print_sysinfo_help();
                std::process::exit(1);
            }
        }
        "tee" => {
            let mut options = ir_cli_utility::tee::TeeOptions {
                files: Vec::new(),
                append: false,
                ignore_interrupts: false,
            };
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();
            while let Some(arg) = args_iter.next() {
                if arg == "--append" {
                    options.append = true;
                } else if arg == "--ignore-interrupts" {
                    options.ignore_interrupts = true;
                } else if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") {
                    for char in arg.chars().skip(1) {
                        match char {
                            'a' => options.append = true,
                            'i' => options.ignore_interrupts = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for tee.", char);
                                valid = false;
                                break;
                            }
                        }
                    }
                    if !valid { break; }
                } else {
                    options.files.push(arg.clone());
                }
            }
            if valid {
                ir_cli_utility::tee(options);
            } else {
                help::print_tee_help();
                std::process::exit(1);
            }
        }
        "head" => {
            let mut files = Vec::new();
            let mut quiet = false;
            let mut verbose = false;
            let mut lines_val: Option<String> = None;
            let mut bytes_val: Option<String> = None;
            let mut valid = true;

            let args_vec = &args[2..];
            let mut i = 0;
            while i < args_vec.len() {
                let arg = &args_vec[i];
                if arg == "--quiet" || arg == "--silent" {
                    quiet = true;
                } else if arg == "--verbose" {
                    verbose = true;
                } else if arg == "--lines" {
                    if i + 1 < args_vec.len() {
                        lines_val = Some(args_vec[i + 1].clone());
                        i += 1;
                    } else {
                        eprintln!("Error: --lines requires an argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "--bytes" {
                    if i + 1 < args_vec.len() {
                        bytes_val = Some(args_vec[i + 1].clone());
                        i += 1;
                    } else {
                        eprintln!("Error: --bytes requires an argument.");
                        valid = false;
                        break;
                    }
                } else if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") {
                    let rest = &arg[1..];
                    if rest.chars().all(|c| c.is_ascii_digit()) {
                        lines_val = Some(rest.to_string());
                    } else {
                        let chars: Vec<char> = arg.chars().skip(1).collect();
                        let mut j = 0;
                        while j < chars.len() {
                            let ch = chars[j];
                            match ch {
                                'q' => quiet = true,
                                'v' => verbose = true,
                                'n' | 'c' => {
                                    let val;
                                    if j + 1 < chars.len() {
                                        val = chars[j+1..].iter().collect();
                                        j = chars.len();
                                    } else {
                                        if i + 1 < args_vec.len() {
                                            val = args_vec[i + 1].clone();
                                            i += 1;
                                        } else {
                                            eprintln!("Error: -{} requires an argument.", ch);
                                            valid = false;
                                            break;
                                        }
                                    }
                                    if ch == 'n' {
                                        lines_val = Some(val);
                                    } else {
                                        bytes_val = Some(val);
                                    }
                                }
                                _ => {
                                    eprintln!("Error: Unknown switch '-{}' for head.", ch);
                                    valid = false;
                                    break;
                                }
                            }
                            j += 1;
                        }
                    }
                    if !valid { break; }
                } else {
                    files.push(arg.clone());
                }
                i += 1;
            }

            if valid {
                if lines_val.is_some() && bytes_val.is_some() {
                    eprintln!("Error: --lines and --bytes cannot be used together.");
                    valid = false;
                }
                if quiet && verbose {
                    eprintln!("Error: --quiet and --verbose cannot be used together.");
                    valid = false;
                }
            }

            if valid {
                let count = if let Some(ref lv) = lines_val {
                    let (neg, val_str) = if lv.starts_with('-') {
                        (true, &lv[1..])
                    } else {
                        (false, lv.as_str())
                    };
                    match val_str.parse::<usize>() {
                        Ok(num) => {
                            if neg {
                                ir_cli_utility::head::HeadCount::LinesAllButLast(num)
                            } else {
                                ir_cli_utility::head::HeadCount::Lines(num)
                            }
                        }
                        Err(_) => {
                            eprintln!("Error: Invalid lines value '{}'.", lv);
                            valid = false;
                            ir_cli_utility::head::HeadCount::Lines(10)
                        }
                    }
                } else if let Some(ref bv) = bytes_val {
                    let (neg, val_str) = if bv.starts_with('-') {
                        (true, &bv[1..])
                    } else {
                        (false, bv.as_str())
                    };
                    match val_str.parse::<usize>() {
                        Ok(num) => {
                            if neg {
                                ir_cli_utility::head::HeadCount::BytesAllButLast(num)
                            } else {
                                ir_cli_utility::head::HeadCount::Bytes(num)
                            }
                        }
                        Err(_) => {
                            eprintln!("Error: Invalid bytes value '{}'.", bv);
                            valid = false;
                            ir_cli_utility::head::HeadCount::Bytes(10)
                        }
                    }
                } else {
                    ir_cli_utility::head::HeadCount::Lines(10)
                };

                if valid {
                    let options = ir_cli_utility::head::HeadOptions {
                        files,
                        count,
                        quiet,
                        verbose,
                    };
                    ir_cli_utility::head(options);
                }
            }

            if !valid {
                help::print_head_help();
                std::process::exit(1);
            }
        }
        "tail" => {
            let mut files = Vec::new();
            let mut quiet = false;
            let mut verbose = false;
            let mut follow = false;
            let mut sleep_interval_ms = 1000;
            let mut lines_val: Option<String> = None;
            let mut bytes_val: Option<String> = None;
            let mut valid = true;

            let args_vec = &args[2..];
            let mut i = 0;
            while i < args_vec.len() {
                let arg = &args_vec[i];
                if arg == "--quiet" || arg == "--silent" {
                    quiet = true;
                } else if arg == "--verbose" {
                    verbose = true;
                } else if arg == "--follow" {
                    follow = true;
                } else if arg == "--lines" {
                    if i + 1 < args_vec.len() {
                        lines_val = Some(args_vec[i + 1].clone());
                        i += 1;
                    } else {
                        eprintln!("Error: --lines requires an argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "--bytes" {
                    if i + 1 < args_vec.len() {
                        bytes_val = Some(args_vec[i + 1].clone());
                        i += 1;
                    } else {
                        eprintln!("Error: --bytes requires an argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "--sleep-interval" {
                    if i + 1 < args_vec.len() {
                        if let Ok(sec) = args_vec[i + 1].parse::<f64>() {
                            sleep_interval_ms = (sec * 1000.0) as u64;
                        } else {
                            eprintln!("Error: Invalid sleep-interval '{}'.", args_vec[i + 1]);
                            valid = false;
                            break;
                        }
                        i += 1;
                    } else {
                        eprintln!("Error: --sleep-interval requires an argument.");
                        valid = false;
                        break;
                    }
                } else if arg.starts_with('+') && arg.len() > 1 {
                    let rest = &arg[1..];
                    if rest.chars().all(|c| c.is_ascii_digit()) {
                        lines_val = Some(arg.clone());
                    } else {
                        files.push(arg.clone());
                    }
                } else if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") {
                    let rest = &arg[1..];
                    if rest.chars().all(|c| c.is_ascii_digit()) {
                        lines_val = Some(rest.to_string());
                    } else {
                        let chars: Vec<char> = arg.chars().skip(1).collect();
                        let mut j = 0;
                        while j < chars.len() {
                            let ch = chars[j];
                            match ch {
                                'q' => quiet = true,
                                'v' => verbose = true,
                                'f' => follow = true,
                                'n' | 'c' | 's' => {
                                    let val;
                                    if j + 1 < chars.len() {
                                        val = chars[j+1..].iter().collect();
                                        j = chars.len();
                                    } else {
                                        if i + 1 < args_vec.len() {
                                            val = args_vec[i + 1].clone();
                                            i += 1;
                                        } else {
                                            eprintln!("Error: -{} requires an argument.", ch);
                                            valid = false;
                                            break;
                                        }
                                    }
                                    if ch == 'n' {
                                        lines_val = Some(val);
                                    } else if ch == 'c' {
                                        bytes_val = Some(val);
                                    } else {
                                        if let Ok(sec) = val.parse::<f64>() {
                                            sleep_interval_ms = (sec * 1000.0) as u64;
                                        } else {
                                            eprintln!("Error: Invalid sleep-interval '{}'.", val);
                                            valid = false;
                                            break;
                                        }
                                    }
                                }
                                _ => {
                                    eprintln!("Error: Unknown switch '-{}' for tail.", ch);
                                    valid = false;
                                    break;
                                }
                            }
                            j += 1;
                        }
                    }
                    if !valid { break; }
                } else {
                    files.push(arg.clone());
                }
                i += 1;
            }

            if valid {
                if lines_val.is_some() && bytes_val.is_some() {
                    eprintln!("Error: --lines and --bytes cannot be used together.");
                    valid = false;
                }
                if quiet && verbose {
                    eprintln!("Error: --quiet and --verbose cannot be used together.");
                    valid = false;
                }
            }

            if valid {
                let count = if let Some(ref lv) = lines_val {
                    if lv.starts_with('+') {
                        match lv[1..].parse::<usize>() {
                            Ok(num) => ir_cli_utility::tail::TailCount::FromKthLine(num),
                            Err(_) => {
                                eprintln!("Error: Invalid lines value '{}'.", lv);
                                valid = false;
                                ir_cli_utility::tail::TailCount::LastLines(10)
                            }
                        }
                    } else {
                        let val_str = if lv.starts_with('-') { &lv[1..] } else { lv.as_str() };
                        match val_str.parse::<usize>() {
                            Ok(num) => ir_cli_utility::tail::TailCount::LastLines(num),
                            Err(_) => {
                                eprintln!("Error: Invalid lines value '{}'.", lv);
                                valid = false;
                                ir_cli_utility::tail::TailCount::LastLines(10)
                            }
                        }
                    }
                } else if let Some(ref bv) = bytes_val {
                    if bv.starts_with('+') {
                        match bv[1..].parse::<usize>() {
                            Ok(num) => ir_cli_utility::tail::TailCount::FromKthByte(num),
                            Err(_) => {
                                eprintln!("Error: Invalid bytes value '{}'.", bv);
                                valid = false;
                                ir_cli_utility::tail::TailCount::LastBytes(10)
                            }
                        }
                    } else {
                        let val_str = if bv.starts_with('-') { &bv[1..] } else { bv.as_str() };
                        match val_str.parse::<usize>() {
                            Ok(num) => ir_cli_utility::tail::TailCount::LastBytes(num),
                            Err(_) => {
                                eprintln!("Error: Invalid bytes value '{}'.", bv);
                                valid = false;
                                ir_cli_utility::tail::TailCount::LastBytes(10)
                            }
                        }
                    }
                } else {
                    ir_cli_utility::tail::TailCount::LastLines(10)
                };

                if valid {
                    let options = ir_cli_utility::tail::TailOptions {
                        files,
                        count,
                        follow,
                        sleep_interval_ms,
                        quiet,
                        verbose,
                    };
                    ir_cli_utility::tail(options);
                }
            }

            if !valid {
                help::print_tail_help();
                std::process::exit(1);
            }
        }
        "stat" => {
            let mut files = Vec::new();
            let mut file_system = false;
            let mut terse = false;
            let mut format_val: Option<String> = None;
            let mut valid = true;

            let args_vec = &args[2..];
            let mut i = 0;
            while i < args_vec.len() {
                let arg = &args_vec[i];
                if arg == "--file-system" {
                    file_system = true;
                } else if arg == "--terse" {
                    terse = true;
                } else if arg == "--format" {
                    if i + 1 < args_vec.len() {
                        format_val = Some(args_vec[i + 1].clone());
                        i += 1;
                    } else {
                        eprintln!("Error: --format requires an argument.");
                        valid = false;
                        break;
                    }
                } else if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") {
                    let chars: Vec<char> = arg.chars().skip(1).collect();
                    let mut j = 0;
                    while j < chars.len() {
                        let ch = chars[j];
                        match ch {
                            'f' => file_system = true,
                            't' => terse = true,
                            'c' => {
                                let val;
                                if j + 1 < chars.len() {
                                    val = chars[j+1..].iter().collect();
                                    j = chars.len();
                                } else {
                                    if i + 1 < args_vec.len() {
                                        val = args_vec[i + 1].clone();
                                        i += 1;
                                    } else {
                                        eprintln!("Error: -c requires an argument.");
                                        valid = false;
                                        break;
                                    }
                                }
                                format_val = Some(val);
                            }
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for stat.", ch);
                                valid = false;
                                break;
                            }
                        }
                        j += 1;
                    }
                    if !valid { break; }
                } else {
                    files.push(arg.clone());
                }
                i += 1;
            }

            if valid {
                if format_val.is_some() && terse {
                    eprintln!("Error: --format and --terse cannot be used together.");
                    valid = false;
                }
            }

            if valid {
                let options = ir_cli_utility::stat::StatOptions {
                    files,
                    file_system,
                    format: format_val,
                    terse,
                };
                ir_cli_utility::stat(options);
            } else {
                help::print_stat_help();
                std::process::exit(1);
            }
        }
        "anispeak" => {
            let mut positionals = Vec::new();
            let mut animal = "cow".to_string();
            let mut width = 40;
            let mut valid = true;

            let args_vec = &args[2..];
            let mut i = 0;
            while i < args_vec.len() {
                let arg = &args_vec[i];
                if arg == "--animal" {
                    if i + 1 < args_vec.len() {
                        animal = args_vec[i + 1].clone();
                        i += 1;
                    } else {
                        eprintln!("Error: --animal requires a name argument.");
                        valid = false;
                        break;
                    }
                } else if arg == "--width" {
                    if i + 1 < args_vec.len() {
                        if let Ok(w) = args_vec[i + 1].parse::<usize>() {
                            width = w;
                        } else {
                            eprintln!("Error: Invalid width '{}'.", args_vec[i + 1]);
                            valid = false;
                            break;
                        }
                        i += 1;
                    } else {
                        eprintln!("Error: --width requires a numeric argument.");
                        valid = false;
                        break;
                    }
                } else if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") {
                    let chars: Vec<char> = arg.chars().skip(1).collect();
                    let mut j = 0;
                    while j < chars.len() {
                        let ch = chars[j];
                        match ch {
                            'a' | 'w' => {
                                let val;
                                if j + 1 < chars.len() {
                                    val = chars[j+1..].iter().collect::<String>();
                                    j = chars.len();
                                } else {
                                    if i + 1 < args_vec.len() {
                                        val = args_vec[i + 1].clone();
                                        i += 1;
                                    } else {
                                        eprintln!("Error: -{} requires an argument.", ch);
                                        valid = false;
                                        break;
                                    }
                                }
                                if ch == 'a' {
                                    animal = val;
                                } else {
                                    if let Ok(w) = val.parse::<usize>() {
                                        width = w;
                                    } else {
                                        eprintln!("Error: Invalid width '{}'.", val);
                                        valid = false;
                                        break;
                                    }
                                }
                            }
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for anispeak.", ch);
                                valid = false;
                                break;
                            }
                        }
                        j += 1;
                    }
                    if !valid { break; }
                } else {
                    positionals.push(arg.clone());
                }
                i += 1;
            }

            if valid {
                let mut message = positionals.join(" ");
                if message.is_empty() {
                    use std::io::Read;
                    let mut buffer = String::new();
                    let _ = std::io::stdin().read_to_string(&mut buffer);
                    message = buffer.trim_end().to_string();
                }

                let allowed_animals = [
                    "cow", "crab", "dino", "cat", "dog", "duck", "owl", "penguin",
                    "elephant", "moose", "stegosaurus", "whale", "snake", "turtle", "sheep"
                ];
                if !allowed_animals.contains(&animal.to_lowercase().as_str()) {
                    eprintln!("Error: Supported animals are: cow, crab, dino, cat, dog, duck, owl, penguin, elephant, moose, stegosaurus, whale, snake, turtle, sheep.");
                    valid = false;
                }

                if valid {
                    let options = ir_cli_utility::anispeak::AnispeakOptions {
                        message,
                        animal,
                        width,
                    };
                    ir_cli_utility::anispeak(options);
                }
            }

            if !valid {
                help::print_anispeak_help();
                std::process::exit(1);
            }
        }
        "envv" | "env" => {
            if let Err(e) = ir_cli_utility::envv::run_envv() {
                eprintln!("Error running envv: {}", e);
            }
        }
        "fm" | "browse" => {
            if let Err(e) = ir_cli_utility::fm::run_fm() {
                eprintln!("Error running fm: {}", e);
            }
        }
        "gitv" => {
            if let Err(e) = ir_cli_utility::gitv::run_gitv() {
                eprintln!("Error running gitv: {}", e);
            }
        }
        "path" => {
            let mut options = PathOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-a" || arg == "--add" {
                    match args_iter.next() {
                        Some(dir) => options.add = Some(dir.clone()),
                        None => {
                            eprintln!("Error: --add requires a directory path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-r" || arg == "--remove" {
                    match args_iter.next() {
                        Some(dir) => options.remove = Some(dir.clone()),
                        None => {
                            eprintln!("Error: --remove requires a directory path.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for path.", arg);
                    valid = false;
                    break;
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                if !positionals.is_empty() {
                    eprintln!("Error: 'path' action does not accept positional arguments.");
                    valid = false;
                }
                if options.add.is_some() && options.remove.is_some() {
                    eprintln!("Error: --add and --remove cannot be specified together.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::path(options);
            } else {
                help::print_path_help();
                std::process::exit(1);
            }
        }
        "df" => {
            let mut options = DfOptions::default();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-a" || arg == "--all" {
                    options.all = true;
                } else if arg == "-h" || arg == "--human-readable" {
                    options.human_readable = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'a' => options.all = true,
                            'h' => options.human_readable = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for df.", char);
                                valid = false;
                                break;
                            }
                        }
                    }
                    if !valid { break; }
                } else {
                    eprintln!("Error: 'df' action does not accept positional arguments.");
                    valid = false;
                    break;
                }
            }

            if valid {
                ir_cli_utility::df(options);
            } else {
                help::print_df_help();
                std::process::exit(1);
            }
        }
        "whoami" => {
            let mut valid = true;
            for arg in &args[2..] {
                eprintln!("Error: Unknown argument '{}' for whoami.", arg);
                valid = false;
                break;
            }

            if valid {
                ir_cli_utility::whoami(WhoamiOptions::default());
            } else {
                help::print_whoami_help();
                std::process::exit(1);
            }
        }
        "sockets" => {
            let mut options = SocketsOptions::default();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-a" || arg == "--all" {
                    options.show_all = true;
                } else if arg == "-t" || arg == "--tcp" {
                    options.tcp_only = true;
                } else if arg == "-u" || arg == "--udp" {
                    options.udp_only = true;
                } else if arg == "-l" || arg == "--listening" {
                    options.listening_only = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'a' => options.show_all = true,
                            't' => options.tcp_only = true,
                            'u' => options.udp_only = true,
                            'l' => options.listening_only = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for sockets.", char);
                                valid = false;
                                break;
                            }
                        }
                    }
                    if !valid { break; }
                } else {
                    eprintln!("Error: 'sockets' action does not accept positional arguments.");
                    valid = false;
                    break;
                }
            }

            if valid {
                ir_cli_utility::sockets(options);
            } else {
                help::print_sockets_help();
                std::process::exit(1);
            }
        }
        "wc" => {
            let mut options = WcOptions::default();
            let mut positionals = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-l" || arg == "--lines" {
                    options.lines = true;
                } else if arg == "-w" || arg == "--words" {
                    options.words = true;
                } else if arg == "-c" || arg == "--bytes" {
                    options.bytes = true;
                } else if arg == "-m" || arg == "--chars" {
                    options.chars = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'l' => options.lines = true,
                            'w' => options.words = true,
                            'c' => options.bytes = true,
                            'm' => options.chars = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for wc.", char);
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
                if options.bytes && options.chars {
                    eprintln!("Error: -c and -m are mutually exclusive size metric switches.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::wc(positionals, options);
            } else {
                help::print_wc_help();
                std::process::exit(1);
            }
        }
        "ln" => {
            let mut options = LnOptions::default();
            let mut positionals = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-s" || arg == "--symbolic" {
                    options.symbolic = true;
                } else if arg == "-f" || arg == "--force" {
                    options.force = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            's' => options.symbolic = true,
                            'f' => options.force = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for ln.", char);
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
                if positionals.len() != 2 {
                    eprintln!("Error: 'ln' action requires exactly two positional arguments: <target> <link_name>.");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::ln(&positionals[0], &positionals[1], options);
            } else {
                help::print_ln_help();
                std::process::exit(1);
            }
        }
        "chmod" => {
            let mut options = ChmodOptions::default();
            let mut positionals = Vec::new();
            let mut valid = true;
            let mut mode_found = false;

            let args_vec = &args[2..];
            let mut i = 0;
            while i < args_vec.len() {
                let arg = &args_vec[i];
                if mode_found {
                    positionals.push(arg.clone());
                } else if arg == "--" {
                    mode_found = true;
                } else if arg.starts_with("--") {
                    match arg.as_str() {
                        "--recursive" => options.recursive = true,
                        "--verbose" => options.verbose = true,
                        "--changes" => options.changes = true,
                        _ => {
                            eprintln!("Error: Unknown switch '{}' for chmod.", arg);
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    let chars: Vec<char> = arg.chars().skip(1).collect();
                    let is_option = chars.iter().all(|&c| c == 'R' || c == 'v' || c == 'c');
                    if is_option {
                        for &c in &chars {
                            match c {
                                'R' => options.recursive = true,
                                'v' => options.verbose = true,
                                'c' => options.changes = true,
                                _ => unreachable!(),
                            }
                        }
                    } else {
                        mode_found = true;
                        positionals.push(arg.clone());
                    }
                } else {
                    mode_found = true;
                    positionals.push(arg.clone());
                }
                i += 1;
            }

            if valid {
                if positionals.len() < 2 {
                    eprintln!("Error: 'chmod' action requires a mode and at least one path: ir chmod [SWITCHES] <MODE> <PATH...>");
                    valid = false;
                }
            }

            if valid {
                let mode = &positionals[0];
                let paths = positionals[1..].to_vec();
                ir_cli_utility::chmod(mode, paths, options);
            } else {
                help::print_chmod_help();
                std::process::exit(1);
            }
        }
        "pmon" => {
            let mut options = ir_cli_utility::PmonOptions::default();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-d" || arg == "--delay" {
                    match args_iter.next() {
                        Some(val) => {
                            let mut raw_val = val.as_str();
                            let mut is_ms = false;
                            if raw_val.ends_with("ms") {
                                raw_val = &raw_val[..raw_val.len() - 2];
                                is_ms = true;
                            } else if raw_val.ends_with('s') {
                                raw_val = &raw_val[..raw_val.len() - 1];
                            }

                            match raw_val.parse::<f64>() {
                                Ok(num) => {
                                    if num <= 0.0 {
                                        eprintln!("Error: Delay must be greater than zero.");
                                        valid = false;
                                        break;
                                    }
                                    options.delay_ms = if is_ms {
                                        num.round() as u64
                                    } else {
                                        (num * 1000.0).round() as u64
                                    };
                                }
                                Err(_) => {
                                    eprintln!("Error: Invalid delay value '{}'.", val);
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        None => {
                            eprintln!("Error: --delay requires a value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for pmon.", arg);
                    valid = false;
                    break;
                } else {
                    eprintln!("Error: 'pmon' action does not accept positional arguments.");
                    valid = false;
                    break;
                }
            }

            if valid {
                ir_cli_utility::pmon(options);
            } else {
                help::print_pmon_help();
                std::process::exit(1);
            }
        }
        "watch" => {
            let mut options = ir_cli_utility::WatchOptions::default();
            let mut command = String::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-n" || arg == "--interval" {
                    match args_iter.next() {
                        Some(val) => {
                            let mut raw_val = val.as_str();
                            let mut is_ms = false;
                            if raw_val.ends_with("ms") {
                                raw_val = &raw_val[..raw_val.len() - 2];
                                is_ms = true;
                            } else if raw_val.ends_with('s') {
                                raw_val = &raw_val[..raw_val.len() - 1];
                            }

                            match raw_val.parse::<f64>() {
                                Ok(num) => {
                                    if num <= 0.0 {
                                        eprintln!("Error: Interval must be greater than zero.");
                                        valid = false;
                                        break;
                                    }
                                    options.interval_ms = if is_ms {
                                        num.round() as u64
                                    } else {
                                        (num * 1000.0).round() as u64
                                    };
                                }
                                Err(_) => {
                                    eprintln!("Error: Invalid interval value '{}'.", val);
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        None => {
                            eprintln!("Error: --interval requires a value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "--diff" {
                    options.diff = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for watch.", arg);
                    valid = false;
                    break;
                } else {
                    if !command.is_empty() {
                        eprintln!("Error: Multiple commands provided. Only one command can be watched.");
                        valid = false;
                        break;
                    }
                    command = arg.clone();
                }
            }

            if command.is_empty() && valid {
                eprintln!("Error: Command argument is missing.");
                valid = false;
            }

            if valid {
                ir_cli_utility::watch(&command, options);
            } else {
                help::print_watch_help();
                std::process::exit(1);
            }
        }
        "nettop" => {
            let mut options = ir_cli_utility::NettopOptions::default();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-d" || arg == "--delay" {
                    match args_iter.next() {
                        Some(val) => {
                            let mut raw_val = val.as_str();
                            let mut is_ms = false;
                            if raw_val.ends_with("ms") {
                                raw_val = &raw_val[..raw_val.len() - 2];
                                is_ms = true;
                            } else if raw_val.ends_with('s') {
                                raw_val = &raw_val[..raw_val.len() - 1];
                            }

                            match raw_val.parse::<f64>() {
                                Ok(num) => {
                                    if num <= 0.0 {
                                        eprintln!("Error: Delay must be greater than zero.");
                                        valid = false;
                                        break;
                                    }
                                    options.delay_ms = if is_ms {
                                        num.round() as u64
                                    } else {
                                        (num * 1000.0).round() as u64
                                    };
                                }
                                Err(_) => {
                                    eprintln!("Error: Invalid delay value '{}'.", val);
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        None => {
                            eprintln!("Error: --delay requires a value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for nettop.", arg);
                    valid = false;
                    break;
                } else {
                    eprintln!("Error: 'nettop' action does not accept positional arguments.");
                    valid = false;
                    break;
                }
            }

            if valid {
                ir_cli_utility::nettop(options);
            } else {
                help::print_nettop_help();
                std::process::exit(1);
            }
        }
        "dua" => {
            let options = ir_cli_utility::DuaOptions::default();
            let mut path = ".".to_string();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for dua.", arg);
                    valid = false;
                    break;
                } else {
                    path = arg.clone();
                }
            }

            if valid {
                ir_cli_utility::dua(&path, options);
            } else {
                help::print_dua_help();
                std::process::exit(1);
            }
        }

        "edit" => {
            let options = ir_cli_utility::EditOptions::default();
            let mut filename = String::new();
            let mut valid = true;

            for arg in &args[2..] {
                if arg.starts_with('-') && arg.len() > 1 {
                    eprintln!("Error: Unknown switch '{}' for edit.", arg);
                    valid = false;
                    break;
                } else {
                    filename = arg.clone();
                }
            }

            if filename.is_empty() {
                eprintln!("Error: 'edit' requires a filename.");
                valid = false;
            }

            if valid {
                ir_cli_utility::edit(&filename, options);
            } else {
                help::print_edit_help();
                std::process::exit(1);
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
                    "fastfetch" => help::print_fastfetch_help(),
                    "monitor" => help::print_monitor_help(),
                    "hash" => help::print_hash_help(),
                    "ps" => help::print_ps_help(),
                    "kill" => help::print_kill_help(),
                    "fetch" => help::print_fetch_help(),
                    "env" | "envv" => help::print_env_help(),
                    "hex" => help::print_hex_help(),
                    "ping" => help::print_ping_help(),
                    "base64" => help::print_base64_help(),
                    "encode" => help::print_encode_help(),
                    "decode" => help::print_decode_help(),
                    "json" => help::print_json_help(),
                    "plot" => help::print_plot_help(),
                    "uuid" => help::print_uuid_help(),
                    "ip" => help::print_ip_help(),
                    "echo" => help::print_echo_help(),
                    "clip" => help::print_clip_help(),
                    "math" => help::print_math_help(),
                    "sleep" => help::print_sleep_help(),
                    "time" => help::print_time_help(),
                    "dns" => help::print_dns_help(),
                    "portscan" => help::print_portscan_help(),
                    "mac" => help::print_mac_help(),
                    "serve" => help::print_serve_help(),
                    "matrix" => help::print_matrix_help(),
                    "gitinfo" => help::print_gitinfo_help(),
                    "gin" => help::print_gitinfo_help(),
                    "dbview" => help::print_dbview_help(),
                    "dbv" => help::print_dbview_help(),
                    "request" => help::print_request_help(),
                    "req" => help::print_request_help(),
                    "hexview" => help::print_hexview_help(),
                    "hexv" => help::print_hexview_help(),
                    "sysinfo" => help::print_sysinfo_help(),
                    "sys" => help::print_sysinfo_help(),
                    "anispeak" => help::print_anispeak_help(),
                    "path" => help::print_path_help(),
                    "df" => help::print_df_help(),
                    "whoami" => help::print_whoami_help(),
                    "sockets" => help::print_sockets_help(),
                    "wc" => help::print_wc_help(),
                    "ln" => help::print_ln_help(),
                    "chmod" => help::print_chmod_help(),
                    "ls" => help::print_list_help(),
                    "touch" => help::print_create_help(),
                    "tar" => help::print_archive_help(),
                    "mv" => help::print_move_help(),
                    "cp" => help::print_copy_help(),
                    "rm" => help::print_remove_help(),
                    "ff" => help::print_fastfetch_help(),
                    "pmon" => help::print_pmon_help(),
                    "ptop" => help::print_pmon_help(),
                    "smon" => help::print_monitor_help(),
                    "watch" => help::print_watch_help(),
                    "nettop" => help::print_nettop_help(),
                    "ntop" => help::print_nettop_help(),
                    "dua" => help::print_dua_help(),
                    "ncdu" => help::print_dua_help(),
                    "browse" => help::print_fm_help(),
                    "fm" => help::print_fm_help(),
                    "gitv" => help::print_gitv_help(),
                    "edit" => help::print_edit_help(),
                    "ed"   => help::print_edit_help(),
                    "scrape" | "dl" => help::print_scrape_help(),
                    "sort" => help::print_sort_help(),
                    "clock" => help::print_clock_help(),
                    "text" => help::print_text_help(),
                    "globe" => help::print_globe_help(),
                    "log" => help::print_log_help(),
                    "life" => help::print_life_help(),
                    "tee" => help::print_tee_help(),
                    "head" => help::print_head_help(),
                    "tail" => help::print_tail_help(),
                    "stat" => help::print_stat_help(),
                    "help" => help::print_general_help(),
                    _ => {
                        eprintln!("Error: Unknown action '{}'", args[2]);
                        help::print_general_help();
                    }
                }
            } else {
                help::print_general_help();
            }
        }
        "scrape" => {
            let mut options = ScrapeOptions {
                dest: "./output".to_string(),
                depth: 1,
                max_pages: 10,
                max_size_bytes: 50 * 1024 * 1024, // 50 MiB
                max_links: 100,
                timeout_secs: 30,
                ..Default::default()
            };
            let mut url: Option<String> = None;
            let mut valid = true;
            let mut rest = args[2..].iter().peekable();

            while let Some(arg) = rest.next() {
                match arg.as_str() {
                    "--format" => {
                        if let Some(val) = rest.next() {
                            if val.starts_with('-') {
                                eprintln!("Error: --format requires a value.");
                                valid = false; break;
                            }
                            options.formats.push(val.to_string());
                        } else {
                            eprintln!("Error: --format requires a value.");
                            valid = false; break;
                        }
                    }
                    "--dest" => {
                        if let Some(val) = rest.next() {
                            if val.starts_with('-') {
                                eprintln!("Error: --dest requires a directory path.");
                                valid = false; break;
                            }
                            options.dest = val.to_string();
                        } else {
                            eprintln!("Error: --dest requires a directory path.");
                            valid = false; break;
                        }
                    }
                    "--depth" => {
                        if let Some(val) = rest.next() {
                            match val.parse::<usize>() {
                                Ok(n) => options.depth = n,
                                Err(_) => {
                                    eprintln!("Error: --depth requires a non-negative integer.");
                                    valid = false; break;
                                }
                            }
                        } else {
                            eprintln!("Error: --depth requires a value.");
                            valid = false; break;
                        }
                    }
                    "--max-pages" => {
                        if let Some(val) = rest.next() {
                            match val.parse::<usize>() {
                                Ok(n) if n > 0 => options.max_pages = n,
                                _ => {
                                    eprintln!("Error: --max-pages requires a positive integer.");
                                    valid = false; break;
                                }
                            }
                        } else {
                            eprintln!("Error: --max-pages requires a value.");
                            valid = false; break;
                        }
                    }
                    "--max-size" => {
                        if let Some(val) = rest.next() {
                            match scrape_parse_size(val) {
                                Some(n) => options.max_size_bytes = n,
                                None => {
                                    eprintln!("Error: --max-size requires a size like '50M', '1G', '500K', or a byte count.");
                                    valid = false; break;
                                }
                            }
                        } else {
                            eprintln!("Error: --max-size requires a value.");
                            valid = false; break;
                        }
                    }
                    "--max-links" => {
                        if let Some(val) = rest.next() {
                            match val.parse::<usize>() {
                                Ok(n) if n > 0 => options.max_links = n,
                                _ => {
                                    eprintln!("Error: --max-links requires a positive integer.");
                                    valid = false; break;
                                }
                            }
                        } else {
                            eprintln!("Error: --max-links requires a value.");
                            valid = false; break;
                        }
                    }
                    "--timeout" => {
                        if let Some(val) = rest.next() {
                            match val.parse::<u64>() {
                                Ok(n) if n > 0 => options.timeout_secs = n,
                                _ => {
                                    eprintln!("Error: --timeout requires a positive integer (seconds).");
                                    valid = false; break;
                                }
                            }
                        } else {
                            eprintln!("Error: --timeout requires a value.");
                            valid = false; break;
                        }
                    }
                    "--rate-limit" => {
                        if let Some(val) = rest.next() {
                            match val.parse::<u64>() {
                                Ok(n) => options.rate_limit_ms = n,
                                Err(_) => {
                                    eprintln!("Error: --rate-limit requires a non-negative integer (milliseconds).");
                                    valid = false; break;
                                }
                            }
                        } else {
                            eprintln!("Error: --rate-limit requires a value.");
                            valid = false; break;
                        }
                    }
                    "--user-agent" => {
                        if let Some(val) = rest.next() {
                            if val.starts_with('-') {
                                eprintln!("Error: --user-agent requires a string value.");
                                valid = false; break;
                            }
                            options.user_agent = Some(val.to_string());
                        } else {
                            eprintln!("Error: --user-agent requires a value.");
                            valid = false; break;
                        }
                    }
                    "--include-video" => options.include_video = true,
                    "--include-audio" => options.include_audio = true,
                    "--no-images"     => options.no_images     = true,
                    "--same-domain"   => options.same_domain   = true,
                    "--ignore-robots" => options.ignore_robots = true,
                    "--dry-run"       => options.dry_run       = true,
                    "--overwrite"     => options.overwrite     = true,
                    "--verbose" | "-v" => options.verbose      = true,
                    other if other.starts_with('-') => {
                        eprintln!("Error: Unknown switch '{}' for scrape.", other);
                        valid = false; break;
                    }
                    other => {
                        if url.is_some() {
                            eprintln!("Error: Only one URL argument is allowed.");
                            valid = false; break;
                        }
                        url = Some(other.to_string());
                    }
                }
            }

            if valid && url.is_none() {
                eprintln!("Error: 'scrape' requires a URL argument.");
                valid = false;
            }
            if valid && options.formats.is_empty() {
                eprintln!("Error: --format is mandatory for 'scrape'.");
                valid = false;
            }
            // Validate URL scheme.
            if valid {
                let u = url.as_deref().unwrap();
                if !u.starts_with("http://") && !u.starts_with("https://") {
                    eprintln!("Error: URL must begin with http:// or https://");
                    valid = false;
                }
            }

            if valid {
                ir_cli_utility::scrape(url.as_deref().unwrap(), options);
            } else {
                help::print_scrape_help();
            }
        }
        "sort" => {
            let mut options = SortOptions::default();
            let mut positionals: Vec<String> = Vec::new();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-r" || arg == "--reverse" {
                    options.reverse = true;
                } else if arg == "-n" || arg == "--numeric-sort" || arg == "--numeric" {
                    options.numeric = true;
                } else if arg == "-u" || arg == "--unique" {
                    options.unique = true;
                } else if arg == "-f" || arg == "--ignore-case" {
                    options.ignore_case = true;
                } else if arg == "--field" {
                    if let Some(val) = args_iter.next() {
                        match val.parse::<usize>() {
                            Ok(n) => options.field = n,
                            Err(_) => {
                                eprintln!("Error: --field requires a positive integer.");
                                valid = false; break;
                            }
                        }
                    } else {
                        eprintln!("Error: --field requires a value.");
                        valid = false; break;
                    }
                } else if arg == "--separator" {
                    if let Some(val) = args_iter.next() {
                        if val.len() != 1 {
                            eprintln!("Error: --separator requires a single character.");
                            valid = false; break;
                        }
                        options.separator = val.chars().next();
                    } else {
                        eprintln!("Error: --separator requires a value.");
                        valid = false; break;
                    }
                } else if arg == "-c" || arg == "--check" {
                    options.check = true;
                } else if arg.starts_with('-') && arg.len() > 1 {
                    for char in arg.chars().skip(1) {
                        match char {
                            'r' => options.reverse = true,
                            'n' => options.numeric = true,
                            'u' => options.unique = true,
                            'f' => options.ignore_case = true,
                            'c' => options.check = true,
                            _ => {
                                eprintln!("Error: Unknown switch '-{}' for sort.", char);
                                valid = false; break;
                            }
                        }
                    }
                    if !valid { break; }
                } else {
                    positionals.push(arg.clone());
                }
            }

            if valid {
                ir_cli_utility::sort(positionals, options);
            } else {
                help::print_sort_help();
                std::process::exit(1);
            }
        }
        "clock" => {
            let mut options = ir_cli_utility::ClockOptions::default();
            let mut valid = true;
            let mut args_iter = args[2..].iter().peekable();

            while let Some(arg) = args_iter.next() {
                if arg == "-t" || arg == "--timer" {
                    match args_iter.next() {
                        Some(t) => options.timer_duration = Some(t.clone()),
                        None => {
                            eprintln!("Error: --timer requires a duration string (e.g. 5m30s).");
                            valid = false;
                            break;
                        }
                    }
                } else if arg == "-m" || arg == "--mode" {
                    match args_iter.next() {
                        Some(m) => {
                            let m_lower = m.to_lowercase();
                            if m_lower == "clock" || m_lower == "stopwatch" || m_lower == "timer" {
                                options.mode = Some(m_lower);
                            } else {
                                eprintln!("Error: Invalid mode '{}'. Supported: clock, stopwatch, timer", m);
                                valid = false;
                                break;
                            }
                        }
                        None => {
                            eprintln!("Error: --mode requires a mode value.");
                            valid = false;
                            break;
                        }
                    }
                } else if arg.starts_with('-') {
                    eprintln!("Error: Unknown switch '{}' for clock.", arg);
                    valid = false;
                    break;
                } else {
                    eprintln!("Error: 'clock' action does not accept positional arguments.");
                    valid = false;
                    break;
                }
            }

            if valid {
                ir_cli_utility::clock(options);
            } else {
                help::print_clock_help();
                std::process::exit(1);
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
