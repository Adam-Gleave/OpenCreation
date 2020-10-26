use std::io;
use bitflags::bitflags;
use super::file::read::{EspReader, Readable};
use super::records::header::{RecordHeader, VersionControlInfo};

bitflags! {
    #[derive(Default)]
    pub struct PluginFlags: u32 {
        const MASTER_FILE  = 0x00000001;
        const LOCALIZED   = 0x00000080;
        const LIGHT_MASTER = 0x00000200;
    }
}

impl Readable for PluginFlags {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(PluginFlags::from_bits(reader.read_u32()?).unwrap_or(Default::default()))
    }

    fn data_size(&self) -> u32 {
        std::mem::size_of::<u32>() as u32
    }
}

pub type PluginHeader = RecordHeader<PluginFlags>;

impl Readable for PluginHeader {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            size: reader.read_u32()?,
            flags: PluginFlags::read(reader)?,
            id: reader.read_u32()?,
            vc_info: VersionControlInfo::read(reader)?,
            version: reader.read_u16()?,
            unknown: reader.read_u16()?,
        })
    }

    fn data_size(&self) -> u32 { self.size }
}