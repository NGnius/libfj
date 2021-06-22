use std::io::{Read, Write};
use crate::techblox::Parsable;
use half::f16;

// reading

pub fn parse_header_u32(reader: &mut dyn Read) -> std::io::Result<u32> {
    // this is possibly wrong
    let mut u32_buf = [0; 4];
    //u32_buf[3] = parse_u8(reader)?;
    //u32_buf[2] = parse_u8(reader)?;
    //u32_buf[1] = parse_u8(reader)?;
    //u32_buf[0] = parse_u8(reader)?;
    reader.read(&mut u32_buf)?;
    Ok(u32::from_le_bytes(u32_buf))
}

pub fn parse_u8(reader: &mut dyn Read) -> std::io::Result<u8> {
    let mut u8_buf = [0; 1];
    reader.read(&mut u8_buf)?;
    Ok(u8_buf[0])
}

pub fn parse_u32(reader: &mut dyn Read) -> std::io::Result<u32> {
    let mut u32_buf = [0; 4];
    reader.read(&mut u32_buf)?;
    Ok(u32::from_le_bytes(u32_buf))
}

pub fn parse_i32(reader: &mut dyn Read) -> std::io::Result<i32> {
    let mut i32_buf = [0; 4];
    reader.read(&mut i32_buf)?;
    Ok(i32::from_le_bytes(i32_buf))
}

pub fn parse_u64(reader: &mut dyn Read) -> std::io::Result<u64> {
    let mut u64_buf = [0; 8];
    reader.read(&mut u64_buf)?;
    Ok(u64::from_le_bytes(u64_buf))
}

pub fn parse_i64(reader: &mut dyn Read) -> std::io::Result<i64> {
    let mut i64_buf = [0; 8];
    reader.read(&mut i64_buf)?;
    Ok(i64::from_le_bytes(i64_buf))
}

pub fn parse_f32(reader: &mut dyn Read) -> std::io::Result<f32> {
    let mut f32_buf = [0; 4];
    reader.read(&mut f32_buf)?;
    Ok(f32::from_le_bytes(f32_buf))
}

// writing

pub fn dump_u8(data: u8, writer: &mut dyn Write) -> std::io::Result<usize> {
    writer.write(&data.to_le_bytes())
}

pub fn dump_u32(data: u32, writer: &mut dyn Write) -> std::io::Result<usize> {
    writer.write(&data.to_le_bytes())
}

pub fn dump_i32(data: i32, writer: &mut dyn Write) -> std::io::Result<usize> {
    writer.write(&data.to_le_bytes())
}

pub fn dump_u64(data: u64, writer: &mut dyn Write) -> std::io::Result<usize> {
    writer.write(&data.to_le_bytes())
}

pub fn dump_i64(data: i64, writer: &mut dyn Write) -> std::io::Result<usize> {
    writer.write(&data.to_le_bytes())
}

pub fn dump_f32(data: f32, writer: &mut dyn Write) -> std::io::Result<usize> {
    writer.write(&data.to_le_bytes())
}

// trait implementations

impl Parsable for u8 {
    fn parse(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut buf = [0; 1];
        reader.read(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        writer.write(&self.to_le_bytes())
    }
}

impl Parsable for u32 {
    fn parse(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut buf = [0; 4];
        reader.read(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        writer.write(&self.to_le_bytes())
    }
}

impl Parsable for i32 {
    fn parse(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut buf = [0; 4];
        reader.read(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        writer.write(&self.to_le_bytes())
    }
}

impl Parsable for u64 {
    fn parse(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut buf = [0; 8];
        reader.read(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        writer.write(&self.to_le_bytes())
    }
}

impl Parsable for i64 {
    fn parse(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut buf = [0; 8];
        reader.read(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        writer.write(&self.to_le_bytes())
    }
}

impl Parsable for f32 {
    fn parse(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut buf = [0; 4];
        reader.read(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        writer.write(&self.to_le_bytes())
    }
}


impl Parsable for f16 {
    fn parse(reader: &mut dyn Read) -> std::io::Result<Self> {
        let mut buf = [0; 2];
        reader.read(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }

    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        writer.write(&self.to_le_bytes())
    }
}
