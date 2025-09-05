use flate2::read::ZlibDecoder;
use std::io::{Read};
use std::fs::File;
use std::path::Path;
use std::env;
use std::process;
use std::io::{BufReader};
use bstr::BString;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(1);
    }
    let path = Path::new(&args[1]);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut decompressor = ZlibDecoder::new(&mut reader);
    // let mut sha1 = Sha1::new();
    let mut buffer = [0; 1024];
    let mut bytes: Vec<u8> = Vec::new();
    loop {
        let bytes_read = decompressor.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        // sha1.update(&buffer[..bytes_read]);
        bytes.extend_from_slice(&buffer[..bytes_read]);
    }
    // println!("SHA1: {}", sha1.digest());
    println!("Bytes: {}", bytes.len());
    // println!("Bytes: {:?}", bytes);
    println!("Text: {}", &String::from_utf8(bytes).unwrap());
}
