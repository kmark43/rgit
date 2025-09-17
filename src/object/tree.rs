use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use bstr::ByteVec;
use flate2::read::ZlibDecoder;
use sha1::{Sha1, Digest};

use crate::object::blob::{compute_file_hash, Blob};
use crate::object::objectreader::ObjectReader;
use crate::object_finder;

#[derive(Debug)]
pub struct TreeEntry {
    pub mode: String,
    pub name: String,
    pub hash: String,
}

impl TreeEntry {
    pub fn new(mode: String, name: String, hash: String) -> Self {
        Self { mode, name, hash }
    }
}

pub struct Tree {
    pub hash: String,
    pub entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn new(hash: String, entries: Vec<TreeEntry>) -> Self {
        Self { hash, entries }
    }

    fn read_entry(bytes: &[u8]) -> TreeEntry {
        let mode_len = bytes[..].iter().position(|&x| x == b' ').unwrap();
        let mode = String::from_utf8_lossy(&bytes[..mode_len]).to_string();
        let name_len = bytes[mode_len + 1..].iter().position(|&x| x == b'\0').unwrap();
        let name = String::from_utf8_lossy(&bytes[mode_len + 1..mode_len + 1 + name_len]).to_string();
        let hash = hex::encode(&bytes[mode_len + name.len() + 2..mode_len + name.len() + 2 + 20]);
        TreeEntry::new(mode, name, hash)
    }

    pub fn from_hash(hash: &str) -> Self {
        let path = object_finder::find_object_path(hash);
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut decompressor = ZlibDecoder::new(&mut reader);
        let mut buffer = [0; 1024];
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            let bytes_read = decompressor.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            bytes.extend_from_slice(&buffer[..bytes_read]);
        }
        let mut entries = Vec::new();
        let mut i = bytes.iter().position(|&x| x == b'\0').unwrap() + 1;
        while i < bytes.len() {
            entries.push(Tree::read_entry(&bytes[i..]));
            i += entries.last().unwrap().name.len() + 8 + 20;
        }
        Self::new(hash.to_string(), entries)
    }

    fn write_dir_entry(entry: &TreeEntry) -> Vec<u8> {
        let mut entry_bytes = Vec::new();
        entry_bytes.extend_from_slice(entry.mode.as_bytes());
        entry_bytes.push(b' ');
        entry_bytes.extend_from_slice(entry.name.as_bytes());
        entry_bytes.push(b'\0');
        entry_bytes.extend_from_slice(&hex::decode(&entry.hash).unwrap());
        entry_bytes
    }

    fn read_dir_entry(entry: &fs::DirEntry) -> TreeEntry {
        let path = entry.path();
        if path.is_file() {
            let mode = "100644";
            let name = entry.file_name().to_string_lossy().to_string();
            let hash = compute_file_hash(&path.to_string_lossy());
            TreeEntry::new(mode.to_string(), name, hash)
        } else {
            let mode = "40000".to_string();
            let name = entry.file_name().to_string_lossy().to_string();
            let hash = Tree::hash_folder(&entry.path().to_string_lossy());
            TreeEntry::new(mode, name, hash)
        }
    }
    
    pub fn hash_folder(folder: &str) -> String {
        let mut hash = Sha1::new();
        let mut tree_bytes = Vec::new();
        let mut entries = Vec::new();
        for entry in fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            if vec![".git", "target"].contains(&entry.file_name().to_string_lossy().as_ref()) {
                continue;
            }
            let entry = Tree::read_dir_entry(&entry);
            entries.push(entry);
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        for entry in entries {
            tree_bytes.extend_from_slice(&Tree::write_dir_entry(&entry));
        }
        let header = format!("tree {}\0", tree_bytes.len());
        tree_bytes.insert_str(0, header);
        hash.update(tree_bytes);
        let hash = hash.finalize();
        hex::encode(&hash)
    }



    fn read_dir_to_set(dir: &str) -> HashSet<String> {
        let dir = std::fs::read_dir(dir).unwrap();
        let mut dir_files = HashSet::<String>::new();
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            if vec![".git", "target"].contains(&path.file_name().unwrap().to_string_lossy().as_ref()) {
                continue;
            }
            if path.is_file() {
                dir_files.insert(entry.file_name().to_string_lossy().to_string());
            } else {
                dir_files.insert(entry.file_name().to_string_lossy().to_string());
            }
        }
        dir_files
    }

    pub fn sync_tree_to_dir(&self, path: &str) {
        let tree_files = self.entries.iter().map(|entry| entry.name.clone()).collect::<HashSet<String>>();
        let dir_files = Tree::read_dir_to_set(&path);
        let delete_files = &dir_files - &tree_files;
        for file in delete_files {
            let file_path = format!("{}/{}", path, file);
            if std::fs::metadata(&file_path).unwrap().is_dir() {
                std::fs::remove_dir_all(file_path).unwrap();
            } else {
                std::fs::remove_file(file_path).unwrap();
            }
        }
        for entry in self.entries.iter() {
            if entry.name.starts_with(".git") {
                continue;
            }
            let file_path = format!("{}/{}", path, entry.name.clone());
            match ObjectReader::find_object_type(&entry.hash) {
                "blob" => {
                    if !Path::new(&file_path).exists() || compute_file_hash(&file_path) != entry.hash {
                        std::fs::write(&file_path, Blob::from_hash(&entry.hash).content).unwrap();
                    }
                }
                "tree" => {
                    if !Path::new(&file_path).exists() || Tree::hash_folder(&file_path) != entry.hash {
                        std::fs::create_dir_all(&file_path).unwrap();
                        let tree = Tree::from_hash(&entry.hash);
                        tree.sync_tree_to_dir(&file_path);
                    }
                }
                _ => {
                    println!("Unknown object type: {}", entry.hash);
                    std::process::exit(1);
                }
            }
        }
    }
}