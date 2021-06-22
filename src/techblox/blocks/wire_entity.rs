use crate::techblox::{SerializedEntityComponent, SerializedEntityDescriptor, Parsable};

use libfj_parsable_macro_derive::*;

/// Wire save data
#[derive(Copy, Clone, Parsable)]
pub struct SerializedWireEntity {
    /// Wiring save data component
    pub save_data_component: WireSaveDataStruct,
}

impl SerializedEntityDescriptor for SerializedWireEntity {
    fn serialized_components() -> u8 {
        1
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        vec![&self.save_data_component]
    }
}

/// Wire connection information that is saved.
#[derive(Copy, Clone, Parsable)]
pub struct WireSaveDataStruct {
    /// Wire source block index in save
    pub source_block_index: u32,
    /// Wire destination block index in save
    pub destination_block_index: u32,
    /// Wire source port index
    pub source_port_usage: u8,
    /// Wire destination port index
    pub destination_port_usage: u8,
}

impl SerializedEntityComponent for WireSaveDataStruct {}

/// Wire settings data for a game
#[derive(Copy, Clone, Parsable)]
pub struct SerializedGlobalWireSettingsEntity {
    /// Global wire settings
    pub settings_component: GlobalWireSettingsEntityStruct,
}

impl SerializedEntityDescriptor for SerializedGlobalWireSettingsEntity {
    fn serialized_components() -> u8 {
        1
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        vec![&self.settings_component]
    }
}

/// Wire settings applied to the whole game save
#[derive(Copy, Clone, Parsable)]
pub struct GlobalWireSettingsEntityStruct {
    /// Is using obsolete wiring system? (bool)
    pub obsolete: u8,
}

impl SerializedEntityComponent for GlobalWireSettingsEntityStruct {}
