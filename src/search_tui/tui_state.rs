use std::collections::HashMap;

use crate::paths::{PathItem, PathItems};

pub enum PathEditCommand {
    Delete,
}

pub enum InputMode {
    Select,
    Search,
}

/// App holds the state of the application
pub struct TuiState<'a> {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// Index of selected PathItem. In filtered list, not in the orignal full list
    pub selected: usize,
    pub highlighted: Option<&'a PathItem>,
    pub filtered: Vec<&'a PathItem>,
    pub items: &'a PathItems,
    pub quit: bool,
    pub selected_path: Option<&'a PathItem>,
    pub edits: HashMap<&'a PathItem, PathEditCommand>,
}

impl<'a> TuiState<'a> {
    pub fn new(items: &'a PathItems) -> Self {
        let mut state = Self {
            items,
            input: String::new(),
            // Empty string in a filter just copies everything
            filtered: items.filter(""),
            input_mode: InputMode::Search,
            quit: false,
            selected: 0,
            selected_path: None,
            highlighted: None,
            edits: HashMap::new(),
        };

        state.set_highlighted();
        state
    }

    /// Set `highlighted` member to match `selected`
    pub fn set_highlighted(&mut self) {
        self.highlighted = if !self.filtered.is_empty() {
            // Copies reference (pointer) not the struct itself!
            self.filtered.get(self.selected).copied()
        } else {
            None
        }
    }

    /// Set command to the currently highlighted item
    pub fn set_path_command(&mut self, cmd: PathEditCommand) {
        let highlighted = if let Some(item) = self.highlighted {
            item
        } else {
            return;
        };

        match cmd {
            PathEditCommand::Delete => {
                if self.edits.contains_key(highlighted) {
                    self.edits.remove(highlighted);
                } else {
                    self.edits.insert(highlighted, cmd);
                }
            }
        }
    }

    pub fn path_command(&self, item: &PathItem) -> Option<&PathEditCommand> {
        self.edits.get(item)
    }
}
