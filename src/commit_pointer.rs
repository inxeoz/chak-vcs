use crate::{bind_ref_object_with_pointer, restricted};
use crate::chak_traits::HashPointerTraits;
use serde::{Deserialize, Serialize};
use crate::config::{get_commit_log_file_path, get_commits_fold_path, get_commit_log_file, get_stage_file_path};
use crate::common::{load_entity, save_entity};

use crate::impl_pointer_common_traits;
use crate::util::{save_or_create_file};
use std::cmp::Ordering;
use crate::chak_traits::ChakPointerTraits;
use crate::commit_object::{ CommitObject};
use crate::custom_error::ChakError;
use crate::hash_pointer::{HashPointer };
use crate::root_tree_pointer::RootTreePointer;

//these custom hash pointer would have other field in future
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct CommitPointer {
    fold_name: String,
    file_name: String,
}

bind_ref_object_with_pointer!(CommitPointer, CommitObject);
impl_pointer_common_traits!(CommitPointer);
impl CommitPointer {

    pub fn save_commit(commit: &CommitObject) -> Result<CommitPointer, ChakError> {
        Self::own(&save_entity(commit)?)
    }

    pub fn load_commit(&self) -> CommitObject {
        load_entity::<Self, CommitObject>(self, &get_commits_fold_path())
    }

    pub fn get_latest_commit_hash_pointer() -> Result<CommitPointer, ChakError> {
        // as commit log file created at initialization
        let commit_file = get_commit_log_file()?;
        Ok( HashPointer::get_latest_pointer_line_from_file::<CommitPointer>(&commit_file, true)?)
    }
}

pub fn create_commit(
    msg: String,
    author: Option<String>,
    root_tree_pointer: RootTreePointer,
) -> CommitObject {
    CommitObject {
        message: msg,
        root_tree_pointer,
        author: author.unwrap_or("unknown".to_string()),
    }
}

pub fn append_commit_hash_pointer_to_commit_log_file(commit_hash_pointer: CommitPointer )  {

    save_or_create_file(
        &get_commit_log_file_path(), Some(&commit_hash_pointer.get_one_hash()), true, Some("\n")
    ).expect("cant save commit to commit log");
}

pub fn handle_command_commit(m:String) -> Result<(), ChakError> {

    if let Ok(all_tree_pointers) = RootTreePointer::get_pointers_from_stage(){

        for  tree_pointer in all_tree_pointers.iter().rev(){//latest pointer from stage
                let commit_pointer = CommitPointer::save_commit(&create_commit(
                    m.clone(),
                    Some("inxeoz".to_string()),
                    tree_pointer.to_owned(),
                ))?;

                append_commit_hash_pointer_to_commit_log_file(commit_pointer);
                return Ok(());
        }

        std::fs::write(get_stage_file_path(), "").map_err(
            |_| ChakError::CustomError("Failed to clear stage ".to_string())
        )
    } else {
        Err(ChakError::CustomError("No stage file".to_string()))
    }
}