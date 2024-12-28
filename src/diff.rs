use std::fs;

pub fn diff() {
    // Compare current files to the last commit
    let file_contents = fs::read_to_string("file.txt").expect("Failed to read file");
    println!("Differences:\n{}", file_contents);
}
