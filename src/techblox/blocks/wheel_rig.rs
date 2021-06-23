use crate::techblox::{SerializedEntityDescriptor, Parsable, SerializedEntityComponent,
blocks::{BlockEntity, TweakableJointDampingComponent}};
use libfj_parsable_macro_derive::*;

/// Wheel rig entity descriptor
#[derive(Copy, Clone, Parsable)]
pub struct WheelRigEntity {
    /// parent block entity
    pub block: BlockEntity,
    /// Wheel tweakables component
    pub tweak_component: WheelRigTweakableStruct,
    /// Joint tweakables component
    pub joint_component: TweakableJointDampingComponent,
}

impl SerializedEntityDescriptor for WheelRigEntity {
    fn serialized_components() -> u8 {
        BlockEntity::serialized_components() + 2
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        let mut c = self.block.components();
        c.push(&self.tweak_component);
        c.push(&self.joint_component);
        return c;
    }

    fn components_mut<'a>(&'a mut self) -> Vec<&'a mut dyn SerializedEntityComponent> {
        let mut c = self.block.components_mut();
        c.push(&mut self.tweak_component);
        c.push(&mut self.joint_component);
        return c;
    }

    fn hash_name(&self) -> u32 {
        Self::hash("WheelRigEntityDescriptor") // 1156723746
    }
}

/// Wheel rig entity descriptor
#[derive(Copy, Clone, Parsable)]
pub struct WheelRigSteerableEntity {
    /// parent wheel rig entity
    pub block: WheelRigEntity,
    /// Steering tweakables component
    pub tweak_component: WheelRigSteerableTweakableStruct,
}

impl SerializedEntityDescriptor for WheelRigSteerableEntity {
    fn serialized_components() -> u8 {
        WheelRigEntity::serialized_components() + 1
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        let mut c = self.block.components();
        c.push(&self.tweak_component);
        return c;
    }

    fn components_mut<'a>(&'a mut self) -> Vec<&'a mut dyn SerializedEntityComponent> {
        let mut c = self.block.components_mut();
        c.push(&mut self.tweak_component);
        return c;
    }

    fn hash_name(&self) -> u32 {
        Self::hash("WheelRigSteerableEntityDescriptor") // 1864425618
    }
}

/// Wheel rig settings entity component.
#[derive(Copy, Clone, Parsable)]
pub struct WheelRigTweakableStruct  {
    /// Brake force (percent?)
    pub braking_strength: f32,
}

impl SerializedEntityComponent for WheelRigTweakableStruct {}

/// Steering wheel rig settings entity component.
#[derive(Copy, Clone, Parsable)]
pub struct WheelRigSteerableTweakableStruct  {
    /// Wheel steering angle (max?)
    pub steer_angle: f32,
}

impl SerializedEntityComponent for WheelRigSteerableTweakableStruct {}
