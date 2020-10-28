use crate::file::read::{Coded, EspReader, Readable};
use crate::file::types::*;
use crate::records::header::{RecordHeader, RecordType};
use crate::records::record::Record;
use crate::subrecords::header::SubrecordType;
use crate::subrecords::subrecord::Subrecord;
use esplugin_derive::*;
use bitflags::bitflags;
use std::io;

pub type TopRecordHeader = RecordHeader<PluginFlags>;
pub type TopRecord = Record<TopRecordHeader, TopRecordData>;

pub type HEDR = Subrecord<HEDRData>;
pub type CNAM = Subrecord<CNAMData>;
pub type SNAM = Subrecord<SNAMData>;
pub type MAST = Subrecord<MASTData>;
pub type DATA = Subrecord<DATAData>;
pub type ONAM = Subrecord<ONAMData>;
pub type INTV = Subrecord<INTVData>;
pub type INCC = Subrecord<INCCData>;

impl Coded<RecordType> for TopRecord {
    fn code() -> RecordType {
        RecordType::FileHeader
    }
}

impl Coded<SubrecordType> for HEDR {
    fn code() -> SubrecordType {
        SubrecordType::HEDR
    }
}

impl Coded<SubrecordType> for CNAM {
    fn code() -> SubrecordType {
        SubrecordType::CNAM
    }
}

impl Coded<SubrecordType> for SNAM {
    fn code() -> SubrecordType {
        SubrecordType::SNAM
    }
}

impl Coded<SubrecordType> for MAST {
    fn code() -> SubrecordType {
        SubrecordType::MAST
    }
}

impl Coded<SubrecordType> for DATA {
    fn code() -> SubrecordType {
        SubrecordType::DATA
    }
}

impl Coded<SubrecordType> for ONAM {
    fn code() -> SubrecordType {
        SubrecordType::ONAM
    }
}

impl Coded<SubrecordType> for INTV {
    fn code() -> SubrecordType {
        SubrecordType::INTV
    }
}

impl Coded<SubrecordType> for INCC {
    fn code() -> SubrecordType {
        SubrecordType::INCC
    }
}

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

#[derive(Debug, Readable)]
pub struct HEDRData {
    pub version: f32,
    pub num_records: i32,
    pub next_id: u32,
}

#[derive(Debug, Readable)]
pub struct CNAMData {
    pub author: ZString,
}

#[derive(Debug, Readable)]
pub struct SNAMData {
    pub description: ZString,
}

#[derive(Debug, Readable)]
pub struct MASTData {
    pub master: ZString,
}

#[derive(Debug, Readable)]
pub struct DATAData {
    pub file_size: u64,
}

#[derive(Debug, Readable)]
pub struct ONAMData {
    pub overrides: Vec::<u32>,
}

#[derive(Debug, Readable)]
pub struct INTVData {
    pub internal_version: u32,
}

#[derive(Debug, Readable)]
pub struct INCCData {
    pub unknown: u32,
}

#[derive(Debug, Default)]
pub struct TopRecordData {
    pub hedr: Option<HEDR>,
    pub cnam: Option<CNAM>,
    pub snam: Option<SNAM>,
    pub mast: Option<MAST>,
    pub data: Option<DATA>,
    pub onam: Option<ONAM>,
    pub intv: Option<INTV>,
    pub incc: Option<INCC>,
}

impl Readable for TopRecordData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: TopRecordData = Default::default();

        while reader.record_left() > 0 {
            match reader.read_subrecord_type()? {
                SubrecordType::HEDR => record.hedr = Some(HEDR::read(reader)?),
                SubrecordType::CNAM => record.cnam = Some(CNAM::read(reader)?),
                SubrecordType::SNAM => record.snam = Some(SNAM::read(reader)?),
                SubrecordType::MAST => record.mast = Some(MAST::read(reader)?),
                SubrecordType::DATA => record.data = Some(DATA::read(reader)?),
                SubrecordType::ONAM => record.onam = Some(ONAM::read(reader)?),
                SubrecordType::INTV => record.intv = Some(INTV::read(reader)?),
                SubrecordType::INCC => record.incc = Some(INCC::read(reader)?),
                _ => (),
            }
        }

        Ok(record)
    }
}
