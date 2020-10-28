use crate::file::read::{EspReader, Readable};
use crate::file::types::*;
use crate::records::record::{Record, RecordFlags};
use crate::subrecords::header::SubrecordType;
use crate::subrecords::subrecord::Subrecord;
use esplugin_derive::*;
use std::io;

pub type KeywordRecord = Record<RecordFlags, KeywordData>;

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
        println!("{:#?}", reader.record_left());
        
        while reader.record_left() > 0 {
            match reader.read_subrecord_type()? {
                SubrecordType::EDID => record.edid = Some(EDIDSubrecord::read(reader)?),
                SubrecordType::CNAM => record.cnam = Some(CNAMSubrecord::read(reader)?),
                _ => (),
            }
        }

        Ok(record)
    }
}
