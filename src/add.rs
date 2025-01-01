use hex::encode;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};

pub struct CHAKRA {
    pub working_dir: PathBuf,
}
impl CHAKRA {
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
            CHAKRA::create_dir_snapshot(&dir_entry);
        }
    }
    pub fn print_compo_path(path: &Path) {
        println!("path --> {}", path.display());
        for component in path.components() {
            println!("{:?}", component);
        }
    }


    /// Resolve a relative path to an absolute path, handling ".." and "./" components.
    pub fn resolve_path(path: &Path, working_root_path: &Path) -> PathBuf {
        let mut stack = Vec::new();

        for component in path.components() {
            match component {
                Component::CurDir => {},
                Component::ParentDir => { stack.pop(); },
                Component::Normal(part) => stack.push(part),
                Component::RootDir => stack.clear(),
                _ => {},
            }
        }

        working_root_path.join(stack.iter().collect::<PathBuf>())
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

    pub fn local_ignore_file_to_set(ignore_file_path: &Path, working_root_path: &Path) -> HashSet<PathBuf> {
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
                    let combined_ignore_from_root = CHAKRA::resolve_path(&ignore_file_parent.join(trimmed), working_root_path);

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

    pub fn parse_ignore_file(ignore_file_path: &Path, global_ignore_path: &Path, working_root_path: &Path) -> HashSet<String> {
        let mut ignore_set = HashSet::new();

        let local_ignore = CHAKRA::local_ignore_file_to_set(ignore_file_path, working_root_path);
        let mut global_ignore = CHAKRA::global_ignore_file_to_set(global_ignore_path);
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

    /// Create a blob for a file and store it in the blob directory.
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

    /// Recursively snapshot a directory, returning a map of file paths to their hashes.
    pub fn snapshot_directory(dir_path: &Path, blob_dir: &Path) -> Result<HashMap<PathBuf, String>, io::Error> {
        let mut snapshot = HashMap::new();

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let sub_snapshot = Self::snapshot_directory(&path, blob_dir)?;
                snapshot.extend(sub_snapshot);
            } else {
                let hash = Self::create_blob(&path, blob_dir)?;
                snapshot.insert(path, hash);
            }
        }

        Ok(snapshot)
    }

    /// Initialize a new repository by creating necessary directories.
    pub fn init_repository(root_dir: &Path) -> Result<(), io::Error> {
        let chakra_dir = root_dir.join(".chakra");
        let blob_dir = chakra_dir.join("blobs");

        fs::create_dir_all(&blob_dir)?;
        fs::write(chakra_dir.join("config"), b"[core]\nrepository=true\n")?;

        Ok(())
    }

    /// Compute the hash of a directory by combining hashes of its contents.
    pub fn compute_dir_hash(snapshot: &HashMap<PathBuf, String>) -> String {
        let mut hasher = Sha256::new();

        for (path, hash) in snapshot {
            hasher.update(path.to_string_lossy().as_bytes());
            hasher.update(hash.as_bytes());
        }

        encode(hasher.finalize())
    }

    /// Generate a list of changes between two directory snapshots.
    pub fn diff_snapshots(old: &HashMap<PathBuf, String>, new: &HashMap<PathBuf, String>) -> Vec<String> {
        let mut changes = Vec::new();

        for (path, new_hash) in new {
            match old.get(path) {
                Some(old_hash) if old_hash == new_hash => {},
                Some(_) => changes.push(format!("Modified: {:?}", path)),
                None => changes.push(format!("Added: {:?}", path)),
            }
        }

        for path in old.keys() {
            if !new.contains_key(path) {
                changes.push(format!("Deleted: {:?}", path));
            }
        }

        changes
    }

    /// Load `.ignore` file rules into a set.
    pub fn load_ignore_rules(ignore_file: &Path) -> Result<HashSet<PathBuf>, io::Error> {
        let mut rules = HashSet::new();

        if ignore_file.exists() {
            let content = fs::read_to_string(ignore_file)?;
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    rules.insert(PathBuf::from(trimmed));
                }
            }
        }

        Ok(rules)
    }

    /// Filter files and directories based on `.ignore` rules.
    pub fn filter_with_ignore(snapshot: HashMap<PathBuf, String>, ignore_rules: &HashSet<PathBuf>) -> HashMap<PathBuf, String> {
        snapshot
            .into_iter()
            .filter(|(path, _)| !ignore_rules.contains(path))
            .collect()
    }
}

