
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use crate::chak_traits::{ChakPointerTraits, HashPointerTraits};
use crate::config_global::MIN_HASH_LENGTH;
use crate::custom_error::ChakError;
use crate::util::file_to_lines;
use crate::util::{file_to_string, save_or_create_file};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::io::{ BufReader, ErrorKind, Read};
use std::path::{Path };
use crate::impl_pointer_common_traits;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct HashPointer {
    pub(crate) fold_name: String,
    pub(crate) file_name: String,
}

impl_pointer_common_traits!(HashPointer);

impl HashPointer {
    fn new(fold_name: String, file_name: String) -> Self {
        Self {
            fold_name,
            file_name,
        }
    }
    fn _from_hash_string(hash: String) -> Self {
        Self::new(hash[..2].to_string(), hash[2..].to_string())
    }

    pub fn from_hash_pointer_string(hash: String) -> Result<Self, ChakError> {
        if hash.len() < MIN_HASH_LENGTH {
            return Err(ChakError::InvalidHashLength(format!(
                "{} Hash length {} is too short, length at least {}",
                hash,
                hash.len(),
                MIN_HASH_LENGTH
            )));
        }
        Ok(Self::_from_hash_string(hash))
    }

    pub fn combine(first: &Self, second: &Self) -> Self {
        Self::from_string(&(first.get_one_hash() + &second.get_one_hash()))
    }

    pub fn from_file_path(file_path: &Path) -> Result<Self, ChakError> {
        let mut file = File::open(file_path)?;
        Ok(Self::from_file(&mut file))
    }

    pub fn from_file(file: &File) -> Self {
        let mut buf_file = BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 1024];

        while let Ok(bytes_read) = buf_file.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        Self::_from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn from_save_string(content: &str, save_dir: &Path) -> Result<Self, ChakError> {
        let hash_pointer = Self::from_string(content);
        save_or_create_file(
            &save_dir.join(hash_pointer.get_path()),
            Some(content),
            false,
            None,
        )?;
        Ok(hash_pointer)
    }

    pub fn from_pointers<T: ChakPointerTraits + HashPointerTraits>(
        pointers: Vec<T>,
    ) -> Result<Self, ChakError> {
        if pointers.is_empty() {
            return Err(ChakError::CustomError(
                "Empty hash pointer vector".to_string(),
            ));
        }
        let mut hasher = Sha256::new();
        for pointer in pointers {
            hasher.update(pointer.get_one_hash().as_bytes());
        }
        Ok(Self::_from_hash_string(format!("{:x}", hasher.finalize())))
    }

    pub fn from_string(content: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        Self::_from_hash_string(format!("{:x}", hasher.finalize()))
    }

    // Rest of your functions remain unchanged...

    pub fn from_string_vec(strings: &[String]) -> Self {
        let mut hasher = Sha256::new();
        for string in strings {
            hasher.update(string.as_bytes());
        }
        Self::_from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn get_latest_pointer_line_from_file<T: HashPointerTraits + ChakPointerTraits + Clone>(
        file: &File,
        from_bottom: bool,
    ) -> Result<T, ChakError> {
        let pointers = HashPointer::get_pointer_lines_from_file::<T>(file)?;

        if from_bottom {
            if let Some(pointer) = pointers.last() {
                Ok(pointer.to_owned())
            } else {
                Err(ChakError::StdIoError(io::Error::new(
                    ErrorKind::NotFound,
                    "last hash pointer line not found in file",
                )))
            }
        } else {
            if let Some(pointer) = pointers.first() {
                Ok(pointer.to_owned())
            } else {
                Err(ChakError::StdIoError(io::Error::new(
                    ErrorKind::NotFound,
                    "first hash pointer line not found in file",
                )))
            }
        }
    }

    pub fn get_pointer_lines_from_file<T: HashPointerTraits + ChakPointerTraits>(
        file: &File,
    ) -> Result<Vec<T>, ChakError> {
        let lines = file_to_lines(file);
        let mut pointers = Vec::<T>::new();

        for line in lines {
            if let Ok(pointer_line) = Self::from_hash_pointer_string(line) {
                pointers.push(T::own(&pointer_line)?);
            }
        }
        Ok(pointers)
    }

    pub fn and_string_from_file_path_ref(file_path: &Path) -> Result<(Self, String), ChakError> {
        let mut file = File::open(file_path)?;
        let content = file_to_string(&mut file)?;
        let hash_pointer = Self::from_string(&content);
        Ok((hash_pointer, content))
    }

}

