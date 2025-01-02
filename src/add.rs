use std::collections::HashSet;
use std::fs;
use hex::encode;
use sha2::{Digest, Sha256};

use std::path::{Component, Path, PathBuf};

pub fn create_dir_snapshot(root_folder: &Path) {

}

/// Resolve a relative path to an absolute path, handling ".." and "./" components.


pub fn folder_entity_to_set(folder_path: &Path) -> HashSet<PathBuf> {
    if !folder_path.exists()
        || (folder_path.is_dir()
        && fs::read_dir(folder_path)
        .expect("Failed to read dir")
        .count()
        == 0)
        || folder_path.is_file()
    {
        return HashSet::new();
    }

    let mut entities_set = HashSet::<PathBuf>::new();
    let current_dir_entities = fs::read_dir(folder_path).expect("Failed to read dir");
    for entry in current_dir_entities {
        match entry {
            Ok(value) => {
                println!("value --> {:?}", &value.path());
                entities_set.insert(value.path());
            }
            Err(e) => {
                eprintln!("Failed to read dir entry {}", e);
            }
        }
    }

    entities_set
}






