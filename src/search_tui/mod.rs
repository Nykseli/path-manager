use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
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

use self::tui_state::TuiInnerState;

pub fn tui_run(items: &PathItems) -> Result<Option<PathItem>, Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = TuiState::new(items);
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(ref err) = res {
        eprintln!("{:?}", err)
    }

    if let Some(path) = res? {
        Ok(path.selected_path.map(|p| p.clone()))
    } else {
        Ok(None)
    }
}

fn run_app<'a, B: Backend>(
    terminal: &mut Terminal<B>,
    app: &'a mut TuiState<'a>,
) -> io::Result<Option<&'a TuiInnerState<'a>>> {
    let mut state = app.state_mut();
    loop {
        terminal.draw(|f| ui(f, state))?;
        state = handle_event(state)?;

        if state.quit {
            return Ok(None);
        }

        if let Some(selected) = &state.selected_path {
            return Ok(Some(state));
        }
    }
}
