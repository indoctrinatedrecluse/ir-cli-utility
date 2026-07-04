use crate::PmonOptions;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::{get_system_stats, get_processes, kill_process, poll_keyboard_input, RawInput};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::{get_system_stats, get_processes, kill_process, poll_keyboard_input, RawInput};

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f64,
    pub rss_bytes: u64,
    pub state: char,
    pub cpu_time_secs: u64,
}

fn format_mem(bytes: u64) -> String {
    let kib = bytes as f64 / 1024.0;
    if kib >= 1024.0 * 1024.0 {
        format!("{:.1} GiB", kib / 1024.0 / 1024.0)
    } else if kib >= 1024.0 {
        format!("{:.1} MiB", kib / 1024.0)
    } else {
        format!("{:.1} KiB", kib)
    }
}

fn format_time(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

fn render_bar(percentage: f64, width: usize) -> String {
    let filled_count = ((percentage / 100.0) * width as f64).round() as usize;
    let filled_count = filled_count.min(width);
    
    let mut bar = String::new();
    // Use fancy color coding: green/cyan -> yellow -> red
    for i in 0..width {
        if i < filled_count {
            let pct = (i as f64 / width as f64) * 100.0;
            if pct < 50.0 {
                bar.push_str("\x1b[32m█\x1b[0m"); // Green
            } else if pct < 80.0 {
                bar.push_str("\x1b[33m█\x1b[0m"); // Yellow
            } else {
                bar.push_str("\x1b[31m█\x1b[0m"); // Red
            }
        } else {
            bar.push_str("░"); // Gray shade block
        }
    }
    bar
}

pub fn pmon(options: PmonOptions) {
    let mut sort_by = "cpu";
    let mut kill_mode = false;
    let mut kill_input = String::new();
    let mut status_message: Option<(String, bool)> = None; // (message, is_error)
    
    // Clear screen first
    print!("\x1b[2J\x1b[H");
    let _ = stdout().flush();

    #[cfg(target_os = "linux")]
    let _raw_guard = linux::set_raw_mode();

    let mut last_update = Instant::now() - Duration::from_millis(options.delay_ms);
    let mut cpu_stats_cache = std::collections::HashMap::new();

    // Loop
    loop {
        let now = Instant::now();
        let mut force_redraw = false;

        // Non-blocking keyboard input polling
        while let Some(input) = poll_keyboard_input() {
            match input {
                RawInput::Char(c) => {
                    if kill_mode {
                        if c.is_digit(10) {
                            kill_input.push(c);
                            force_redraw = true;
                        } else if c == 'q' || c == '\x1b' { // esc or q cancels
                            kill_mode = false;
                            kill_input.clear();
                            force_redraw = true;
                        }
                    } else {
                        match c {
                            'q' => {
                                // Quit raw mode and restore screen
                                print!("\x1b[2J\x1b[H");
                                let _ = stdout().flush();
                                return;
                            }
                            'c' => { sort_by = "cpu"; force_redraw = true; }
                            'm' => { sort_by = "mem"; force_redraw = true; }
                            'n' => { sort_by = "name"; force_redraw = true; }
                            'p' => { sort_by = "pid"; force_redraw = true; }
                            'k' => {
                                kill_mode = true;
                                kill_input.clear();
                                status_message = None;
                                force_redraw = true;
                            }
                            _ => {}
                        }
                    }
                }
                RawInput::Enter => {
                    if kill_mode {
                        if let Ok(pid) = kill_input.trim().parse::<u32>() {
                            match kill_process(pid) {
                                Ok(_) => {
                                    status_message = Some((format!("Successfully killed process {}", pid), false));
                                }
                                Err(e) => {
                                    status_message = Some((format!("Failed to kill process {}: {}", pid, e), true));
                                }
                            }
                        } else {
                            status_message = Some(("Invalid PID".to_string(), true));
                        }
                        kill_mode = false;
                        kill_input.clear();
                        force_redraw = true;
                    }
                }
                RawInput::Backspace => {
                    if kill_mode {
                        kill_input.pop();
                        force_redraw = true;
                    }
                }
            }
        }

        // Check if update is due
        if force_redraw || now.duration_since(last_update).as_millis() >= options.delay_ms as u128 {
            last_update = now;

            // Fetch statistics
            let (cpu_pct, mem_used, mem_total) = get_system_stats().unwrap_or((0.0, 0, 1));
            let mut processes = get_processes(&mut cpu_stats_cache).unwrap_or_else(|_| Vec::new());

            // Sort
            match sort_by {
                "cpu" => processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
                "mem" => processes.sort_by(|a, b| b.rss_bytes.cmp(&a.rss_bytes)),
                "name" => processes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
                "pid" => processes.sort_by(|a, b| a.pid.cmp(&b.pid)),
                _ => {}
            }

            // Position cursor to top left
            print!("\x1b[H");

            // Header line
            println!(
                "\x1b[1;36mIR PROCESS MONITOR (pmon)\x1b[0m -- Delay: \x1b[1;33m{}ms\x1b[0m -- Sort: \x1b[1;32m{}\x1b[0m                 ",
                options.delay_ms,
                sort_by.to_uppercase()
            );
            println!("\x1b[36m================================================================================\x1b[0m");

            // Draw system CPU
            let cpu_bar = render_bar(cpu_pct, 40);
            println!("  \x1b[1mCPU\x1b[0m:    [{}] {:>5.1}%", cpu_bar, cpu_pct);

            // Draw system Memory
            let mem_pct = (mem_used as f64 / mem_total as f64) * 100.0;
            let mem_bar = render_bar(mem_pct, 40);
            println!(
                "  \x1b[1mMEM\x1b[0m:    [{}] {:>5.1}%  ({} / {})",
                mem_bar,
                mem_pct,
                format_mem(mem_used),
                format_mem(mem_total)
            );

            // Draw processes counts
            let total_procs = processes.len();
            let running_procs = processes.iter().filter(|p| p.state == 'R').count();
            let sleeping_procs = total_procs.saturating_sub(running_procs);
            println!(
                "  Processes: \x1b[1;32m{}\x1b[0m total -- \x1b[1;33m{}\x1b[0m running, \x1b[1;34m{}\x1b[0m sleeping           ",
                total_procs, running_procs, sleeping_procs
            );
            println!("\x1b[36m--------------------------------------------------------------------------------\x1b[0m");

            // Table headers
            println!(
                "  \x1b[1;36m{:>8}  {:>7}  {:>11}  {:^5}  {:>8}  {}\x1b[0m",
                "PID", "%CPU", "MEM", "STATE", "TIME", "COMMAND"
            );

            // Retrieve terminal size limit or default to 20 processes
            let display_limit = 20;
            for proc in processes.iter().take(display_limit) {
                // Color CPU or memory column if active sort
                let cpu_str = if sort_by == "cpu" {
                    format!("\x1b[1;33m{:>6.1}%\x1b[0m", proc.cpu_usage)
                } else {
                    format!("{:>6.1}%", proc.cpu_usage)
                };

                let mem_str = if sort_by == "mem" {
                    format!("\x1b[1;33m{:>11}\x1b[0m", format_mem(proc.rss_bytes))
                } else {
                    format!("{:>11}", format_mem(proc.rss_bytes))
                };

                let state_str = match proc.state {
                    'R' => "\x1b[1;32mR\x1b[0m",
                    'S' => "\x1b[34mS\x1b[0m",
                    other => &other.to_string(),
                };

                // Clear to end of line to overwrite leftovers: \x1b[K
                println!(
                    "  {:>8}  {}  {}  {:^5}  {:>8}  {}\x1b[K",
                    proc.pid,
                    cpu_str,
                    mem_str,
                    state_str,
                    format_time(proc.cpu_time_secs),
                    proc.name
                );
            }
            println!("\x1b[36m--------------------------------------------------------------------------------\x1b[0m");

            // Interactive inputs & Status messages
            if kill_mode {
                print!(
                    "  \x1b[1;31mKILL MODE\x1b[0m -- Enter PID to kill: \x1b[1;33m{}\x1b[0m_ (Enter: kill, Esc: cancel)\x1b[K\r",
                    kill_input
                );
            } else if let Some((msg, is_error)) = &status_message {
                let color_prefix = if *is_error { "\x1b[1;31m" } else { "\x1b[1;32m" };
                print!("  {}{} \x1b[0m\x1b[K\r", color_prefix, msg);
            } else {
                print!("  \x1b[1;32m[q]\x1b[0m Quit  \x1b[1;32m[c]\x1b[0m CPU  \x1b[1;32m[m]\x1b[0m Mem  \x1b[1;32m[n]\x1b[0m Name  \x1b[1;32m[p]\x1b[0m PID  \x1b[1;32m[k]\x1b[0m Kill\x1b[K\r");
            }
            let _ = stdout().flush();
        }

        // Wait a short time to avoid pinning CPU
        std::thread::sleep(Duration::from_millis(20));
    }
}
