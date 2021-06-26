use std::convert::AsRef;

use crate::techblox::{SerializedEntityDescriptor, Parsable, SerializedEntityComponent,
blocks::{BlockEntity, Block}};
use libfj_parsable_macro_derive::*;

/// Engine entity descriptor
#[derive(Copy, Clone, Parsable)]
pub struct EngineBlockEntity {
    /// parent block entity
    pub block: BlockEntity,
    /// Engine tweakables component
    pub tweak_component: EngineBlockTweakableComponent,
}

impl SerializedEntityDescriptor for EngineBlockEntity {
    fn serialized_components() -> u8 {
        BlockEntity::serialized_components() + 1
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
        Self::hash("EngineBlockEntityDescriptor") // 1757314505
    }
}

impl AsRef<BlockEntity> for EngineBlockEntity {
    fn as_ref(&self) -> &BlockEntity {
        &self.block
    }
}

impl Block for EngineBlockEntity {}

/// Engine settings entity component.
#[derive(Copy, Clone, Parsable)]
pub struct EngineBlockTweakableComponent  {
    /// Engine power (percent?)
    pub power: f32,
    /// Is the engine's transmission automatic? (bool)
    pub automatic_gears: u32, // why is this not stored as u8 like the other bools?
}

impl SerializedEntityComponent for EngineBlockTweakableComponent  {}
