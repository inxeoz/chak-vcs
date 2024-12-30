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
    let resolved_path = my_root.join(PathBuf::from(stack.join("/")));
    resolved_path
}

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

pub fn local_ignore_file_to_set(ignore_file_path: &Path) -> HashSet<PathBuf> {
    if !ignore_file_path.exists() || ignore_file_path.is_dir() {
        return HashSet::new();
    }

    let mut rules = HashSet::new();
    let ignore_file_parent = ignore_file_path
        .parent()
        .expect("Failed to read parent dir");
    if let Ok(contents) = fs::read_to_string(ignore_file_path) {
        for line in contents.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                let combined_ignore_from_root = resolve_path(&ignore_file_parent.join(trimmed));

                rules.insert(combined_ignore_from_root);
            }
        }
    }
    rules
}

pub fn global_ignore_file_to_set(global_ignore_file_path: &Path) -> HashSet<PathBuf> {
    if !global_ignore_file_path.exists() || global_ignore_file_path.is_dir() {
        return HashSet::new();
    }

    let mut rules = HashSet::<PathBuf>::new();
    if let Ok(contents) = fs::read_to_string(global_ignore_file_path) {
        for line in contents.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                let mut ignore_path = Path::new(trimmed);
                if ignore_path.exists() {
                    rules.insert(ignore_path.to_path_buf());
                }
            }
        }
    }
    rules
}

pub fn parse_ignore_file(ignore_file_path: &Path, global_ignore_path: &Path) -> HashSet<String> {
    let mut ignore_set = HashSet::new();

    let local_ignore = local_ignore_file_to_set(ignore_file_path);
    let mut global_ignore = global_ignore_file_to_set(global_ignore_path);
    let global_ignore_intial_size = global_ignore.len();

    let local_ignore_parent = ignore_file_path
        .parent()
        .expect("Failed to read parent dir");
    let local_ignore_parent_compo_count = local_ignore_parent.components().count() + 1;

    let global_ignore_copy = global_ignore.clone();
    let combined_ignore_file = local_ignore.union(&global_ignore_copy);

    for ignore_entity in combined_ignore_file {
        println!(
            "entity {} with compo count {}",
            ignore_entity.display(),
            ignore_entity.components().count()
        );
        println!(
            "local_parent {} with compo count {}\n",
            local_ignore_parent.display(),
            local_ignore_parent_compo_count
        );

        let is_start_with_parent = ignore_entity.starts_with(&local_ignore_parent);
        println!("is_start_with_parent {:?}", is_start_with_parent);
        if is_start_with_parent {
            if local_ignore_parent_compo_count == ignore_entity.components().count() {
                println!(
                    "i am adding this entity {}",
                    ignore_entity.display().to_string()
                );
                ignore_set.insert(ignore_entity.display().to_string());
            } else {
                println!(
                    "i am adding this entity {} to global ignore ",
                    ignore_entity.display().to_string()
                );
                global_ignore.insert(ignore_entity.to_path_buf());
            }
        } else if ignore_entity.exists() {
            global_ignore.insert(ignore_entity.to_path_buf());
        }
    }

    if global_ignore_intial_size != global_ignore.len() {
        let second_ignore = global_ignore_path
            .parent()
            .expect("Failed to read parent dir")
            .join(".ignore");

        fs::write(&second_ignore, "").expect("Failed to write ignore file");

        let mut second_global_ignore = OpenOptions::new().append(true).open(second_ignore).unwrap();
        // Join all elements with newlines and write to the file
        for ignore_entry in global_ignore {
            writeln!(second_global_ignore, "{}", ignore_entry.to_str().unwrap()).unwrap();
        }
    }

    ignore_set
}

/// Recursively process directories with local and global ignore rules
fn process_directory(root: &Path, global_rules: &mut HashSet<String>) {}
