use crate::file::read::{EspReader, Readable};
use esplugin_derive::*;
use num_derive::FromPrimitive;

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

#[derive(Debug, Readable)]
pub struct VersionControlInfo {
    pub vc_day: u8,
    pub vc_month: u8,
    pub vc_previous_id: u8,
    pub vc_current_id: u8,
}

#[derive(Debug, Readable)]
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
