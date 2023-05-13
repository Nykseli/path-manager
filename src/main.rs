use clap::Parser;
use search_tui::tui_run;
use std::{
    fs,
    io::{self, BufRead, Write},
};

mod cli;
mod config_path;
mod paths;
mod search_tui;
use cli::{Args, Mode};

use crate::config_path::{load_saved_paths, save_paths};

fn add_path(args: &Args) {
    let path: String = if let Some(path) = args.path() {
        fs::canonicalize(path)
            .unwrap_or_else(|_| panic!("Path '{path}' was not found"))
            .to_str()
            .unwrap()
            .into()
    } else {
        // TODO: handle this in cli.rs
        eprintln!("Missing path argument in add-path mode");
        std::process::exit(1);
    };

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

    let new_path = paths::PathItem {
        description: description_buf.trim().into(),
        name: name_buf.trim().into(),
        full_path: path.trim().into(),
    };

    paths.add_path(new_path);
    save_paths(paths);
}

fn main() {
    let args = Args::parse();
    match args.mode() {
        Mode::AddPath => add_path(&args),
        Mode::Tui => {
            let items = load_saved_paths();
            if let Ok(Some(path)) = tui_run(&items) {
                println!("Selected path: {:?}", path);
            }
        }
    };
}
