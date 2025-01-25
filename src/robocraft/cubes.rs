use std::io::Read;
use base64::Engine;

// TODO(maybe) parse iteratively instead of one-shot

/// A collection of cube data
///
/// This holds all data parsed from cube_data and colour_data. 
/// Individual Cube structs can be iterated through.
#[derive(Clone)]
pub struct Cubes {
    /// Parsed cube count (the first 32 bits of data parsed to `u32`)
    pub provided_len: u32,
    cubes: Vec<Cube>,
}

impl Cubes {
    /// Process the raw bytes containing block data from a Robocraft CRF bot
    ///
    /// `cube_data` and `colour_data` correspond to the `cube_data` and `colour_data` fields of FactoryRobotGetInfo.
    /// In general, you should use `Cubes::from<FactoryRobotGetInfo>(data)` instead of this lower-level function.
    pub fn parse(cube_data: &mut Vec<u8>, colour_data: &mut Vec<u8>) -> Result<Self, ()> {
        // read first 4 bytes (cube count) from both arrays and make sure they match
        let mut cube_buf = [0; 4];
        let mut colour_buf = [0; 4];
        let mut cube_slice = cube_data.as_slice();
        let mut colour_slice = colour_data.as_slice();
        if let Ok(len) = cube_slice.read(&mut cube_buf) {
            if len != 4 {
                //println!("Failed reading cube_data len");
                return Err(());
            }
        } else {
            //println!("Failed to read cube_data");
            return Err(());
        }
        if let Ok(len) = colour_slice.read(&mut colour_buf) {
            if len != 4 {
                //println!("Failed reading colour_data len");
                return Err(());
            }
        } else {
            //println!("Failed to read colour_data");
            return Err(());
        }
        if !(cube_buf[0] == colour_buf[0]
            && cube_buf[1] == colour_buf[1]
            && cube_buf[2] == colour_buf[2]
            && cube_buf[3] == colour_buf[3]) {
            //println!("Values do not match");
            return Err(());
        }
        let mut cube_i = 4;
        let mut colour_i = 4;
        let mut parsed_cubes = Vec::with_capacity(cube_data.len() / 8);
        while cube_i < cube_data.len() && colour_i < colour_data.len() {
            let mut new_cube = Cube::default();
            if let Ok(cube_add) = new_cube.parse_cube_data(&mut cube_slice) {
                if let Ok(colour_add) = new_cube.parse_colour_data(&mut colour_slice) {
                    cube_i += cube_add;
                    colour_i += colour_add;
                    parsed_cubes.push(new_cube);
                } else {
                    // colour_data read error
                    return Err(());
                }
            } else {
                // cube_data read error
                return Err(());
            }
        }
        Ok(Self {
            provided_len: u32::from_le_bytes(cube_buf),
            cubes: parsed_cubes,
        })
    }
    
    /// Dump the raw bytes containing block data for a Robocraft bot.
    ///
    /// The first tuple item is cube data, and the second item is colour data.
    /// Use this to write a modified robot to file.
    /// This is the inverse of `Cubes::parse(...)`.
    ///
    /// I'm not sure what this would actually be useful for...
    pub fn dump(&self) -> (Vec<u8>, Vec<u8>) {
        let mut cube_buf = Vec::new();
        let mut colour_buf = Vec::new();
        cube_buf.extend(&self.provided_len.to_le_bytes());
        colour_buf.extend(&self.provided_len.to_le_bytes());
        for c in self.into_iter() {
            cube_buf.extend(&c.dump_cube_data());
            colour_buf.extend(&c.dump_colour_data());
        }
        (cube_buf, colour_buf)
    }
    
    /// Get the actual amount of cubes.
    ///
    /// This differs from `provided_len` by being the amount of cubes parsed (successfully), instead of something parsed from block data.
    /// For any valid robot data, `data.provided_len == data.len()`.
    pub fn len(&self) -> usize {
        self.cubes.len()
    }
}

impl<'a> std::iter::IntoIterator for &'a Cubes {
    type Item = &'a Cube;
    
    type IntoIter = std::slice::Iter<'a, Cube>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.cubes.iter()
    }
}

/// A single block in a Robocraft robot.
///
/// From the front of a Robocraft garage bay, looking at the back, all positions are measured from the back bottom right corner.
#[derive(Copy, Clone)]
pub struct Cube {
    /// The cube id
    pub id: u32,
    /// The cube's x position (left to right)
    pub x: u8, // left to right
    /// The cube's y position (bottom to top)
    pub y: u8, // bottom to top
    /// The cube's z position (back to front)
    pub z: u8, // back to front
    /// The cube's orientation
    pub orientation: u8,
    /// The cube's colour, one of the 24 possible colours in Robocraft
    pub colour: u8,
}

impl Cube {
    fn parse_cube_data(&mut self, reader: &mut dyn Read) -> Result<usize, ()> {
        let mut buf = [0; 4];
        // read cube id
        if let Ok(len) = reader.read(&mut buf) {
            if len != 4 {
                return Err(());
            }
            self.id = u32::from_le_bytes(buf);
        } else {
            return Err(());
        }
        // read x, y, z, orientation
        if let Ok(len) = reader.read(&mut buf) {
            if len != 4 {
                return Err(());
            }
            self.x = buf[0];
            self.y = buf[1];
            self.z = buf[2];
            self.orientation = buf[3];
        } else {
            return Err(());
        }
        Ok(8)
    }
    
    fn parse_colour_data(&mut self, reader: &mut dyn Read) -> Result<usize, ()> {
        let mut buf = [0; 4];
        if let Ok(len) = reader.read(&mut buf) {
            if len != 4 {
                return Err(());
            }
            self.colour = buf[0];
        } else {
            return Err(());
        }
        Ok(4)
    }
    
    /// Dump the raw cube data as used in the Robocraft CRF.
    ///
    /// This is useless by itself, use `Cubes.dump()` for a valid robot.
    pub fn dump_cube_data(&self) -> [u8; 8] {
        let id_buf = self.id.to_le_bytes();
        [id_buf[0], id_buf[1], id_buf[2], id_buf[3], self.x, self.y, self.z, self.orientation]
    }
    
    /// Dump the raw colour data as used in the Robocraft CRF.
    ///
    /// This is useless by itself, use `Cubes.dump()` for a valid robot.
    pub fn dump_colour_data(&self) -> [u8; 4] {
        [self.colour, self.x, self.y, self.z]
    }
}

impl std::default::Default for Cube {
    fn default() -> Self {
        Self {
            id: 0,
            x: 0,
            y: 0,
            z: 0,
            orientation: 0,
            colour: 0,
        }
    }
}

impl std::convert::From<crate::robocraft::FactoryRobotGetInfo> for Cubes {
    fn from(other: crate::robocraft::FactoryRobotGetInfo) -> Self {
        let mut cube_buf = base64::engine::general_purpose::STANDARD.decode(other.cube_data).unwrap();
        let mut colour_buf = base64::engine::general_purpose::STANDARD.decode(other.colour_data).unwrap();
        Self::parse(&mut cube_buf, &mut colour_buf).unwrap()
    }
}

impl std::convert::From<crate::robocraft::FactoryInfo<crate::robocraft::FactoryRobotGetInfo>> for Cubes {
    fn from(other: crate::robocraft::FactoryInfo<crate::robocraft::FactoryRobotGetInfo>) -> Self {
        Self::from(other.response)
    }
}

impl std::string::ToString for Cube {
    fn to_string(&self) -> String {
        format!("{{x: {}, y: {}, z: {}}} ({})", self.x, self.y, self.z, self.id)
    }
}
