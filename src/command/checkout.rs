use std::process;

use crate::head;
use crate::object::commit;
use crate::object::tree;

pub fn checkout(args: &Vec<String>) {
    if args.len() != 3 {
        println!("Usage: {} checkout <branch>", args[0]);
        process::exit(1);
    }
    let branch = &args[2];
    let head = head::Head::from_branch(&branch);
    let commit = commit::Commit::from_hash(&head.head_hash);
    let tree = tree::Tree::from_hash(&commit.tree);
    tree.sync_tree_to_dir(".");
    head::Head::update_head_to_branch(&branch);
}
