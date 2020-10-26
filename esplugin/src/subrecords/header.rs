use crate::file::read::{EspReader, Readable};
use num_derive::FromPrimitive;
use std::io;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
pub enum SubrecordType {
    HEDR = 0x48454452,
    Unknown,
}

impl From<u32> for SubrecordType {
    fn from(num: u32) -> Self {
        num::FromPrimitive::from_u32(num).unwrap_or(Self::Unknown)
    }
}

#[derive(Debug)]
pub struct SubrecordHeader {
    pub size: u16,
}

impl Readable for SubrecordHeader {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            size: reader.read_u16()?,
        })
    }
}
