use crate::techblox::{SerializedEntityDescriptor, Parsable, SerializedEntityComponent, UnityFloat3, UnityHalf3};

use libfj_parsable_macro_derive::*;

/// Player editing camera entity descriptor.
#[derive(Copy, Clone, Parsable)]
pub struct SerializedFlyCamEntity {
    /// Player camera in-game location
    pub rb_component: SerializedRigidBodyEntityStruct,
}

impl SerializedEntityDescriptor for SerializedFlyCamEntity {
    fn serialized_components() -> u8 {
        2
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        vec![&self.rb_component]
    }

    fn hash_name(&self) -> u32 {
        Self::hash("FlyCamEntityDescriptorV0") // 252528354
    }
}

/// Physical object info for simulation
#[derive(Copy, Clone, Parsable)]
pub struct SerializedRigidBodyEntityStruct {
    /// Rigid body location
    pub position: UnityFloat3,
}

impl SerializedEntityComponent for SerializedRigidBodyEntityStruct {}

/// Player simulation camera entity descriptor.
#[derive(Copy, Clone, Parsable)]
pub struct SerializedPhysicsCameraEntity {
    /// In-game camera location information
    pub cam_component: SerializedCameraEntityStruct,
}

impl SerializedEntityDescriptor for SerializedPhysicsCameraEntity {
    fn serialized_components() -> u8 {
        1
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        vec![&self.cam_component]
    }

    fn hash_name(&self) -> u32 {
        Self::hash("CharacterCameraEntityDescriptorV1") // 3850144645
    }
}

/// Physics camera component
#[derive(Copy, Clone, Parsable)]
pub struct SerializedCameraEntityStruct {
    /// Camera position in game world
    pub position: UnityHalf3,
    /// Camera euler rotation in game world
    pub rotation: UnityHalf3,
}

impl SerializedEntityComponent for SerializedCameraEntityStruct {}
