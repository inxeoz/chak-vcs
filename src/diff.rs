
use std::collections::HashMap;
use std::path::PathBuf;

pub fn diff_snapshots(
    old: &HashMap<PathBuf, String>,
    new: &HashMap<PathBuf, String>,
) -> Vec<String> {
    let mut changes = Vec::new();

    for (path, new_hash) in new {
        match old.get(path) {
            Some(old_hash) if old_hash == new_hash => {}
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