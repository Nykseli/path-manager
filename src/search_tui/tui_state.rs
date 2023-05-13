use crate::paths::{PathItem, PathItems};

pub enum InputMode {
    Select,
    Search,
}

pub struct TuiInnerState<'a> {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// Index of selected PathItem. In filtered list, not in the orignal full list
    pub selected: usize,
    pub highlighted: Option<&'a PathItem>,
    pub filtered_len: usize,
    pub items: &'a PathItems,
    pub quit: bool,
    pub selected_path: Option<&'a PathItem>,
}

/// App holds the state of the application
pub struct TuiState<'a> {
    inner: TuiInnerState<'a>,
}

impl<'a> TuiState<'a> {
    pub fn new(items: &'a PathItems) -> Self {
        Self {
            inner: TuiInnerState {
                items,
                input: String::new(),
                input_mode: InputMode::Search,
                quit: false,
                selected: 0,
                filtered_len: 0,
                selected_path: None,
                highlighted: None,
            },
        }
    }

    pub fn state_mut(&'a mut self) -> &'a mut TuiInnerState<'a> {
        &mut self.inner
    }
}
