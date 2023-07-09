/// tmux.rs contains functions to interract with the current tmux instance
use std::{marker::PhantomData, process::Command};

#[derive(Debug)]
pub struct Uninit;
#[derive(Debug)]
pub struct Initialized;

#[derive(Debug)]
pub struct Tmux<State = Uninit> {
    /// #{pane_index} of the tmux pane we're running this program
    pane_index: u32,
    /// #{window_index} of the tmux window we're running this program
    window_index: u32,
    state: PhantomData<State>,
}

impl Tmux<Uninit> {
    pub fn new() -> Self {
        Self {
            pane_index: 0,
            window_index: 0,
            state: PhantomData::<Uninit>,
        }
    }

    pub fn init(self) -> Tmux<Initialized> {
        let tmux_pane = std::env::var("TMUX_PANE")
            .expect("Couldn't find the 'TMUX_PANE' environment variable. Make sure you're in tmux");

        // Get the process tmux window and pane ids
        let output = Command::new("tmux")
            .arg("display")
            .arg("-pt")
            .arg(tmux_pane)
            .arg("#{window_index} #{pane_index}")
            .output()
            .unwrap()
            .stdout;
        let stdout = String::from_utf8(output).unwrap();
        let output = stdout.trim();

        let mut ouput_split = output.split(' ');

        let window_index = ouput_split.next().unwrap();
        let window_index = window_index.parse::<u32>().unwrap();

        let pane_index = ouput_split.next().unwrap();
        let pane_index = pane_index.parse::<u32>().unwrap();

        Tmux::<Initialized> {
            pane_index,
            window_index,
            state: PhantomData::<Initialized>,
        }
    }
}

impl Tmux<Initialized> {
    /// Send 'cd [`path`]' command to the tmux pane
    pub fn cd_into(&self, path: &str) {
        Command::new("tmux")
            .arg("send-keys")
            .arg(format!("-t:{}.{}", self.window_index, self.pane_index))
            .arg(format!("cd {path}"))
            .arg("C-m")
            .spawn()
            .unwrap();
    }

    pub fn cd_pwd(&self) {
        let output = Command::new("tmux")
            .arg("showenv")
            .arg("-g")
            .arg("PATH_MANAGER_PWD")
            .output()
            .unwrap();

        let output = if output.stderr.is_empty() && !output.stdout.is_empty() {
            String::from_utf8(output.stdout).unwrap()
        } else {
            eprintln!("Path manager pwd is not defined");
            return;
        };

        let path = output.split('=').last().unwrap().trim();
        self.cd_into(path);
    }

    pub fn save_pwd(&self, path: &str) {
        let path: String = std::fs::canonicalize(path)
            .unwrap_or_else(|_| panic!("Path '{path}' was not found"))
            .to_str()
            .unwrap()
            .into();

        Command::new("tmux")
            .arg("setenv")
            .arg("-g")
            .arg("PATH_MANAGER_PWD")
            .arg(path)
            .spawn()
            .unwrap();
    }
}
