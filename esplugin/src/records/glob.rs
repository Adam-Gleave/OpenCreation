use crate::file::read::{EspReader, Readable};
use crate::records::record::{Record, RecordFlags};
use crate::subrecords::common::EDIDData;
use crate::subrecords::subrecord::{Subrecord, SubrecordType};
use esplugin_derive::*;
use std::io;

pub type GLOBRecord = Record<RecordFlags, GLOBData>;

pub type EDIDSubrecord = Subrecord<EDIDData>;
pub type FNAMSubrecord = Subrecord<FNAMData>;
pub type FLTVSubrecord = Subrecord<FLTVData>;

#[derive(Debug, Readable)]
pub struct FNAMData {
    ty: char,
}

#[derive(Debug, Readable)]
pub struct FLTVData {
    value: f32,
}

#[derive(Debug, Default)]
pub struct GLOBData {
    pub edid: Option<EDIDSubrecord>,
    pub fnam: Option<FNAMSubrecord>,
    pub fltv: Option<FLTVSubrecord>,
}

impl Readable for GLOBData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: GLOBData = Default::default();
        
        while reader.record_left() > 0 {
            let subrecord_type = reader.read_subrecord_type()?;
            
            match subrecord_type {
                SubrecordType::EDID => record.edid = Some(EDIDSubrecord::read(reader)?),
                SubrecordType::FNAM => record.fnam = Some(FNAMSubrecord::read(reader)?),
                SubrecordType::FLTV => record.fltv = Some(FLTVSubrecord::read(reader)?),
                _ => {
                    let msg = format!("Unexpected subrecord {:#?} found in GLOB", subrecord_type);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
                },
            }
        }

        Ok(record)
    }
}
