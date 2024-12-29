use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn init_vcs(base_dir: &Path) -> io::Result<()> {
    let directories = vec![
        "store/blobs",
        "store/trees",
        "store/commits",
        "store/pack",
        "staging_area",
        "metadata/branches",
        "metadata/tags",
        "history",
        "temp/stash",
        "remotes/origin",
        "remotes/upstream",
        "scripts",
        "conflicts",
        "undo",
        "stats",
        "docs",
    ];

    let files = vec![
        "staging_area/staged_files.json",
        "metadata/HEAD",
        "history/commits.log",
        "history/merges.log",
        "temp/merge_state.json",
        "scripts/pre_commit",
        "scripts/post_merge",
        "conflicts/file_a.conflict",
        "undo/stage_undo.log",
        "stats/contributors.json",
        "stats/file_stats.json",
        "docs/readme.md",
        "docs/vcs_internals.md",
        "config.json",
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

    // Step 3: Write initial content to specific files
    write_to_file(base_dir.join("staging_area/staged_files.json"), b"{}")?;
    write_to_file(base_dir.join("metadata/HEAD"), b"refs/heads/main")?;
    write_to_file(base_dir.join("history/commits.log"), b"")?;
    write_to_file(base_dir.join("history/merges.log"), b"")?;
    write_to_file(base_dir.join("temp/merge_state.json"), b"{}")?;
    write_to_file(base_dir.join("stats/contributors.json"), b"[]")?;
    write_to_file(base_dir.join("stats/file_stats.json"), b"{}")?;
    write_to_file(base_dir.join("docs/readme.md"), b"# Version Control System\n")?;
    write_to_file(base_dir.join("docs/vcs_internals.md"), b"# VCS Internals\n")?;
    write_to_file(base_dir.join("config.json"), b"{}")?;
    write_to_file(base_dir.join("chak.ignore"), b"")?;

    Ok(())
}

fn write_to_file(path: PathBuf, content: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)?;
    Ok(())
}

