pub mod list;
pub mod help;
pub mod rename;
pub mod copy;
pub mod remove;
pub mod create;
pub mod r#move;

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
