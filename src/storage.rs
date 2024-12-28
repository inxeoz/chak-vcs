pub fn save_file(path: &str, content: &str) {
    std::fs::write(path, content).expect("Failed to save file");
}

pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to read file")
}
