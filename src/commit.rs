use std::fs;

pub fn commit(message: &str) {
    // Generate commit hash (simplified for now)
    let commit_hash = "abc123"; // This should be generated dynamically

    // Write commit to the repository history
    let commit_data = format!("{}: {}\n", commit_hash, message);
    fs::write(".chak/commits.txt", commit_data).expect("Failed to write commit");
}
