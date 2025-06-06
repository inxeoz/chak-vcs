use crate::{bind_ref_object_with_pointer, restricted};
use crate::config::{get_nested_trees_fold_path};
use crate::common::{load_entity, save_entity};
use crate::custom_error::ChakError;
use crate::impl_pointer_common_traits;
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::chak_traits::ChakPointerTraits;
use crate::chak_traits::{ HashPointerTraits};
use crate::nested_tree_object::NestedTreeObject;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct NestedTreeHashPointer {
    fold_name: String,
    file_name: String,
}
bind_ref_object_with_pointer!(NestedTreeHashPointer, NestedTreeObject);
impl_pointer_common_traits!(NestedTreeHashPointer);
impl NestedTreeHashPointer {
    pub fn save_tree(tree: &mut NestedTreeObject) -> Result<NestedTreeHashPointer, ChakError> {
        tree.sort_children();
        Self::own(&save_entity(tree)?)
    }
    pub fn load_tree(&self) -> NestedTreeObject {
        load_entity::<Self, NestedTreeObject>(self, &get_nested_trees_fold_path())
    }

}
