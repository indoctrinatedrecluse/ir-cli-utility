use crate::NettopOptions;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::{get_net_interfaces, poll_keyboard_input};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::{get_net_interfaces, poll_keyboard_input, set_raw_mode};

fn format_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec < 1024.0 {
        format!("{:.0} B/s", bytes_per_sec)
    } else if bytes_per_sec < 1024.0 * 1024.0 {
        format!("{:.2} KB/s", bytes_per_sec / 1024.0)
    } else {
        format!("{:.2} MB/s", bytes_per_sec / 1024.0 / 1024.0)
    }
}

fn draw_sparkline(history: &[f64], max_val: f64, color_code: &str) -> Vec<String> {
    let mut lines = vec![String::new(); 4];
    let block_chars = [' ', ' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    for y_reverse in 0..4 {
        let y = 3 - y_reverse;
        let mut line = String::new();
        line.push_str(color_code);

        for &val in history {
            if max_val <= 0.1 {
                line.push(' ');
                continue;
            }
            let height = (val / max_val) * 4.0; // range 0..=4
            let min_y = y as f64;
            let max_y = (y + 1) as f64;

            if height >= max_y {
                line.push('█');
            } else if height <= min_y {
                line.push(' ');
            } else {
                let frac = height - min_y;
                let idx = (frac * 8.0).round().clamp(0.0, 8.0) as usize;
                line.push(block_chars[idx]);
            }
        }
        line.push_str("\x1b[0m");
        lines[y_reverse] = line;
    }
    lines
}

pub fn nettop(options: NettopOptions) {
    #[cfg(target_os = "linux")]
    let _raw_guard = set_raw_mode();

    let mut last_update = Instant::now() - Duration::from_millis(options.delay_ms);
    let mut rx_history = vec![0.0; 40];
    let mut tx_history = vec![0.0; 40];

    let mut last_rx = 0u64;
    let mut last_tx = 0u64;
    let mut selected_idx = 0usize;

    // Clear screen
    print!("\x1b[2J\x1b[H");
    let _ = stdout().flush();

    loop {
        // Keyboard input polling
        while let Some(input) = poll_keyboard_input() {
            match input {
                'q' | 'Q' | '\x1b' => {
                    print!("\x1b[2J\x1b[H");
                    let _ = stdout().flush();
                    return;
                }
                'i' | 'I' => {
                    if let Ok(ifaces) = get_net_interfaces() {
                        if !ifaces.is_empty() {
                            selected_idx = (selected_idx + 1) % ifaces.len();
                            // Reset totals to avoid huge spike in speed
                            last_rx = 0;
                            last_tx = 0;
                            rx_history = vec![0.0; 40];
                            tx_history = vec![0.0; 40];
                        }
                    }
                }
                _ => {}
            }
        }

        let now = Instant::now();
        if now.duration_since(last_update).as_millis() >= options.delay_ms as u128 {
            let elapsed_secs = now.duration_since(last_update).as_secs_f64();
            last_update = now;

            if let Ok(ifaces) = get_net_interfaces() {
                if !ifaces.is_empty() {
                    if selected_idx >= ifaces.len() {
                        selected_idx = 0;
                    }
                    let iface = &ifaces[selected_idx];

                    let rx_speed = if last_rx > 0 {
                        (iface.rx_bytes.saturating_sub(last_rx) as f64 / elapsed_secs).max(0.0)
                    } else {
                        0.0
                    };
                    let tx_speed = if last_tx > 0 {
                        (iface.tx_bytes.saturating_sub(last_tx) as f64 / elapsed_secs).max(0.0)
                    } else {
                        0.0
                    };

                    last_rx = iface.rx_bytes;
                    last_tx = iface.tx_bytes;

                    // Push to history
                    rx_history.remove(0);
                    rx_history.push(rx_speed);

                    tx_history.remove(0);
                    tx_history.push(tx_speed);

                    // Render TUI
                    print!("\x1b[H");

                    // Title Header
                    println!("\x1b[1;34m=== Network Traffic Monitor (nettop) ===\x1b[0m\x1b[K");
                    println!("Interface: \x1b[33m{}\x1b[0m\x1b[K", iface.name);
                    println!("Controls:  \x1b[36m'i'\x1b[0m next interface | \x1b[36m'q'\x1b[0m quit\x1b[K");
                    println!("\x1b[34m--------------------------------------------------------------------------------\x1b[0m\x1b[K");

                    // Stats
                    println!(
                        "Download Speed: \x1b[1;32m{:<12}\x1b[0m  |  Upload Speed: \x1b[1;35m{:<12}\x1b[0m\x1b[K",
                        format_speed(rx_speed),
                        format_speed(tx_speed)
                    );
                    println!("\x1b[K");

                    // Graphs
                    let rx_max = rx_history.iter().cloned().fold(0.0, f64::max);
                    let tx_max = tx_history.iter().cloned().fold(0.0, f64::max);

                    let rx_graph = draw_sparkline(&rx_history, rx_max, "\x1b[32m");
                    let tx_graph = draw_sparkline(&tx_history, tx_max, "\x1b[35m");

                    println!("Download History (Max: {}):", format_speed(rx_max));
                    for line in rx_graph {
                        println!("  {}\x1b[K", line);
                    }
                    println!("\x1b[K");

                    println!("Upload History (Max: {}):", format_speed(tx_max));
                    for line in tx_graph {
                        println!("  {}\x1b[K", line);
                    }
                    println!("\x1b[K");
                    
                    // Clear trailing rows if terminal was resized
                    print!("\x1b[J");
                    let _ = stdout().flush();
                } else {
                    print!("\x1b[H\x1b[31mNo active network interfaces found.\x1b[0m\x1b[K");
                    let _ = stdout().flush();
                }
            }
        }

        std::thread::sleep(Duration::from_millis(20));
    }
}
