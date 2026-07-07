use crate::LogOptions;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, IsTerminal, Read};

#[derive(Debug, Clone)]
struct LogEntry {
    ip: String,
    user: String,
    time: String,
    method: String,
    path: String,
    status: u16,
    size: u64,
    referrer: String,
    agent: String,
}

struct Query {
    field: String,
    op: String,
    value: String,
}

fn parse_query(q_str: &str) -> Option<Query> {
    let operators = ["==", "!=", ">=", "<=", ">", "<", "contains"];
    for op in operators {
        if let Some((field, val)) = q_str.split_once(op) {
            return Some(Query {
                field: field.trim().to_lowercase(),
                op: op.to_string(),
                value: val.trim().trim_matches(|c| c == '\'' || c == '"').to_string(),
            });
        }
    }
    None
}

impl Query {
    fn evaluate(&self, entry: &LogEntry) -> bool {
        let field_val = match self.field.as_str() {
            "ip" => entry.ip.clone(),
            "user" => entry.user.clone(),
            "time" => entry.time.clone(),
            "method" => entry.method.clone(),
            "path" => entry.path.clone(),
            "status" => entry.status.to_string(),
            "size" => entry.size.to_string(),
            "referrer" | "referer" => entry.referrer.clone(),
            "agent" | "user_agent" => entry.agent.clone(),
            _ => return false,
        };

        match self.op.as_str() {
            "==" => field_val == self.value,
            "!=" => field_val != self.value,
            "contains" => field_val.to_lowercase().contains(&self.value.to_lowercase()),
            ">" => {
                if let (Ok(a), Ok(b)) = (field_val.parse::<f64>(), self.value.parse::<f64>()) {
                    a > b
                } else {
                    field_val > self.value
                }
            }
            "<" => {
                if let (Ok(a), Ok(b)) = (field_val.parse::<f64>(), self.value.parse::<f64>()) {
                    a < b
                } else {
                    field_val < self.value
                }
            }
            ">=" => {
                if let (Ok(a), Ok(b)) = (field_val.parse::<f64>(), self.value.parse::<f64>()) {
                    a >= b
                } else {
                    field_val >= self.value
                }
            }
            "<=" => {
                if let (Ok(a), Ok(b)) = (field_val.parse::<f64>(), self.value.parse::<f64>()) {
                    a <= b
                } else {
                    field_val <= self.value
                }
            }
            _ => false,
        }
    }
}

fn parse_clf_or_combined(line: &str) -> Option<LogEntry> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut in_brackets = false;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '"' {
            in_quotes = !in_quotes;
            current.push(c);
        } else if c == '[' && !in_quotes {
            in_brackets = true;
            current.push(c);
        } else if c == ']' && !in_quotes {
            in_brackets = false;
            current.push(c);
        } else if c.is_whitespace() && !in_quotes && !in_brackets {
            if !current.is_empty() {
                parts.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
        i += 1;
    }
    if !current.is_empty() {
        parts.push(current);
    }

    if parts.len() < 7 {
        return None;
    }

    let ip = parts[0].clone();
    let user = parts[2].clone();
    let time = parts[3].trim_matches(|c| c == '[' || c == ']').to_string();
    
    let request = parts[4].trim_matches('"').to_string();
    let mut req_parts = request.split_whitespace();
    let method = req_parts.next().unwrap_or("").to_string();
    let path = req_parts.next().unwrap_or("").to_string();

    let status = parts[5].parse::<u16>().unwrap_or(0);
    let size = parts[6].parse::<u64>().unwrap_or(0);

    let referrer = if parts.len() > 7 {
        parts[7].trim_matches('"').to_string()
    } else {
        "-".to_string()
    };

    let agent = if parts.len() > 8 {
        parts[8..].join(" ").trim_matches('"').to_string()
    } else {
        "-".to_string()
    };

    Some(LogEntry {
        ip,
        user,
        time,
        method,
        path,
        status,
        size,
        referrer,
        agent,
    })
}

fn parse_json_log(line: &str) -> Option<LogEntry> {
    let val: serde_json::Value = serde_json::from_str(line).ok()?;
    let ip = val.get("ip").or_else(|| val.get("client_ip")).or_else(|| val.get("host")).and_then(|v| v.as_str()).unwrap_or("-").to_string();
    let user = val.get("user").or_else(|| val.get("username")).and_then(|v| v.as_str()).unwrap_or("-").to_string();
    let time = val.get("time").or_else(|| val.get("timestamp")).and_then(|v| v.as_str()).unwrap_or("-").to_string();
    let method = val.get("method").and_then(|v| v.as_str()).unwrap_or("-").to_string();
    let path = val.get("path").or_else(|| val.get("uri")).or_else(|| val.get("url")).and_then(|v| v.as_str()).unwrap_or("-").to_string();
    
    let status = val.get("status").and_then(|v| {
        v.as_u64().map(|n| n as u16).or_else(|| v.as_str().and_then(|s| s.parse::<u16>().ok()))
    }).unwrap_or(0);
    
    let size = val.get("size").or_else(|| val.get("bytes")).and_then(|v| {
        v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse::<u64>().ok()))
    }).unwrap_or(0);
    
    let referrer = val.get("referrer").or_else(|| val.get("referer")).and_then(|v| v.as_str()).unwrap_or("-").to_string();
    let agent = val.get("agent").or_else(|| val.get("user_agent")).and_then(|v| v.as_str()).unwrap_or("-").to_string();

    Some(LogEntry {
        ip,
        user,
        time,
        method,
        path,
        status,
        size,
        referrer,
        agent,
    })
}

fn parse_csv_log(line: &str) -> Option<LogEntry> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            parts.push(current.clone());
            current.clear();
        } else {
            current.push(c);
        }
        i += 1;
    }
    parts.push(current);

    if parts.len() < 7 {
        return None;
    }

    let ip = parts[0].clone();
    let user = parts[1].clone();
    let time = parts[3].clone();
    let request = parts[4].clone();
    let mut req_parts = request.split_whitespace();
    let method = req_parts.next().unwrap_or("").to_string();
    let path = req_parts.next().unwrap_or("").to_string();
    let status = parts[5].parse::<u16>().unwrap_or(0);
    let size = parts[6].parse::<u64>().unwrap_or(0);
    let referrer = parts.get(7).cloned().unwrap_or_else(|| "-".to_string());
    let agent = parts.get(8).cloned().unwrap_or_else(|| "-".to_string());

    Some(LogEntry {
        ip,
        user,
        time,
        method,
        path,
        status,
        size,
        referrer,
        agent,
    })
}

pub fn run_log(input_path: Option<&str>, options: LogOptions) {
    let mut input_bytes = Vec::new();
    if let Some(path) = input_path {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to open file '{}': {}", path, e);
                std::process::exit(1);
            }
        };
        if let Err(e) = file.read_to_end(&mut input_bytes) {
            eprintln!("Error: Failed to read file '{}': {}", path, e);
            std::process::exit(1);
        }
    } else {
        let mut stdin = io::stdin();
        if stdin.is_terminal() {
            eprintln!("Error: No input file specified and stdin is a terminal.");
            std::process::exit(1);
        }
        if let Err(e) = stdin.read_to_end(&mut input_bytes) {
            eprintln!("Error: Failed to read standard input: {}", e);
            std::process::exit(1);
        }
    }

    // Strip BOM
    let bytes_to_parse = if input_bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &input_bytes[3..]
    } else {
        &input_bytes
    };

    let content = String::from_utf8_lossy(bytes_to_parse);
    let lines = content.lines();

    // Query parser
    let query_filter = options.query.as_ref().and_then(|q| {
        let parsed = parse_query(q);
        if parsed.is_none() {
            eprintln!("Error: Invalid query expression syntax. Supported: <field> <op> <value>");
            std::process::exit(1);
        }
        parsed
    });

    let mut parsed_entries = Vec::new();
    let format = options.format.clone().unwrap_or_else(|| "auto".to_string());

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

        let entry = match format.as_str() {
            "json" => parse_json_log(trimmed),
            "csv" => parse_csv_log(trimmed),
            "common" | "combined" => parse_clf_or_combined(trimmed),
            _ => {
                // Auto detection
                if trimmed.starts_with('{') {
                    parse_json_log(trimmed)
                } else if trimmed.contains(',') && !trimmed.contains('[') {
                    parse_csv_log(trimmed)
                } else {
                    parse_clf_or_combined(trimmed)
                }
            }
        };

        if let Some(e) = entry {
            if let Some(ref q) = query_filter {
                if q.evaluate(&e) {
                    parsed_entries.push(e);
                }
            } else {
                parsed_entries.push(e);
            }
        }
    }

    if options.stats {
        // Accumulate statistics
        let total_requests = parsed_entries.len();
        let mut failed_requests = 0;
        let mut total_bytes = 0;
        let mut ip_counts = HashMap::new();
        let mut path_counts = HashMap::new();
        let mut status_counts = HashMap::new();

        for entry in &parsed_entries {
            if entry.status >= 400 {
                failed_requests += 1;
            }
            total_bytes += entry.size;
            *ip_counts.entry(entry.ip.clone()).or_insert(0) += 1;
            *path_counts.entry(entry.path.clone()).or_insert(0) += 1;
            *status_counts.entry(entry.status).or_insert(0) += 1;
        }

        let mut ip_list: Vec<(String, u64)> = ip_counts.into_iter().collect();
        ip_list.sort_by(|a, b| b.1.cmp(&a.1));

        let mut path_list: Vec<(String, u64)> = path_counts.into_iter().collect();
        path_list.sort_by(|a, b| b.1.cmp(&a.1));

        let mut status_list: Vec<(u16, u64)> = status_counts.into_iter().collect();
        status_list.sort_by(|a, b| b.1.cmp(&a.1));

        let mut out = String::new();
        out.push_str("=== LOG METRICS SUMMARY ===\n");
        out.push_str(&format!("Total Requests:      {}\n", total_requests));
        let failed_pct = if total_requests > 0 { (failed_requests as f64 / total_requests as f64) * 100.0 } else { 0.0 };
        out.push_str(&format!("Failed Requests:     {} ({:.2}%)\n", failed_requests, failed_pct));
        out.push_str(&format!("Data Transferred:    {} bytes ({:.2} MB)\n\n", total_bytes, total_bytes as f64 / (1024.0 * 1024.0)));

        out.push_str("Top 5 Client IPs:\n");
        for (ip, count) in ip_list.iter().take(5) {
            out.push_str(&format!("  - {}: {} requests\n", ip, count));
        }
        out.push_str("\nTop 5 Requested Paths:\n");
        for (path, count) in path_list.iter().take(5) {
            out.push_str(&format!("  - {}: {} requests\n", path, count));
        }
        out.push_str("\nStatus Code Distribution:\n");
        for (status, count) in status_list.iter() {
            out.push_str(&format!("  - HTTP {}: {} requests\n", status, count));
        }

        if let Some(ref out_path) = options.output {
            if let Err(e) = std::fs::write(out_path, out.as_bytes()) {
                eprintln!("Error: Failed to write stats to '{}': {}", out_path, e);
                std::process::exit(1);
            }
        } else {
            print!("{}", out);
        }
    } else {
        // Output parsed lines
        let limit = options.limit.unwrap_or(parsed_entries.len());
        let mut out = String::new();
        for entry in parsed_entries.iter().take(limit) {
            out.push_str(&format!(
                "{} - [{}] \"{} {}\" {} {}\n",
                entry.ip, entry.time, entry.method, entry.path, entry.status, entry.size
            ));
        }

        if let Some(ref out_path) = options.output {
            if let Err(e) = std::fs::write(out_path, out.as_bytes()) {
                eprintln!("Error: Failed to write to output file '{}': {}", out_path, e);
                std::process::exit(1);
            }
        } else {
            print!("{}", out);
        }
    }
}
