use crate::techblox::{SerializedEntityDescriptor, Parsable, SerializedEntityComponent,
blocks::{BlockEntity, SeatFollowCamComponent}};
use libfj_parsable_macro_derive::*;

/// Passenger seat entity descriptor (V4)
#[derive(Copy, Clone, Parsable)]
pub struct PassengerSeatEntity {
    /// parent block entity
    pub block: BlockEntity,
    /// Seat following camera component
    pub cam_component: SeatFollowCamComponent,
}

impl SerializedEntityDescriptor for PassengerSeatEntity {
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
        Self::hash("PassengerSeatEntityDescriptorV4") // 1360086092
    }
}
