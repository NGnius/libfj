use crate::techblox::{UnityFloat3, UnityQuaternion, SerializedEntityComponent, SerializedEntityDescriptor, Parsable};
use libfj_parsable_macro_derive::*;

/// Block group entity descriptor.
#[derive(Clone, Copy, Parsable)]
pub struct BlockGroupEntity {
    /// Block group identifier
    pub saved_block_group_id: SavedBlockGroupIdComponent,
    /// Block group location information
    pub block_group_transform: BlockGroupTransformEntityComponent,
}

impl BlockGroupEntity {}

impl SerializedEntityDescriptor for BlockGroupEntity {
    fn serialized_components() -> u8 {
        2
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        vec![&self.saved_block_group_id,
        &self.block_group_transform]
    }

    fn hash_name(&self) -> u32 {
        Self::hash("BlockGroupEntityDescriptorV0")
    }
}

/// Saved block group identifier entity component.
#[derive(Clone, Copy, Parsable)]
pub struct SavedBlockGroupIdComponent {
    /// Block group identifier
    pub saved_block_group_id: i32,
}

impl SerializedEntityComponent for SavedBlockGroupIdComponent {}

/// Block group entity component for storing position and rotation.
#[derive(Clone, Copy, Parsable)]
pub struct BlockGroupTransformEntityComponent {
    /// Block group position
    pub block_group_grid_position: UnityFloat3,
    /// Block group rotation
    pub block_group_grid_rotation: UnityQuaternion,
}

impl SerializedEntityComponent for BlockGroupTransformEntityComponent {}
