use std::collections::HashSet;
use std::fs;
use std::time::SystemTime;
use colored::*;

use crate::git::gitignore::GitIgnore;
use crate::git::head::Head;
use crate::git::index::Index;
use crate::git::object::blob::compute_file_hash;
use crate::git::object::commit::Commit;
use crate::git::object::objectreader::ObjectReader;
use crate::git::object::tree::Tree;

fn get_untracked_files(index: &Index, gitignore: &GitIgnore) -> Vec<String> {
    let mut untracked_files = Vec::new();
    let mut tracked_directories = HashSet::new();
    let mut tracked_files = HashSet::new();
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
                untracked_files.push(path + "/");
            } else if file.path().is_file() && !tracked_files.contains(path.as_str()) {
                untracked_files.push(path);
            }
        }
    }
    untracked_files
}


#[derive(Debug)]
enum FileStatus {
    Created,
    Modified,
    Deleted,
}

#[derive(Debug)]
struct ChangedFile {
    pub path: String,
    pub status: FileStatus,
}

impl ChangedFile {
    pub fn new(path: String, status: FileStatus) -> Self {
        Self { path, status }
    }
}

fn get_unstaged_files(commit: &Commit, index: &Index, gitignore: &GitIgnore) -> Vec<ChangedFile> {
    let mut unstaged_files = Vec::new();
    let commit_timestamp = commit.timestamp.parse::<u64>().unwrap();
    for entry in index.entries.iter() {
        if !fs::exists(&entry.name).unwrap() {
            unstaged_files.push(ChangedFile::new(entry.name.clone(), FileStatus::Deleted));
            continue;
        }
        let modified_time = fs::metadata(&entry.name).unwrap().modified().unwrap()
                    .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        if !gitignore.is_ignored(&entry.name) && modified_time > commit_timestamp {
            let hash = compute_file_hash(&entry.name);
            if entry.sha1 != hash {
                unstaged_files.push(ChangedFile::new(entry.name.clone(), FileStatus::Modified));
            }
        }
    }
    unstaged_files
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct TreeFile {
    pub path: String,
    pub hash: String,
}

impl TreeFile {
    pub fn new(path: String, hash: String) -> Self {
        Self { path, hash }
    }
}

fn create_tree_files(path: &str, tree: &Tree, tree_files: &mut Vec<TreeFile>, gitignore: &GitIgnore) {
    for entry in tree.entries.iter() {
        let file_path = format!("{}/{}", path, entry.name);
        if gitignore.is_ignored(&file_path[2..]) {
            continue;
        }
        if ObjectReader::find_object_type(&entry.hash) == "tree" {
            create_tree_files(&file_path, &Tree::from_hash(&entry.hash), tree_files, gitignore);
        } else {
            tree_files.push(TreeFile::new(file_path[2..].to_string(), entry.hash.clone()));
        }
    }
}

fn create_index_files(index: &Index, gitignore: &GitIgnore) -> Vec<TreeFile> {
    let mut index_files = Vec::new();
    for entry in index.entries.iter() {
        if !gitignore.is_ignored(&entry.name) {
            index_files.push(TreeFile::new(entry.name.clone(), entry.sha1.clone()));
        }
    }
    index_files
}

fn get_staged_files(index: &Index, tree: &Tree, gitignore: &GitIgnore) -> Vec<ChangedFile> {
    let mut staged_files = Vec::new();
    let mut tree_files = Vec::new();
    create_tree_files(".", &tree, &mut tree_files, gitignore);
    let index_files = create_index_files(index, gitignore);
    let tree_paths = tree_files.iter().map(|file| file.path.clone()).collect::<HashSet<String>>();
    let index_paths = index_files.iter().map(|file| file.path.clone()).collect::<HashSet<String>>();

    let added_files = &index_paths - &tree_paths;
    let removed_files = &tree_paths - &index_paths;
    let mut modified_files = HashSet::new();
    for file in index_files.iter() {
        let tree_file = tree_files.iter().find(|f| f.path == file.path);
        if !tree_file.is_none() && file.hash != tree_file.unwrap().hash {
            modified_files.insert(file.clone());
        }
    }
    staged_files.extend(added_files.iter().map(|file| ChangedFile::new(file.clone(), FileStatus::Created)));
    staged_files.extend(removed_files.iter().map(|file| ChangedFile::new(file.clone(), FileStatus::Deleted)));
    staged_files.extend(modified_files.iter().map(|file| ChangedFile::new(file.path.clone(), FileStatus::Modified)));
    staged_files.sort_by(|a, b| a.path.cmp(&b.path));
    staged_files
}

fn print_files(files: &Vec<ChangedFile>) {
    for file in files.iter() {
        let status = match file.status {
            FileStatus::Created =>  "new file:   ",
            FileStatus::Modified => "modified:   ",
            FileStatus::Deleted =>  "deleted:    ",
        };
        let color = match file.status {
            FileStatus::Created => "green",
            FileStatus::Modified => "green",
            FileStatus::Deleted => "red",
        };
        println!("        {} {}", status.color(color), file.path.color(color));
    }
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
    let staged_files = get_staged_files(&index, &tree, &gitignore);
    let unstaged_files = get_unstaged_files(&commit, &index, &gitignore);
    let untracked_files = get_untracked_files(&index, &gitignore);
    println!("On branch {}", head.ref_path.file_name().unwrap().to_string_lossy());
    if !staged_files.is_empty() {
        println!();
        println!("Changes to be committed:");
        println!("  (use \"git restore --staged <file>...\" to unstage)");
        print_files(&staged_files);
    }

    if !unstaged_files.is_empty() {
        println!();
        println!("Changes not staged for commit:");
        println!("  (use \"git add <file>...\" to update what will be committed)");
        println!("  (use \"git restore <file>...\" to discard changes in working directory)");
        print_files(&unstaged_files);
    }

    if !untracked_files.is_empty() {
        println!();
        println!("Untracked files:");
        println!("  (use \"git add <file>...\" to include in what will be committed)");
        for file in untracked_files.iter() {
            println!("        {}", file.color("red"));
        }
    }

    println!();
}