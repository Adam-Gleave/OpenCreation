mod parser;

pub use parser::{
    Plugin,
    Group,
    GroupHeader,
    Record,
    RecordHeader,
    Subrecord,
    SubrecordHeader,
    TypeCode,
};

use std::io::{BufReader, Read};

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing plugin file")
    }
}

pub fn read_plugin<R>(readable: R) -> Result<Plugin>
where
    R: std::io::Read
{
    let mut reader = BufReader::new(readable);
    let mut bytes = vec!();
    reader.read_to_end(&mut bytes).unwrap();

    let (remaining, plugin) = parser::parse_plugin(&bytes).unwrap();
    let bytes_remaining: Vec<u8> = remaining.iter().cloned().collect();
    
    if bytes_remaining.len() == 0 {
        Ok(plugin)
    } else {
        Err(ParseError)
    }
}

#[cfg(test)]
mod tests {
    use crate::{read_plugin, Plugin};
    use lazy_static::lazy_static;
    use std::fs::File;
    use std::path::PathBuf;

    lazy_static! {
        static ref SKYRIM_PLUGIN: Plugin = {
            let path = PathBuf::from(format!(
                "{}{}",
                env!("CARGO_MANIFEST_DIR"),
                "/../data/Skyrim.esm"
            ));
            let file = File::open(path).unwrap();
            read_plugin(file).unwrap()
        };
    }

    #[test]
    fn test_file_header() {
        assert_eq!(SKYRIM_PLUGIN.header.header.version, 40);
        assert_eq!(SKYRIM_PLUGIN.header.subrecords.len(), 3);
    }
}

