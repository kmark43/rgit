use std::fs::File;
use std::io::{Read};
use std::io::{BufReader};
use flate2::read::ZlibDecoder;
use std::fmt;

use crate::object_finder;

pub struct Commit {
    pub hash: String,
    pub tree: String,
    pub parent: Option<String>,
    pub author: String,
    pub committer: String,
    pub message: String,
    pub timestamp: String,
    pub timezone: String,
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "commit {}", self.hash)
    }
}

impl Commit {
    pub fn format_log(&self) -> String {
        format!("commit {}\nAuthor: {}\nDate:   {}\n\n    {}", 
                self.hash, self.author, self.timestamp, self.message)
    }
    pub fn new(hash: String, tree: String, parent: Option<String>, author: String, committer: String, message: String, timestamp: String, timezone: String) -> Self {
        Self { hash,tree, parent, author, committer, message, timestamp, timezone }
    }

    pub fn from_bytes(hash: &str, bytes: &[u8]) -> Self {
        let content = String::from_utf8_lossy(bytes);
        let lines: Vec<&str> = content.lines().collect();
        
        let mut tree = String::new();
        let mut parent: Option<String> = None;
        let mut author = String::new();
        let mut committer = String::new();
        let mut message = String::new();
        let mut timestamp = String::new();
        let mut timezone = String::new();
        
        let mut in_message = false;
        
        for line in lines {
            if in_message {
                message.push_str(line);
                message.push('\n');
            } else if line.starts_with("commit ") {
                // Skip the commit header line
                continue;
            } else if line.starts_with("tree ") {
                tree = line[5..].to_string();
            } else if line.starts_with("parent ") {
                parent = Some(line[7..].to_string());
            } else if line.starts_with("author ") {
                let author_line = &line[7..];
                if let Some(space_pos) = author_line.rfind(' ') {
                    if let Some(space_pos2) = author_line[..space_pos].rfind(' ') {
                        author = author_line[..space_pos2].to_string();
                        timestamp = author_line[space_pos2 + 1..space_pos].to_string();
                        timezone = author_line[space_pos + 1..].to_string();
                    }
                }
            } else if line.starts_with("committer ") {
                let committer_line = &line[10..];
                if let Some(space_pos) = committer_line.rfind(' ') {
                    if let Some(space_pos2) = committer_line[..space_pos].rfind(' ') {
                        committer = committer_line[..space_pos2].to_string();
                        // timestamp and timezone already set from author
                    }
                }
            } else if line.is_empty() {
                in_message = true;
            }
        }
        
        // Remove trailing newline from message
        if message.ends_with('\n') {
            message.pop();
        }
        
        Self { hash: hash.to_string(), tree, parent, author, committer, message, timestamp, timezone }
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
        Self::from_bytes(hash, &bytes)
    }
}
