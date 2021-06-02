#[cfg(feature = "techblox")]
use libfj::techblox;
#[cfg(feature = "techblox")]
use libfj::techblox::{SerializedEntityDescriptor, Parsable, blocks};
#[cfg(feature = "techblox")]
use std::io::Read;
#[cfg(feature = "techblox")]
use std::fs::File;

#[cfg(feature = "techblox")]
const GAMESAVE_PATH: &str = "tests/GameSave.Techblox";

#[cfg(feature = "techblox")]
const HASHNAMES: &[&str] = &[
    "StandardBlockEntityDescriptorV4",
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
