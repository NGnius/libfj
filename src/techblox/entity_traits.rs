use std::io::{Read, Write};

/// Standard trait for parsing Techblox game save data.
pub trait Parsable {
    /// Process information from raw data.
    fn parse(reader: &mut dyn Read) -> std::io::Result<Self> where Self: Sized;
    /// Convert struct data back into raw bytes
    fn dump(&self, writer: &mut dyn Write) -> std::io::Result<usize>;
}

/// Entity descriptor containing serialized components.
pub trait SerializedEntityDescriptor: Parsable {
    /// Count of entity components that this descriptor contains
    fn serialized_components() -> u8 where Self: Sized;
    /// Components that this entity is comprised of
    fn components<'a>(&'a self) -> Vec<&'a dyn SerializedEntityComponent>;
    /// Components that this entity is comprised of, for modification
    fn components_mut<'a>(&'a mut self) -> Vec<&'a mut dyn SerializedEntityComponent>;
    /// Hash of descriptor name
    fn hash_name(&self) -> u32;
    /// Hash of descriptor name
    fn hash(s: &str) -> u32 where Self: Sized {
        crate::techblox::hashname(s)
    }
}

/// Serializable entity component.
/// Components are the atomic unit of entities.
pub trait SerializedEntityComponent: Parsable {
    /// Raw size of struct, in bytes.
    fn size() -> usize where Self: Sized {
        std::mem::size_of::<Self>()
    }
}
