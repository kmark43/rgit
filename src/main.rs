use std::env;
use std::process;

use crate::git::object::tree::Tree;

mod git;
mod object_finder;
mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        process::exit(1);
    }
    // if args[1] == "log" {
    //     command::log::log(&args);
    // }
    if args[1] == "checkout" {
        command::checkout::checkout(&args);
    } else if args[1] == "ls-tree" {
        Tree::from_hash(&args[2]);
    } else if args[1] == "ls-tree-folder" {
        Tree::hash_folder(&args[2]);
    }
}
