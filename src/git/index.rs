use std::{fs::File, io::{BufReader, Read}};

pub struct IndexEntry {
    pub ctime: u32,
    pub ctime_nsec: u32,
    pub mtime: u32,
    pub mtime_nsec: u32,
    pub device: u32,
    pub inode: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u32,
    pub sha1: String,
}

impl IndexEntry {
    // pub fn new(ctime: u32, ctime_nsec: u32, mtime: u32, mtime_nsec: u32, working_hash: String, staging_hash: String, repo_hash: String) -> Self {
    //     Self { ctime, ctime_nsec, mtime, mtime_nsec, working_hash, staging_hash, repo_hash }
    // }
}

pub struct Index {
    pub version: u32,
    pub num_entries: u32,
    pub entries: Vec<IndexEntry>,
}

impl Index {
    pub fn new(version: u32, entries: Vec<IndexEntry>) -> Self {
        Self { version, num_entries: entries.len() as u32, entries }
    }

    pub fn read_index() -> Self {
        let file = File::open(".git/index").unwrap();
        let mut reader = BufReader::new(file);
        let mut entries: Vec<IndexEntry> = Vec::new();
        let mut array = Vec::new();
        let bytes = reader.read_to_end(&mut array).unwrap();
        // let mut array = [0u8; 4];
        let dirc: &[u8; 4] = &array[0..4].try_into().unwrap();
        if dirc != b"DIRC" {
            println!("Invalid index file");
            std::process::exit(1);
        }
        let version = Some(u32::from_be_bytes(array[4..8].try_into().unwrap()));
        let num_entries = Some(u32::from_be_bytes(array[8..12].try_into().unwrap()));
        let entries = Vec::new();
        println!("num_entries: {}", num_entries.unwrap());
        println!("version: {}", version.unwrap());
        for i in 0..num_entries.unwrap() {
            // let entry = IndexEntry::new(array[12 + i * 12..12 + i * 12 + 12].try_into().unwrap());
            // entries.push(entry);
        }
        Self::new(version.unwrap(), entries)
        // while let Ok(line) = reader.read_line() {
        //     let entry = IndexEntry::new(line);
        //     entries.push(entry); 
        // }
        // Self::new(entries)
    }
}