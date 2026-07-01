use std::process::{Command, Stdio};
use std::time::Instant;

pub fn run_time(cmd_args: Vec<String>) {
    if cmd_args.is_empty() {
        eprintln!("Error: 'time' action requires a command to execute.");
        std::process::exit(1);
    }

    let command = &cmd_args[0];
    let args = &cmd_args[1..];

    let start = Instant::now();
    let mut child = match Command::new(command)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to spawn command '{}': {}", command, e);
            std::process::exit(1);
        }
    };

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Failed waiting for child process: {}", e);
            std::process::exit(1);
        }
    };

    let elapsed = start.elapsed();
    let seconds = elapsed.as_secs_f64();

    // Print timing results to stderr so it doesn't pollute stdout redirections
    eprintln!("\nExecution Time: {:.3}s", seconds);

    // Exit with same status code as child process
    if let Some(code) = status.code() {
        std::process::exit(code);
    } else {
        std::process::exit(1);
    }
}
