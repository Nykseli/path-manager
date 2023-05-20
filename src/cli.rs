use clap::{Parser, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    AddPath,
    Tui,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] //TODO: fill values
pub struct Args {
    #[arg(
        short = 'm',
        long = "mode",
        help = "Which mode the program is launch in",
        value_enum
    )]
    mode: Option<Mode>,
    path: Option<String>,
}

impl Args {
    pub fn mode(&self) -> Mode {
        self.mode.unwrap_or(Mode::AddPath)
    }

    pub fn path(&self) -> &Option<String> {
        &self.path
    }
}
