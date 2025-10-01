use std::{path::Path};

use crate::git::{index::Index, object::blob::Blob};

#[derive(Debug)]
pub struct GitIgnore {
    pub ignore_list: Vec<glob::Pattern>,
}

impl GitIgnore {
    pub fn from_file() -> Self {
        let index = Index::read_index();
        let entry = index.get_entry(".gitignore");
        if entry.is_none() {
            return Self { ignore_list: Vec::new() };
        }
        let entry = entry.unwrap();
        let file = Blob::from_hash(&entry.sha1);
        let ignore_list = String::from_utf8_lossy(&file.content);
        Self { ignore_list: ignore_list.split("\n").map(|s| glob::Pattern::new(s).unwrap()).collect() }
    }

    pub fn is_ignored(&self, path: &str) -> bool {
        self.ignore_list.iter().any(|ignore| ignore.matches_path(Path::new(path)))
    }
}