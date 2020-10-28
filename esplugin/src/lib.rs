mod file;
mod records;
mod subrecords;

use crate::file::read::{EspReader, Readable};
use crate::records::toprecord::TopRecord;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn read_plugin(filepath: PathBuf) -> io::Result<TopRecord> {
    let file = File::open(filepath)?;
    let mut esp_reader = EspReader::new(file);
    let _code = esp_reader.read_record_type()?;

    Ok(TopRecord::read(&mut esp_reader)?)
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::records::toprecord::{PluginFlags, TopRecord};
    use super::*;
    use lazy_static::lazy_static;
    use std::path::PathBuf;

    lazy_static! {
        static ref SKYRIM_MASTER: TopRecord = {
            let filename = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../data/Skyrim.esm");
            let filepath = PathBuf::from(filename);
            read_plugin(filepath).unwrap()
        };

        static ref DAWNGUARD_MASTER: TopRecord = {
            let filename = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../data/Dawnguard.esm");
            let filepath = PathBuf::from(filename);
            read_plugin(filepath).unwrap()
        };
    }

    #[test]
    fn test_read_skyrim_header() {
        println!("{:#?}", *SKYRIM_MASTER);
        assert_eq!(SKYRIM_MASTER.header.flags, PluginFlags::MASTER_FILE | PluginFlags::LOCALIZED);
        assert_eq!(SKYRIM_MASTER.data.hedr.as_ref().unwrap().data.version, 0.94);
        assert_eq!(
            SKYRIM_MASTER.data.cnam.as_ref().unwrap().data.author.value,
            "mcarofano".to_owned()
        );
    }

    #[test]
    fn test_read_dawnguard_header() {
        println!("{:#?}", *DAWNGUARD_MASTER);
        assert_eq!(DAWNGUARD_MASTER.header.flags, PluginFlags::MASTER_FILE | PluginFlags::LOCALIZED);
        assert_eq!(DAWNGUARD_MASTER.data.hedr.as_ref().unwrap().data.version, 0.94);
        assert_eq!(DAWNGUARD_MASTER.data.onam.as_ref().unwrap().data.overrides.len(), 772);
        assert_eq!(
            DAWNGUARD_MASTER.data.cnam.as_ref().unwrap().data.author.value,
            "bnesmith".to_owned()
        );
    }
}
