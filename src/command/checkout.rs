use std::process;

use crate::head;
use crate::object::blob::Blob;
use crate::object::commit;
use crate::object::objectreader::ObjectReader;
use crate::object::tree;
use std::collections::HashSet;

fn read_dir_to_set(dir: &str) -> HashSet<String> {
    let dir = std::fs::read_dir(dir).unwrap();
    let mut dir_files = HashSet::<String>::new();
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if vec![".git", "target"].contains(&path.file_name().unwrap().to_string_lossy().as_ref()) {
            continue;
        }
        if path.is_file() {
            dir_files.insert(entry.file_name().to_string_lossy().to_string());
        } else {
            dir_files.insert(entry.file_name().to_string_lossy().to_string());
        }
        println!("{}", path.display());
    }
    dir_files
}

fn load_dir(path: &str, tree: &tree::Tree) {
    let tree_files = tree.entries.iter().map(|entry| entry.name.clone()).collect::<HashSet<String>>();
    println!("{:?}", tree_files);
    let dir_files = read_dir_to_set(".");
    let delete_files = &dir_files - &tree_files;
    println!("{:?}", delete_files);
    for file in delete_files {
        std::fs::remove_file(file).unwrap();
    }
    for entry in tree.entries.iter() {
        if entry.name.starts_with(".git") {
            continue;
        }
        let file_path = format!("{}/{}", path, entry.name.clone());
        match ObjectReader::find_object_type(&entry.hash) {
            "blob" => {
                std::fs::write(&file_path, Blob::from_hash(&entry.hash).content).unwrap();
            }
            "tree" => {
                std::fs::create_dir_all(&file_path).unwrap();
                let tree = tree::Tree::from_hash(&entry.hash);
                load_dir(&file_path, &tree);
            }
            _ => {
                println!("Unknown object type: {}", entry.hash);
                process::exit(1);
            }
        }
        std::fs::write(file_path, Blob::from_hash(&entry.hash).content).unwrap();
    }
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
    load_dir(".", &tree);
}
