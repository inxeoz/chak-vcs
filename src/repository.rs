use std::fs;
use std::path::Path;

pub fn init() {
    if !Path::new(".chak").exists() {
        fs::create_dir(".chak").expect("Failed to create .chak directory");
        println!("Initialized empty Chak repository in the current directory.");
    } else {
        println!("Repository already initialized.");
    }
}
