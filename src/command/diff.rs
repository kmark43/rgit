use std::{collections::{HashSet, VecDeque}, fs, time::SystemTime};
use colored::*;
use crate::git::{gitignore::GitIgnore, head::Head, index::{self, Index}, object::{blob::{compute_file_hash, Blob}, commit::Commit}};

pub fn diff_blobs(blob1: &Blob, blob2: &Blob) -> String {
    let mut diff = String::new();
    let old_lines: Vec<&str> = str::from_utf8(&blob1.content).unwrap().split('\n').collect();
    let new_lines: Vec<&str> = str::from_utf8(&blob2.content).unwrap().split('\n').collect();
    let mut old_index = 0;
    let mut new_index = 0;
    while old_index < old_lines.len() && new_index < new_lines.len() {
        if old_lines[old_index] == new_lines[new_index] {
            diff.push_str("  ");
            diff.push_str(new_lines[new_index]);
            diff.push_str("\n");
            old_index += 1;
            new_index += 1;
        } else {
            let mut x;
            let mut y;
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            visited.insert((0, 0));
            queue.push_back((0, 0));
            (x, y) = queue.pop_front().unwrap();
            while !(x + old_index == old_lines.len() && y + new_index == new_lines.len()) && old_lines[old_index + x] != new_lines[new_index + y] {
                if !visited.contains(&(x + 1, y)) && x + old_index + 1 < old_lines.len() {
                    queue.push_back((x + 1, y));
                    visited.insert((x + 1, y));
                }
                if !visited.contains(&(x, y + 1)) && y + new_index + 1 < new_lines.len() {
                    queue.push_back((x, y + 1));
                    visited.insert((x, y + 1));
                }
                (x, y) = queue.pop_front().unwrap();
            }
            for i in 0..x {
                diff.push_str("- ");
                diff.push_str(old_lines[old_index + i]);
                diff.push_str("\n");
            }
            for i in 0..y {
                diff.push_str("+ ");
                diff.push_str(new_lines[new_index + i]);
                diff.push_str("\n");
            }
            old_index += x;
            new_index += y;
        }
    }

    for i in old_index..old_lines.len() {
        diff.push_str("- ");
        diff.push_str(old_lines[i]);
        diff.push_str("\n");
    }
    for i in new_index..new_lines.len() {
        diff.push_str("+ ");
        diff.push_str(new_lines[i]);
        diff.push_str("\n");
    }
    return diff;
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
    pub old_hash: Option<String>,
    pub new_hash: Option<String>,
}

impl ChangedFile {
    pub fn new(path: String, status: FileStatus, old_hash: Option<String>, new_hash: Option<String>) -> Self {
        Self { path, status, old_hash, new_hash }
    }
}

fn get_unstaged_files(commit: &Commit, index: &Index, gitignore: &GitIgnore) -> Vec<ChangedFile> {
    let mut unstaged_files = Vec::new();
    let commit_timestamp = commit.timestamp.parse::<u64>().unwrap();
    for entry in index.entries.iter() {
        if !fs::exists(&entry.name).unwrap() {
            unstaged_files.push(ChangedFile::new(entry.name.clone(), FileStatus::Deleted, Some(entry.sha1.clone()), None));
            continue;
        }
        let modified_time = fs::metadata(&entry.name).unwrap().modified().unwrap()
                    .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        if !gitignore.is_ignored(&entry.name) && modified_time > commit_timestamp {
            let hash = compute_file_hash(&entry.name);
            if entry.sha1 != hash {
                unstaged_files.push(ChangedFile::new(entry.name.clone(), FileStatus::Modified, Some(entry.sha1.clone()), Some(hash)));
            }
        }
    }
    unstaged_files
}

pub fn diff(args: &Vec<String>) {
    if args.len() != 2 {
        println!("Usage: {} diff", args[0]);
        std::process::exit(1);
    }
    let index = index::Index::read_index();
    let head = Head::from_head();
    let commit = Commit::from_hash(&head.head_hash);
    // let tree = tree::Tree::from_hash(&commit.tree);
    let gitignore = GitIgnore::from_file();
    let unstaged_files = get_unstaged_files(&commit, &index, &gitignore);
    for file in unstaged_files.iter() {
        println!("{}", file.path);
        println!("{}", diff_blobs(&Blob::from_hash(&file.old_hash.as_ref().unwrap()), &Blob::from_file(&file.path)));
    }
}