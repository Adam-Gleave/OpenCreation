
use nom::number::streaming::{le_u16, le_u32};
use nom::{do_parse, named, tag};

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub size: u32,
    pub flags: u32,
    pub id: u32,
    pub vc_info: u32,
    pub version: u16,
    pub unknown: u16,
}

named!(pub header<Header>,
    do_parse!(
        tag!("TES4")       >>
        size:       le_u32 >>
        flags:      le_u32 >>
        id:         le_u32 >>
        vc_info:    le_u32 >>
        version:    le_u16 >>
        unknown:    le_u16 >>

        (Header {
            size: size,
            flags: flags,
            id: id,
            vc_info: vc_info,
            version: version,
            unknown: unknown,
        })
    )
);

#[cfg(test)]
mod tests {
    use crate::{header, Header};
    use lazy_static::lazy_static;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::PathBuf;
    use std::sync::Mutex;

    lazy_static! {
        static ref SKYRIM_READER: Mutex<BufReader<File>> = {
            let path = PathBuf::from(format!(
                "{}{}",
                env!("CARGO_MANIFEST_DIR"),
                "/../data/Skyrim.esm"
            ));

            Mutex::new(BufReader::new(File::open(path).unwrap()))
        };
    }

    #[test]
    fn test_read_skyrim_header() {
        let mut buf = [0u8; 24];
        SKYRIM_READER.lock().unwrap().read_exact(&mut buf).unwrap();

        assert_eq!(
            header(&buf),
            Ok((
                &b""[..],
                Header {
                    size: 44,
                    flags: 129,
                    id: 0,
                    vc_info: 0,
                    version: 40,
                    unknown: 0,
                }
            ))
        )
    }
}
