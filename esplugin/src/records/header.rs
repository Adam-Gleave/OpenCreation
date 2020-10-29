use crate::file::read::{EspReader, Readable};
use crate::file::types::VersionControlInfo;
use esplugin_derive::*;

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
