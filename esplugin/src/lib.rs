use std::io;
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use byteorder::{BigEndian, ReadBytesExt};
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

pub fn read_plugin(filepath: PathBuf) -> io::Result<RecordType> {
    let file = File::open(filepath)?;
    let mut buf_reader = BufReader::new(file);
    let code = buf_reader.read_u32::<BigEndian>()?;

    Ok(RecordType::from(code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let filename = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../data/Skyrim.esm");
        let filepath = PathBuf::from(filename);
        let record_type = read_plugin(filepath).unwrap();
        
        assert_eq!(record_type, RecordType::FileHeader);
    }
}
