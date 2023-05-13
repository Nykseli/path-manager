use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;

use super::tui_state::{InputMode, TuiState};

pub fn handle_event(app: &mut TuiState) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        // Ctrl-C will close out the program instantly
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            app.quit = true;
            return Ok(());
        }

        match app.input_mode {
            InputMode::Select => match key.code {
                KeyCode::Char('e') => {
                    app.input_mode = InputMode::Search;
                }
                KeyCode::Char('q') => {
                    app.quit = true;
                }
                // j is down in vi!
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                // k is up in vi!
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.filtered_len > 0 && app.selected < app.filtered_len - 1 {
                        app.selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if app.filtered_len > 0 {
                        app.selected_path = Some(
                            app.items
                                .filter_paths(&app.input)
                                .nth(app.selected)
                                .unwrap()
                                .clone(),
                        );
                    }
                }
                _ => {}
            },
            InputMode::Search => match key.code {
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Esc | KeyCode::Enter => {
                    app.input_mode = InputMode::Select;
                }
                _ => {}
            },
        }
    }

    Ok(())
}
