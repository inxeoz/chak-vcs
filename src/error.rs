#[derive(Debug)]
pub enum ChakError {
    IoError(std::io::Error),
    CommitError(String),
}

impl std::fmt::Display for ChakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
