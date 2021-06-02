use std::io::Read;

use crate::techblox::{Parsable, SerializedEntityDescriptor};
#[cfg(debug_assertions)]
use crate::techblox::blocks::*;

pub fn lookup_hashname(hash: u32, data: &mut dyn Read) -> std::io::Result<Box<dyn SerializedEntityDescriptor>> {
    Ok(match hash {
        1357220432 /*StandardBlockEntityDescriptorV4*/ => Box::new(BlockEntity::parse(data)?),
        _ => {
            #[cfg(debug_assertions)]
            println!("Unknown hash ID {}", hash);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Unrecognised hash {}", hash)))
        }
    })
}
