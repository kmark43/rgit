use std::{fs::File, io::{BufReader, Read}};

use flate2::bufread::ZlibDecoder;

use crate::object_finder;

pub struct ObjectReader;

impl ObjectReader {
    pub fn find_object_type(hash: &str) -> &str {
        let path = object_finder::find_object_path(hash);
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut decompressor = ZlibDecoder::new(&mut reader);
        let mut buffer = [0; 1024];
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            let bytes_read = decompressor.read(&mut buffer).unwrap();
            bytes.extend_from_slice(&buffer[..bytes_read]);
            if bytes_read == 0 || bytes.len() > 4 {
                break;
            }
        }
        let header = String::from_utf8_lossy(&bytes[..4]);
        if header == "blob" {
            return "blob";
        } else if header == "tree" {
            return "tree";
        }
        return "commit";
    }
}