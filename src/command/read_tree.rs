use std::process;

use crate::git::object::tree;

pub fn read_tree(args: &Vec<String>) {
    if args.len() != 2 {
        println!("Usage: {} read-tree", args[0]);
        process::exit(1);
    }
    let tree = tree::Tree::from_folder(".");
    println!("{:?}", tree);
}