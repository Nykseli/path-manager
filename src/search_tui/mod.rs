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

pub fn tui_run(items: PathItems) -> Result<Option<PathItem>, Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = TuiState::new(items);
    let res = run_app(&mut terminal, app);

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

    Ok(res?)
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: TuiState,
) -> io::Result<Option<PathItem>> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        handle_event(&mut app)?;

        if app.quit {
            return Ok(None);
        }

        if let Some(selected) = app.selected_path {
            return Ok(Some(selected));
        }
    }
}
