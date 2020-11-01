use nom::bytes::complete::take;
use nom::combinator::map;
use nom::multi::many0;
use nom::number::complete::{le_i32, le_u16, le_u32};
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TypeCode {
    pub code: [u8; 4],
}

impl fmt::Debug for TypeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = str::from_utf8(&self.code[..]).unwrap().to_owned();
        f.debug_struct("TypeCode").field("code", &code).finish()
    }
}

impl From<u32> for TypeCode {
    fn from(input: u32) -> Self {
        Self {
            code: unsafe { std::mem::transmute(input.to_le()) },
        }
    }
}

impl Into<u32> for TypeCode {
    fn into(self) -> u32 {
        unsafe { std::mem::transmute(self.code) }
    }
}

impl TypeCode {
    pub fn from_utf8(input: &str) -> Result<Self, std::io::Error> {
        let bytes = input.as_bytes();

        if let Ok(code_byte_arr) = bytes.try_into() {
            Ok(Self { code: code_byte_arr })
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Type code string is required to be 4 characters long",
            ))
        }
    }

    pub fn to_utf8(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.code)
    }
}

pub enum GroupType {
    Top = 0,
    WorldChildren = 1,
    InteriorCellBlock = 2,
    InteriorSubCellBlock = 3,
    ExteriorCellBlock = 4,
    ExteriorSubCellBlock = 5,
    CellChildren = 6,
    TopicChildren = 7,
    CellPersistenChildren = 8,
    CellTemporaryChildren = 9,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    pub header: GroupHeader,
    pub records: Vec<Record>,
}

impl Group {
    pub fn type_code(&self) -> Option<TypeCode> {
        // Make sure this is a top group
        if self.header.group_type == GroupType::Top as i32 {
            Some(self.header.label.into())
        } else {
            None
        }
    }

    pub fn top_group_matches_type(&self, code: TypeCode) -> bool {
        if let Some(group_type_code) = self.type_code() {
            code == group_type_code
        } else {
            false
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub header: RecordHeader,
    pub subrecords: Vec<Subrecord>,
}

impl Record {
    pub fn editor_id(&self) -> Option<String> {
        if self.subrecords.is_empty() {
            None
        } else {
            let edid_subrecord = self.subrecords.iter().find(|s| {
                return s.header.code == TypeCode::from_utf8("EDID").unwrap();
            });

            if let Some(edid) = edid_subrecord {
                Some(String::from_utf8(edid.data.clone()).unwrap())
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordHeader {
    pub code: TypeCode,
    pub size: u32,
    pub flags: u32,
    pub id: u32,
    pub vc_info: u32,
    pub version: u16,
    pub unknown: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Subrecord {
    pub header: SubrecordHeader,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubrecordHeader {
    pub code: TypeCode,
    pub size: u16,
}

pub fn parse_plugin(input: &[u8]) -> IResult<&[u8], Plugin> {
    let (remaining, header) = record(input)?;
    let (remaining, top_groups) = many0(group)(remaining)?;
    Ok((remaining, Plugin { header, top_groups }))
}

fn type_code(input: &[u8]) -> IResult<&[u8], TypeCode> {
    let (remaining, code) = take(4usize)(input)?;
    Ok((
        remaining,
        TypeCode {
            code: code.try_into().expect("Parsing type code with incorrect length"),
        },
    ))
}

pub fn group(input: &[u8]) -> IResult<&[u8], Group> {
    let (remaining, header) = group_header(input)?;
    let (remaining, records_bytes) = take(header.size - 24)(remaining)?;
    let (_, records) = many0(record)(records_bytes)?;
    Ok((remaining, Group { header, records }))
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
    Ok((remaining, Record { header, subrecords }))
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
            data: data.iter().cloned().collect(),
        },
    ))
}

fn subrecord_header(input: &[u8]) -> IResult<&[u8], SubrecordHeader> {
    map(tuple((type_code, le_u16)), |(code, size)| SubrecordHeader { code, size })(input)
}
