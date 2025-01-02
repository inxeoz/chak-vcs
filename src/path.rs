use std::env::current_dir;
use std::path::{Component, Path, PathBuf};


pub fn get_relative_path<'a>(path: &'a PathBuf, base_path: &'a Path) -> Option<&'a Path> {
    if path.is_relative() {
        Some(path.as_path())
    } else if let Ok(relative_path) = path.strip_prefix(base_path) {
        Some(relative_path)
    } else {
        None
    }
}
pub fn resolve_path(path: &Path) -> PathBuf {
    let mut stack = Vec::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                stack.pop();
            }
            Component::Normal(part) => stack.push(part),
            Component::RootDir => stack.clear(),
            _ => {}
        }
    }

    PathBuf::new().join("./").join(stack.iter().collect::<PathBuf>())
}