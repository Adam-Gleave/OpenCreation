use crate::file::read::{EspReader, Readable, Peekable};
use crate::records::form::Form;
use crate::records::header::RecordHeader;
use bitflags::bitflags;
use byteorder::{ReadBytesExt, BigEndian};
use esplugin_derive::*;
use num_derive::FromPrimitive;
use std::io::{Seek, SeekFrom};
use std::io;

#[derive(Debug, Eq, PartialEq, FromPrimitive, Hash)]
pub enum RecordType {
    Keyword     = 0x4B595744,       // "KYWD"
    GameSetting = 0x474D5354,       // "GMST"
    Unknown,
}

impl From<u32> for RecordType {
    fn from(num: u32) -> Self {
        num::FromPrimitive::from_u32(num).unwrap_or(Self::Unknown)
    }
}

impl Peekable for RecordType {
    fn peek(reader: &mut EspReader, offset: i64) -> io::Result<Self> {
        reader.buf_reader.seek(SeekFrom::Current(offset))?;
        let num = reader.buf_reader.read_u32::<BigEndian>()?;
        let reverse = 0 - (offset + std::mem::size_of::<u32>() as i64);
        reader.buf_reader.seek(SeekFrom::Current(reverse))?;

        Ok(num.into())
    }
}

pub trait Coded {
    fn code() -> RecordType;
}

bitflags! {
    #[derive(Default)]
    pub struct RecordFlags: u32 {
        const DELETED                   = 0x00000020;
        const CONSTANT                  = 0x00000040;
        const MUST_UPDATE_ANIMS         = 0x00000100;
        const QUEST_ITEM                = 0x00000400;
        const INITIALLY_DISABLED        = 0x00000800;
        const IGNORED                   = 0x00001000;
        const VISIBLE_WHEN_DISTANT      = 0x00008000;
        const COMPRESSED                = 0x00040000;
        const CANNOT_WAIT               = 0x00080000;
        const IS_MARKER                 = 0x00800000;
        const NAV_MESH_GEN_FILTER       = 0x04000000;
        const NAV_MESH_GEN_BOUNDING_BOX = 0x08000000;
        const NAV_MESH_GEN_GROUND       = 0x10000000;
    }
}

impl Readable for RecordFlags {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(RecordFlags::from_bits(u32::read(reader)?).unwrap_or(Default::default()))
    }
}

#[derive(Default, Debug, Form, Readable)]
#[record_header(true)]
#[size_var(header, size)]
pub struct Record<HeaderFlags, Data>
where
    HeaderFlags: Readable,
    Data: Readable,
{
    pub header: RecordHeader<HeaderFlags>,
    pub data: Data,
}
