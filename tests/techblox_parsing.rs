#[cfg(feature = "techblox")]
use libfj::techblox;
#[cfg(feature = "techblox")]
use libfj::techblox::{SerializedEntityDescriptor, Parsable, blocks, EntityHeader};
#[cfg(feature = "techblox")]
use std::io::{Read, Seek};
#[cfg(feature = "techblox")]
use std::fs::{File, OpenOptions};

#[cfg(feature = "techblox")]
const GAMESAVE_PATH: &str = "tests/GameSave.Techblox";
#[cfg(feature = "techblox")]
const GAMESAVE_PATH2: &str = "tests/GameSave2.Techblox";

#[cfg(feature = "techblox")]
const HASHNAMES: &[&str] = &[
    "StandardBlockEntityDescriptorV4",
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
    }

    //println!("Parsed wire settings hash: {} obsolete? {}", gs.wire_settings_header.hash, gs.wire_settings_entity.settings_component.obsolete != 0);
    assert_eq!(gs.wire_settings_header.hash, EntityHeader::from_name("GlobalWireSettingsEntityDescriptor", 0, 0, 0).hash);

    //println!("Parsed Flycam hash: {}", gs.flycam_header.hash);
    assert_eq!(gs.flycam_header.hash, EntityHeader::from_name("FlyCamEntityDescriptorV0", 0, 0, 0).hash);

    //println!("Parsed Phycam hash: {}", gs.phycam_header.hash);
    assert_eq!(gs.phycam_header.hash, EntityHeader::from_name("CharacterCameraEntityDescriptorV1", 0, 0, 0).hash);
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
    let mut in_file = File::open(GAMESAVE_PATH).map_err(|_| ())?;
    let mut buf = Vec::new();
    in_file.read_to_end(&mut buf).map_err(|_| ())?;
    let gs = techblox::GameSave::parse(&mut buf.as_slice()).map_err(|_| ())?;
    let mut out_file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(GAMESAVE_PATH2)
                    .map_err(|_| ())?;
    gs.dump(&mut out_file).map_err(|_| ())?;
    assert_eq!(in_file.stream_position().unwrap(), out_file.stream_position().unwrap());
    Ok(())
}
