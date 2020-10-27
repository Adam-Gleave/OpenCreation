mod file;
mod records;
mod subrecords;

use std::fs::File;
use std::io;
use std::path::PathBuf;

use self::file::read::{EspReader, Readable};
use self::records::toprecord::TopRecord;

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
        static ref PLUGIN: TopRecord = {
            let filename = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../data/Skyrim.esm");
            let filepath = PathBuf::from(filename);
            read_plugin(filepath).unwrap()
        };
    }

    #[test]
    fn test_read_plugin_header() {
        println!("{:#?}", *PLUGIN);
        assert_eq!(PLUGIN.header.flags, PluginFlags::MASTER_FILE | PluginFlags::LOCALIZED);
        assert_eq!(PLUGIN.data.hedr.as_ref().unwrap().data.version, 0.94);
        assert_eq!(PLUGIN.data.cnam.as_ref().unwrap().data.author, "mcarofano".to_owned());
    }
}
