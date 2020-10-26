use std::io;
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use bitflags::bitflags;
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
            record_to_read: 0,
        }
    }

    pub fn next_record_data(&mut self, data_size: u32) {
        self.record_to_read = data_size as i64;
    }

    pub fn record_left(&self) -> i64 {
        self.record_to_read
    }

    pub fn read_record_type(&mut self) -> io::Result<RecordType> {
        let code = self.buf_reader.read_u32::<BigEndian>()?;
        self.record_to_read -= std::mem::size_of::<u32>() as i64;
        Ok(RecordType::from(code))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        self.buf_reader.read_u32::<LittleEndian>()
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        self.buf_reader.read_u16::<LittleEndian>()
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        self.buf_reader.read_u8()
    }
}

pub trait Readable where Self: Sized {
    fn read(reader: &mut EspReader) -> io::Result<Self>;
    fn data_size(&self) -> u32;
}

bitflags! {
    #[derive(Default)]
    pub struct PluginFlags: u32 {
        const MASTER_FILE  = 0x00000001;
        const LOCALIZED   = 0x00000080;
        const LIGHT_MASTER = 0x00000200;
    }
}

impl Readable for PluginFlags {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(PluginFlags::from_bits(reader.read_u32()?).unwrap_or(Default::default()))
    }

    fn data_size(&self) -> u32 {
        std::mem::size_of::<u32>() as u32
    }
}

#[derive(Debug)]
pub struct VersionControlInfo {
    vc_day: u8,
    vc_month: u8,
    vc_previous_id: u8,
    vc_current_id: u8,
}

impl Readable for VersionControlInfo {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            vc_day: reader.read_u8()?,
            vc_month: reader.read_u8()?,
            vc_previous_id: reader.read_u8()?,
            vc_current_id: reader.read_u8()?,
        })
    }

    fn data_size(&self) -> u32 {
        std::mem::size_of::<VersionControlInfo>() as u32
    }
}

#[derive(Debug)]
pub struct RecordHeader<RecordFlags>
    where RecordFlags: Readable
{
    size: u32,
    flags: RecordFlags,
    id: u32,
    vc_info: VersionControlInfo,
    version: u16,
    unknown: u16,
}

type PluginHeader = RecordHeader<PluginFlags>;

impl Readable for PluginHeader {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            size: reader.read_u32()?,
            flags: PluginFlags::read(reader)?,
            id: reader.read_u32()?,
            vc_info: VersionControlInfo::read(reader)?,
            version: reader.read_u16()?,
            unknown: reader.read_u16()?,
        })
    }

    fn data_size(&self) -> u32 { self.size }
}

pub fn read_plugin(filepath: PathBuf) -> io::Result<PluginHeader> {
    let file = File::open(filepath)?;
    let mut esp_reader = EspReader::new(file);
    let _code = esp_reader.read_record_type()?;

    Ok(RecordHeader::read(&mut esp_reader)?)
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use lazy_static::lazy_static;
    use super::*;

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
