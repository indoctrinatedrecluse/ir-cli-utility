use std::{env, io};
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

pub fn run_envv() -> io::Result<()> {
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
    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        let mut env_vars: Vec<(String, String)> = env::vars().collect();
        env_vars.sort_by(|a, b| a.0.cmp(&b.0));

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = env_vars
                .iter()
                .map(|(k, v)| {
                    let content = format!("{} = {}", k, v);
                    ListItem::new(content)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title(" Environment Variables ").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut state);

            let help = Paragraph::new("UP/DOWN: nav | e: edit | d: delete | q/ESC: quit")
                .block(Block::default().borders(Borders::ALL).title(" Help "));
            f.render_widget(help, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down => {
                    let i = match state.selected() {
                        Some(i) => if env_vars.is_empty() { 0 } else if i >= env_vars.len() - 1 { 0 } else { i + 1 },
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match state.selected() {
                        Some(i) => if env_vars.is_empty() { 0 } else if i == 0 { env_vars.len() - 1 } else { i - 1 },
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Char('e') => {
                    if let Some(i) = state.selected() {
                        if i < env_vars.len() {
                            let (k, _v) = &env_vars[i];
                            suspend_tui()?;
                            println!("Editing variable: {}", k);
                            print!("Enter new value: ");
                            use std::io::Write;
                            io::stdout().flush()?;
                            let mut new_val = String::new();
                            io::stdin().read_line(&mut new_val)?;
                            let new_val = new_val.trim();
                            
                            print!("Save changes to {}? (y/n): ", k);
                            io::stdout().flush()?;
                            let mut conf = String::new();
                            io::stdin().read_line(&mut conf)?;
                            if conf.trim().eq_ignore_ascii_case("y") {
                                env::set_var(k, new_val);
                                #[cfg(windows)]
                                {
                                    let _ = std::process::Command::new("setx")
                                        .arg(k)
                                        .arg(new_val)
                                        .output();
                                    println!("Updated permanently via setx.");
                                }
                                #[cfg(unix)]
                                {
                                    println!("Updated in current session. To make permanent, add 'export {}={}' to your profile.", k, new_val);
                                }
                            }
                            
                            println!("Press ENTER to continue...");
                            let mut buf = String::new();
                            io::stdin().read_line(&mut buf)?;
                            resume_tui()?;
                        }
                    }
                }
                KeyCode::Char('d') => {
                    if let Some(i) = state.selected() {
                        if i < env_vars.len() {
                            let (k, _) = &env_vars[i];
                            suspend_tui()?;
                            println!("Are you sure you want to delete variable: {}?", k);
                            print!("Type 'yes' to confirm: ");
                            use std::io::Write;
                            io::stdout().flush()?;
                            let mut conf1 = String::new();
                            io::stdin().read_line(&mut conf1)?;
                            
                            if conf1.trim().eq_ignore_ascii_case("yes") {
                                print!("Are you REALLY sure? (y/n): ");
                                io::stdout().flush()?;
                                let mut conf2 = String::new();
                                io::stdin().read_line(&mut conf2)?;
                                
                                if conf2.trim().eq_ignore_ascii_case("y") {
                                    env::remove_var(k);
                                    #[cfg(windows)]
                                    {
                                        let _ = std::process::Command::new("reg")
                                            .args(&["delete", "HKCU\\Environment", "/F", "/V", k])
                                            .output();
                                        println!("Deleted permanently from registry.");
                                    }
                                    #[cfg(unix)]
                                    {
                                        println!("Deleted in current session.");
                                    }
                                }
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
