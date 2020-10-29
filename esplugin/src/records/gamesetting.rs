use crate::file::read::{EspReader, Readable};
use crate::file::types::*;
use crate::records::record::{Record, RecordFlags, RecordType, Coded};
use crate::subrecords::subrecord::{Subrecord, SubrecordType};
use esplugin_derive::*;
use std::io;

pub type GameSettingRecord = Record<RecordFlags, GameSettingData>;

impl Coded for GameSettingRecord {
    fn code() -> RecordType {
        RecordType::GameSetting
    }
}

pub type EDIDSubrecord = Subrecord<EDIDData>;
pub type DATASubrecord = Subrecord<DATAData>;

#[derive(Debug, Readable)]
pub struct EDIDData {
    pub editor_id: ZString,
}

#[derive(Debug, Readable)]
pub struct DATAData {
    pub color: VariantBytes,
}

#[derive(Debug, Default)]
pub struct GameSettingData {
    pub edid: Option<EDIDSubrecord>,
    pub data: Option<DATASubrecord>,
}

impl Readable for GameSettingData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: GameSettingData = Default::default();
        
        while reader.record_left() > 0 {
            let subrecord_type = reader.read_subrecord_type()?;

            match subrecord_type {
                SubrecordType::EDID => record.edid = Some(EDIDSubrecord::read(reader)?),
                SubrecordType::DATA => record.data = Some(DATASubrecord::read(reader)?),
                _ => {
                    let msg = format!("Unexpected subrecord code {:#?} found in GMST", subrecord_type);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
                },
            }
        }
        
        Ok(record)
    }
}
