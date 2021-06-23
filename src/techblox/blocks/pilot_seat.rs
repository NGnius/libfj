use crate::techblox::{SerializedEntityDescriptor, Parsable, SerializedEntityComponent, blocks::BlockEntity};
use libfj_parsable_macro_derive::*;

/// Pilot seat entity descriptor (V4)
#[derive(Copy, Clone, Parsable)]
pub struct PilotSeatEntity {
    /// parent block entity
    pub block: BlockEntity,
    /// Seat following camera component
    pub cam_component: SeatFollowCamComponent,
}

impl SerializedEntityDescriptor for PilotSeatEntity {
    fn serialized_components() -> u8 {
        BlockEntity::serialized_components() + 1
    }

    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent> {
        let mut c = self.block.components();
        c.push(&self.cam_component);
        return c;
    }

    fn components_mut<'a>(&'a mut self) -> Vec<&'a mut dyn SerializedEntityComponent> {
        let mut c = self.block.components_mut();
        c.push(&mut self.cam_component);
        return c;
    }

    fn hash_name(&self) -> u32 {
        Self::hash("PilotSeatEntityDescriptorV4") // 2281299333
    }
}

/// Seat settings entity component.
#[derive(Copy, Clone, Parsable)]
pub struct SeatFollowCamComponent {
    /// Should the camera follow the seat? (bool)
    pub follow: u8,
}

impl SerializedEntityComponent for SeatFollowCamComponent {}
