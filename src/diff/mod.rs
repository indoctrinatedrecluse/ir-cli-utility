use crate::DiffOptions;
use std::fs;

pub fn diff(left: &str, right: &str, options: DiffOptions) {
    let left_content = match fs::read_to_string(left) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading '{}': {}", left, error);
            return;
        }
    };

    let right_content = match fs::read_to_string(right) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading '{}': {}", right, error);
            return;
        }
    };

    let left_lines: Vec<&str> = left_content.lines().collect();
    let right_lines: Vec<&str> = right_content.lines().collect();

    if lines_equal(&left_lines, &right_lines, options.ignore_case) {
        return;
    }

    if options.brief {
        println!("Files {} and {} differ", left, right);
    } else if options.unified {
        print_unified(left, right, &left_lines, &right_lines, options.ignore_case);
    } else {
        print_normal(&left_lines, &right_lines, options.ignore_case);
    }
}

fn lines_equal(left: &[&str], right: &[&str], ignore_case: bool) -> bool {
    left.len() == right.len()
        && left.iter().zip(right.iter()).all(|(left_line, right_line)| {
            line_equal(left_line, right_line, ignore_case)
        })
}

fn line_equal(left: &str, right: &str, ignore_case: bool) -> bool {
    if ignore_case {
        left.eq_ignore_ascii_case(right)
    } else {
        left == right
    }
}

fn print_normal(left: &[&str], right: &[&str], ignore_case: bool) {
    let max_len = left.len().max(right.len());

    for index in 0..max_len {
        match (left.get(index), right.get(index)) {
            (Some(left_line), Some(right_line)) if line_equal(left_line, right_line, ignore_case) => {}
            (Some(left_line), Some(right_line)) => {
                println!("{}c{}", index + 1, index + 1);
                println!("< {}", left_line);
                println!("---");
                println!("> {}", right_line);
            }
            (Some(left_line), None) => {
                println!("{}d{}", index + 1, right.len());
                println!("< {}", left_line);
            }
            (None, Some(right_line)) => {
                println!("{}a{}", left.len(), index + 1);
                println!("> {}", right_line);
            }
            (None, None) => {}
        }
    }
}

fn print_unified(left_path: &str, right_path: &str, left: &[&str], right: &[&str], ignore_case: bool) {
    println!("--- {}", left_path);
    println!("+++ {}", right_path);

    let max_len = left.len().max(right.len());
    for index in 0..max_len {
        match (left.get(index), right.get(index)) {
            (Some(left_line), Some(right_line)) if line_equal(left_line, right_line, ignore_case) => {
                println!(" {}", left_line);
            }
            (Some(left_line), Some(right_line)) => {
                println!("-{}", left_line);
                println!("+{}", right_line);
            }
            (Some(left_line), None) => println!("-{}", left_line),
            (None, Some(right_line)) => println!("+{}", right_line),
            (None, None) => {}
        }
    }
}
