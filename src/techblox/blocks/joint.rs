use std::convert::AsRef;

use crate::techblox::{SerializedEntityDescriptor, Parsable, SerializedEntityComponent,
blocks::{BlockEntity, Block}};
use libfj_parsable_macro_derive::*;

/// Joint block entity descriptor
#[derive(Copy, Clone, Parsable)]
pub struct JointBlockEntity {
    /// parent block entity
    pub block: BlockEntity,
}

impl SerializedEntityDescriptor for JointBlockEntity {
    fn serialized_components() -> u8 {
        BlockEntity::serialized_components()
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        self.block.components()
    }

    fn components_mut<'a>(&'a mut self) -> Vec<&'a mut dyn SerializedEntityComponent> {
        self.block.components_mut()
    }

    fn hash_name(&self) -> u32 {
        Self::hash("JointBlockEntityDescriptorV3") // 3586818581
    }
}

impl AsRef<BlockEntity> for JointBlockEntity {
    fn as_ref(&self) -> &BlockEntity {
        &self.block
    }
}

impl Block for JointBlockEntity {}
