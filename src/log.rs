use std::process;
use crate::head;
use crate::commit;

pub fn log(args: &Vec<String>) {
    if args.len() != 3 {
        println!("Usage: {} log <branch>", args[0]);
        process::exit(1);
    }
    let head = head::Head::from_head();
    println!("Head: {}", head.ref_path.display());
    println!("Hash: {}", head.head_hash);
    let mut commit = commit::Commit::from_hash(&head.head_hash);
    println!("Commit: {}", commit);
    while commit.parent.is_some() {
        commit = commit::Commit::from_hash(&commit.parent.unwrap());
        println!("Commit: {}", commit);
    }
}