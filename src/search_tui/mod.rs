use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, Stdout},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::paths::{PathItem, PathItems};

mod event_handler;
mod tui_state;
mod tui_ui;
use event_handler::handle_event;
use tui_state::TuiState;
use tui_ui::ui;

type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;

fn setup_terminal() -> Result<CrossTerminal, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut CrossTerminal) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

/// Return new PathItems if it has been edited
pub fn run_edit_tui(items: &PathItems) -> Result<Option<PathItems>, Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let mut app = TuiState::new(items, true);
    let res = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal)?;

    if let Err(ref err) = res {
        eprintln!("{:?}", err)
    }

    if let Some(path) = res? {
        Ok(path.edited_items())
    } else {
        Ok(None)
    }
}

pub fn run_select_tui(items: &PathItems) -> Result<Option<PathItem>, Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let mut app = TuiState::new(items, false);
    let res = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal)?;

    if let Err(ref err) = res {
        eprintln!("{:?}", err)
    }

    if let Some(path) = res? {
        Ok(path.selected_path.cloned())
    } else {
        Ok(None)
    }
}

fn run_app<'a, B: Backend>(
    terminal: &mut Terminal<B>,
    app: &'a mut TuiState<'a>,
) -> io::Result<Option<&'a TuiState<'a>>> {
    let mut state = app;
    loop {
        terminal.draw(|f| ui(f, state))?;
        state = handle_event(state)?;

        if state.quit {
            return Ok(None);
        }

        if state.selected_path.is_some() {
            return Ok(Some(state));
        }
    }
}
