use crate::file::read::{EspReader, Readable};
use crate::records::{toprecord, keyword};
use crate::groups::group;
use std::io;

#[derive(Debug, Default)]
pub struct Plugin {
    pub header: toprecord::TopRecord,
    pub first_group: group::Group<keyword::KeywordRecord>,
}

impl Readable for Plugin {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut plugin: Plugin = Default::default();

        let tes4_code = reader.read_record_type()?;
        if tes4_code != toprecord::CODE.into() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid file, no TES4 code"));
        } else {
            plugin.header = toprecord::TopRecord::read(reader)?;
        }

        let group_code = reader.read_record_type()?;
        if group_code != group::CODE.into() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "No top group found"));
        } else {
            plugin.first_group = group::Group::read(reader)?;
        }

        Ok(plugin)
    }
}