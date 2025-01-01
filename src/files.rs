use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};

pub fn write_to_file_with_vec(file_path: &Path, content: Vec<String>, append: bool) -> String {
    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .create(true)
        .open(file_path)
        .expect("Failed to open/create file");

    if !append {
        file.set_len(0).expect("Failed to clear file");
    }

    for line in content {
        writeln!(file, "{}", line).expect("Failed to write to file");
    }

    let mut hasher = Sha256::new();
    io::copy(&mut File::open(file_path).expect("Failed to open file for hashing"), &mut hasher)
        .expect("Failed to hash file");

    format!("{:x}", hasher.finalize())
}

pub fn sort_by_component_count(paths: &HashSet<PathBuf>) -> Vec<String> {
    let mut sorted_paths: Vec<(&PathBuf, usize)> = paths
        .iter()
        .map(|path| (path, path.components().count()))
        .collect();

    sorted_paths.sort_by(|a, b| a.1.cmp(&b.1));

    sorted_paths
        .iter()
        .map(|(path, _)| path.to_string_lossy().to_string())
        .collect()
}
