use crate::file::read::{EspReader, Readable};
use crate::file::types::VersionControlInfo;
use esplugin_derive::*;
use num_derive::FromPrimitive;
use std::io;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
enum GroupType {
    Top = 0,
    WorldChildren = 1,
    InteriorCellBlock = 2,
    InteriorCellSubBlock = 3,
    ExteriorCellBlock = 4,
    ExteriorCellSubBlock = 5,
    CellChildren = 6,
    TopicChildren = 7,
    CellPersistentChildren = 8,
    CellTemporaryChildren = 9,
    Unknown = 10,
}

impl From<u32> for GroupType {
    fn from(num: u32) -> Self {
        num::FromPrimitive::from_u32(num).unwrap_or(Self::Unknown)
    }
}

impl Readable for GroupType {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let num = u32::read(reader)?;
        Ok(GroupType::from(num))
    }
}

#[derive(Debug, Default, Readable)]
pub struct GroupHeader {
    pub size: u32,
    pub label: u32,
    pub group_type: u32,
    pub vc_info: VersionControlInfo,
    pub unkwnown: u32,
}