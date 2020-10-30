
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::multi::many0;
use nom::number::complete::{be_u32, le_u32, le_i32, le_u16};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Plugin {
    pub header: Record,
    pub top_groups: Vec<Group>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    pub header: GroupHeader,
    pub records: Vec<Record>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GroupHeader {
    pub code: u32,
    pub size: u32,
    pub label: u32,
    pub group_type: i32,
    pub vc_info: u32,
    pub unknown: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub header: RecordHeader,
    pub subrecords: Vec<Subrecord>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RecordHeader {
    pub code: u32, 
    pub size: u32,
    pub flags: u32,
    pub id: u32,
    pub vc_info: u32,
    pub version: u16,
    pub unknown: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Subrecord {
    pub header: SubrecordHeader,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SubrecordHeader {
    pub code: u32,
    pub size: u16,
}

pub fn plugin(input: &[u8]) -> IResult<&[u8], Plugin> {
    let (remaining, header) = record(input)?;
    let (remaining, top_groups) = many0(group)(remaining)?;
    Ok((
        remaining,
        Plugin {
            header,
            top_groups,
        }
    ))
}

pub fn group(input: &[u8]) -> IResult<&[u8], Group> {
    let (remaining, header) = group_header(input)?;
    let (remaining, records_bytes) = take(header.size - 24)(remaining)?;
    let (_, records) = many0(record)(records_bytes)?;
    Ok((
        remaining,
        Group {
            header,
            records,
        }
    ))
}

pub fn group_header(input: &[u8]) -> IResult<&[u8], GroupHeader> {
    map(
    tuple((be_u32, le_u32, le_u32, le_i32, le_u32, le_u32)),
    |(code, size, label, group_type, vc_info, unknown)| GroupHeader {
            code,
            size,
            label,
            group_type,
            vc_info,
            unknown,
        },
    )(input)
}

pub fn record(input: &[u8]) -> IResult<&[u8], Record> {
    let (remaining, header) = record_header(input)?;
    let (remaining, subrecords_bytes) = take(header.size)(remaining)?;
    let (_, subrecords) = many0(subrecord)(subrecords_bytes)?;
    Ok((
        remaining,
        Record {
            header,
            subrecords,
        }
    ))
}

pub fn record_header(input: &[u8]) -> IResult<&[u8], RecordHeader> {
    map(
        tuple((be_u32, le_u32, le_u32, le_u32, le_u32, le_u16, le_u16)),
        |(code, size, flags, id, vc_info, version, unknown)| RecordHeader {
            code,
            size,
            flags,
            id,
            vc_info,
            version,
            unknown,
        },
    )(input)
}

pub fn subrecord(input: &[u8]) -> IResult<&[u8], Subrecord> {
    let (remaining, header) = subrecord_header(input)?;
    let (remaining, data) = take(header.size)(remaining)?;
    Ok((
        remaining,
        Subrecord { 
            header, 
            data: data.iter().cloned().collect() 
        }
    ))
}

pub fn subrecord_header(input: &[u8]) -> IResult<&[u8], SubrecordHeader> {
    map(
        tuple((be_u32, le_u16)),
        |(code, size)| SubrecordHeader {
            code,
            size,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{plugin, Plugin};
    use lazy_static::lazy_static;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::PathBuf;

    lazy_static! {
        static ref SKYRIM_PLUGIN: Plugin = {
            let path = PathBuf::from(format!(
                "{}{}",
                env!("CARGO_MANIFEST_DIR"),
                "/../data/Skyrim.esm"
            ));

            let mut reader = BufReader::new(File::open(path).unwrap());
            let mut bytes = vec!();
            reader.read_to_end(&mut bytes).unwrap();

            let (remaining, plugin) = plugin(&bytes).unwrap();
            let bytes_remaining: Vec<u8> = remaining.iter().cloned().collect();
            assert_eq!(bytes_remaining.len(), 0);

            plugin
        };
    }

    #[test]
    fn test_file_header() {
        assert_eq!(SKYRIM_PLUGIN.header.header.version, 40);
        assert_eq!(SKYRIM_PLUGIN.header.subrecords.len(), 3);
    }
}
