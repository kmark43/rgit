use flate2::read::ZlibDecoder;
use std::io::{Read};
use std::fs::File;
use std::path::Path;
use std::env;
use std::process;
use std::io::{BufReader};
use bstr::BString;

mod commit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(1);
    }
    let commit = commit::Commit::from_file(Path::new(&args[1]));
    println!("Commit: {}", commit);
}
