use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use ureq;
use crate::tui_util::{self, Key};

pub struct RequestConfig {
    pub initial_url: Option<String>,
}

#[derive(Clone)]
struct ResponseData {
    status: u16,
    status_text: String,
    #[allow(dead_code)]
    headers: Vec<(String, String)>,
    body: String,
    elapsed_ms: u128,
}

#[derive(Clone)]
enum RequestState {
    Idle,
    Loading(Instant),
    Success(ResponseData),
    Error(String),
}

pub fn run_request(config: RequestConfig) -> io::Result<()> {
    let _raw_mode = tui_util::set_raw_mode();

    // Enter alternate screen buffer
    print!("\x1B[?1049h\x1B[?25l\x1B[2J\x1B[H");
    let _ = io::stdout().flush();

    let result = request_loop(config);

    // Exit alternate screen buffer
    print!("\x1B[?1049l\x1B[?25h\x1B[0m");
    let _ = io::stdout().flush();

    result
}

fn request_loop(config: RequestConfig) -> io::Result<()> {
    let mut current_tab = 0; // 0: Method/URL, 1: Headers, 2: Body, 3: Response
    let mut active_field = 0; // Tab-specific focus indices

    let mut url = config.initial_url.unwrap_or_else(|| "https://httpbin.org/get".to_string());
    let methods = vec!["GET", "POST", "PUT", "DELETE"];
    let mut selected_method_idx = 0;

    let mut headers = vec![
        ("User-Agent".to_string(), "ir-request/0.1.0".to_string()),
        ("Accept".to_string(), "application/json".to_string()),
    ];
    let mut new_header_key = String::new();
    let mut new_header_val = String::new();

    let mut body = String::new();

    let req_state = Arc::new(Mutex::new(RequestState::Idle));
    let mut response_scroll = 0;

    let (mut cols, mut rows) = tui_util::terminal_size();

    loop {
        let (new_cols, new_rows) = tui_util::terminal_size();
        if new_cols != cols || new_rows != rows {
            cols = new_cols;
            rows = new_rows;
            print!("\x1B[2J\x1B[H");
            let _ = io::stdout().flush();
        }

        // Render Header
        let mut frame = String::new();
        frame.push_str("\x1B[H");
        frame.push_str("\x1B[1;30;45m ir request TUI \x1B[0m  ");

        let tab_labels = &["[1] Method & URL", "[2] Headers", "[3] Body Payload", "[4] Response Inspector"];
        for (i, label) in tab_labels.iter().enumerate() {
            if i == current_tab {
                frame.push_str(&format!("\x1B[1;35m> {} <\x1B[0m  ", label));
            } else {
                frame.push_str(&format!("  {}  ", label));
            }
        }
        frame.push_str("\x1B[K\n");
        frame.push_str(&"━".repeat(cols as usize));
        frame.push_str("\x1B[K\n");

        let content_height = rows.saturating_sub(6) as usize;

        // Render active Tab body
        match current_tab {
            0 => {
                // Tab 1: Method & URL Builder
                frame.push_str("\n  \x1B[1;37mHTTP METHOD:\x1B[0m\n  ");
                for (idx, method) in methods.iter().enumerate() {
                    let is_active_method = idx == selected_method_idx;
                    let is_focused = active_field == 0;
                    if is_active_method && is_focused {
                        frame.push_str(&format!(" \x1B[1;30;42m> {} <\x1B[0m ", method));
                    } else if is_active_method {
                        frame.push_str(&format!(" \x1B[1;32m[{}]\x1B[0m ", method));
                    } else if is_focused && idx == selected_method_idx {
                        frame.push_str(&format!("  {}  ", method));
                    } else {
                        frame.push_str(&format!("  {}  ", method));
                    }
                }
                frame.push_str("\x1B[K\n\n");

                frame.push_str("  \x1B[1;37mREQUEST TARGET URL:\x1B[0m\n  ");
                let is_url_focused = active_field == 1;
                if is_url_focused {
                    frame.push_str(&format!(" \x1B[4m{}_ \x1B[0m", url));
                } else {
                    frame.push_str(&format!("  {}", url));
                }
                frame.push_str("\x1B[K\n\n");

                frame.push_str("\n\n  \x1B[90m[Tab] to toggle focus between Method & URL input. [Enter] to fire request.\x1B[0m\n");
                for _ in 8..content_height {
                    frame.push_str("\x1B[K\n");
                }
            }
            1 => {
                // Tab 2: Custom Headers Editor
                frame.push_str("  \x1B[1;37mConfigure Request Headers:\x1B[0m\n");
                frame.push_str(&format!("    {:<30} | {}\n", "HEADER KEY", "HEADER VALUE"));
                frame.push_str(&format!("    {}\n", "━".repeat(cols.saturating_sub(8) as usize)));

                let list_height = content_height.saturating_sub(6);
                for i in 0..list_height {
                    if i < headers.len() {
                        let (k, v) = &headers[i];
                        let is_focused = active_field == i;
                        if is_focused {
                            frame.push_str(&format!("  \x1B[1;32m> \x1B[33m{:<28} | {}\x1B[0m\n", k, v));
                        } else {
                            frame.push_str(&format!("    {:<28} | {}\n", k, v));
                        }
                    } else if i == headers.len() {
                        // "Add New Header" row
                        let is_focused = active_field == headers.len();
                        let key_disp = if new_header_key.is_empty() { "Add Key..." } else { &new_header_key };
                        let val_disp = if new_header_val.is_empty() { "Add Value..." } else { &new_header_val };
                        if is_focused {
                            frame.push_str(&format!("  \x1B[1;32m+ \x1B[36m{:<28} | {}\x1B[0m\n", key_disp, val_disp));
                        } else {
                            frame.push_str(&format!("    \x1B[90m{:<28} | {}\x1B[0m\n", key_disp, val_disp));
                        }
                    } else {
                        frame.push_str("\x1B[K\n");
                    }
                }
                frame.push_str("\n  \x1B[90m[Backspace] on selected header deletes it. Edit fields below. [Enter] to insert.\x1B[0m\n");
            }
            2 => {
                // Tab 3: Payload Body Editor
                frame.push_str("  \x1B[1;37mRequest Payload Body (raw text / JSON):\x1B[0m\n");
                frame.push_str(&format!("  {}\n", "━".repeat(cols.saturating_sub(4) as usize)));
                
                let is_focused = active_field == 0;
                let display_body = if body.is_empty() {
                    if is_focused { "Type payload body here..." } else { "No payload body configured." }
                } else {
                    &body
                };
                
                if is_focused {
                    frame.push_str(&format!("  \x1B[4m{}_ \x1B[0m\n", display_body));
                } else {
                    frame.push_str(&format!("  {}\n", display_body));
                }

                for _ in 3..content_height {
                    frame.push_str("\x1B[K\n");
                }
            }
            _ => {
                // Tab 4: Response Inspector
                let state = req_state.lock().unwrap().clone();
                match state {
                    RequestState::Idle => {
                        frame.push_str("\n  \x1B[1;33mNo response yet. Fire request in Tab 1.\x1B[0m\n");
                        for _ in 2..content_height {
                            frame.push_str("\x1B[K\n");
                        }
                    }
                    RequestState::Loading(start) => {
                        let dots = ".".repeat((start.elapsed().as_millis() / 200) as usize % 4);
                        frame.push_str(&format!("\n  \x1B[1;36mSending request{} \x1B[0m\n", dots));
                        for _ in 2..content_height {
                            frame.push_str("\x1B[K\n");
                        }
                    }
                    RequestState::Error(err) => {
                        frame.push_str(&format!("\n  \x1B[1;31mRequest Error:\x1B[0m {}\n", err));
                        for _ in 2..content_height {
                            frame.push_str("\x1B[K\n");
                        }
                    }
                    RequestState::Success(res) => {
                        let status_color = if res.status < 300 { "\x1B[1;32m" } else { "\x1B[1;31m" };
                        frame.push_str(&format!(
                            "  \x1B[1mStatus:\x1B[0m {}{} {}\x1B[0m  |  \x1B[1mTime:\x1B[0m {} ms\n",
                            status_color, res.status, res.status_text, res.elapsed_ms
                        ));
                        frame.push_str(&format!("  {}\n", "━".repeat(cols.saturating_sub(4) as usize)));

                        let split_h = content_height.saturating_sub(3);
                        let response_lines: Vec<&str> = res.body.lines().collect();

                        for y in 0..split_h {
                            let line_idx = y + response_scroll;
                            if line_idx < response_lines.len() {
                                let line = response_lines[line_idx];
                                let truncated = if line.chars().count() > cols as usize - 4 {
                                    line.chars().take(cols as usize - 7).collect::<String>() + "..."
                                } else {
                                    line.to_string()
                                };
                                frame.push_str(&format!("  {}\x1B[K\n", truncated));
                            } else {
                                frame.push_str("\x1B[K\n");
                            }
                        }
                    }
                }
            }
        }

        // Render Footer
        frame.push_str(&"━".repeat(cols as usize));
        frame.push_str("\x1B[K\n");
        frame.push_str(" \x1B[1;30;47m Q / Esc \x1B[0m Quit  | \x1B[1;30;47m 1-4 \x1B[0m Switch tabs  | \x1B[1;30;47m Tab \x1B[0m Cycle inputs  | \x1B[1;30;47m Enter \x1B[0m Send / Confirm");

        print!("{}", frame);
        let _ = io::stdout().flush();

        // Keyboard & State checks
        let key = tui_util::poll_key();
        match key {
            Key::Esc | Key::Char('q') | Key::Char('Q') => {
                break;
            }
            Key::Char('1') => { current_tab = 0; active_field = 0; }
            Key::Char('2') => { current_tab = 1; active_field = 0; }
            Key::Char('3') => { current_tab = 2; active_field = 0; }
            Key::Char('4') => { current_tab = 3; active_field = 0; }
            Key::Tab => {
                if current_tab == 0 {
                    active_field = (active_field + 1) % 2;
                } else if current_tab == 1 {
                    active_field = (active_field + 1) % (headers.len() + 1);
                }
            }
            _ => {
                // Tab-specific key processing
                match current_tab {
                    0 => {
                        // Tab 1 input processing
                        if active_field == 0 {
                            // Method list focused
                            match key {
                                Key::Left => {
                                    selected_method_idx = selected_method_idx.saturating_sub(1);
                                }
                                Key::Right => {
                                    if selected_method_idx + 1 < methods.len() {
                                        selected_method_idx += 1;
                                    }
                                }
                                Key::Enter => {
                                    // Trigger HTTP Request
                                    trigger_http_request(
                                        methods[selected_method_idx].to_string(),
                                        url.clone(),
                                        headers.clone(),
                                        body.clone(),
                                        Arc::clone(&req_state),
                                    );
                                    current_tab = 3; // Switch to response inspector
                                }
                                _ => {}
                            }
                        } else {
                            // URL text box focused
                            match key {
                                Key::Char(c) => {
                                    url.push(c);
                                }
                                Key::Backspace => {
                                    url.pop();
                                }
                                Key::Enter => {
                                    trigger_http_request(
                                        methods[selected_method_idx].to_string(),
                                        url.clone(),
                                        headers.clone(),
                                        body.clone(),
                                        Arc::clone(&req_state),
                                    );
                                    current_tab = 3; // Switch to response inspector
                                }
                                _ => {}
                            }
                        }
                    }
                    1 => {
                        // Headers list input
                        let current_row = active_field;
                        if current_row < headers.len() {
                            match key {
                                Key::Backspace => {
                                    headers.remove(current_row);
                                    active_field = 0;
                                }
                                _ => {}
                            }
                        } else {
                            // "Add Header" input fields
                            match key {
                                Key::Char(c) => {
                                    if new_header_key.contains(':') || new_header_key.len() > 20 {
                                        new_header_val.push(c);
                                    } else {
                                        new_header_key.push(c);
                                    }
                                }
                                Key::Backspace => {
                                    if !new_header_val.is_empty() {
                                        new_header_val.pop();
                                    } else {
                                        new_header_key.pop();
                                    }
                                }
                                Key::Enter => {
                                    if !new_header_key.is_empty() {
                                        headers.push((new_header_key.trim().to_string(), new_header_val.trim().to_string()));
                                        new_header_key = String::new();
                                        new_header_val = String::new();
                                        active_field = headers.len();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    2 => {
                        // Body payload editor input
                        match key {
                            Key::Char(c) => {
                                body.push(c);
                            }
                            Key::Backspace => {
                                body.pop();
                            }
                            _ => {}
                        }
                    }
                    3 => {
                        // Response inspector scroll
                        match key {
                            Key::Up => {
                                response_scroll = response_scroll.saturating_sub(1);
                            }
                            Key::Down => {
                                response_scroll += 1;
                            }
                            Key::PageUp => {
                                response_scroll = response_scroll.saturating_sub(content_height);
                            }
                            Key::PageDown => {
                                response_scroll += content_height;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

fn trigger_http_request(
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: String,
    state_lock: Arc<Mutex<RequestState>>,
) {
    let now = Instant::now();
    {
        let mut s = state_lock.lock().unwrap();
        *s = RequestState::Loading(now);
    }

    thread::spawn(move || {
        let agent = ureq::agent();
        let mut req = match method.as_str() {
            "POST" => agent.post(&url),
            "PUT" => agent.put(&url),
            "DELETE" => agent.delete(&url),
            _ => agent.get(&url),
        };

        for (k, v) in headers {
            req = req.set(&k, &v);
        }

        let start_time = Instant::now();
        let result = if method == "POST" || method == "PUT" {
            req.send_string(&body)
        } else {
            req.call()
        };

        let elapsed = start_time.elapsed().as_millis();

        match result {
            Ok(res) => {
                let status = res.status();
                let status_text = res.status_text().to_string();
                let mut res_headers = Vec::new();
                for name in res.headers_names() {
                    if let Some(val) = res.header(&name) {
                        res_headers.push((name, val.to_string()));
                    }
                }
                let body_str = res.into_string().unwrap_or_else(|_| "[Binary Response Content]".to_string());
                let mut s = state_lock.lock().unwrap();
                *s = RequestState::Success(ResponseData {
                    status,
                    status_text,
                    headers: res_headers,
                    body: body_str,
                    elapsed_ms: elapsed,
                });
            }
            Err(e) => {
                let mut s = state_lock.lock().unwrap();
                *s = RequestState::Error(e.to_string());
            }
        }
    });
}
