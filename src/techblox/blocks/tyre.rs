use crate::techblox::{SerializedEntityDescriptor, Parsable, SerializedEntityComponent,
blocks::{BlockEntity}};
use libfj_parsable_macro_derive::*;

/// Tire entity descriptor
#[derive(Copy, Clone, Parsable)]
pub struct TyreEntity {
    /// parent block entity
    pub block: BlockEntity,
}

impl SerializedEntityDescriptor for TyreEntity {
    fn serialized_components() -> u8 {
        BlockEntity::serialized_components()
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        self.block.components()
    }

    fn hash_name(&self) -> u32 {
        Self::hash("TyreEntityDescriptorV1") // 1517625162
    }
}
