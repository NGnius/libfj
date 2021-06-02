use crate::techblox::{hashname, brute_force, Parsable};
use libfj_parsable_macro_derive::*;

/// An entity's header information.
///
/// This holds entity data common to all entities, such as entity type and ID.
#[derive(Clone, Copy, Parsable)]
pub struct EntityHeader {
    /// Entity type hash
    pub hash: u32,
    /// Entity identifier
    pub entity_id: u32,
    /// Entity group identifier
    pub group_id: u32,
    /// Count of serialized components after this header (this is not the size in bytes)
    pub component_count: u8,
}

impl EntityHeader {
    /// Guess the original name from the hashed value by brute-force.
    ///
    /// This is slow and cannot guarantee a correct result. Use is discouraged.
    pub fn guess_name(&self) -> String {
        brute_force(self.hash)
    }

    /// Create an entity header using the hash of `name`.
    pub fn from_name(name: &str, entity_id: u32, group_id: u32, component_count: u8) -> Self {
        Self {
            hash: hashname(name),
            entity_id,
            group_id,
            component_count,
        }
    }
}

impl std::convert::Into<EntityGroupID> for EntityHeader {
    fn into(self) -> EntityGroupID {
        EntityGroupID {
            entity_id: self.entity_id,
            group_id: self.group_id,
        }
    }
}

/// Entity identifier common among all components in the same entity
#[derive(Clone, Copy, Parsable)]
pub struct EntityGroupID {
    /// Entity identifier
    pub entity_id: u32,
    /// Entity group identifier
    pub group_id: u32
}
