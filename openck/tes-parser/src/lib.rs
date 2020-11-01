mod error;
mod parser;

#[rustfmt::skip]
pub use parser::{
    Group, 
    GroupHeader, 
    Plugin, 
    Record, 
    RecordHeader, 
    Subrecord, 
    SubrecordHeader, 
    TypeCode,
};

pub use error::ParseError;
use std::io::{BufReader, Read};

pub type Result<T> = std::result::Result<T, ParseError>;

pub fn read_plugin<R>(readable: R) -> Result<Plugin>
where
    R: std::io::Read,
{
    let mut reader = BufReader::new(readable);
    let mut bytes = vec![];
    reader.read_to_end(&mut bytes)?;

    let (remaining, plugin) = parser::parse_plugin(&bytes)?;
    let bytes_remaining: Vec<u8> = remaining.iter().cloned().collect();

    if bytes_remaining.len() == 0 {
        Ok(plugin)
    } else {
        Err(ParseError::new("Parser failed to interpret all bytes"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::TypeCode, read_plugin, Plugin};
    use lazy_static::lazy_static;
    use std::fs::File;
    use std::path::PathBuf;

    lazy_static! {
        static ref SKYRIM_PLUGIN: Plugin = {
            let path = PathBuf::from(format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../../data/Skyrim.esm"));
            let file = File::open(path).unwrap();
            read_plugin(file).unwrap()
        };
    }

    #[test]
    fn test_file_header() {
        assert_eq!(SKYRIM_PLUGIN.header.header.version, 40);
        assert_eq!(SKYRIM_PLUGIN.header.subrecords.len(), 3);
    }

    #[test]
    fn test_record_type_equality() {
        let a: TypeCode = 0x5f43504e.into();
        let b = TypeCode::from_utf8("NPC_").unwrap();
        println!("{:#?}, {:#?}", a, b);
        assert_eq!(a, b);
    }
}
