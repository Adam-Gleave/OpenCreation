use crate::file::read::{EspReader, Readable};
use crate::records::record::{Record, RecordFlags};
use crate::subrecords::common::{CNAMData, EDIDData};
use crate::subrecords::subrecord::{Subrecord, SubrecordType};
use std::io;

pub type AACTRecord = Record<RecordFlags, AACTData>;

pub type EDIDSubrecord = Subrecord<EDIDData>;
pub type CNAMSubrecord = Subrecord<CNAMData>;

#[derive(Debug, Default)]
pub struct AACTData {
    pub edid: Option<EDIDSubrecord>,
    pub cnam: Option<CNAMSubrecord>,
}

impl Readable for AACTData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: AACTData = Default::default();
        
        while reader.record_left() > 0 {
            let subrecord_type = reader.read_subrecord_type()?;
            
            match subrecord_type {
                SubrecordType::EDID => record.edid = Some(EDIDSubrecord::read(reader)?),
                SubrecordType::CNAM => record.cnam = Some(CNAMSubrecord::read(reader)?),
                _ => {
                    let msg = format!("Unexpected subrecord {:#?} found in AACT", subrecord_type);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
                },
            }
        }

        Ok(record)
    }
}
