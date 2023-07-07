use clap::Parser;
use search_tui::{run_edit_tui, run_select_tui};
use std::{
    fs,
    io::{self, BufRead, Write},
};

mod cli;
mod config_path;
mod paths;
mod search_tui;
mod tmux;
use cli::{Args, Mode};
use tmux::Tmux;

use crate::config_path::{load_saved_paths, save_paths};

fn add_path(path: &str) {
    let path: String = fs::canonicalize(path)
        .unwrap_or_else(|_| panic!("Path '{path}' was not found"))
        .to_str()
        .unwrap()
        .into();

    let stdin = io::stdin();
    let mut paths = load_saved_paths();
    let existing = paths.exists(&path);

    if existing {
        print!("Path '{path}' already exists.\nWant to override (y/n): ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        let _ = stdin.lock().read_line(&mut answer).unwrap();
        if answer.trim().to_lowercase().starts_with('n') {
            std::process::exit(0);
        }
    }

    if existing {
        println!("Overriding path: {path}");
    } else {
        println!("Adding path: {path}");
    }

    print!("Path name: ");
    io::stdout().flush().unwrap();
    let mut name_buf = String::new();
    {
        let _ = stdin.lock().read_line(&mut name_buf).unwrap();
    }
    print!("Path description: ");
    io::stdout().flush().unwrap();
    let mut description_buf = String::new();
    {
        let _ = stdin.lock().read_line(&mut description_buf).unwrap();
    }

    let new_path = paths::PathItem::new(
        name_buf.trim().into(),
        path.trim().into(),
        description_buf.trim().into(),
    );

    paths.add_path(new_path);
    save_paths(paths);
}

fn main() {
    let args = Args::parse();
    match args.mode() {
        Mode::Pwd { path } => {
            let tmux = Tmux::new().init();
            if let Some(path) = path {
                tmux.save_pwd(path)
            } else {
                tmux.cd_pwd()
            }
        }
        Mode::AddPath { path } => add_path(path),
        Mode::Tui { edit, input } => {
            let mut items = load_saved_paths();
            items.sort();
            if *edit {
                if let Ok(Some(items)) = run_edit_tui(&items, input) {
                    save_paths(items);
                }
            } else if let Ok(Some(path)) = run_select_tui(&items, input) {
                let tmux = Tmux::new();
                let tmux = tmux.init();
                tmux.cd_into(&path.full_path);
            }
        }
    };
}
