use std::path::Path;
use crate::add;

pub fn test_create_dir_snapshot() {

    let root_dir = Path::new("./example/");
    add::create_dir_snapshot(root_dir)
}