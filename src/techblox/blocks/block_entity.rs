use crate::techblox::{SerializedEntityDescriptor, Parsable, SerializedEntityComponent};
use crate::techblox::blocks::{DBEntityStruct, PositionEntityStruct, ScalingEntityStruct, RotationEntityStruct,
SkewComponent, GridRotationStruct, SerializedGridConnectionsEntityStruct, SerializedBlockPlacementInfoStruct,
SerializedCubeMaterialStruct, SerializedUniformBlockScaleEntityStruct, SerializedColourParameterEntityStruct,
BlockGroupEntityComponent};
use libfj_parsable_macro_derive::*;

/// Block entity descriptor.
#[derive(Copy, Clone, Parsable)]
pub struct BlockEntity {
    /// Database component
    pub db_component: DBEntityStruct,
    /// Position component
    pub pos_component: PositionEntityStruct,
    /// Scale component
    pub scale_component: ScalingEntityStruct,
    /// Rotation component
    pub rot_component: RotationEntityStruct,
    /// Skew matrix component
    pub skew_component: SkewComponent,
    /// Grid component
    pub grid_component: GridRotationStruct,
    // GridConnectionsEntityStruct is not serialized to disk
    /// No-op serializer (this has no data!)
    pub grid_conn_component: SerializedGridConnectionsEntityStruct,
    // BlockPlacementInfoStruct has a disk serializer that does nothing (?)
    /// No-op serializer (this has no data!)
    pub placement_component: SerializedBlockPlacementInfoStruct,
    /// Cube material component
    pub material_component: SerializedCubeMaterialStruct,
    /// Uniform scale component
    pub uscale_component: SerializedUniformBlockScaleEntityStruct,
    /// Colour component
    pub colour_component: SerializedColourParameterEntityStruct,
    /// Group component
    pub group_component: BlockGroupEntityComponent,
}

impl SerializedEntityDescriptor for BlockEntity {
    fn serialized_components() -> u8 {
        12
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        vec![&self.db_component,
        &self.pos_component,
        &self.scale_component,
        &self.rot_component,
        &self.skew_component,
        &self.grid_component,
        &self.grid_conn_component,
        &self.placement_component,
        &self.material_component,
        &self.uscale_component,
        &self.colour_component,
        &self.group_component]
    }

    fn hash_name(&self) -> u32 {
        Self::hash("StandardBlockEntityDescriptorV4") // 1357220432
    }
}
