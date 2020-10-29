use crate::file::read::{EspReader, Readable};
use crate::file::types::*;
use crate::records::record::{Record, RecordFlags, RecordType, Coded};
use crate::subrecords::subrecord::{Subrecord, SubrecordType};
use esplugin_derive::*;
use std::io;

pub type KeywordRecord = Record<RecordFlags, KeywordData>;

impl Coded for KeywordRecord {
    fn code() -> RecordType {
        RecordType::Keyword
    }
}

pub type EDIDSubrecord = Subrecord<EDIDData>;
pub type CNAMSubrecord = Subrecord<CNAMData>;

#[derive(Debug, Readable)]
pub struct EDIDData {
    pub editor_id: ZString,
}

#[derive(Debug, Readable)]
pub struct CNAMData {
    pub color: RGB,
}

#[derive(Debug, Default)]
pub struct KeywordData {
    pub edid: Option<EDIDSubrecord>,
    pub cnam: Option<CNAMSubrecord>,
}

impl Readable for KeywordData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: KeywordData = Default::default();
        
        while reader.record_left() > 0 {
            let subrecord_type = reader.read_subrecord_type()?;
            match subrecord_type {
                SubrecordType::EDID => record.edid = Some(EDIDSubrecord::read(reader)?),
                SubrecordType::CNAM => record.cnam = Some(CNAMSubrecord::read(reader)?),
                _ => {
                    let msg = format!("Unexpected subrecord {:#?} found in KYWD", subrecord_type);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
                },
            }
        }

        Ok(record)
    }
}
