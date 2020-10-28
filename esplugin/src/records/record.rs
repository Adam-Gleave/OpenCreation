use crate::file::read::{EspReader, Readable};
use crate::records::header::RecordHeader;
use bitflags::bitflags;
use esplugin_derive::*;
use std::io;

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

#[derive(Default, Debug, Readable)]
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
