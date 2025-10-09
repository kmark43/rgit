use std::collections::HashSet;
use std::fs;

use crate::git::gitignore::GitIgnore;
use crate::git::head::Head;
use crate::git::index::Index;
use crate::git::object::commit::Commit;
use crate::git::object::tree::Tree;

// Grab files
fn get_untracked_files(index: &Index, gitignore: &GitIgnore) -> Vec<String> {
    let mut untracked_files = Vec::new();
    let mut tracked_directories = HashSet::new();
    let mut tracked_files = HashSet::new();
    // for entry in fs::read_dir(".").unwrap() {
    //     let entry = entry.unwrap();
    //     if !gitignore.is_ignored(&entry.path().to_string_lossy()[2..]) {
    //         untracked_files.push(entry.path().to_string_lossy()[2..].to_string());
    //     }
    // }
    tracked_directories.insert(".".to_string());
    for entry in index.entries.iter() {
        if !gitignore.is_ignored(&entry.name) {
            tracked_files.insert(entry.name.clone());
            let mut paths: Vec<&str> = entry.name.split("/").collect();
            // Remove file at end of path
            paths.pop();
            let mut path = String::new();
            for dir in paths {
                path.push_str(&dir);
                tracked_directories.insert(path.clone());
                path.push_str("/");
            }
        }
    }

    for directory in &tracked_directories {
        let files = fs::read_dir(directory).unwrap();
        for file in files {
            let file = file.unwrap();
            if file.path().file_name().unwrap().to_string_lossy().as_ref() == ".git" {
                continue;
            }
            let mut path = file.path().to_string_lossy().to_string();
            if directory == "." {
                path = path[2..].to_string();
            }
            if gitignore.is_ignored(path.as_str()) {
                continue;
            }
            if file.path().is_dir() && !tracked_directories.contains(path.as_str()) {
                untracked_files.push(path);
            } else if file.path().is_file() && !tracked_files.contains(path.as_str()) {
                untracked_files.push(path);
            }
        }
    }
    untracked_files
}

pub fn status(args: &Vec<String>) {
    if args.len() != 2 {
        println!("Usage: {} status", args[0]);
        std::process::exit(1);
    }
    let head = Head::from_head();
    let commit = Commit::from_hash(&head.head_hash);
    let tree = Tree::from_hash(&commit.tree);
    let index = Index::read_index();
    let gitignore = GitIgnore::from_file();
    let untracked_files = get_untracked_files(&index, &gitignore);
    // let tracked_directories = index.entries.iter()
    // let untracked_files = 
    println!("On branch {}", head.ref_path.file_name().unwrap().to_string_lossy());
    println!("untracked files {:?}", untracked_files);
    // println!("Your branch is up to date with 'origin/main'.");
    // println!("Changes to be committed:");
    // println!("  (use \"git restore --staged <file>...\" to unstage)");
    // println!("        new file:   README.md");
    // println!("Untracked files:");
    // println!("  (use \"git add <file>...\" to include in what will be committed)");
    // println!("        README.md");
}