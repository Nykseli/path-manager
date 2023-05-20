use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;

use super::tui_state::{InputMode, PathEditCommand, TuiState};

pub fn handle_event<'a>(app: &'a mut TuiState<'a>) -> io::Result<&'a mut TuiState<'a>> {
    if let Event::Key(key) = event::read()? {
        // Ctrl-C will close out the program instantly
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            app.quit = true;
            return Ok(app);
        }

        let mut input_changed = false;

        match app.input_mode {
            InputMode::Select => match key.code {
                KeyCode::Char('s') => {
                    app.input_mode = InputMode::Search;
                }
                KeyCode::Char('q') => {
                    app.quit = true;
                }
                // j is down in vi!
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.selected > 0 {
                        app.selected -= 1;
                        app.set_highlighted();
                    }
                }
                // k is up in vi!
                KeyCode::Up | KeyCode::Char('k') => {
                    if !app.filtered.is_empty() && app.selected < app.filtered.len() - 1 {
                        app.selected += 1;
                        app.set_highlighted();
                    }
                }
                KeyCode::Enter => {
                    if !app.filtered.is_empty() {
                        app.selected_path = app.highlighted;
                    }
                }
                KeyCode::Char('d') => {
                    if app.edit_mode {
                        app.set_path_command(PathEditCommand::Delete);
                    }
                }
                _ => {}
            },
            InputMode::Search => match key.code {
                KeyCode::Char(c) => {
                    app.input.push(c);
                    input_changed = true;
                }
                KeyCode::Backspace => {
                    app.input.pop();
                    input_changed = true;
                }
                KeyCode::Esc | KeyCode::Enter => {
                    app.input_mode = InputMode::Select;
                }
                _ => {}
            },
        }

        if input_changed {
            app.filtered = app.items.filter(&app.input);
            app.set_highlighted();
        }
    }

    Ok(app)
}
