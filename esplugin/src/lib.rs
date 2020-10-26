mod file;
mod plugin;
mod records;

use std::io;
use std::fs::File;
use std::path::PathBuf;

use self::file::read::{EspReader, Readable};
use self::plugin::PluginHeader;
use self::records::header::RecordHeader;

pub fn read_plugin(filepath: PathBuf) -> io::Result<PluginHeader> {
    let file = File::open(filepath)?;
    let mut esp_reader = EspReader::new(file);
    let _code = esp_reader.read_record_type()?;

    Ok(RecordHeader::read(&mut esp_reader)?)
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use std::path::PathBuf;
    use lazy_static::lazy_static;
    use super::*;
    use super::plugin::{PluginFlags, PluginHeader};

    lazy_static! {
        static ref PLUGIN: PluginHeader = {
            let filename = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../data/Skyrim.esm");
            let filepath = PathBuf::from(filename);
            read_plugin(filepath).unwrap()
        };
    }

    #[test]
    fn test_read_plugin_header() {
        assert_eq!(PLUGIN.flags, PluginFlags::MASTER_FILE | PluginFlags::LOCALIZED);
    }
}
