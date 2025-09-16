use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::os::unix::fs::PermissionsExt;
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
        let mut entries = Vec::new();
        let mut i = bytes.iter().position(|&x| x == b'\0').unwrap() + 1;
        while i < bytes.len() {
            entries.push(Tree::read_entry(&bytes[i..]));
            i += entries.last().unwrap().name.len() + 8 + 20;
        }
        Self::new(hash.to_string(), entries)
    }

    fn write_dir_entry(entry: &TreeEntry) -> String {
        format!("{} {}\0{}", entry.mode, entry.name, String::from_utf8_lossy(&hex::decode(&entry.hash).unwrap()))
    }

    fn read_dir_entry(path: &str, entry: &fs::DirEntry) -> TreeEntry {
        let path = entry.path();
        if path.is_file() {
            let permissions = fs::metadata(&path).unwrap().permissions().mode();
            let mode = format!("{}", permissions);
            let name = path.to_string_lossy().to_string();
            let hash = compute_file_hash(&path.to_string_lossy());
            TreeEntry::new(mode, name, hash)
        } else {
            let permissions = fs::metadata(&path).unwrap().permissions().mode();
            let mode = format!("{}", permissions);
            println!("{}", format!("{}/{}", path.to_string_lossy(), entry.file_name().to_string_lossy()));
            let name = format!("{}/{}", path.to_string_lossy(), entry.file_name().to_string_lossy());
            let hash = Tree::hash_folder(&path.to_string_lossy());
            TreeEntry::new(mode, name, hash)
        }
    }
    
    pub fn hash_folder(folder: &str) -> String {
        let mut hash = Sha1::new();
        let mut tree_string = String::new();
        let mut entries = Vec::new();
        for entry in fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            let entry = Tree::read_dir_entry(&folder, &entry);
            entries.push(entry);
        }
        for entry in entries {
            tree_string.push_str(&Tree::write_dir_entry(&entry));
        }
        tree_string.insert_str(0, &format!("tree {}\0", tree_string.len()));
        println!("{}", tree_string);
        hash.update(tree_string);
        hex::encode(hash.finalize())
    }
}