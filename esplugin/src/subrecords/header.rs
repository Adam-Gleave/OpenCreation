use crate::file::read::{EspReader, Readable};
use num_derive::FromPrimitive;
use std::io;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
pub enum SubrecordType {
    HEDR = 0x48454452,
    CNAM = 0x434E414D,
    SNAM = 0x534E414D,
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
        let header = Self {
            size: reader.read_u16()?,
        };

        reader.next_subrecord_data(header.size);
        Ok(header)
    }
}
