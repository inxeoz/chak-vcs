use std::any::{type_name, Any};
use std::fs::read_dir;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::config::{ REGISTER, VCS_FOLDER};
use crate::custom_error::ChakError;
use crate::chak_traits::{ HashPointerTraits};

pub fn deserialize_file_content<T: DeserializeOwned>(path: &Path) -> Result<T, io::Error> {
    let content_string = fs::read_to_string(path)?; // Reads file, propagates error if any
    let content = toml::from_str(&content_string)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?; // Converts serde error into io::Error
    Ok(content)
}

pub fn serialize_struct<T: Serialize>(data: &T) -> String {
    let serialized = toml::to_string(data).expect(format!("serialization failed for {}", type_name::<T>()).as_str());
   // let serialized = serde_json::to_string_pretty(&data).expect("Failed to serialize");
    println!("{}", serialized);
    serialized
}

pub fn check_vcs_presence_in_dir(fold: &Path) -> bool {
        if fold.join(VCS_FOLDER).exists() {
            return true;
        }
        // Read the directory and check subdirectories recursively
        if let Ok(entries) = read_dir(fold) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Recursively check each subdirectory
                    return check_vcs_presence_in_dir(&entry.path())
                }
            }
        }
    false
}

pub fn read_directory_entries(path: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), ChakError> {
    let entries = read_dir(path)?;
    let mut detected_dir_entries = Vec::new();
    let mut detected_file_entries = Vec::new();
    for entry in entries {
        let entry = entry?.path();
        if entry.is_dir() {
            detected_dir_entries.push(entry);
        }else {
            detected_file_entries.push(entry);
        }
    }
    Ok((detected_dir_entries, detected_file_entries))
}


/// Saves content to a file, creating it if it doesn't exist.
pub fn save_or_create_file(
    file_path: &Path,
    content: Option<&str>,
    append: bool,
    append_with_separator: Option<&str>,
) -> Result<File, ChakError> {
    if file_path.is_dir() {
        return Err(ChakError::StdIoError(io::Error::new(
            ErrorKind::IsADirectory,
            "path is a directory, not a file",
        )));
    }

    if let Some(parent_path) = file_path.parent() {
        fs::create_dir_all(parent_path)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .truncate(!append) // Truncate if not appending
        .create(true) // Create the file if it doesn't exist
        .open(file_path)?;

    if let Some(content) = content {
        file.write_all(content.as_bytes())?;
        if let Some(sep_string) = append_with_separator {
            file.write_all(sep_string.as_bytes())?;
        }
    }

    Ok(file) // Return Ok even if content is None
}

/// Function to get input from the command line.
pub fn input_from_commandline(prompt: &str) -> Result<String, ChakError> {
    let mut input = String::new();

    if prompt.len() > 0 {
        print!("{}", prompt);
        io::stdout().flush()?; // Ensure the prompt is displayed immediately
    }

    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}

pub fn file_to_string(file: &mut File) -> Result<String, ChakError> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn file_to_lines(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect()
}


pub fn string_content_to_string_vec(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}



pub fn was_it_registered<T : HashPointerTraits> (pointer: T, dir:&Path) -> bool {

    if let Ok(register )= File::open(dir.join(REGISTER)) {
        for line in file_to_lines(&register) {
           return line.trim() == pointer.get_one_hash()
        }
    }
    false
}
#[cfg(test)]
pub mod tests {
    use crate::config::get_current_dir;
    use crate::util::save_or_create_file;

    #[test]
    pub fn test_save_or_create() {
        save_or_create_file(&get_current_dir().join("test.txt"), Some("i am"), false, None).unwrap();
        save_or_create_file(&get_current_dir().join("test.txt"), Some("i am"), true, Some("\n")).unwrap();
    }
}
