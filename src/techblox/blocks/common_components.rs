use crate::techblox::{Parsable, SerializedEntityComponent, UnityFloat3,
UnityQuaternion, UnityFloat4x4};
use libfj_parsable_macro_derive::*;

/// Database entity component.
#[derive(Copy, Clone, Parsable)]
pub struct DBEntityStruct {
    /// Database identifier
    pub dbid: u32,
}

impl SerializedEntityComponent for DBEntityStruct {}

/// Position entity component.
#[derive(Copy, Clone, Parsable)]
pub struct PositionEntityStruct {
    /// Entity position
    pub position: UnityFloat3,
}

impl SerializedEntityComponent for PositionEntityStruct {}

/// Scaling entity component.
#[derive(Copy, Clone, Parsable)]
pub struct ScalingEntityStruct {
    /// Entity position
    pub scale: UnityFloat3,
}

impl SerializedEntityComponent for ScalingEntityStruct {}

/// Scaling entity component.
#[derive(Copy, Clone, Parsable)]
pub struct RotationEntityStruct {
    /// Entity position
    pub rotation: UnityQuaternion,
}

impl SerializedEntityComponent for RotationEntityStruct {}

/// Block skew component.
#[derive(Copy, Clone, Parsable)]
pub struct SkewComponent {
    /// Block skew matrix
    pub skew_matrix: UnityFloat4x4,
}

impl SerializedEntityComponent for SkewComponent {}

/// Block placement grid rotation component.
#[derive(Copy, Clone, Parsable)]
pub struct GridRotationStruct {
    /// Grid rotation
    pub rotation: UnityQuaternion,
    /// Grid position
    pub position: UnityFloat3,
}

impl SerializedEntityComponent for GridRotationStruct {}

// *** These don't contain anything but the game thinks they do ***
// GridConnectionsEntityStruct is not serialized to disk
// BlockPlacementInfoStruct has a disk serializer that does nothing (?)

/// Empty, basically useless except that Techblox says it exists while serializing.
#[derive(Copy, Clone, Parsable)]
pub struct SerializedGridConnectionsEntityStruct {}

impl SerializedEntityComponent for SerializedGridConnectionsEntityStruct {}

/// Empty, basically useless except that Techblox says it exists while serializing.
#[derive(Copy, Clone, Parsable)]
pub struct SerializedBlockPlacementInfoStruct {}

impl SerializedEntityComponent for SerializedBlockPlacementInfoStruct {}

// *** These do contain data again ***

/// Block material component.
#[derive(Copy, Clone, Parsable)]
pub struct SerializedCubeMaterialStruct {
    /// Material identifier
    pub material_id: u8,
}

impl SerializedEntityComponent for SerializedCubeMaterialStruct {}

/// Block uniform scale component.
#[derive(Copy, Clone, Parsable)]
pub struct SerializedUniformBlockScaleEntityStruct {
    /// Uniform scale factor
    pub scale_factor: u8,
}

impl SerializedEntityComponent for SerializedUniformBlockScaleEntityStruct {}

/// Block colour component.
#[derive(Copy, Clone, Parsable)]
pub struct SerializedColourParameterEntityStruct {
    /// Index of colour in Techblox palette
    pub index_in_palette: u8,
}

impl SerializedEntityComponent for SerializedColourParameterEntityStruct {}

/// Block group component.
#[derive(Copy, Clone, Parsable)]
pub struct BlockGroupEntityComponent {
    /// Index of block in Techblox block groups (deserialized in earlier part of game save)
    pub current_block_group: i32,
}

impl SerializedEntityComponent for BlockGroupEntityComponent {}
