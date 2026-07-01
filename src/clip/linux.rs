use std::process::{Command, Stdio};
use std::io::Write;

fn has_tool(cmd_name: &str) -> bool {
    // Check if the command exists by running it with --help or similar
    Command::new(cmd_name)
        .arg(if cmd_name == "xclip" { "-version" } else { "--version" })
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn run_command_with_input(cmd_name: &str, args: &[&str], input: &str) -> Result<(), String> {
    let mut child = Command::new(cmd_name)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to spawn {}: {}", cmd_name, e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes()).map_err(|e| e.to_string())?;
    }

    let status = child.wait().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{} exited with non-zero status", cmd_name))
    }
}

fn run_command_capture_output(cmd_name: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd_name)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", cmd_name, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format!("{} exited with non-zero status", cmd_name))
    }
}

pub fn get_clipboard() -> Result<String, String> {
    if has_tool("wl-paste") {
        run_command_capture_output("wl-paste", &["-n"])
    } else if has_tool("xclip") {
        run_command_capture_output("xclip", &["-selection", "clipboard", "-o"])
    } else if has_tool("xsel") {
        run_command_capture_output("xsel", &["--clipboard", "--output"])
    } else {
        Err("No clipboard utility found. Please install 'wl-clipboard', 'xclip', or 'xsel'.".to_string())
    }
}

pub fn set_clipboard(text: &str) -> Result<(), String> {
    if has_tool("wl-copy") {
        run_command_with_input("wl-copy", &[], text)
    } else if has_tool("xclip") {
        run_command_with_input("xclip", &["-selection", "clipboard"], text)
    } else if has_tool("xsel") {
        run_command_with_input("xsel", &["--clipboard", "--input"], text)
    } else {
        Err("No clipboard utility found. Please install 'wl-clipboard', 'xclip', or 'xsel'.".to_string())
    }
}

pub fn clear_clipboard() -> Result<(), String> {
    if has_tool("wl-copy") {
        run_command_with_input("wl-copy", &["--clear"], "")
    } else if has_tool("xclip") {
        run_command_with_input("xclip", &["-selection", "clipboard"], "")
    } else if has_tool("xsel") {
        run_command_with_input("xsel", &["--clipboard", "--clear"], "")
    } else {
        Err("No clipboard utility found. Please install 'wl-clipboard', 'xclip', or 'xsel'.".to_string())
    }
}
