use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;

use super::tui_state::{InputMode, TuiState};

pub fn handle_event<'a>(app: &'a mut TuiState<'a>) -> io::Result<&'a mut TuiState<'a>> {
    if let Event::Key(key) = event::read()? {
        // Ctrl-C will close out the program instantly
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            app.quit = true;
            return Ok(app);
        }

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
                        app.selected_path = app.highlighted
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

    // TODO: handle this more cleanly. Now were filtering results here, and in the UI
    app.filtered_len = app.items.filter_paths(&app.input).count();
    app.highlighted = if app.filtered_len > 0 {
        Some(app.items.filter(&app.input).get(app.selected).unwrap())
    } else {
        None
    };

    Ok(app)
}
