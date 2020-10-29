use crate::file::read::{EspReader, Readable};
use crate::file::types::*;
use crate::records::record::Record;
use crate::subrecords::subrecord::{Subrecord, SubrecordType};
use bitflags::bitflags;
use esplugin_derive::*;
use std::io;

pub type TES4 = Record<PluginFlags, TES4Data>;

pub type HEDRSubrecord = Subrecord<HEDRData>;
pub type CNAMSubrecord = Subrecord<CNAMData>;
pub type SNAMSubrecord = Subrecord<SNAMData>;
pub type MASTSubrecord = Subrecord<MASTData>;
pub type DATASubrecord = Subrecord<DATAData>;
pub type ONAMSubrecord = Subrecord<ONAMData>;
pub type INTVSubrecord = Subrecord<INTVData>;
pub type INCCSubrecord = Subrecord<INCCData>;

pub const CODE: u32 = 0x54455334;   // "TES4"

bitflags! {
    #[derive(Default)]
    pub struct PluginFlags: u32 {
        const MASTER_FILE  = 0x00000001;
        const LOCALIZED    = 0x00000080;
        const LIGHT_MASTER = 0x00000200;
    }
}

impl Readable for PluginFlags {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(PluginFlags::from_bits(u32::read(reader)?).unwrap_or(Default::default()))
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
pub struct TES4Data {
    pub hedr: Option<HEDRSubrecord>,
    pub cnam: Option<CNAMSubrecord>,
    pub snam: Option<SNAMSubrecord>,
    pub mast: Option<MASTSubrecord>,
    pub data: Option<DATASubrecord>,
    pub onam: Option<ONAMSubrecord>,
    pub intv: Option<INTVSubrecord>,
    pub incc: Option<INCCSubrecord>,
}

impl Readable for TES4Data {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: TES4Data = Default::default();

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
