use crate::file::read::{Coded, EspReader, Readable};
use crate::file::types::*;
use crate::records::header::{RecordHeader, RecordType};
use crate::records::record::Record;
use crate::subrecords::header::SubrecordType;
use crate::subrecords::subrecord::Subrecord;
use bitflags::bitflags;
use esplugin_derive::*;
use std::io;

pub type TopRecordHeader = RecordHeader<PluginFlags>;
pub type TopRecord = Record<TopRecordHeader, TopRecordData>;

pub type HEDRSubrecord = Subrecord<HEDRData>;
pub type CNAMSubrecord = Subrecord<CNAMData>;
pub type SNAMSubrecord = Subrecord<SNAMData>;
pub type MASTSubrecord = Subrecord<MASTData>;
pub type DATASubrecord = Subrecord<DATAData>;
pub type ONAMSubrecord = Subrecord<ONAMData>;
pub type INTVSubrecord = Subrecord<INTVData>;
pub type INCCSubrecord = Subrecord<INCCData>;

impl Coded<RecordType> for TopRecord {
    fn code() -> RecordType {
        RecordType::FileHeader
    }
}

impl Coded<SubrecordType> for HEDRSubrecord {
    fn code() -> SubrecordType {
        SubrecordType::HEDR
    }
}

impl Coded<SubrecordType> for CNAMSubrecord {
    fn code() -> SubrecordType {
        SubrecordType::CNAM
    }
}

impl Coded<SubrecordType> for SNAMSubrecord {
    fn code() -> SubrecordType {
        SubrecordType::SNAM
    }
}

impl Coded<SubrecordType> for MASTSubrecord {
    fn code() -> SubrecordType {
        SubrecordType::MAST
    }
}

impl Coded<SubrecordType> for DATASubrecord {
    fn code() -> SubrecordType {
        SubrecordType::DATA
    }
}

impl Coded<SubrecordType> for ONAMSubrecord {
    fn code() -> SubrecordType {
        SubrecordType::ONAM
    }
}

impl Coded<SubrecordType> for INTVSubrecord {
    fn code() -> SubrecordType {
        SubrecordType::INTV
    }
}

impl Coded<SubrecordType> for INCCSubrecord {
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
    pub overrides: Vec<u32>,
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
    pub hedr: Option<HEDRSubrecord>,
    pub cnam: Option<CNAMSubrecord>,
    pub snam: Option<SNAMSubrecord>,
    pub mast: Option<MASTSubrecord>,
    pub data: Option<DATASubrecord>,
    pub onam: Option<ONAMSubrecord>,
    pub intv: Option<INTVSubrecord>,
    pub incc: Option<INCCSubrecord>,
}

impl Readable for TopRecordData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: TopRecordData = Default::default();

        while reader.record_left() > 0 {
            match reader.read_subrecord_type()? {
                SubrecordType::HEDR => record.hedr = Some(HEDRSubrecord::read(reader)?),
                SubrecordType::CNAM => record.cnam = Some(CNAMSubrecord::read(reader)?),
                SubrecordType::SNAM => record.snam = Some(SNAMSubrecord::read(reader)?),
                SubrecordType::MAST => record.mast = Some(MASTSubrecord::read(reader)?),
                SubrecordType::DATA => record.data = Some(DATASubrecord::read(reader)?),
                SubrecordType::ONAM => record.onam = Some(ONAMSubrecord::read(reader)?),
                SubrecordType::INTV => record.intv = Some(INTVSubrecord::read(reader)?),
                SubrecordType::INCC => record.incc = Some(INCCSubrecord::read(reader)?),
                _ => (),
            }
        }

        Ok(record)
    }
}
