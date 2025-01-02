use std::collections::HashSet;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use hex::encode;
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


pub fn write_to_file(path: PathBuf, content: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)?;
    Ok(())
}


pub fn print_compo_path(path: &Path) {
    println!("path --> {}", path.display());
    for component in path.components() {
        println!("{:?}", component);
    }
}


pub fn print_file(file_path: &Path) {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("TODO: panic message");
    println!("{}", content);
}

pub fn create_blob(file_path: &Path, blob_dir: &Path) -> Result<String, io::Error> {
    if !file_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let file_data = fs::read(file_path)?;
    let hash_str = encode(Sha256::digest(&file_data));

    let subdir = blob_dir.join(&hash_str[0..2]);
    fs::create_dir_all(&subdir)?;

    let blob_path = subdir.join(&hash_str[2..]);
    fs::write(&blob_path, &file_data)?;

    Ok(hash_str)
}