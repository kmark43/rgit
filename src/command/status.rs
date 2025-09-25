use crate::git::head::Head;
use crate::git::object::commit::Commit;
use crate::git::object::tree::Tree;

pub fn status(args: &Vec<String>) {
    if args.len() != 2 {
        println!("Usage: {} status", args[0]);
        std::process::exit(1);
    }
    let head = Head::from_head();
    let commit = Commit::from_hash(&head.head_hash);
    let tree = Tree::from_hash(&commit.tree);
    
}