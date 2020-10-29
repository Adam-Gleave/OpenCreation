use crate::file::read::{EspReader, Readable, Peekable};
use crate::records::record::RecordType;
use crate::records::{gamesetting::GameSettingRecord, keyword::KeywordRecord};
use crate::records::toprecord;
use crate::groups::group::Group;
use crate::groups::group;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub enum GroupVariant {
    GameSetting(Group<GameSettingRecord>),
    Keyword(Group<KeywordRecord>),
    Unknown,
}

#[derive(Debug)]
pub struct Plugin {
    pub header: toprecord::TopRecord,
    pub top_groups: HashMap<RecordType, GroupVariant>,
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            header: Default::default(),
            top_groups: HashMap::new(),
        }
    }
}

impl Readable for Plugin {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut plugin: Plugin = Default::default();

        if reader.read_record_type()? != toprecord::CODE.into() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                "Invalid file, no TES4 code"
            ));
        } else {
            plugin.header = toprecord::TopRecord::read(reader)?;
        }

        for _ in 0..2 {            
            if reader.read_record_type()? != group::CODE.into() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData, 
                    "No top group found"
                ));
            } else {
                let record_type = RecordType::peek(reader, 4i64)?;

                let group = match &record_type {
                    RecordType::GameSetting => {  GroupVariant::GameSetting(Group::<GameSettingRecord>::read(reader)?) },
                    RecordType::Keyword     => { GroupVariant::Keyword(Group::<KeywordRecord>::read(reader)?) },
                    _ => { 
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData, 
                            "Unknown record type found"
                        ));
                    },
                };

                plugin.top_groups.insert(record_type, group);
            }
        }

        Ok(plugin)
    }
}