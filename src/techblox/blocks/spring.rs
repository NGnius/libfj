use std::convert::AsRef;

use crate::techblox::{SerializedEntityDescriptor, Parsable, SerializedEntityComponent,
blocks::{BlockEntity, Block}};
use libfj_parsable_macro_derive::*;

/// Damped angular spring entity descriptor
#[derive(Copy, Clone, Parsable)]
pub struct DampedAngularSpringEntity {
    /// parent block entity
    pub block: BlockEntity,
    /// Joint tweakables component
    pub tweak_component: TweakableJointDampingComponent,
    /// Spring tweakables component
    pub spring_component: DampedAngularSpringROStruct,
}

impl SerializedEntityDescriptor for DampedAngularSpringEntity {
    fn serialized_components() -> u8 {
        BlockEntity::serialized_components() + 2
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        let mut c = self.block.components();
        c.push(&self.tweak_component);
        c.push(&self.spring_component);
        return c;
    }

    fn components_mut<'a>(&'a mut self) -> Vec<&'a mut dyn SerializedEntityComponent> {
        let mut c = self.block.components_mut();
        c.push(&mut self.tweak_component);
        c.push(&mut self.spring_component);
        return c;
    }

    fn hash_name(&self) -> u32 {
        Self::hash("DampedAngularSpringEntityDescriptorV4") // 3789998433
    }
}

impl AsRef<BlockEntity> for DampedAngularSpringEntity {
    fn as_ref(&self) -> &BlockEntity {
        &self.block
    }
}

impl Block for DampedAngularSpringEntity {}

/// Damped spring entity descriptor
#[derive(Copy, Clone, Parsable)]
pub struct DampedSpringEntity {
    /// parent block entity
    pub block: BlockEntity,
    /// Joint tweakables component
    pub tweak_component: TweakableJointDampingComponent,
    /// Spring tweakables component
    pub spring_component: DampedSpringROStruct,
}

impl SerializedEntityDescriptor for DampedSpringEntity {
    fn serialized_components() -> u8 {
        BlockEntity::serialized_components() + 2
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        let mut c = self.block.components();
        c.push(&self.tweak_component);
        c.push(&self.spring_component);
        return c;
    }

    fn components_mut<'a>(&'a mut self) -> Vec<&'a mut dyn SerializedEntityComponent> {
        let mut c = self.block.components_mut();
        c.push(&mut self.tweak_component);
        c.push(&mut self.spring_component);
        return c;
    }

    fn hash_name(&self) -> u32 {
        Self::hash("DampedSpringEntityDescriptorV5") // 2892049599
    }
}

impl AsRef<BlockEntity> for DampedSpringEntity {
    fn as_ref(&self) -> &BlockEntity {
        &self.block
    }
}

impl Block for DampedSpringEntity {}

/// Joint settings entity component.
#[derive(Copy, Clone, Parsable)]
pub struct TweakableJointDampingComponent  {
    /// Joint stiffness (percent?)
    pub stiffness: f32,
    /// Force damping (percent?)
    pub damping: f32,
}

impl SerializedEntityComponent for TweakableJointDampingComponent {}

/// Damped angular spring settings entity component.
#[derive(Copy, Clone, Parsable)]
pub struct DampedSpringROStruct  {
    /// Maximum spring extension
    pub max_extension: f32,
}

impl SerializedEntityComponent for DampedSpringROStruct {}

/// Damped angular spring settings entity component.
#[derive(Copy, Clone, Parsable)]
pub struct DampedAngularSpringROStruct  {
    /// Minimum sprint extension
    pub joint_min: f32,
    /// Maximum sprint extension
    pub joint_max: f32,
}

impl SerializedEntityComponent for DampedAngularSpringROStruct {}
