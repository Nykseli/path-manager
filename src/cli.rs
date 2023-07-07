use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Mode {
    AddPath {
        path: String,
    },
    Pwd {
        path: Option<String>,
    },
    Tui {
        #[arg(short, long, help = "Launch TUI in edit mode")]
        edit: bool,
        input: Vec<String>,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] //TODO: fill values
pub struct Args {
    #[command(help = "Which mode the program is launch in", subcommand)]
    mode: Mode,
}

impl Args {
    pub fn mode(&self) -> &Mode {
        &self.mode
    }
}
