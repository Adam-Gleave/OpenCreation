use crate::file::read::{EspReader, Readable, Peekable};
use crate::records::record::RecordType;
use crate::records::{
    tes4,
    aact::AACTRecord,
    glob::GLOBRecord,
    gmst::GMSTRecord, 
    kywd::KYWDRecord,
    lcrt::LCRTRecord,
    txst::TXSTRecord,
};
use crate::groups::group::Group;
use crate::groups::group;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub enum GroupVariant {
    GMST(Group<GMSTRecord>),
    KYWD(Group<KYWDRecord>),
    AACT(Group<AACTRecord>),
    LCRT(Group<LCRTRecord>),
    TXST(Group<TXSTRecord>),
    GLOB(Group<GLOBRecord>),
    Unknown,
}

#[derive(Debug)]
pub struct Plugin {
    pub header: tes4::TES4,
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

        if reader.read_record_type()? != tes4::CODE.into() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                "Invalid file, no TES4 code"
            ));
        } else {
            plugin.header = tes4::TES4::read(reader)?;
        }

        for _ in 0..6 {            
            if reader.read_record_type()? != group::CODE.into() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData, 
                    "No top group found"
                ));
            } else {
                let record_type = RecordType::peek(reader, 4i64)?;

                let group = match &record_type {
                    RecordType::GMST => { GroupVariant::GMST(Group::<GMSTRecord>::read(reader)?) },
                    RecordType::KYWD => { GroupVariant::KYWD(Group::<KYWDRecord>::read(reader)?) },
                    RecordType::AACT => { GroupVariant::AACT(Group::<AACTRecord>::read(reader)?) },
                    RecordType::LCRT => { GroupVariant::LCRT(Group::<LCRTRecord>::read(reader)?) },
                    RecordType::TXST => { GroupVariant::TXST(Group::<TXSTRecord>::read(reader)?) },
                    RecordType::GLOB => { GroupVariant::GLOB(Group::<GLOBRecord>::read(reader)?) },
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