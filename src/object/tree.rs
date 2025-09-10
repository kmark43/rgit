use std::fs::File;
use std::io::{BufReader, Read};
use flate2::read::ZlibDecoder;

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
}