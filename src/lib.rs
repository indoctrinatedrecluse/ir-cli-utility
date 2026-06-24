pub mod list;
pub mod help;
pub mod rename;

#[derive(Default)]
pub struct ListOptions {
    pub show_all: bool,
    pub sort_by_size: bool,
    pub sort_by_time: bool,
    pub filter: Option<String>,
}

#[derive(Default)]
pub struct RenameOptions {
    pub force: bool,
    pub interactive: bool,
    pub force_links: bool,
}

pub fn list(options: ListOptions) {
    list::list(options);
}

pub fn rename(source: &str, destination: &str, options: RenameOptions) {
    rename::rename(source, destination, options);
}
