use crate::file::read::{EspReader, Readable};
use crate::file::types::VersionControlInfo;
use esplugin_derive::*;
use num_derive::FromPrimitive;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
pub enum RecordType {
    Keyword = 0x4B595744,     // "KYWD"
    Unknown,
}

impl From<u32> for RecordType {
    fn from(num: u32) -> Self {
        num::FromPrimitive::from_u32(num).unwrap_or(Self::Unknown)
    }
}

#[derive(Debug, Default, Readable)]
pub struct RecordHeader<F>
where
    F: Readable,
{
    pub size: u32,
    pub flags: F,
    pub id: u32,
    pub vc_info: VersionControlInfo,
    pub version: u16,
    pub unknown: u16,
}
