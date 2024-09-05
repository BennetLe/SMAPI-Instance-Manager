use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::Terminal;
use std::error::Error;
use std::io;

mod app;
mod ui;

use app::{App, CurrentScreen, CurrentlyAdding};
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            match app.screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Up => {
                        app.select_prev();
                    }
                    KeyCode::Down => {
                        app.select_next();
                    }
                    KeyCode::Enter => app.manager.run(
                        app.manager
                            .instances
                            .get(&app.current_instance)
                            .unwrap()
                            .clone(),
                    ),
                    KeyCode::Char('a') => {
                        app.screen = CurrentScreen::Add;
                        app.adding = Some(CurrentlyAdding::Name);
                    }
                    KeyCode::Char('q') => {
                        app.screen = CurrentScreen::Exit;
                    }
                    KeyCode::Char('r') => {
                        app.screen = CurrentScreen::Remove;
                    }
                    KeyCode::Char('o') => {
                        app.manager.open(
                            app.manager
                                .instances
                                .get(&app.current_instance)
                                .unwrap()
                                .clone(),
                        );
                    }
                    _ => {}
                },
                CurrentScreen::Add => match key.code {
                    KeyCode::Enter => {
                        if let Some(adding) = &app.adding {
                            match adding {
                                CurrentlyAdding::Name => {
                                    if app.manager.instances.contains_key(&app.name_input) {
                                        continue;
                                    }
                                    app.adding = Some(CurrentlyAdding::FolderName);
                                }
                                CurrentlyAdding::FolderName => {
                                    app.adding = Some(CurrentlyAdding::SmapiPath);
                                }
                                CurrentlyAdding::SmapiPath => {
                                    app.save_instance();
                                    app.screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(adding) = &app.adding {
                            match adding {
                                CurrentlyAdding::Name => {
                                    app.name_input.pop();
                                }
                                CurrentlyAdding::FolderName => {
                                    app.folder_name_input.pop();
                                }
                                CurrentlyAdding::SmapiPath => {
                                    app.smapi_path_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.screen = CurrentScreen::Main;
                        app.adding = None;
                    }
                    KeyCode::Tab => {
                        app.toggle_adding();
                    }
                    KeyCode::Char(value) => {
                        if let Some(adding) = &app.adding {
                            match adding {
                                CurrentlyAdding::Name => {
                                    app.name_input.push(value);
                                }
                                CurrentlyAdding::FolderName => {
                                    app.folder_name_input.push(value);
                                }
                                CurrentlyAdding::SmapiPath => {
                                    app.smapi_path_input.push(value);
                                }
                            }
                        }
                    }
                    _ => (),
                },
                CurrentScreen::Exit => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('n') => {
                        app.screen = CurrentScreen::Main;
                    }
                    _ => (),
                },
                CurrentScreen::Remove => match key.code {
                    KeyCode::Char('y') => {
                        // TODO: Delete without folder
                        let to_delete: String = app.current_instance.clone();
                        app.select_next();
                        app.manager.remove_instance(to_delete, false);
                        app.screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('a') => {
                        // TODO: Delete with folder
                        let to_delete: String = app.current_instance.clone();
                        app.select_next();
                        app.manager.remove_instance(to_delete, true);
                        app.screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('n') => {
                        app.screen = CurrentScreen::Main;
                    }
                    _ => (),
                },
            }
        }
    }
}
