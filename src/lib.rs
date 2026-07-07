pub mod list;
pub mod help;
pub mod rename;
pub mod copy;
pub mod remove;
pub mod create;
pub mod r#move;
pub mod archive;
pub mod cat;
pub mod grep;
pub mod find;
pub mod diff;
pub mod search;
pub mod which;
pub mod tree;
pub mod du;
pub mod fastfetch;
pub mod monitor;
pub mod hash;
pub mod ps;
pub mod kill;
pub mod fetch;
pub mod env_action;
pub mod hex;
pub mod ping;
pub mod base64;
pub mod uuid;
pub mod ip;
pub mod echo;
pub mod clip;
pub mod math;
pub mod sleep;
pub mod time;
pub mod dns;
pub mod path;
pub mod df;
pub mod whoami;
pub mod sockets;
pub mod wc;
pub mod ln;
pub mod chmod;
pub mod pmon;
pub mod watch;
pub mod nettop;
pub mod dua;
pub mod browse;
pub mod edit;
pub mod scrape;
pub mod sort;
pub mod encode;
pub mod json;
pub mod plot;
pub mod clock;
pub mod text;
pub mod globe;
pub mod log_parser;
pub mod life;





#[derive(Default)]
pub struct ListOptions {
    pub show_all: bool,
    pub sort_by_size: bool,
    pub sort_by_time: bool,
    pub filter: Option<String>,
    pub files_only: bool,
    pub folders_only: bool,
    /// Use IEC human-readable size suffixes (KiB, MiB, GiB).
    pub human_readable: bool,
}

#[derive(Default)]
pub struct RenameOptions {
    pub force: bool,
    pub interactive: bool,
    pub force_links: bool,
}

#[derive(Default)]
pub struct CopyOptions {
    pub force: bool,
    pub recursive: bool,
    pub files_only: bool,
    pub folders_only: bool,
    pub rename: Option<String>,
}

#[derive(Default)]
pub struct RemoveOptions {
    pub force: bool,
    pub trash: bool,
    pub interactive: bool,
    pub yes: bool,
    pub verbose: bool,
}

#[derive(Default, Clone)]
pub struct CreateOptions {
    pub create_file: bool,
    pub force_subdirs: bool,
}

#[derive(Default)]
pub struct MoveOptions {
    pub force: bool,
    pub rename: Option<String>,
}

#[derive(Default, Clone)]
pub struct ArchiveOptions {
    pub dest: Option<String>,
    pub arc: bool,
    pub unarc: bool,
    pub format: Option<String>,
    pub test: bool,
    pub force: bool,
    pub verbose: bool,
}

#[derive(Default, Clone)]
pub struct CatOptions {
    pub line_numbers: bool,
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub range: Option<(usize, usize)>,
    pub binary: bool,
    pub encoding: Option<String>,
    /// Collapse consecutive blank lines into a single blank line.
    pub squeeze_blank: bool,
}

#[derive(Default, Clone)]
pub struct GrepOptions {
    pub case_insensitive: bool,
    pub line_numbers: bool,
    pub count: bool,
    pub list: bool,
    pub invert_match: bool,
    pub entire_line: bool,
    pub fixed_string: bool,
    pub extended_regex: bool,
    /// Lines of context to print before each match.
    pub before_context: usize,
    /// Lines of context to print after each match.
    pub after_context: usize,
}

#[derive(Default, Clone)]
pub struct FindOptions {
    pub name: Option<String>,
    pub case_insensitive_name: Option<String>,
    pub item_type: Option<FindItemType>,
    pub max_depth: Option<usize>,
    pub min_depth: usize,
    pub empty: bool,
    /// Minimum file size in bytes (inclusive).
    pub min_size: Option<u64>,
    /// Maximum file size in bytes (inclusive).
    pub max_size: Option<u64>,
    /// Match only entries modified more recently than this path's mtime.
    pub newer: Option<String>,
    /// Match only entries modified less recently than this path's mtime.
    pub older: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FindItemType {
    File,
    Directory,
}

#[derive(Default, Clone)]
pub struct DiffOptions {
    pub brief: bool,
    pub ignore_case: bool,
    pub unified: bool,
}

#[derive(Default, Clone)]
pub struct SearchOptions {
    pub case_insensitive: bool,
    pub line_numbers: bool,
    pub files_with_matches: bool,
    pub count: bool,
    pub name: Option<String>,
    pub case_insensitive_name: Option<String>,
    pub max_depth: Option<usize>,
    pub min_depth: usize,
    pub include_extensions: Vec<String>,
    pub exclude_extensions: Vec<String>,
    pub include_skipped: bool,
}

#[derive(Default, Clone)]
pub struct TreeOptions {
    pub show_all: bool,
    pub dirs_only: bool,
    pub max_depth: Option<usize>,
    pub full_path: bool,
    pub no_indent: bool,
    pub show_size: bool,
    pub human_readable: bool,
    pub show_perms: bool,
    pub no_report: bool,
}

#[derive(Default, Clone)]
pub struct DuOptions {
    pub show_all: bool,
    pub total: bool,
    pub human_readable: bool,
    pub summarize: bool,
    pub max_depth: Option<usize>,
    pub kilobytes: bool,
    pub megabytes: bool,
}

#[derive(Default, Clone)]
pub struct HashOptions {
    pub algorithm: String,
    pub verify: Option<String>,
    pub checksum_file: bool,
}

#[derive(Default, Clone)]
pub struct PsOptions {
    pub sort_by: String,
    pub filter: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Default, Clone)]
pub struct KillOptions {
    pub force: bool,
    pub all: bool,
}

#[derive(Default, Clone)]
pub struct FetchOptions {
    pub headers: Vec<String>,
    pub method: String,
    pub data: Option<String>,
    pub output: Option<String>,
    pub include_headers: bool,
    /// Per-request timeout in seconds (0 = no timeout).
    pub timeout_secs: u64,
    /// Show a download progress bar when writing to a file.
    pub progress: bool,
    /// Disable automatic redirect following.
    pub no_follow_redirects: bool,
}

#[derive(Default, Clone)]
pub struct EnvOptions {
    pub search: Option<String>,
}

#[derive(Default, Clone)]
pub struct HexOptions {
    pub limit: Option<usize>,
    pub cols: usize,
}

#[derive(Default, Clone)]
pub struct PingOptions {
    pub count: usize,
    pub timeout_ms: u64,
}

#[derive(Default, Clone)]
pub struct Base64Options {
    pub decode: bool,
    pub url: bool,
    pub no_padding: bool,
    pub output: Option<String>,
}

#[derive(Default, Clone)]
pub struct EncodeOptions {
    pub format: String,
    pub output: Option<String>,
    pub no_padding: bool,
    pub hex_upper: bool,
    pub hex_separator: Option<String>,
    pub url_encode_all: bool,
}

#[derive(Default, Clone)]
pub struct DecodeOptions {
    pub format: String,
    pub output: Option<String>,
    pub no_padding: bool,
    pub hex_separator: Option<String>,
}

#[derive(Default, Clone)]
pub struct JsonOptions {
    pub query: Option<String>,
    pub minify: bool,
    pub pretty: bool,
    pub indent: usize,
    pub output: Option<String>,
}

#[derive(Default, Clone)]
pub struct PlotOptions {
    pub chart_type: String,
    pub title: Option<String>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub csv_col: Option<usize>,
    pub csv_headers: bool,
    pub source_format: String,
    pub smooth: bool,
    pub log_scale: bool,
    pub json_key: Option<String>,
    pub horizontal: bool,
}

#[derive(Default, Clone)]
pub struct UuidOptions {
    pub version: usize,
    pub count: usize,
    pub uppercase: bool,
    pub no_hyphens: bool,
}

#[derive(Default, Clone)]
pub struct IpOptions {
    pub public: bool,
    pub all: bool,
}

#[derive(Default, Clone)]
pub struct EchoOptions {
    pub no_newline: bool,
    pub escapes: bool,
}

#[derive(Default, Clone)]
pub struct ClipOptions {
    pub clear: bool,
}

#[derive(Default, Clone)]
pub struct PathOptions {
    pub add: Option<String>,
    pub remove: Option<String>,
}

#[derive(Default, Clone)]
pub struct WhichOptions {
    pub all: bool,
}

#[derive(Default, Clone)]
pub struct DfOptions {
    pub all: bool,
    pub human_readable: bool,
}

#[derive(Default, Clone)]
pub struct WhoamiOptions {}

#[derive(Default, Clone)]
pub struct SocketsOptions {
    pub show_all: bool,
    pub tcp_only: bool,
    pub udp_only: bool,
    pub listening_only: bool,
}

#[derive(Default, Clone)]
pub struct WcOptions {
    pub lines: bool,
    pub words: bool,
    pub bytes: bool,
    pub chars: bool,
}

#[derive(Default, Clone)]
pub struct LnOptions {
    pub symbolic: bool,
    pub force: bool,
}

#[derive(Default, Clone)]
pub struct ChmodOptions {
    pub recursive: bool,
}

#[derive(Clone)]
pub struct PmonOptions {
    pub delay_ms: u64,
}

impl Default for PmonOptions {
    fn default() -> Self {
        PmonOptions { delay_ms: 1000 }
    }
}

#[derive(Clone)]
pub struct WatchOptions {
    pub interval_ms: u64,
    pub diff: bool,
}

impl Default for WatchOptions {
    fn default() -> Self {
        WatchOptions { interval_ms: 2000, diff: false }
    }
}

#[derive(Clone)]
pub struct NettopOptions {
    pub delay_ms: u64,
}

impl Default for NettopOptions {
    fn default() -> Self {
        NettopOptions { delay_ms: 1000 }
    }
}

#[derive(Clone, Default)]
pub struct DuaOptions {}

#[derive(Clone, Default)]
pub struct BrowseOptions {}

#[derive(Clone, Default)]
pub struct EditOptions {}



pub fn list(options: ListOptions) {
    list::list(options);
}

pub fn rename(source: &str, destination: &str, options: RenameOptions) {
    rename::rename(source, destination, options);
}

pub fn copy(source: &str, destination: &str, options: CopyOptions) {
    copy::copy(source, destination, options);
}

pub fn remove(path: &str, options: &RemoveOptions) {
    remove::remove(path, options);
}

pub fn create(path: &str, options: CreateOptions) {
    create::create(path, options);
}

pub fn move_item(source: &str, destination: &str, options: MoveOptions) {
    r#move::move_item(source, destination, options);
}

pub fn archive(path: &str, options: ArchiveOptions) {
    archive::archive(path, options);
}

pub fn cat(path: &str, options: CatOptions) {
    cat::cat(path, options);
}

pub fn cat_to_writer(path: &str, options: CatOptions, writer: &mut dyn std::io::Write) -> Result<(), String> {
    cat::cat_to_writer(path, options, writer)
}

pub fn grep(pattern: &str, paths: Vec<String>, options: GrepOptions) {
    grep::grep(pattern, paths, options);
}

pub fn find(paths: Vec<String>, options: FindOptions) {
    find::find(paths, options);
}

pub fn diff(left: &str, right: &str, options: DiffOptions) {
    diff::diff(left, right, options);
}

pub fn search(phrase: &str, paths: Vec<String>, options: SearchOptions) {
    search::search(phrase, paths, options);
}

pub fn which(command: &str, options: WhichOptions) {
    which::which(command, options);
}

pub fn df(options: DfOptions) {
    df::df(options);
}

pub fn whoami(options: WhoamiOptions) {
    whoami::whoami(options);
}

pub fn sockets(options: SocketsOptions) {
    sockets::sockets(options);
}

pub fn wc(paths: Vec<String>, options: WcOptions) {
    wc::wc(paths, options);
}

pub fn ln(target: &str, link_name: &str, options: LnOptions) {
    ln::ln(target, link_name, options);
}

pub fn chmod(mode: &str, paths: Vec<String>, options: ChmodOptions) {
    chmod::chmod(mode, paths, options);
}

pub fn pmon(options: PmonOptions) {
    pmon::pmon(options);
}

pub fn watch(command: &str, options: WatchOptions) {
    watch::watch(command, options);
}

pub fn nettop(options: NettopOptions) {
    nettop::nettop(options);
}

pub fn dua(path: &str, options: DuaOptions) {
    dua::dua(path, options);
}

pub fn browse(path: &str, options: BrowseOptions) {
    browse::browse(path, options);
}

pub fn edit(filename: &str, options: EditOptions) {
    edit::edit(filename, options);
}



pub fn tree(path: &str, options: TreeOptions) {
    tree::tree(path, options);
}

pub fn du(paths: Vec<String>, options: DuOptions) {
    du::du(paths, options);
}

pub fn fastfetch() {
    fastfetch::fastfetch();
}

pub fn monitor() {
    monitor::monitor();
}

pub fn hash(file_path: &str, options: HashOptions) {
    hash::hash(file_path, options);
}

pub fn ps(options: PsOptions) {
    ps::ps(options);
}

pub fn kill(target: &str, options: KillOptions) {
    kill::kill(target, options);
}

pub fn fetch(url: &str, options: FetchOptions) {
    fetch::fetch(url, options);
}

pub fn env_action(var_name: Option<&str>, options: EnvOptions) {
    env_action::env_action(var_name, options);
}

pub fn hex(file_path: &str, options: HexOptions) {
    hex::hex(file_path, options);
}

pub fn ping(host: &str, options: PingOptions) {
    ping::ping(host, options);
}

pub fn base64(input_path: Option<&str>, options: Base64Options) {
    base64::run_base64(input_path, options);
}

pub fn encode(input_path: Option<&str>, options: EncodeOptions) {
    encode::run_encode(input_path, options);
}

pub fn decode(input_path: Option<&str>, options: DecodeOptions) {
    encode::run_decode(input_path, options);
}

pub fn uuid(options: UuidOptions) {
    uuid::run_uuid(options);
}

pub fn ip(options: IpOptions) {
    ip::run_ip(options);
}

pub fn echo(args: Vec<String>, options: EchoOptions) {
    echo::run_echo(args, options);
}

pub fn clip(options: ClipOptions) {
    clip::run_clip(options);
}

pub fn math(expr: Option<&str>) {
    math::evaluate(expr);
}

pub fn sleep(duration_str: &str) {
    sleep::run_sleep(duration_str);
}

pub fn time(cmd_args: Vec<String>) {
    time::run_time(cmd_args);
}

pub fn dns(host: &str) {
    dns::run_dns(host);
}

pub fn path(options: PathOptions) {
    path::run_path(options);
}

// ---------------------------------------------------------------------------
// Scrape
// ---------------------------------------------------------------------------

#[derive(Default, Clone)]
pub struct ScrapeOptions {
    /// File extensions to download (may include group aliases like "documents").
    pub formats: Vec<String>,
    /// Output directory (default: "./output").
    pub dest: String,
    /// Max crawl depth; 1 = only the start page (default).
    pub depth: usize,
    /// Max number of pages fetched during crawl (default: 10).
    pub max_pages: usize,
    /// Max total bytes downloaded across all files (default: 50 MiB).
    pub max_size_bytes: u64,
    /// Max links followed per page (default: 100).
    pub max_links: usize,
    /// Per-request timeout in seconds (default: 30).
    pub timeout_secs: u64,
    /// Allow video file downloads (default: false / blocked).
    pub include_video: bool,
    /// Allow audio file downloads (default: false / blocked).
    pub include_audio: bool,
    /// Skip image files even if their extension matches (default: false).
    pub no_images: bool,
    /// Only follow links that stay on the same domain as the start URL.
    pub same_domain: bool,
    /// Ignore robots.txt restrictions.
    pub ignore_robots: bool,
    /// Override the User-Agent string (otherwise a realistic browser UA is chosen).
    pub user_agent: Option<String>,
    /// Print what would be downloaded without actually writing any files.
    pub dry_run: bool,
    /// Print detailed per-URL decisions.
    pub verbose: bool,
    /// Overwrite existing files in the destination (default: rename with counter).
    pub overwrite: bool,
    /// Milliseconds to sleep between HTTP requests (default: 0 = no delay).
    pub rate_limit_ms: u64,
}

pub fn scrape(url: &str, options: ScrapeOptions) {
    scrape::run_scrape(url, options);
}

// ---------------------------------------------------------------------------
// Sort
// ---------------------------------------------------------------------------

#[derive(Default, Clone)]
pub struct SortOptions {
    /// Reverse the sort order.
    pub reverse: bool,
    /// Numeric sort (compare as floating-point numbers).
    pub numeric: bool,
    /// Output only unique lines (remove duplicates).
    pub unique: bool,
    /// Case-insensitive comparison.
    pub ignore_case: bool,
    /// Sort by the Nth whitespace-delimited field (1-indexed; 0 = whole line).
    pub field: usize,
    /// Field separator character (default: whitespace).
    pub separator: Option<char>,
    /// Check if input is already sorted; exit 0 if yes, 1 if no.
    pub check: bool,
}

pub fn sort(paths: Vec<String>, options: SortOptions) {
    sort::run_sort(paths, options);
}

pub fn json(input_path: Option<&str>, options: JsonOptions) {
    json::run_json(input_path, options);
}

pub fn plot(input_path: Option<&str>, options: PlotOptions) {
    plot::run_plot(input_path, options);
}

#[derive(Default, Clone)]
pub struct ClockOptions {
    pub timer_duration: Option<String>,
    pub mode: Option<String>,
}

pub fn clock(options: ClockOptions) {
    clock::run_clock(options);
}

#[derive(Default, Clone)]
pub struct TextOptions {
    pub case: Option<String>,
    pub width: Option<usize>,
    pub align: Option<String>,
    pub truncate: bool,
    pub ellipsis: Option<String>,
    pub strip_ansi: bool,
    pub strip_non_alphanumeric: bool,
    pub output: Option<String>,
}

pub fn text(input_path: Option<&str>, options: TextOptions) {
    text::run_text(input_path, options);
}

#[derive(Default, Clone)]
pub struct GlobeOptions {
    pub mode: Option<String>,
    pub center: Option<String>,
    pub day_night: bool,
}

pub fn globe(input_path: Option<&str>, options: GlobeOptions) {
    globe::run_globe(input_path, options);
}

#[derive(Default, Clone)]
pub struct LogOptions {
    pub format: Option<String>,
    pub query: Option<String>,
    pub stats: bool,
    pub limit: Option<usize>,
    pub output: Option<String>,
}

pub fn log_action(input_path: Option<&str>, options: LogOptions) {
    log_parser::run_log(input_path, options);
}

#[derive(Default, Clone)]
pub struct LifeOptions {
    pub fps: Option<u32>,
    pub preset: Option<String>,
}

pub fn life_action(options: LifeOptions) {
    life::run_life(options);
}

pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    clip::set_clipboard(text)
}

pub fn read_from_clipboard() -> Result<String, String> {
    clip::get_clipboard()
}

