use chrono::{naive::NaiveDate, Datelike};
use std::io::{Read, Write};

use crate::techblox::{EntityHeader, BlockGroupEntity, parse_i64, parse_u32, Parsable, SerializedEntityDescriptor,
SerializedFlyCamEntity, SerializedPhysicsCameraEntity};
use crate::techblox::blocks::{lookup_hashname, SerializedWireEntity, SerializedGlobalWireSettingsEntity};

/// A collection of cubes and other data from a GameSave.techblox file
//#[derive(Clone)]
pub struct GameSave {
    /// Game version that this save was created by.
    /// This may affect how the rest of the save file was parsed.
    pub version: NaiveDate,

    /// Time when file was saved, corresponding to ticks since 0 AD
    /// https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?view=netframework-4.7.2
    /// Not used for deserialization so not required to be sensible.
    pub ticks: i64,

    /// Amount of cubes present in the save data, as claimed by the file header.
    pub cube_len: u32,

    /// Maximum block entity identifier in the game save.
    /// Not used for deserialization so not required to be correct.
    pub max_entity_id: u32,

    /// Amount of block groups, as claimed by the file header.
    pub group_len: u32,

    /// Entity group descriptors for block group entities.
    pub group_headers: Vec<EntityHeader>,

    /// Block group entities.
    pub cube_groups: Vec<BlockGroupEntity>,

    /// Entity group descriptors for block entities.
    pub cube_headers: Vec<EntityHeader>,

    /// Blocks
    pub cube_entities: Vec<Box<dyn SerializedEntityDescriptor>>,

    /// Amount of wires in the save data, as claimed by the file.
    pub wire_len: u32,

    /// Entity group descriptor for wire entities.
    pub wire_headers: Vec<EntityHeader>,

    /// Wires
    pub wire_entities: Vec<SerializedWireEntity>,

    /// Entity group descriptor for wire settings
    pub wire_settings_header: EntityHeader,

    /// Wire settings
    pub wire_settings_entity: SerializedGlobalWireSettingsEntity,

    /// Entity group descriptor for player fly camera
    pub flycam_header: EntityHeader,

    /// Player edit mode fly camera
    pub flycam_entity: SerializedFlyCamEntity,

    /// Entity group descriptor for player simulation mode camera
    pub phycam_header: EntityHeader,

    /// Player simulation mode camera
    pub phycam_entity: SerializedPhysicsCameraEntity,
}

impl Parsable for GameSave {
    /// Process a Techblox save file from raw bytes.
    fn parse(data: &mut dyn Read) -> std::io::Result<Self> {
        // parse version
        let year = parse_u32(data)?; // parsed as i32 in-game for some reason
        let month = parse_u32(data)?;
        let day = parse_u32(data)?;
        let date = NaiveDate::from_ymd(year as i32, month, day);
        let ticks = parse_i64(data)?; // unused
        let cube_count = parse_u32(data)?; // parsed as i32 in-game for some reason
        let max_e_id = parse_u32(data)?; // unused
        let group_count = parse_u32(data)?; // parsed as i32 in-game for some reason
        // parse block groups
        let mut groups_h = Vec::<EntityHeader>::with_capacity(group_count as usize);
        let mut groups_e = Vec::<BlockGroupEntity>::with_capacity(group_count as usize);
        for _i in 0..group_count {
            groups_h.push(EntityHeader::parse(data)?);
            groups_e.push(BlockGroupEntity::parse(data)?);
        }

        // parse cube data
        let mut cubes_h = Vec::<EntityHeader>::with_capacity(cube_count as usize);
        let mut cubes_e = Vec::<Box<dyn SerializedEntityDescriptor>>::with_capacity(cube_count as usize);
        for _i in 0..cube_count {
            let header = EntityHeader::parse(data)?;
            let hash = header.hash;
            cubes_h.push(header);
            cubes_e.push(lookup_hashname(hash, data)?);
        }

        // parse wire data
        let wire_count = parse_u32(data)?;
        let mut wires_h = Vec::<EntityHeader>::with_capacity(wire_count as usize);
        let mut wires_e = Vec::<SerializedWireEntity>::with_capacity(wire_count as usize);
        for _i in 0..wire_count {
            wires_h.push(EntityHeader::parse(data)?);
            wires_e.push(SerializedWireEntity::parse(data)?);
        }

        // parse global wire settings
        let wire_settings_h = EntityHeader::parse(data)?;
        let wire_settings_e = SerializedGlobalWireSettingsEntity::parse(data)?;

        // parse player cameras
        let flycam_h = EntityHeader::parse(data)?;
        let flycam_e = SerializedFlyCamEntity::parse(data)?;

        let phycam_h = EntityHeader::parse(data)?;
        let phycam_e = SerializedPhysicsCameraEntity::parse(data)?;

        // build struct
        Ok(Self {
            version: date,
            ticks: ticks,
            cube_len: cube_count,
            max_entity_id: max_e_id,
            group_len: group_count,
            group_headers: groups_h,
            cube_groups: groups_e,
            cube_headers: cubes_h,
            cube_entities: cubes_e,
            wire_len: wire_count,
            wire_headers: wires_h,
            wire_entities: wires_e,
            wire_settings_header: wire_settings_h,
            wire_settings_entity: wire_settings_e,
            flycam_header: flycam_h,
            flycam_entity: flycam_e,
            phycam_header: phycam_h,
            phycam_entity: phycam_e,
        })
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        let mut write_count: usize = 0;
        // version
        write_count += self.version.year().dump(writer)?;
        write_count += self.version.month().dump(writer)?;
        write_count += self.version.day().dump(writer)?;
        // unused separator \/
        write_count += self.ticks.dump(writer)?;
        write_count += self.cube_len.dump(writer)?;
        // unused separator \/
        write_count += self.max_entity_id.dump(writer)?;
        write_count += self.group_len.dump(writer)?;

        // dump block groups
        for i in 0..self.group_len as usize {
            write_count += self.group_headers[i].dump(writer)?;
            write_count += self.cube_groups[i].dump(writer)?;
        }

        // dump cube data
        for i in 0..self.cube_len as usize {
            write_count += self.cube_headers[i].dump(writer)?;
            write_count += self.cube_entities[i].dump(writer)?;
        }

        // dump wire data
        write_count += self.wire_len.dump(writer)?;
        for i in 0..self.wire_len as usize {
            write_count += self.wire_headers[i].dump(writer)?;
            write_count += self.wire_entities[i].dump(writer)?;
        }

        // dump global wire settings
        write_count += self.wire_settings_header.dump(writer)?;
        write_count += self.wire_settings_entity.dump(writer)?;

        // dump player cameras
        write_count += self.flycam_header.dump(writer)?;
        write_count += self.flycam_entity.dump(writer)?;

        write_count += self.phycam_header.dump(writer)?;
        write_count += self.phycam_entity.dump(writer)?;
        Ok(write_count)
    }
}

impl std::string::ToString for GameSave {
    fn to_string(&self) -> String {
        format!("{}g {}c {}w (v{})", self.group_len, self.cube_len, self.wire_len, self.version)
    }
}
