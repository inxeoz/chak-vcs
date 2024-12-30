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
pub fn print_compo_path(path: &Path) {
    println!("path --> {}", path.display());
    for component in path.components() {
        println!("{:?}", component);
    }
}

pub fn resolve_path(path: &Path) -> PathBuf {
    let mut stack = Vec::new(); // This stack will hold the components of the resolved path
                                // Iterate over each component of the path

    println!("resolving path {:?}", path);

    let mut my_root = PathBuf::new();
    for component in path.components() {
        match component {
            // The current directory (`./`), we skip it
            Component::CurDir => {
                // No operation needed, `./` doesn't affect the path
                my_root.push(component.as_os_str());
            }

            // The root directory (`/`), we start fresh with an empty stack
            // Any other component, like a prefix or unknown, can be ignored (but this should not happen in most cases)
            Component::RootDir => {
                stack.clear(); // Reset the stack, we are starting from the root
                my_root.push(component.as_os_str());
            }
            // A component that is part of the path (a directory or file)
            Component::Normal(part) => {
                stack.push(part.to_string_lossy().to_string());
            }
            // The parent directory (`..`): Pop the last component from the stack
            Component::ParentDir => {
                if !stack.is_empty() {
                    stack.pop(); // Go up one directory, if possible
                }
            }

            _ => {}
        }
    }

    // Join the components in the stack to form the resolved path
    let resolved_path = my_root.join(  PathBuf::from(stack.join("/")) );
    if resolved_path.exists() {
        if resolved_path.is_dir() {
            println!("{} is a directory", resolved_path.display());
        } else {
            let data = fs::read_to_string(&resolved_path).expect("Failed to read path");
            println!("{}", data);
        }
    } else {
        println!("{} is not a directory", resolved_path.display());
    }

    println!("resolved path: {:?}", resolved_path.display());
    resolved_path
}

pub fn folder_entity_to_set(folder_path: &Path) -> HashSet<PathBuf> {
    if !folder_path.exists()
        || (folder_path.is_dir()
            && fs::read_dir(folder_path)
                .expect("Failed to read dir")
                .count()
                == 0)
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
