#[cfg(feature = "techblox")]
use libfj::techblox;
#[cfg(feature = "techblox")]
use libfj::techblox::{SerializedEntityDescriptor, Parsable, blocks, EntityHeader};
#[cfg(feature = "techblox")]
use std::io::{Read, Seek};
#[cfg(feature = "techblox")]
use std::fs::{File, OpenOptions};
#[cfg(feature = "techblox")]
use std::convert::AsRef;

#[cfg(feature = "techblox")]
const GAMESAVE_PATH: &str = "tests/GameSave.Techblox";
#[cfg(feature = "techblox")]
const GAMESAVE_PATH_OUT: &str = "tests/GameSave.out.Techblox";
#[cfg(feature = "techblox")]
const GAMESAVE_PATH_ALL: &str = "tests/All.Techblox";
#[cfg(feature = "techblox")]
const GAMESAVE_PATH_ALL_OUT: &str = "tests/All.out.Techblox";

#[cfg(feature = "techblox")]
const HASHNAMES: &[&str] = &[
    "BlockGroupEntityDescriptorV0",

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

    "WireEntityDescriptorMock",
    "GlobalWireSettingsEntityDescriptor",
    "FlyCamEntityDescriptorV0",
    "CharacterCameraEntityDescriptorV1",
];

#[cfg(feature = "techblox")]
#[test]
fn techblox_gamesave_parse() -> Result<(), ()> {
    let mut f = File::open(GAMESAVE_PATH).map_err(|_| ())?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).map_err(|_| ())?;
    let gs = techblox::GameSave::parse(&mut buf.as_slice()).map_err(|_| ())?;
    for i in 1..(gs.group_len as usize) {
        assert_eq!(gs.group_headers[i-1].hash, gs.group_headers[i].hash);
        //println!("#{} count {} vs {}", i, gs.group_headers[i-1].component_count, gs.group_headers[i].component_count);
        assert_eq!(gs.group_headers[i-1].component_count, gs.group_headers[i].component_count);
    }
    for i in 0..(gs.group_len as usize) {
        assert_eq!(gs.group_headers[i].component_count, techblox::BlockGroupEntity::serialized_components());
        assert_eq!(gs.group_headers[i].hash, gs.cube_groups[i].hash_name());
    }
    for i in 1..(gs.cube_len as usize) {
        //assert_eq!(gs.cube_headers[i-1].hash, gs.cube_headers[i].hash);
        //println!("#{} count {} vs {}", i, gs.cube_headers[i-1].component_count, gs.cube_headers[i].component_count);
        if gs.cube_headers[i-1].hash == gs.cube_headers[i].hash {
            assert_eq!(gs.group_headers[i-1].component_count, gs.group_headers[i].component_count);
        }
    }
    for i in 0..(gs.cube_len as usize) {
        assert!(gs.cube_headers[i].component_count >= blocks::BlockEntity::serialized_components());
        //println!("#{} components: {}", i, gs.cube_headers[i].component_count);
        assert_eq!(gs.cube_headers[i].hash, gs.cube_entities[i].hash_name());
    }

    //println!("Parsed wire settings hash: {} obsolete? {}", gs.wire_settings_header.hash, gs.wire_settings_entity.settings_component.obsolete != 0);
    assert_eq!(gs.wire_settings_header.hash, EntityHeader::from_name("GlobalWireSettingsEntityDescriptor", 0, 0, 0).hash);
    assert_eq!(gs.wire_settings_header.hash, gs.wire_settings_entity.hash_name());

    //println!("Parsed Flycam hash: {}", gs.flycam_header.hash);
    assert_eq!(gs.flycam_header.hash, EntityHeader::from_name("FlyCamEntityDescriptorV0", 0, 0, 0).hash);
    assert_eq!(gs.flycam_header.hash, gs.flycam_entity.hash_name());

    //println!("Parsed Phycam hash: {}", gs.phycam_header.hash);
    assert_eq!(gs.phycam_header.hash, EntityHeader::from_name("CharacterCameraEntityDescriptorV1", 0, 0, 0).hash);
    assert_eq!(gs.phycam_header.hash, gs.phycam_entity.hash_name());

    println!("{}", gs.to_string());
    Ok(())
}

#[allow(dead_code)]
#[cfg(feature = "techblox")]
//#[test]
fn techblox_gamesave_brute_force() -> Result<(), ()> {
    // this is slow and not very important, so it's probably better to not test this
    let mut f = File::open(GAMESAVE_PATH).map_err(|_| ())?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).map_err(|_| ())?;
    let gs = techblox::GameSave::parse(&mut buf.as_slice()).map_err(|_| ())?;
    println!("murmurhash3: {} -> {}", gs.group_headers[0].guess_name(), gs.group_headers[0].hash);
    Ok(())
}

#[cfg(feature = "techblox")]
#[test]
fn hash_tb_name() {
    for name in HASHNAMES {
        println!("MurmurHash3: {} -> {}", name, crate::techblox::EntityHeader::from_name(name, 0, 0, 0).hash);
    }
}

#[cfg(feature = "techblox")]
#[test]
fn techblox_gamesave_perfect_parse() -> Result<(), ()> {
    let mut in_file = File::open(GAMESAVE_PATH_ALL).map_err(|_| ())?;
    let mut buf = Vec::new();
    in_file.read_to_end(&mut buf).map_err(|_| ())?;
    let gs = techblox::GameSave::parse(&mut buf.as_slice()).map_err(|_| ())?;
    let mut out_file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(GAMESAVE_PATH_OUT)
                    .map_err(|_| ())?;
    gs.dump(&mut out_file).map_err(|_| ())?;
    assert_eq!(in_file.stream_position().unwrap(), out_file.stream_position().unwrap());
    Ok(())
}

#[cfg(feature = "techblox")]
#[test]
fn techblox_gamesave_parse_all() -> Result<(), ()> {
    let mut in_file = File::open(GAMESAVE_PATH_ALL).map_err(|_| ())?;
    let mut buf = Vec::new();
    in_file.read_to_end(&mut buf).map_err(|_| ())?;
    let gs = techblox::GameSave::parse(&mut buf.as_slice()).map_err(|_| ())?;

    // verify
    for i in 1..(gs.group_len as usize) {
        assert_eq!(gs.group_headers[i-1].hash, gs.group_headers[i].hash);
        //println!("#{} count {} vs {}", i, gs.group_headers[i-1].component_count, gs.group_headers[i].component_count);
        assert_eq!(gs.group_headers[i-1].component_count, gs.group_headers[i].component_count);
    }
    for i in 0..(gs.group_len as usize) {
        assert_eq!(gs.group_headers[i].component_count, techblox::BlockGroupEntity::serialized_components());
        assert_eq!(gs.group_headers[i].hash, gs.cube_groups[i].hash_name());
        /*let pos = format!("({}, {}, {})", gs.cube_groups[i].block_group_transform.block_group_grid_position.x, gs.cube_groups[i].block_group_transform.block_group_grid_position.y, gs.cube_groups[i].block_group_transform.block_group_grid_position.z);
        let rot = format!("({}, {}, {}, {})", gs.cube_groups[i].block_group_transform.block_group_grid_rotation.value.x, gs.cube_groups[i].block_group_transform.block_group_grid_rotation.value.y, gs.cube_groups[i].block_group_transform.block_group_grid_rotation.value.z,
        gs.cube_groups[i].block_group_transform.block_group_grid_rotation.value.w);
        println!("block id: {}, position: {}, rotation: {}", gs.cube_groups[i].saved_block_group_id.saved_block_group_id, pos, rot);*/
    }
    for i in 1..(gs.cube_len as usize) {
        //assert_eq!(gs.cube_headers[i-1].hash, gs.cube_headers[i].hash);
        //println!("#{} count {} vs {}", i, gs.cube_headers[i-1].component_count, gs.cube_headers[i].component_count);
        if gs.cube_headers[i-1].hash == gs.cube_headers[i].hash {
            assert_eq!(gs.group_headers[i-1].component_count, gs.group_headers[i].component_count);
        }
    }
    for i in 0..(gs.cube_len as usize) {
        assert!(gs.cube_headers[i].component_count >= blocks::BlockEntity::serialized_components());
        //println!("#{} components: {}", i, gs.cube_headers[i].component_count);
        assert_eq!(gs.cube_headers[i].hash, gs.cube_entities[i].hash_name());
    }

    //println!("Parsed wire settings hash: {} obsolete? {}", gs.wire_settings_header.hash, gs.wire_settings_entity.settings_component.obsolete != 0);
    assert_eq!(gs.wire_settings_header.hash, EntityHeader::from_name("GlobalWireSettingsEntityDescriptor", 0, 0, 0).hash);
    assert_eq!(gs.wire_settings_header.hash, gs.wire_settings_entity.hash_name());

    //println!("Parsed Flycam hash: {}", gs.flycam_header.hash);
    assert_eq!(gs.flycam_header.hash, EntityHeader::from_name("FlyCamEntityDescriptorV0", 0, 0, 0).hash);
    assert_eq!(gs.flycam_header.hash, gs.flycam_entity.hash_name());

    //println!("Parsed Phycam hash: {}", gs.phycam_header.hash);
    assert_eq!(gs.phycam_header.hash, EntityHeader::from_name("CharacterCameraEntityDescriptorV1", 0, 0, 0).hash);
    assert_eq!(gs.phycam_header.hash, gs.phycam_entity.hash_name());

    // write out
    let mut out_file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(GAMESAVE_PATH_ALL_OUT)
                    .map_err(|_| ())?;
    gs.dump(&mut out_file).map_err(|_| ())?;
    assert_eq!(in_file.stream_position().unwrap(), out_file.stream_position().unwrap());
    Ok(())
}

#[cfg(feature = "techblox")]
#[test]
fn techblox_gamesave_block_groups() -> Result<(), ()> {
    let mut in_file = File::open(GAMESAVE_PATH_ALL).map_err(|_| ())?;
    let mut buf = Vec::new();
    in_file.read_to_end(&mut buf).map_err(|_| ())?;
    let gs = techblox::GameSave::parse(&mut buf.as_slice()).map_err(|_| ())?;

    for block_trait in &gs.cube_entities {
        let block: &blocks::BlockEntity = block_trait.as_ref().as_ref();
        //println!("Block @ ({}, {}, {})", block.pos_component.position.x, block.pos_component.position.y, block.pos_component.position.z);
        assert!(is_in_block_groups(block.group_component.current_block_group, &gs.cube_groups));
    }
    Ok(())
}

#[cfg(feature = "techblox")]
fn is_in_block_groups(id: i32, block_groups: &Vec<techblox::BlockGroupEntity>) -> bool {
    for bg in block_groups {
        if bg.saved_block_group_id.saved_block_group_id == id {
            return true;
        }
    }
    false
}
