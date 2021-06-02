use chrono::{naive::NaiveDate, Datelike};
use std::io::{Read, Write};

use crate::techblox::{EntityHeader, BlockGroupEntity, parse_i64, parse_u32, Parsable, SerializedEntityDescriptor};
use crate::techblox::blocks::lookup_hashname;

/// A collection of cubes and other data from a GameSave.techblox file
//#[derive(Clone)]
pub struct GameSave {
    /// Game version that this save was created by.
    /// This may affect how the rest of the save file was parsed.
    pub version: NaiveDate,

    /// Unused magic value in file header.
    pub magic1: i64,

    /// Amount of cubes present in the save data, as claimed by the file header.
    pub cube_len: u32,

    /// Unused magic value in file header.
    pub magic2: u32,

    /// Amount of block groups, as claimed by the file header.
    pub group_len: u32,

    /// Entity group descriptors for block group entities.
    pub group_headers: Vec<EntityHeader>,

    /// Block group entities.
    pub cube_groups: Vec<BlockGroupEntity>,

    /// Entity group descriptors for block entities.
    pub cube_headers: Vec<EntityHeader>,

    /// Blocks
    pub cube_entities: Vec<Box<dyn SerializedEntityDescriptor>>
}

impl Parsable for GameSave {
    /// Process a Techblox save file from raw bytes.
    fn parse(data: &mut dyn Read) -> std::io::Result<Self> {
        // parse version
        let year = parse_u32(data)?; // parsed as i32 in-game for some reason
        let month = parse_u32(data)?;
        let day = parse_u32(data)?;
        let date = NaiveDate::from_ymd(year as i32, month, day);
        let magic_val1 = parse_i64(data)?; // unused
        let cube_count = parse_u32(data)?; // parsed as i32 in-game for some reason
        let magic_val2 = parse_u32(data)?; // unused
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
        // TODO
        Ok(Self {
            version: date,
            magic1: magic_val1,
            cube_len: cube_count,
            magic2: magic_val2,
            group_len: group_count,
            group_headers: groups_h,
            cube_groups: groups_e,
            cube_headers: cubes_h,
            cube_entities: cubes_e,
        })
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        let mut write_count: usize = 0;
        // version
        write_count += self.version.year().dump(writer)?;
        write_count += self.version.month().dump(writer)?;
        write_count += self.version.day().dump(writer)?;
        // magic separator
        write_count += self.magic1.dump(writer)?;
        write_count += self.cube_len.dump(writer)?;
        // magic separator
        write_count += self.magic2.dump(writer)?;
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
        // TODO
        Ok(write_count)
    }
}

impl std::string::ToString for GameSave {
    fn to_string(&self) -> String {
        format!("{}g {}c (v{})", self.group_len, self.cube_len, self.version)
    }
}
