use std::io::Read;

use crate::techblox::{Parsable, SerializedEntityDescriptor};
use crate::techblox::blocks::*;

const HASHNAMES: &[&str] = &[
    // Block group info entities
    "BlockGroupEntityDescriptorV0",
    // Block entities
    "StandardBlockEntityDescriptorV4",
    "BatteryEntityDescriptorV4",
    "MotorEntityDescriptorV7",
    "LeverEntityDescriptorV7",
    "ButtonEntityDescriptorV6",
    "JointBlockEntityDescriptorV3",
    "ServoEntityDescriptorV7",
    "PistonEntityDescriptorV6",
    "DampedSpringEntityDescriptorV5",
    "DampedAngularSpringEntityDescriptorV4",
    "SpawnPointEntityDescriptorV6",
    "BuildingSpawnPointEntityDescriptorV4",
    "TriggerEntityDescriptorV6",
    "PilotSeatEntityDescriptorV4",
    "PilotSeatEntityDescriptorV3",
    "TextBlockEntityDescriptorV4",
    "PassengerSeatEntityDescriptorV4",
    "PassengerSeatEntityDescriptorV3",
    "LogicBlockEntityDescriptorV1",
    "TyreEntityDescriptorV1",
    "ObjectIDEntityDescriptorV1",
    "MoverEntityDescriptorV1",
    "RotatorEntityDescriptorV1",
    "DamperEntityDescriptorV1",
    "AdvancedDamperEntityDescriptorV1",
    "CoMEntityDescriptor",
    "FilterBlockEntityDescriptorV1",
    "ConstrainerEntityDescriptorV1",
    "NumberToTextBlockEntityDescriptorV1",
    "CentreHudBlockEntityDescriptorV1",
    "ObjectiveHudBlockEntityDescriptorV1",
    "GameStatsHudBlockEntityDescriptorV1",
    "GameOverHudBlockEntityDescriptorV1",
    "TimerBlockEntityDescriptorV1",
    "BitBlockEntityDescriptorV2",
    "ConstantBlockEntityDescriptor",
    "CounterBlockEntityDescriptorV1",
    "SimpleSfxEntityDescriptorV1",
    "LoopedSfxEntityDescriptorV1",
    "MusicBlockEntityDescriptorV1",
    "ProjectileBlockEntityDescriptorV1",
    "DamagingSurfaceEntityDescriptorV1",
    "DestructionManagerEntityDescriptorV1",
    "ChunkDestructionBlockEntityDescriptorV1",
    "ClusterDestructionBlockEntityDescriptorV1",
    "PickupBlockEntityDescriptorV1",
    "PointLightEntityDescriptorV1",
    "SpotLightEntityDescriptorV1",
    "SunLightEntityDescriptorV1",
    "AmbientLightEntityDescriptorV1",
    "FogEntityDescriptorV1",
    "SkyEntityDescriptorV1",
    "SynchronizedWireBlockEntityDescriptor",
    "WheelRigEntityDescriptor",
    "WheelRigSteerableEntityDescriptor",
    "EngineBlockEntityDescriptor",
    // Other Non-block entities (stored after blocks in game saves)
    "WireEntityDescriptorMock",
    "GlobalWireSettingsEntityDescriptor",
    "FlyCamEntityDescriptorV0",
    "CharacterCameraEntityDescriptorV1",
];

pub fn lookup_hashname(hash: u32, data: &mut dyn Read) -> std::io::Result<Box<dyn SerializedEntityDescriptor>> {
    Ok(match hash {
        1357220432 /*StandardBlockEntityDescriptorV4*/ => Box::new(BlockEntity::parse(data)?),
        2281299333 /*PilotSeatEntityDescriptorV4*/ => Box::new(PilotSeatEntity::parse(data)?),
        1360086092 /*PassengerSeatEntityDescriptorV4*/ => Box::new(PassengerSeatEntity::parse(data)?),
        1757314505 /*EngineBlockEntityDescriptor*/ => Box::new(EngineBlockEntity::parse(data)?),
        3586818581 /*JointBlockEntityDescriptorV3*/ => Box::new(JointBlockEntity::parse(data)?),
        3789998433 /*DampedAngularSpringEntityDescriptorV4*/ => Box::new(DampedAngularSpringEntity::parse(data)?),
        2892049599 /*DampedSpringEntityDescriptorV5*/ => Box::new(DampedSpringEntity::parse(data)?),
        1156723746 /*WheelRigEntityDescriptor*/ => Box::new(WheelRigEntity::parse(data)?),
        1864425618 /*WheelRigSteerableEntityDescriptor*/ => Box::new(WheelRigSteerableEntity::parse(data)?),
        1517625162 /*TyreEntityDescriptorV1*/ => Box::new(TyreEntity::parse(data)?),
        _ => {
            #[cfg(debug_assertions)]
            println!("Unknown hash ID {} (missing entry for {})", hash, lookup_name_by_hash(hash).unwrap_or("<Unknown>"));
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Unrecognised hash {}", hash)))
        }
    })
}

pub fn lookup_name_by_hash(hash: u32) -> Option<&'static str> {
    for name in HASHNAMES {
        if crate::techblox::hashname(name) == hash {
            return Some(name);
        }
    }
    None
}
