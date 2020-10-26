use std::io;
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use num_derive::FromPrimitive;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
pub enum RecordType {
    FileHeader = 0x54455334,    // "TES4"
    Unknown,
}

impl From<u32> for RecordType {
    fn from(num: u32) -> Self {
        num::FromPrimitive::from_u32(num).unwrap_or(Self::Unknown)
    }
}

pub struct EspReader {
    buf_reader: BufReader<File>,
    record_to_read: i64,
}

impl EspReader {
    pub fn new(file: File) -> Self {
        Self { 
            buf_reader: BufReader::new(file), 
            record_to_read: 0 
        }
    }

    pub fn next_record_data(&mut self, data_size: u32) {
        self.record_to_read = data_size as i64;
    }

    pub fn read_record_type(&mut self) -> io::Result<RecordType> {
        let code = self.buf_reader.read_u32::<BigEndian>()?;
        self.record_to_read -= std::mem::size_of::<u32>() as i64;
        Ok(RecordType::from(code))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        self.buf_reader.read_u32::<LittleEndian>()
    }
}

pub trait Readable {
    fn read(&mut self, reader: &mut EspReader) -> io::Result<()>;
    fn data_size(&self) -> u32;
}

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
pub enum PluginFlags {
    MasterFile  = 0x00000001,
    Localized   = 0x00000080,
    LightMaster = 0x00000200,
    Unknown     = 0x00000000,
}

impl Default for PluginFlags {
    fn default() -> Self { Self::Unknown }
}
    
impl From<u32> for PluginFlags {
    fn from(num: u32) -> Self {
        num::FromPrimitive::from_u32(num).unwrap_or(Self::Unknown)
    }
}

#[derive(Debug, Default)]
pub struct PluginHeader {
    size: u32,
    flags: PluginFlags,
}

impl Readable for PluginHeader {
    fn read(&mut self, reader: &mut EspReader) -> io::Result<()> {
        self.size = reader.read_u32()?;
        self.flags = PluginFlags::from(reader.read_u32()?);
        Ok(())
    }

    fn data_size(&self) -> u32 { self.size }
}

pub fn read_plugin(filepath: PathBuf) -> io::Result<PluginHeader> {
    let file = File::open(filepath)?;
    let mut esp_reader = EspReader::new(file);

    let _code = esp_reader.read_record_type()?;

    let mut plugin_header = PluginHeader::default();
    plugin_header.read(&mut esp_reader)?;

    Ok(plugin_header)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let filename = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../data/Skyrim.esm");
        let filepath = PathBuf::from(filename);
        let plugin_header = read_plugin(filepath).unwrap();

        println!("{:?}", plugin_header);
    }
}
