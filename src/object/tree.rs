use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::os::unix::fs::PermissionsExt;
use bstr::ByteVec;
use flate2::read::ZlibDecoder;
use sha1::{Sha1, Digest};

use crate::object::blob::compute_file_hash;
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
        println!("{}", String::from_utf8_lossy(&bytes));
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
            let hash = Tree::hash_folder(&entry.path().to_string_lossy(), false);
            TreeEntry::new(mode, name, hash)
        }
    }
    
    pub fn hash_folder(folder: &str, show_tree: bool) -> String {
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
        if show_tree {
            println!("{}", String::from_utf8_lossy(&tree_bytes));
        }
        hash.update(tree_bytes);
        let hash = hash.finalize();
        if show_tree {
            println!("hash: {}", hex::encode(&hash));
        }
        hex::encode(&hash)
    }
}