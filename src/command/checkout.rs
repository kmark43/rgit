use std::process;
use crate::head;
use crate::object::commit;
use crate::object::tree;
use std::collections::HashSet;

fn read_dir_to_set() -> HashSet<String> {
    let dir = std::fs::read_dir(".").unwrap();
    let mut dir_files = HashSet::<String>::new();
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if vec![".git", "target"].contains(&path.file_name().unwrap().to_string_lossy().as_ref()) {
            continue;
        }
        dir_files.insert(entry.file_name().to_string_lossy().to_string());
        println!("{}", path.display());
    }
    dir_files
}

fn load_dir(tree: &tree::Tree) {
    let treeFiles = tree.entries.iter().map(|entry| entry.name.clone()).collect::<HashSet<String>>();
    let dirFiles = read_dir_to_set();
    let newFiles = &treeFiles - &dirFiles;
    let deleteFiles = &dirFiles - &treeFiles;
    println!("{:?}", newFiles);
    println!("{:?}", deleteFiles);
}

pub fn checkout(args: &Vec<String>) {
    if args.len() != 3 {
        println!("Usage: {} checkout <branch>", args[0]);
        process::exit(1);
    }
    let branch = &args[2];
    let head = head::Head::from_branch(&branch);
    println!("head, {}", head.head_hash);
    let commit = commit::Commit::from_hash(&head.head_hash);
    println!("commit, {}", commit.hash);
    println!("tree, {}", commit.tree);
    let tree = tree::Tree::from_hash(&commit.tree);
    load_dir(&tree);
}
