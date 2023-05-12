use crate::paths::{PathItem, PathItems};

pub enum InputMode {
    Select,
    Search,
}

/// App holds the state of the application
pub struct TuiState {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// Index of selected PathItem. In filtered list, not in the orignal full list
    pub selected: usize,
    pub filtered_len: usize,
    pub items: PathItems,
    pub quit: bool,
    pub selected_path: Option<PathItem>,
}

impl TuiState {
    pub fn new(items: PathItems) -> Self {
        Self {
            items,
            ..Default::default()
        }
    }
}

impl Default for TuiState {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Search,
            items: PathItems::default(),
            quit: false,
            selected: 0,
            filtered_len: 0,
            selected_path: None,
        }
    }
}
