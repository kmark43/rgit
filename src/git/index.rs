use std::{fs::File, io::{BufReader, Read}};

#[derive(Debug)]
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
    pub flags: u16,
    pub name: String,
}

impl IndexEntry {
    pub fn new(ctime: u32, ctime_nsec: u32, mtime: u32, mtime_nsec: u32, device: u32, inode: u32, mode: u32, uid: u32, gid: u32, size: u32, sha1: String, flags: u16, name: String) -> Self {
        Self { ctime, ctime_nsec, mtime, mtime_nsec, device, inode, mode, uid, gid, size, sha1, flags, name }
    }
}

#[derive(Debug)]
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
        let mut array: Vec<u8> = Vec::new();
        let mut bytes = reader.read_to_end(&mut array).unwrap();
        let dirc: &[u8; 4] = &array[0..4].try_into().unwrap();
        if dirc != b"DIRC" {
            println!("Invalid index file");
            std::process::exit(1);
        }
        let version = Some(u32::from_be_bytes(array[4..8].try_into().unwrap()));
        if version.unwrap() != 2 {
            println!("Unsupported index version: {}", version.unwrap());
            std::process::exit(1);
        }
        let num_entries = Some(u32::from_be_bytes(array[8..12].try_into().unwrap()));
        let mut entries = Vec::new();
        let mut index = 12;
        for _ in 0..num_entries.unwrap() {
            let ctime = u32::from_be_bytes(array[index..index + 4].try_into().unwrap());
            let ctime_nsec = u32::from_be_bytes(array[index + 4..index + 8].try_into().unwrap());
            let mtime = u32::from_be_bytes(array[index + 8..index + 12].try_into().unwrap());
            let mtime_nsec = u32::from_be_bytes(array[index + 12..index + 16].try_into().unwrap());
            let device = u32::from_be_bytes(array[index + 16..index + 20].try_into().unwrap());
            let inode = u32::from_be_bytes(array[index + 20..index + 24].try_into().unwrap());
            let mode = u32::from_be_bytes(array[index + 24..index + 28].try_into().unwrap());
            let uid = u32::from_be_bytes(array[index + 28..index + 32].try_into().unwrap());
            let gid = u32::from_be_bytes(array[index + 32..index + 36].try_into().unwrap());
            let file_size = u32::from_be_bytes(array[index + 36..index + 40].try_into().unwrap());
            let sha1 = hex::encode(&array[index + 40..index + 60]).to_string();
            let flags = u16::from_be_bytes(array[index + 60..index + 62].try_into().unwrap());
            let name_len = flags & 0xfff;
            let name = String::from_utf8_lossy(&array[index + 62..index + 62 + name_len as usize]).to_string();
            let mut offset = 62 + name_len as usize + 1;
            let offset_rem = (8 - (offset % 8)) % 8;
            offset = offset + offset_rem;
            index = index + offset;
            entries.push(IndexEntry::new(ctime, ctime_nsec, mtime, mtime_nsec, device, inode, mode, uid, gid, file_size, sha1, flags, name));
        }
        Self::new(version.unwrap(), entries)
    }
}