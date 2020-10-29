mod file;
mod groups;
mod plugin;
mod records;
mod subrecords;

use crate::file::read::{EspReader, Readable};
use crate::plugin::plugin::Plugin;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn read_plugin(filepath: PathBuf) -> io::Result<Plugin> {
    let file = File::open(filepath)?;
    let mut esp_reader = EspReader::new(file);

    Ok(Plugin::read(&mut esp_reader)?)
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::records::tes4::PluginFlags;
    use super::*;
    use lazy_static::lazy_static;
    use std::path::PathBuf;

    lazy_static! {
        static ref SKYRIM_MASTER: Plugin = {
            let filename = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../data/Skyrim.esm");
            let filepath = PathBuf::from(filename);
            read_plugin(filepath).unwrap()
        };

        static ref DAWNGUARD_MASTER: Plugin = {
            let filename = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../data/Dawnguard.esm");
            let filepath = PathBuf::from(filename);
            read_plugin(filepath).unwrap()
        };
    }

    #[test]
    fn test_read_skyrim_header() {
        assert_eq!(SKYRIM_MASTER.header.header.flags, PluginFlags::MASTER_FILE | PluginFlags::LOCALIZED);
        assert_eq!(SKYRIM_MASTER.header.data.hedr.as_ref().unwrap().data.version, 0.94);
        assert_eq!(
            SKYRIM_MASTER.header.data.cnam.as_ref().unwrap().data.author.value,
            "mcarofano".to_owned()
        );
    }

    #[test]
    fn test_read_dawnguard_header() {
        assert_eq!(DAWNGUARD_MASTER.header.header.flags, PluginFlags::MASTER_FILE | PluginFlags::LOCALIZED);
        assert_eq!(DAWNGUARD_MASTER.header.data.hedr.as_ref().unwrap().data.version, 0.94);
        assert_eq!(DAWNGUARD_MASTER.header.data.onam.as_ref().unwrap().data.overrides.len(), 772);
        assert_eq!(
            DAWNGUARD_MASTER.header.data.cnam.as_ref().unwrap().data.author.value,
            "bnesmith".to_owned()
        );
    }
}
