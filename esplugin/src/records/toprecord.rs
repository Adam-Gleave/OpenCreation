use crate::file::read::{EspReader, Readable};
use crate::records::header::{RecordHeader, VersionControlInfo};
use crate::records::record::Record;
use crate::subrecords::header::SubrecordType;
use crate::subrecords::subrecord::Subrecord;
use bitflags::bitflags;
use std::io;

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
}

pub type TopRecordHeader = RecordHeader<PluginFlags>;

impl Readable for TopRecordHeader {
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
}

#[derive(Debug)]
pub struct HEDRData {
    pub version: f32,
    pub num_records: i32,
    pub next_id: u32,
}

impl Readable for HEDRData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            version: reader.read_f32()?,
            num_records: reader.read_i32()?,
            next_id: reader.read_u32()?,
        })
    }
}

pub type HEDR = Subrecord<HEDRData>;

#[derive(Debug)]
pub struct TopRecordData {
    hedr: Option<HEDR>,
}

impl Readable for TopRecordData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let code = reader.read_subrecord_type()?;

        let hedr = if code == SubrecordType::HEDR {
            Some(HEDR::read(reader)?)
        } else {
            None
        };

        Ok(Self { hedr })
    }
}

pub type TopRecord = Record<TopRecordHeader, TopRecordData>;
