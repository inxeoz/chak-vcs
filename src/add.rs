use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use hex::encode;
pub fn create_blob(file_path: &Path, blob_dir: &Path) -> io::Result<String> {
    // Step 1: Read the file data
    let mut file = File::open(file_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to open file {:?}: {}", file_path, e)))?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read file {:?}: {}", file_path, e)))?;

    // Step 2: Generate a hash of the file (for uniqueness)
    let mut hasher = Sha256::new();
    hasher.update(&file_data);
    let hash = hasher.finalize();
    let hash_str = encode(hash); // Convert the hash to a hex string

    // Step 3: Create the subdirectory based on the first two characters of the hash
    let subdir = blob_dir.join(&hash_str[0..2]); // First two characters of hash
    fs::create_dir_all(&subdir).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to create blob subdirectory {:?}: {}", subdir, e),
        )
    })?;

    // Step 4: Save the blob to a new file named by the hash
    let blob_path = subdir.join(&hash_str[2..]);
    let mut blob_file = File::create(&blob_path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to create blob file {:?}: {}", blob_path, e),
        )
    })?;
    blob_file.write_all(&file_data).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to write to blob file {:?}: {}", blob_path, e),
        )
    })?;

    Ok(hash_str)
}



//Tomorrow TODO to implement global ignore and actual entity, ignore entity using set operation
pub fn create_dir_snapshot(root_folder: &Path) {
    let current_dir_entities = fs::read_dir(root_folder).expect("Failed to read dir");

    let mut bfs_dir_hashset = HashSet::<PathBuf>::new();

    for entry in current_dir_entities {
        match entry {
            Ok(value) => {
                let entity_path = value.path();

                if entity_path.is_dir() {
                    bfs_dir_hashset.insert(entity_path);
                }else {
                    println!("file ---> {}", entity_path.display());
                }


            }
            Err(e) => {eprintln!("Failed to read dir entry {}",e);}
        }
    }

    for dir_entry in bfs_dir_hashset {
        println!("fold ---> {}", dir_entry.display());
        create_dir_snapshot(&dir_entry);
    }
}
