use std::{env, fs, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

use crate::{RenameOptions, CopyOptions, RemoveOptions};
use crate::rename::rename;
use crate::copy::copy;
use crate::remove::remove;

pub fn run_fm() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }
    Ok(())
}

fn suspend_tui() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn resume_tui() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut current_dir = env::current_dir()?;
    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        let mut entries: Vec<String> = vec!["..".to_string()];
        if let Ok(read_dir) = fs::read_dir(&current_dir) {
            for entry in read_dir.flatten() {
                entries.push(entry.file_name().to_string_lossy().to_string());
            }
        }
        entries.sort();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = entries
                .iter()
                .map(|name| ListItem::new(name.clone()))
                .collect();

            let title = format!(" File Manager: {} ", current_dir.display());
            let list = List::new(items)
                .block(Block::default().title(title).borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightBlue)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut state);

            let help = Paragraph::new("UP/DOWN: nav | ENTER: open | c: copy | r: rename | d: delete | q/ESC: quit")
                .block(Block::default().borders(Borders::ALL).title(" Help "));
            f.render_widget(help, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down => {
                    let i = match state.selected() {
                        Some(i) => if entries.is_empty() { 0 } else if i >= entries.len() - 1 { 0 } else { i + 1 },
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match state.selected() {
                        Some(i) => if entries.is_empty() { 0 } else if i == 0 { entries.len() - 1 } else { i - 1 },
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Enter => {
                    if let Some(i) = state.selected() {
                        if i < entries.len() {
                            let selected = &entries[i];
                            if selected == ".." {
                                if let Some(parent) = current_dir.parent() {
                                    current_dir = parent.to_path_buf();
                                    state.select(Some(0));
                                }
                            } else {
                                let new_path = current_dir.join(selected);
                                if new_path.is_dir() {
                                    current_dir = new_path;
                                    state.select(Some(0));
                                }
                            }
                        }
                    }
                }
                KeyCode::Char('d') => {
                    if let Some(i) = state.selected() {
                        if i > 0 && i < entries.len() { // Don't delete ".."
                            let target = current_dir.join(&entries[i]);
                            suspend_tui()?;
                            println!("Deleting {}", target.display());
                            let mut opts = RemoveOptions::default();
                            opts.interactive = true; // Use ir remove's confirmation
                            remove(target.to_str().unwrap(), &opts);
                            println!("Press ENTER to continue...");
                            let mut buf = String::new();
                            io::stdin().read_line(&mut buf)?;
                            resume_tui()?;
                        }
                    }
                }
                KeyCode::Char('c') => {
                    if let Some(i) = state.selected() {
                        if i > 0 && i < entries.len() {
                            let target = current_dir.join(&entries[i]);
                            suspend_tui()?;
                            println!("Copying {}", target.display());
                            print!("Enter destination path: ");
                            use std::io::Write;
                            io::stdout().flush()?;
                            let mut dest = String::new();
                            io::stdin().read_line(&mut dest)?;
                            let dest = dest.trim();
                            if !dest.is_empty() {
                                let opts = CopyOptions { recursive: true, ..Default::default() };
                                copy(target.to_str().unwrap(), dest, opts);
                            }
                            println!("Press ENTER to continue...");
                            let mut buf = String::new();
                            io::stdin().read_line(&mut buf)?;
                            resume_tui()?;
                        }
                    }
                }
                KeyCode::Char('r') => {
                    if let Some(i) = state.selected() {
                        if i > 0 && i < entries.len() {
                            let target = current_dir.join(&entries[i]);
                            suspend_tui()?;
                            println!("Renaming {}", target.display());
                            print!("Enter new name: ");
                            use std::io::Write;
                            io::stdout().flush()?;
                            let mut new_name = String::new();
                            io::stdin().read_line(&mut new_name)?;
                            let new_name = new_name.trim();
                            if !new_name.is_empty() {
                                let opts = RenameOptions::default();
                                rename(target.to_str().unwrap(), new_name, opts);
                            }
                            println!("Press ENTER to continue...");
                            let mut buf = String::new();
                            io::stdin().read_line(&mut buf)?;
                            resume_tui()?;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
