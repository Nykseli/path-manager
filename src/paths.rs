use serde::{Deserialize, Serialize};

// TODO: remove Clone!
#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn filter<'a>(&'a self, search: &str) -> Vec<&'a PathItem> {
        self.paths
            .iter()
            .filter(move |&path| {
                path.full_path.contains(search) || path.name.to_lowercase().contains(search)
            })
            .collect()
    }

    /// Find Path items that match the search
    /// search will be OK if [search] is part of PathItem::name or PathItem::full_path
    pub fn filter_paths<'a>(&'a self, search: &'a str) -> impl Iterator<Item = &'a PathItem> {
        // TODO: filter already filtered paths
        // TODO: fuzzy search
        self.paths.iter().filter(move |&path| {
            path.full_path.contains(search) || path.name.to_lowercase().contains(search)
        })
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

        assert_eq!(items.filter_paths("").count(), 2);
        assert_eq!(items.filter_paths("path").count(), 2);
        assert_eq!(items.filter_paths("name").count(), 1);
        assert_eq!(items.filter_paths("root").count(), 1);
        assert_eq!(items.filter_paths("foobar").count(), 0);
    }
}
