use serde::{Deserialize, Serialize};
use std::hash::Hash;

/* Implementing Custom Deserializer is painful so let's do this this the hacky way */
#[derive(Debug, Serialize, Deserialize)]
struct _PathItem {
    pub name: String,
    pub full_path: String,
    pub description: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct _PathItems {
    pub paths: Vec<_PathItem>,
}
/* End of private helper structs */

// TODO: remove Clone!
#[derive(Debug, Clone)]
pub struct PathItem {
    pub name: String,
    /// Lowercase name
    lname: String,
    pub full_path: String,
    /// Lowercase full_path
    lfull_path: String,
    pub description: String,
}

impl PathItem {
    pub fn new(name: String, full_path: String, description: String) -> Self {
        let lname = name.to_lowercase();
        let lfull_path = full_path.to_lowercase();

        Self {
            name,
            lname,
            full_path,
            lfull_path,
            description,
        }
    }
}

impl PartialEq for PathItem {
    fn eq(&self, other: &Self) -> bool {
        self.full_path == other.full_path
    }
}

impl Eq for PathItem {}

/// Hasing for PathItem assumes that are paths in the whole program are unique
impl Hash for PathItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.full_path.as_bytes())
    }
}

#[derive(Debug, Default)]
pub struct PathItems {
    pub paths: Vec<PathItem>,
}

impl PathItems {
    /// Turn structure into formatted json string
    pub fn into_json(self) -> String {
        let paths: Vec<_PathItem> = self
            .paths
            .into_iter()
            .map(|p| _PathItem {
                name: p.name,
                full_path: p.full_path,
                description: p.description,
            })
            .collect();
        serde_json::to_string_pretty(&_PathItems { paths }).unwrap()
    }
    /// Create PathItems from json string
    pub fn from_json(json: &str) -> Self {
        let items: _PathItems = serde_json::from_str(json).unwrap_or_default();
        let paths: Vec<PathItem> = items
            .paths
            .into_iter()
            .map(|p| PathItem {
                lname: p.name.to_lowercase(),
                lfull_path: p.full_path.to_lowercase(),
                name: p.name,
                full_path: p.full_path,
                description: p.description,
            })
            .collect();
        Self { paths }
    }

    pub fn sort(&mut self) {
        self.paths.sort_by(|a, b| a.full_path.cmp(&b.full_path))
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

    /// Find Path items that match the search
    /// search will be OK if all words in [search] are part of PathItem::name or PathItem::full_path
    pub fn filter<'a>(&'a self, search: &str) -> Vec<&'a PathItem> {
        let words: Vec<&str> = search.split_whitespace().collect();
        // Search is empty or only contains whitespace
        if words.is_empty() {
            return self.paths.iter().collect();
        }

        self.paths
            .iter()
            .filter(move |&path| {
                for word in &words {
                    if !path.lfull_path.contains(word) && !path.lname.contains(word) {
                        return false;
                    }
                }
                true
            })
            .collect()
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
                    lname: "Home Name".to_lowercase().into(),
                    full_path: "/home/path".into(),
                    lfull_path: "/home/path".into(),
                    description: "The path user's home folder".into(),
                },
                PathItem {
                    name: "Secret Way!".into(),
                    lname: "Secret Way!".to_lowercase().into(),
                    full_path: "/root/path".into(),
                    lfull_path: "/root/path".into(),
                    description: "Secret path for a root user".into(),
                },
            ],
        };

        assert_eq!(items.filter("").len(), 2);
        assert_eq!(items.filter("   ").len(), 2);
        assert_eq!(items.filter("\t  ").len(), 2);
        assert_eq!(items.filter("path").len(), 2);
        assert_eq!(items.filter("name").len(), 1);
        assert_eq!(items.filter("root").len(), 1);
        assert_eq!(items.filter("foobar").len(), 0);
    }

    #[test]
    fn test_words_find() {
        let items = PathItems {
            paths: vec![
                PathItem {
                    name: "Home Name Word".into(),
                    lname: "Home Name Word".to_lowercase().into(),
                    full_path: "/home/path/user".into(),
                    lfull_path: "/home/path/user".into(),
                    description: "The path user's home folder".into(),
                },
                PathItem {
                    name: "Secret Way Word!".into(),
                    lname: "Secret Way Word!".to_lowercase().into(),
                    full_path: "/root/path/".into(),
                    lfull_path: "/root/path/".into(),
                    description: "Secret path for a root user".into(),
                },
            ],
        };

        assert_eq!(items.filter("home word").len(), 1);
        assert_eq!(items.filter("home user").len(), 1);
        assert_eq!(items.filter("secret word").len(), 1);
        assert_eq!(items.filter("secret       word").len(), 1);
        assert_eq!(items.filter("secret home word").len(), 0);
        assert_eq!(items.filter("root user").len(), 0);
    }
}
