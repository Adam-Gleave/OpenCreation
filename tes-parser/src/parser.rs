use nom::bytes::complete::take;
use nom::combinator::map;
use nom::multi::many0;
use nom::number::complete::{le_u32, le_i32, le_u16};
use nom::sequence::tuple;
use nom::IResult;
use std::convert::TryInto;
use std::fmt;
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub struct Plugin {
    pub header: Record,
    pub top_groups: Vec<Group>,
}

#[derive(PartialEq, Eq)]
pub struct TypeCode {
    pub code: [u8; 4],
}

impl fmt::Debug for TypeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = str::from_utf8(&self.code[..]).unwrap().to_owned();
        f.debug_struct("TypeCode")
         .field("code", &code)
         .finish()
    } 
}

#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    pub header: GroupHeader,
    pub records: Vec<Record>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GroupHeader {
    pub code: TypeCode,
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
    pub code: TypeCode, 
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
    pub code: TypeCode,
    pub size: u16,
}

pub fn parse_plugin(input: &[u8]) -> IResult<&[u8], Plugin> {
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

fn type_code(input: &[u8]) -> IResult<&[u8], TypeCode> {
    let (remaining, code) = take(4usize)(input)?;
    Ok((
        remaining,
        TypeCode {
            code: code.try_into().expect("Parsing type code with incorrect length"),
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

fn group_header(input: &[u8]) -> IResult<&[u8], GroupHeader> {
    map(
    tuple((type_code, le_u32, le_u32, le_i32, le_u32, le_u32)),
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

fn record_header(input: &[u8]) -> IResult<&[u8], RecordHeader> {
    map(
        tuple((type_code, le_u32, le_u32, le_u32, le_u32, le_u16, le_u16)),
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

fn subrecord(input: &[u8]) -> IResult<&[u8], Subrecord> {
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

fn subrecord_header(input: &[u8]) -> IResult<&[u8], SubrecordHeader> {
    map(
        tuple((type_code, le_u16)),
        |(code, size)| SubrecordHeader {
            code,
            size,
        },
    )(input)
}