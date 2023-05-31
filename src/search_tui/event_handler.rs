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
                KeyCode::Down => {
                    if app.selected > 0 {
                        app.selected -= 1;
                        app.set_highlighted();
                    }
                }
                KeyCode::Up => {
                    if !app.filtered.is_empty() && app.selected < app.filtered.len() - 1 {
                        app.selected += 1;
                        app.set_highlighted();
                    }
                }
                KeyCode::End => {
                    app.cursor = app.input.chars().count() as u16;
                }
                KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.cursor = app.input.chars().count() as u16;
                }
                KeyCode::Home => {
                    app.cursor = 0;
                }
                KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.cursor = 0;
                }
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.input.clear();
                    app.cursor = 0;
                    input_changed = true;
                }
                KeyCode::Left => {
                    app.cursor = app.cursor.saturating_sub(1);
                }
                KeyCode::Right => {
                    // as usize cast is safe since u16 always fits in usize
                    app.cursor = if (app.cursor as usize) < app.input.chars().count() {
                        app.cursor + 1
                    } else {
                        app.cursor
                    }
                }
                KeyCode::Char(c) => {
                    // as usize cast is safe since u16 always fits in usize
                    app.input.insert(app.cursor as usize, c);
                    app.cursor += 1;
                    input_changed = true;
                }
                KeyCode::Backspace => {
                    app.cursor = app.cursor.saturating_sub(1);
                    if !app.input.is_empty() {
                        // as usize cast is safe since u16 always fits in usize
                        app.input.remove(app.cursor as usize);
                    }
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
