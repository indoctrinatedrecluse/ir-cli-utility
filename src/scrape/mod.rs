use crate::ScrapeOptions;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

// ---------------------------------------------------------------------------
// Constants / type tables
// ---------------------------------------------------------------------------

/// Pool of real-world browser User-Agent strings.  We pick one per session.
const UA_POOL: &[&str] = &[
    // Chrome 126 on Windows
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    // Chrome 126 on macOS
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    // Chrome 125 on Linux
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
    // Firefox 127 on Windows
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:127.0) Gecko/20100101 Firefox/127.0",
    // Firefox 127 on Linux
    "Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0",
    // Edge 126 on Windows
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36 Edg/126.0.0.0",
    // Safari 17 on macOS
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Safari/605.1.15",
];

/// HTML-page-like extensions — URLs with these extensions are crawlable pages.
const PAGE_EXTS: &[&str] = &[
    "html", "htm", "php", "asp", "aspx", "jsp", "cfm", "cgi", "shtml", "xhtml",
];

/// Video extensions blocked by default.
const VIDEO_EXTS: &[&str] = &[
    "mp4", "webm", "avi", "mkv", "mov", "flv", "wmv", "m4v", "ts", "mpeg", "mpg",
    "3gp", "ogv", "rm", "rmvb",
];

/// Audio extensions blocked by default.
const AUDIO_EXTS: &[&str] = &[
    "mp3", "flac", "wav", "ogg", "aac", "m4a", "opus", "wma", "aiff", "mid",
];

/// Image extensions.
const IMAGE_EXTS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "svg", "webp", "ico", "bmp", "tiff", "avif",
    "heic", "apng",
];

/// Format group aliases → expanded extension lists.
const FORMAT_GROUPS: &[(&str, &[&str])] = &[
    ("documents", &["pdf", "doc", "docx", "rtf", "odt", "ppt", "pptx", "xls", "xlsx", "odp", "ods"]),
    ("images",    &["jpg", "jpeg", "png", "gif", "svg", "webp", "ico", "bmp", "tiff", "avif", "heic"]),
    ("data",      &["json", "csv", "xml", "yaml", "yml", "toml", "ndjson", "jsonl"]),
    ("web",       &["html", "htm", "css", "js", "wasm"]),
    ("archives",  &["zip", "tar", "gz", "bz2", "xz", "7z", "rar", "tgz", "zst"]),
    ("text",      &["txt", "md", "rst", "log", "conf", "cfg", "ini", "env"]),
    ("audio",     &["mp3", "flac", "wav", "ogg", "aac", "m4a", "opus", "wma"]),
    ("video",     &["mp4", "webm", "avi", "mkv", "mov", "flv", "wmv", "m4v", "ts"]),
];

// ---------------------------------------------------------------------------
// Internal state carried through the crawl
// ---------------------------------------------------------------------------

struct ScrapeState {
    visited_pages: HashSet<String>,
    downloaded_urls: HashSet<String>,
    pages_fetched: usize,
    bytes_downloaded: u64,
    files_saved: usize,
}

impl ScrapeState {
    fn new() -> Self {
        ScrapeState {
            visited_pages: HashSet::new(),
            downloaded_urls: HashSet::new(),
            pages_fetched: 0,
            bytes_downloaded: 0,
            files_saved: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn run_scrape(url: &str, options: ScrapeOptions) {
    // Expand format group aliases into concrete extensions.
    let formats = expand_formats(&options.formats);

    // Validate: at least one format must remain after expansion.
    if formats.is_empty() {
        eprintln!("Error: --format produced an empty extension list after expansion.");
        std::process::exit(1);
    }

    // Prepare output directory.
    let dest = PathBuf::from(&options.dest);
    if let Err(e) = fs::create_dir_all(&dest) {
        eprintln!("Error: Cannot create destination directory '{}': {}", dest.display(), e);
        std::process::exit(1);
    }
    // Quick write-permission test.
    let probe = dest.join(".ir_scrape_probe");
    match fs::File::create(&probe) {
        Ok(_) => { let _ = fs::remove_file(&probe); }
        Err(e) => {
            eprintln!("Error: Destination directory '{}' is not writable: {}", dest.display(), e);
            std::process::exit(1);
        }
    }

    // Pick a User-Agent for this session.
    let ua = pick_ua(&options);

    // Build the ureq agent with session-wide settings.
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(options.timeout_secs))
        .build();

    // Cache robots.txt for the start domain (if not ignored).
    let start_origin = url_origin(url);
    let robots_rules: Vec<String> = if options.ignore_robots {
        Vec::new()
    } else {
        fetch_robots(&agent, &ua, &start_origin, options.verbose)
    };

    let mut state = ScrapeState::new();

    // BFS queue: (url, depth)
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    queue.push_back((normalize_url(url), 0));

    println!(
        "ir-scrape: starting at {}\n  formats: {}\n  dest:    {}\n  depth:   {}, max-pages: {}, max-size: {}",
        url,
        formats.join(", "),
        dest.display(),
        options.depth,
        options.max_pages,
        human_size(options.max_size_bytes),
    );
    if options.dry_run {
        println!("  [DRY RUN — no files will be written]");
    }
    println!();

    while let Some((page_url, depth)) = queue.pop_front() {
        // --- safety guards ---
        if state.pages_fetched >= options.max_pages {
            if options.verbose {
                println!("[limit] max-pages ({}) reached, stopping.", options.max_pages);
            }
            break;
        }
        if state.bytes_downloaded >= options.max_size_bytes {
            if options.verbose {
                println!("[limit] max-size ({}) reached, stopping.", human_size(options.max_size_bytes));
            }
            break;
        }
        if state.visited_pages.contains(&page_url) {
            continue;
        }
        state.visited_pages.insert(page_url.clone());

        // Robots check.
        if !options.ignore_robots && is_disallowed(&page_url, &start_origin, &robots_rules) {
            if options.verbose {
                println!("[robots] blocked: {}", page_url);
            }
            continue;
        }

        // Same-domain guard.
        if options.same_domain && url_host(&page_url) != url_host(url) {
            if options.verbose {
                println!("[domain] skipped (different domain): {}", page_url);
            }
            continue;
        }

        if options.verbose {
            println!("[page] fetching {} (depth {})", page_url, depth);
        }

        // Fetch the page.
        let html = match fetch_page(&agent, &ua, &page_url) {
            Ok(h) => { state.pages_fetched += 1; h }
            Err(e) => {
                eprintln!("[error] {}: {}", page_url, e);
                continue;
            }
        };

        // Extract all links.
        let raw_links = extract_links(&html);
        let mut links_followed = 0;

        for raw_href in &raw_links {
            if links_followed >= options.max_links {
                if options.verbose {
                    println!("[limit] max-links ({}) reached for this page.", options.max_links);
                }
                break;
            }

            let resolved = match resolve_url(&page_url, raw_href) {
                Some(u) => u,
                None => continue,
            };

            let ext = url_extension(&resolved).map(|e| e.to_lowercase());

            // ---- decide what to do with this URL ----
            if let Some(ref e) = ext {
                // Active block: video / audio unless user opted in.
                if VIDEO_EXTS.contains(&e.as_str()) && !options.include_video {
                    if options.verbose {
                        println!("[blocked] video (use --include-video): {}", resolved);
                    }
                    continue;
                }
                if AUDIO_EXTS.contains(&e.as_str()) && !options.include_audio {
                    if options.verbose {
                        println!("[blocked] audio (use --include-audio): {}", resolved);
                    }
                    continue;
                }
                if IMAGE_EXTS.contains(&e.as_str()) && options.no_images {
                    if options.verbose {
                        println!("[blocked] image (--no-images active): {}", resolved);
                    }
                    continue;
                }

                // Does this extension match requested formats?
                if formats.contains(e) {
                    links_followed += 1;
                    if state.downloaded_urls.contains(&resolved) {
                        if options.verbose {
                            println!("[skip] already downloaded: {}", resolved);
                        }
                        continue;
                    }
                    state.downloaded_urls.insert(resolved.clone());

                    // Download it.
                    match download_file(
                        &agent, &ua, &resolved, &dest, &options, &mut state,
                    ) {
                        Ok(()) => {}
                        Err(e) => eprintln!("[error] download {}: {}", resolved, e),
                    }
                    continue;
                }

                // If it's a crawlable page extension and depth allows, enqueue.
                if PAGE_EXTS.contains(&e.as_str()) && depth < options.depth {
                    links_followed += 1;
                    queue.push_back((resolved, depth + 1));
                }
            } else {
                // No extension — treat as a crawlable page if depth allows.
                if depth < options.depth {
                    links_followed += 1;
                    queue.push_back((resolved, depth + 1));
                }
            }
        }
    }

    // Summary.
    println!(
        "\nDone. {} file(s) {} — {} total.",
        state.files_saved,
        if options.dry_run { "would be saved" } else { "saved" },
        human_size(state.bytes_downloaded),
    );
}

// ---------------------------------------------------------------------------
// Format expansion
// ---------------------------------------------------------------------------

fn expand_formats(raw: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for f in raw {
        // comma-separated within a single --format value
        for part in f.split(',') {
            let part = part.trim().trim_start_matches('.').to_lowercase();
            if part.is_empty() { continue; }
            // group alias?
            if let Some((_, exts)) = FORMAT_GROUPS.iter().find(|(g, _)| *g == part) {
                for e in *exts {
                    let es = e.to_string();
                    if !out.contains(&es) { out.push(es); }
                }
            } else {
                if !out.contains(&part) { out.push(part); }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// User-Agent selection
// ---------------------------------------------------------------------------

fn pick_ua(options: &ScrapeOptions) -> String {
    if let Some(ref ua) = options.user_agent {
        return ua.clone();
    }
    // Deterministic pseudo-random pick based on process start time in millis,
    // so it's stable within a session but varies between runs.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_millis() as usize;
    UA_POOL[now % UA_POOL.len()].to_string()
}

// ---------------------------------------------------------------------------
// HTTP helpers
// ---------------------------------------------------------------------------

/// Fetch an HTML page, returning its body as a String.
fn fetch_page(agent: &ureq::Agent, ua: &str, url: &str) -> Result<String, String> {
    let resp = agent
        .get(url)
        .set("User-Agent", ua)
        .set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
        .set("Accept-Language", "en-US,en;q=0.9")
        .set("Accept-Encoding", "gzip, deflate, br")
        .set("Connection", "keep-alive")
        .set("Upgrade-Insecure-Requests", "1")
        .set("Sec-Fetch-Dest", "document")
        .set("Sec-Fetch-Mode", "navigate")
        .set("Sec-Fetch-Site", "none")
        .set("Sec-Fetch-User", "?1")
        .set("Cache-Control", "max-age=0")
        .call()
        .map_err(|e| e.to_string())?;

    resp.into_string().map_err(|e| e.to_string())
}

/// Download a file URL to the destination directory, respecting all limits.
fn download_file(
    agent: &ureq::Agent,
    ua: &str,
    url: &str,
    dest: &Path,
    options: &ScrapeOptions,
    state: &mut ScrapeState,
) -> Result<(), String> {
    let filename = filename_from_url(url);
    let dest_path = unique_path(dest, &filename, options.overwrite);

    if options.dry_run {
        println!("[dry-run] would download: {} -> {}", url, dest_path.display());
        state.files_saved += 1;
        return Ok(());
    }

    // Make the request.
    let resp = agent
        .get(url)
        .set("User-Agent", ua)
        .set("Accept", "*/*")
        .set("Accept-Language", "en-US,en;q=0.9")
        .set("Accept-Encoding", "gzip, deflate, br")
        .set("Connection", "keep-alive")
        .set("Sec-Fetch-Dest", "document")
        .set("Sec-Fetch-Mode", "navigate")
        .set("Sec-Fetch-Site", "none")
        .call()
        .map_err(|e| format!("request failed: {}", e))?;

    // Check Content-Length against remaining budget.
    if let Some(cl) = resp.header("content-length").and_then(|v| v.parse::<u64>().ok()) {
        let remaining = options.max_size_bytes.saturating_sub(state.bytes_downloaded);
        if cl > remaining {
            return Err(format!(
                "skipped (would exceed max-size; file is {}, remaining budget is {})",
                human_size(cl),
                human_size(remaining),
            ));
        }
    }

    // Stream body to file.
    let mut reader = resp.into_reader();
    let mut file = fs::File::create(&dest_path)
        .map_err(|e| format!("cannot create '{}': {}", dest_path.display(), e))?;

    let mut buf = [0u8; 65536];
    let mut bytes_written: u64 = 0;

    loop {
        // Check budget mid-download.
        if state.bytes_downloaded + bytes_written >= options.max_size_bytes {
            drop(file);
            let _ = fs::remove_file(&dest_path);
            return Err(format!(
                "aborted mid-download — max-size ({}) reached",
                human_size(options.max_size_bytes),
            ));
        }
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                file.write_all(&buf[..n])
                    .map_err(|e| format!("write error: {}", e))?;
                bytes_written += n as u64;
            }
            Err(e) => {
                drop(file);
                let _ = fs::remove_file(&dest_path);
                return Err(format!("read error: {}", e));
            }
        }
    }

    state.bytes_downloaded += bytes_written;
    state.files_saved += 1;
    println!("[saved] {} ({}) -> {}", url, human_size(bytes_written), dest_path.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// robots.txt
// ---------------------------------------------------------------------------

fn fetch_robots(agent: &ureq::Agent, ua: &str, origin: &str, verbose: bool) -> Vec<String> {
    let robots_url = format!("{}/robots.txt", origin);
    match agent.get(&robots_url).set("User-Agent", ua).call() {
        Ok(resp) => {
            let body = resp.into_string().unwrap_or_default();
            parse_robots_disallowed(&body)
        }
        Err(_) => {
            if verbose {
                println!("[robots] could not fetch {}/robots.txt — assuming unrestricted", origin);
            }
            Vec::new()
        }
    }
}

fn parse_robots_disallowed(body: &str) -> Vec<String> {
    let mut disallowed = Vec::new();
    let mut in_wildcard_block = false;

    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.to_ascii_lowercase().starts_with("user-agent:") {
            let ua_val = line["user-agent:".len()..].trim();
            in_wildcard_block = ua_val == "*";
        } else if in_wildcard_block && line.to_ascii_lowercase().starts_with("disallow:") {
            let path = line["disallow:".len()..].trim();
            if !path.is_empty() {
                disallowed.push(path.to_string());
            }
        }
    }
    disallowed
}

fn is_disallowed(url: &str, origin: &str, disallowed: &[String]) -> bool {
    // Only apply rules from the scraped origin.
    if !url.starts_with(origin) { return false; }
    let path = &url[origin.len()..];
    disallowed.iter().any(|d| path.starts_with(d.as_str()))
}

// ---------------------------------------------------------------------------
// HTML link extraction
// ---------------------------------------------------------------------------

fn extract_links(html: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    // Scan for href="..." / href='...' / src="..." / src='...'
    // (case-insensitive prefix check for robustness)
    let patterns: &[(&str, char)] = &[
        ("href=\"", '"'),
        ("href='",  '\''),
        ("src=\"",  '"'),
        ("src='",   '\''),
        // data-src patterns used by lazy-loaders
        ("data-src=\"", '"'),
        ("data-src='",  '\''),
    ];
    let html_lower = html.to_ascii_lowercase();

    for (pattern, closer) in patterns {
        let mut search_from = 0usize;
        while let Some(rel) = html_lower[search_from..].find(pattern) {
            let abs = search_from + rel;
            let value_start = abs + pattern.len();
            if let Some(end_rel) = html[value_start..].find(*closer) {
                let raw = html[value_start..value_start + end_rel].trim();
                if !raw.is_empty() {
                    found.push(raw.to_string());
                }
                search_from = value_start + end_rel + 1;
            } else {
                break;
            }
        }
    }
    found
}

// ---------------------------------------------------------------------------
// URL utilities
// ---------------------------------------------------------------------------

fn url_origin(url: &str) -> String {
    // "https://example.com/path" → "https://example.com"
    if let Some(scheme_end) = url.find("://") {
        let after = &url[scheme_end + 3..];
        let host_end = after.find('/').unwrap_or(after.len());
        return format!("{}{}", &url[..scheme_end + 3], &after[..host_end]);
    }
    url.to_string()
}

fn url_host(url: &str) -> &str {
    if let Some(scheme_end) = url.find("://") {
        let after = &url[scheme_end + 3..];
        let host_end = after.find('/').unwrap_or(after.len());
        return &after[..host_end];
    }
    url
}

fn normalize_url(url: &str) -> String {
    // Strip trailing fragment.
    let url = url.split('#').next().unwrap_or(url);
    url.to_string()
}

fn resolve_url(base: &str, href: &str) -> Option<String> {
    let href = href.split('#').next().unwrap_or(href).trim();
    if href.is_empty() { return None; }

    if href.starts_with("http://") || href.starts_with("https://") {
        return Some(href.to_string());
    }
    if href.starts_with("//") {
        let scheme = if base.starts_with("https://") { "https:" } else { "http:" };
        return Some(format!("{}{}", scheme, href));
    }
    // Skip non-HTTP schemes entirely.
    if href.contains("://") { return None; }
    // Skip mailto:, javascript:, tel:, data:
    if href.starts_with("mailto:") || href.starts_with("javascript:") ||
       href.starts_with("tel:")    || href.starts_with("data:") {
        return None;
    }
    if href.starts_with('/') {
        return Some(format!("{}{}", url_origin(base), href));
    }
    // Relative URL: resolve against the base directory.
    let base_dir = base.rfind('/').map(|i| &base[..i]).unwrap_or(base);
    Some(format!("{}/{}", base_dir, href))
}

fn url_extension(url: &str) -> Option<&str> {
    // Strip query string and fragment.
    let url = url.split('?').next().unwrap_or(url);
    let url = url.split('#').next().unwrap_or(url);
    let filename = url.rsplit('/').next().unwrap_or("");
    if let Some(dot) = filename.rfind('.') {
        let ext = &filename[dot + 1..];
        // Guard against degenerate cases: very long "extensions" or ones with slashes.
        if !ext.is_empty() && ext.len() <= 10 && ext.chars().all(|c| c.is_alphanumeric()) {
            return Some(ext);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// File-path helpers
// ---------------------------------------------------------------------------

fn filename_from_url(url: &str) -> String {
    let url = url.split('?').next().unwrap_or(url);
    let url = url.split('#').next().unwrap_or(url);
    let raw = url.rsplit('/').find(|s| !s.is_empty()).unwrap_or("download");
    // Sanitise: keep alphanumeric, dots, hyphens, underscores.
    let sanitised: String = raw.chars()
        .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
        .collect();
    if sanitised.is_empty() || sanitised == "." { "download".to_string() } else { sanitised }
}

/// Return a path that does not collide with an existing file (unless `overwrite`).
fn unique_path(dir: &Path, filename: &str, overwrite: bool) -> PathBuf {
    let candidate = dir.join(filename);
    if overwrite || !candidate.exists() {
        return candidate;
    }
    // Split stem / extension and append _1, _2 …
    let (stem, ext) = match filename.rfind('.') {
        Some(i) => (&filename[..i], &filename[i..]),
        None    => (filename, ""),
    };
    let mut n = 1u32;
    loop {
        let name = format!("{}_{}{}", stem, n, ext);
        let p = dir.join(&name);
        if !p.exists() { return p; }
        n += 1;
    }
}

// ---------------------------------------------------------------------------
// Miscellaneous helpers
// ---------------------------------------------------------------------------

fn human_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;
    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Parse a size string like "50M", "1G", "500K", "1048576".
pub fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();
    if let Some(n) = s.strip_suffix('G') {
        n.trim().parse::<f64>().ok().map(|v| (v * 1_073_741_824.0) as u64)
    } else if let Some(n) = s.strip_suffix('M') {
        n.trim().parse::<f64>().ok().map(|v| (v * 1_048_576.0) as u64)
    } else if let Some(n) = s.strip_suffix('K') {
        n.trim().parse::<f64>().ok().map(|v| (v * 1_024.0) as u64)
    } else if let Some(n) = s.strip_suffix('B') {
        n.trim().parse::<u64>().ok()
    } else {
        s.parse::<u64>().ok()
    }
}
