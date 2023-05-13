use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use super::tui_state::{InputMode, TuiInnerState};

fn help_message_widget<'a>(app: &'a TuiInnerState<'a>) -> Paragraph<'a> {
    let (msg, style) = match app.input_mode {
        InputMode::Select => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start searching."),
            ],
            Style::default(),
        ),
        InputMode::Search => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop searching. "),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    Paragraph::new(text)
}

fn input_widget<'a>(app: &'a TuiInnerState<'a>) -> Paragraph<'a> {
    Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Select => Style::default(),
            InputMode::Search => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"))
}

fn paths_view_widget<'a>(app: &'a TuiInnerState<'a>) -> List<'a> {
    // TODO: show name and optional description!
    let paths: Vec<ListItem> = app
        .items
        .filter_paths(&app.input)
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m.full_path)))];
            if i == app.selected {
                ListItem::new(content).style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                ListItem::new(content)
            }
        })
        .collect();

    List::new(paths)
        .start_corner(Corner::BottomLeft)
        .block(Block::default().borders(Borders::ALL).title("Paths"))
}

fn path_description_widget<'a>(app: &'a TuiInnerState<'a>) -> Paragraph<'a> {
    if app.highlighted.is_none() {
        return Paragraph::new("No item seleceted");
    }

    let highlighted = app.highlighted.unwrap();

    let name = Spans::from(vec![
        Span::styled("Name ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(&highlighted.name),
    ]);

    let description = Spans::from(vec![
        Span::styled(
            "Description ",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(&highlighted.description),
    ]);

    Paragraph::new(vec![name, description])
        .block(Block::default().borders(Borders::all()).title("Info"))
}

pub fn ui<'a, B: Backend>(f: &mut Frame<B>, app: &'a TuiInnerState<'a>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                // Input help message
                Constraint::Length(1),
                // Path description
                Constraint::Length(4),
                // List of paths
                Constraint::Min(1),
                // Input
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let help_message = help_message_widget(app);
    f.render_widget(help_message, chunks[0]);

    // TODO: Display extra info about the path, split text based on the width of
    //       current terminal window
    let description = path_description_widget(app);
    f.render_widget(description, chunks[1]);

    let paths = paths_view_widget(app);
    f.render_widget(paths, chunks[2]);

    match app.input_mode {
        InputMode::Select =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Search => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[3].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[3].y + 1,
            )
        }
    }

    let input = input_widget(app);
    f.render_widget(input, chunks[3]);
}
