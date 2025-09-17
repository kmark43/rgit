use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub struct Head {
    pub ref_path: PathBuf,
    pub head_hash: String,
}

impl Head {
    fn new(ref_path: PathBuf, head_hash: String) -> Self {
        Self { ref_path, head_hash }
    }

    pub fn from_file(path: &PathBuf) -> Self {
        let file = File::open(&path).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = [0; 1024];
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            let bytes_read = reader.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            bytes.extend_from_slice(&buffer[..bytes_read]);
        }
        Self::new(PathBuf::from(path), String::from_utf8_lossy(&bytes).trim().to_string())
    }

    pub fn from_branch(branch: &str) -> Self {
        let ref_path = PathBuf::from(format!(".git/refs/heads/{}", branch));
        Self::from_file(&ref_path)
    }

    pub fn update_head_to_branch(branch: &str) {
        std::fs::write(".git/HEAD", format!("ref: refs/heads/{}", branch)).unwrap();
    }

    pub fn from_head() -> Self {
        let file = File::open(Path::new(".git/HEAD")).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = [0; 1024];
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            let bytes_read = reader.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            bytes.extend_from_slice(&buffer[..bytes_read]);
        }
        let ref_path = &String::from_utf8_lossy(&bytes)[5..];
        let ref_path = ref_path.trim();
        Self::from_file(&PathBuf::from(format!(".git/{}", ref_path.to_string())))
    }
}
