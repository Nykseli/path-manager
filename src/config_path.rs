use std::{
    env, fs,
    io::{ErrorKind, Write},
    path,
};

use crate::paths::PathItems;

fn paths_file() -> String {
    let home = env::var("HOME").expect("HOME not found in env");
    let full_dir_path = format!("{home}/.config/path-manager");
    let dir_path = path::Path::new(&full_dir_path);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path).unwrap_or_else(|_| panic!("Cannot create {full_dir_path}"))
    }

    format!("{full_dir_path}/paths.json")
}

fn read_paths_file() -> String {
    let path = paths_file();

    match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => {
            if e.kind() != ErrorKind::NotFound {
                panic!("{:?}", e.kind())
            } else {
                "".into()
            }
        }
    }
}

/// Load saved file paths.
/// PathItems will be empty if no files are saved.
pub fn load_saved_paths() -> PathItems {
    let file_data = read_paths_file();
    PathItems::from_json(&file_data)
}

pub fn save_paths(items: PathItems) {
    let path = paths_file();
    let json_str = items.into_json();
    let mut output = fs::File::create(path).expect("Cannot paths.json file");
    write!(output, "{json_str}").unwrap();
}
