use std::process;
use crate::head;
use crate::commit;

pub fn log(args: &Vec<String>) {
    if args.len() != 2 {
        println!("Usage: {} log", args[0]);
        process::exit(1);
    }
    let head = head::Head::from_head();
    let mut commit = commit::Commit::from_hash(&head.head_hash);
    println!("{}", commit.format_log());
    while commit.parent.is_some() {
        commit = commit::Commit::from_hash(&commit.parent.unwrap());
        println!("{}", commit.format_log());
    }
}