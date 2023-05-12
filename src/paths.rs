use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PathItem {
    pub name: String,
    pub full_path: String,
    pub description: String,
}

impl PartialEq for PathItem {
    fn eq(&self, other: &Self) -> bool {
        self.full_path == other.full_path
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathItems {
    pub paths: Vec<PathItem>,
}

impl PathItems {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    /// Add new path, overriding old entries based oh full_path
    pub fn add_path(&mut self, path: PathItem) {
        let idx = self.paths.iter().position(|p| p == &path);
        if let Some(idx) = idx {
            self.paths.remove(idx);
        }
        self.paths.push(path);
    }

    /// Check if there's a PathItem that has the same path
    pub fn exists(&self, path: &str) -> bool {
        self.paths.iter().any(|p| p.full_path == path)
    }
}

impl Default for PathItems {
    fn default() -> Self {
        PathItems::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{PathItem, PathItems};

    #[test]
    fn test_basic_find() {
        let items = PathItems {
            paths: vec![
                PathItem {
                    name: "Home Name".into(),
                    full_path: "/home/path".into(),
                    description: "The path user's home folder".into(),
                },
                PathItem {
                    name: "Secret Way!".into(),
                    full_path: "/root/path".into(),
                    description: "Secret path for a root user".into(),
                },
            ],
        };

        assert_eq!(items.find_paths("").len(), 2);
        assert_eq!(items.find_paths("path").len(), 2);
        assert_eq!(items.find_paths("name").len(), 1);
        assert_eq!(items.find_paths("root").len(), 1);
        assert_eq!(items.find_paths("foobar").len(), 0);
    }
}
