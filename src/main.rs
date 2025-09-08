use flate2::read::ZlibDecoder;
use std::io::{Read};
use std::fs::File;
use std::path::Path;
use std::env;
use std::process;
use std::io::{BufReader};
use bstr::BString;

mod commit;
mod object_finder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <hash>", args[0]);
        process::exit(1);
    }
    let commit = commit::Commit::from_hash(&args[1]);
    println!("Commit: {}", commit);
}
