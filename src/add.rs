use hex::encode;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
pub fn create_blob(file_path: &Path, blob_dir: &Path) -> io::Result<String> {
    // Step 1: Read the file data
    let mut file = File::open(file_path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open file {:?}: {}", file_path, e),
        )
    })?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to read file {:?}: {}", file_path, e),
        )
    })?;

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
    let ignore = root_folder.join(".ignore");

    if ignore.exists() {}
    let current_dir_entities = fs::read_dir(root_folder).expect("Failed to read dir");

    let mut bfs_dir_hashset = HashSet::<PathBuf>::new();

    for entry in current_dir_entities {
        match entry {
            Ok(value) => {
                let entity_path = value.path();

                if entity_path.is_dir() {
                    bfs_dir_hashset.insert(entity_path);
                } else {
                    println!("file ---> {}", entity_path.display());
                }
            }
            Err(e) => {
                eprintln!("Failed to read dir entry {}", e);
            }
        }
    }

    for dir_entry in bfs_dir_hashset {
        println!("fold ---> {}", dir_entry.display());
        create_dir_snapshot(&dir_entry);
    }
}

// fn resolve_path(input: &str) -> PathBuf {
//     let path = Path::new(input);
//     path.components().fold(PathBuf::new(), |mut acc, component| {
//         match component {
//             std::path::Component::Prefix(_) => acc.push(component),
//             std::path::Component::RootDir => acc.push(component),
//             std::path::Component::Normal(_) => acc.push(component),
//             std::path::Component::ParentDir => {
//                 // Remove the last component if it's not the root
//                 if acc.pop().is_none() {
//                     acc.push(component);
//                 }
//             }
//             _ => {}
//         }
//         acc
//     })
// }

pub fn print_compo_path(path: &Path) {
    println!("path --> {}", path.display());
    for component in path.components() {

        println!("{:?}", component);
    }

}


pub fn folder_entity_to_set(folder_path: &Path) -> HashSet<PathBuf> {

    if ! folder_path.exists() || (folder_path.is_dir() &&  fs::read_dir(folder_path).expect("Failed to read dir").count() == 0 ) {
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

pub fn ignore_file_to_set(ignore_file_path: &Path) -> HashSet<PathBuf> {
    let mut rules = HashSet::new();
    let ignore_file_parent = ignore_file_path
        .parent()
        .expect("Failed to read parent dir");
    if let Ok(contents) = fs::read_to_string(ignore_file_path) {
        for line in contents.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                let combined_ignore_from_root = ignore_file_parent.join(trimmed);
                match combined_ignore_from_root.canonicalize() {
                    Ok(resolved_path) => {
                        println!("Resolved path: {}", resolved_path.display());

                        rules.insert(resolved_path);
                    }
                    Err(err) => {
                        eprintln!(
                            "Failed to canonicalize path {}: {}",
                            combined_ignore_from_root.display(),
                            err
                        );
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read file: {}", ignore_file_path.display());
    }
    rules
}

pub fn parse_ignore_file(ignore_file_path: &Path, global_ignore_path: &Path) -> HashSet<String> {
    let mut rules = HashSet::new();
    let ignore_file_parent = ignore_file_path
        .parent()
        .expect("Failed to read parent dir")
        .canonicalize()
        .expect("Failed to canonicalize");
    let ignore_file_compo_count = ignore_file_parent.components().count() + 1;

    println!("parent --> {}", ignore_file_parent.display());
    println!("parent coount {ignore_file_compo_count}");

    if let Ok(contents) = fs::read_to_string(ignore_file_path) {
        for line in contents.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                let combined_ignore_from_root = ignore_file_parent.join(trimmed);
                match combined_ignore_from_root.canonicalize() {
                    Ok(resolved_path) => {
                        println!("Resolved path: {}", resolved_path.display());

                        let resolved_path_compo_count = resolved_path.components().count();

                        println!("RES compo count {resolved_path_compo_count}");

                        if resolved_path_compo_count == ignore_file_compo_count {
                            rules.insert(resolved_path.to_string_lossy().to_string());
                        } else {
                            if global_ignore_path.exists() && global_ignore_path.is_file() {
                                let mut global_ignore_file = OpenOptions::new()
                                    .append(true)
                                    .open(global_ignore_path)
                                    .expect("Failed to open global ignore file");

                                // Write the string content to the file
                                writeln!(
                                    global_ignore_file,
                                    "{}",
                                    resolved_path.display().to_string()
                                )
                                .expect("Failed to write global ignore file");
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!(
                            "Failed to canonicalize path {}: {}",
                            combined_ignore_from_root.display(),
                            err
                        );
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read file: {}", ignore_file_path.display());
    }

    rules
}

/// Recursively process directories with local and global ignore rules
fn process_directory(root: &Path, global_rules: &mut HashSet<String>) {}
