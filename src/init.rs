use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn init_vcs(base_dir: &Path) -> io::Result<()> {
    let directories = vec![
        "store/blobs",
        "store/trees",
        "store/commits",
        "staging_area"
    ];

    let files = vec![
        "staging_area/staged_files.json",
        "history/commits.log",
    ];

    // Step 1: Create all directories
    for dir in directories {
        let dir_path = base_dir.join(dir);
        fs::create_dir_all(&dir_path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to create directory {:?}: {}", dir_path, e),
            )
        })?;
    }

    // Step 2: Create all files
    for file in files {
        let file_path = base_dir.join(file);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!("Failed to create parent directory {:?}: {}", parent, e),
                )
            })?;
        }
        File::create(&file_path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to create file {:?}: {}", file_path, e),
            )
        })?;
    }

    Ok(())
}



