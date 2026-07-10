use std::{io, process::Command};
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

pub fn run_gitv() -> io::Result<()> {
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let output = Command::new("git")
        .args(&["log", "--oneline", "--graph", "--color=never", "-n", "100"])
        .output();
        
    let commits = match output {
        Ok(out) if out.status.success() => {
            let stdout_str = String::from_utf8_lossy(&out.stdout);
            stdout_str.lines().map(|s| s.to_string()).collect()
        }
        _ => vec!["Not a git repository, or git is not installed.".to_string()],
    };

    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = commits
                .iter()
                .map(|line| ListItem::new(line.clone()))
                .collect();

            let list = List::new(items)
                .block(Block::default().title(" Git Log ").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightYellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut state);

            let help = Paragraph::new("Use UP/DOWN to navigate, 'q' or ESC to quit")
                .block(Block::default().borders(Borders::ALL).title(" Help "));
            f.render_widget(help, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down => {
                    let i = match state.selected() {
                        Some(i) => {
                            if commits.is_empty() { 0 }
                            else if i >= commits.len() - 1 { 0 } 
                            else { i + 1 }
                        }
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match state.selected() {
                        Some(i) => {
                            if commits.is_empty() { 0 }
                            else if i == 0 { commits.len() - 1 } 
                            else { i - 1 }
                        }
                        None => 0,
                    };
                    state.select(Some(i));
                }
                _ => {}
            }
        }
    }
}
