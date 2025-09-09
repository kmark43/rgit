use crate::object_finder;
use std::fs::File;
use std::io::{BufReader, Read};
use flate2::read::ZlibDecoder;
use sha1::{Sha1, Digest};
use hex;

pub struct Blob {
    pub hash: String,
    pub content: Vec<u8>,
}

impl Blob {
    pub fn new(hash: String, content: Vec<u8>) -> Self {
        Self { hash, content }
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

        let null_pos = bytes.iter().position(|&x| x == b'\0');
        bytes = bytes[null_pos.unwrap() + 1..].to_vec();
        Self::new(hash.to_string(), bytes)
    }
}