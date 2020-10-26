use crate::file::read::{EspReader, Readable};
use num_derive::FromPrimitive;
use std::io;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
pub enum RecordType {
    FileHeader = 0x54455334, // "TES4"
    Unknown,
}

impl From<u32> for RecordType {
    fn from(num: u32) -> Self {
        num::FromPrimitive::from_u32(num).unwrap_or(Self::Unknown)
    }
}

#[derive(Debug)]
pub struct VersionControlInfo {
    pub vc_day: u8,
    pub vc_month: u8,
    pub vc_previous_id: u8,
    pub vc_current_id: u8,
}

impl Readable for VersionControlInfo {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            vc_day: reader.read_u8()?,
            vc_month: reader.read_u8()?,
            vc_previous_id: reader.read_u8()?,
            vc_current_id: reader.read_u8()?,
        })
    }
}

#[derive(Debug)]
pub struct RecordHeader<RecordFlags>
where
    RecordFlags: Readable,
{
    pub size: u32,
    pub flags: RecordFlags,
    pub id: u32,
    pub vc_info: VersionControlInfo,
    pub version: u16,
    pub unknown: u16,
}
