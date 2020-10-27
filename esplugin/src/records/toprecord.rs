use crate::file::read::{EspReader, Readable};
use crate::records::header::RecordHeader;
use crate::records::record::Record;
use crate::subrecords::header::SubrecordType;
use crate::subrecords::subrecord::Subrecord;
use bitflags::bitflags;
use std::io;

pub type TopRecordHeader = RecordHeader<PluginFlags>;
pub type TopRecord = Record<TopRecordHeader, TopRecordData>;

pub type HEDR = Subrecord<HEDRData>;
pub type CNAM = Subrecord<CNAMData>;
pub type SNAM = Subrecord<SNAMData>;

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

#[derive(Debug)]
pub struct CNAMData {
    pub author: String,
}

impl Readable for CNAMData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            author: reader.read_zstring()?,
        })
    }
}

#[derive(Debug)]
pub struct SNAMData {
    pub description: String,
}

#[derive(Debug, Default)]
pub struct TopRecordData {
    pub hedr: Option<HEDR>,
    pub cnam: Option<CNAM>,
    pub snam: Option<SNAM>,
}

impl Readable for SNAMData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            description: reader.read_zstring()?,
        })
    }
}

impl Readable for TopRecordData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: TopRecordData = Default::default();

        while reader.record_left() > 0 {
            match reader.read_subrecord_type()? {
                SubrecordType::HEDR => record.hedr = Some(HEDR::read(reader)?),
                SubrecordType::CNAM => record.cnam = Some(CNAM::read(reader)?),
                SubrecordType::SNAM => record.snam = Some(SNAM::read(reader)?),
                _ => (),
            }
        }

        Ok(record)
    }
}
