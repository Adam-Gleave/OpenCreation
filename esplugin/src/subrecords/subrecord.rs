use crate::file::read::{EspReader, Readable};
use crate::subrecords::header::SubrecordHeader;
use esplugin_derive::*;
use num_derive::FromPrimitive;
use std::fmt::Debug;

#[derive(Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum SubrecordType {
    HEDR = 0x48454452,
    CNAM = 0x434E414D,
    SNAM = 0x534E414D,
    MAST = 0x4D415354,
    DATA = 0x44415441,
    ONAM = 0x4F4E414D,
    INTV = 0x494E5456,
    INCC = 0x494E4343,
    EDID = 0x45444944,
    Unknown = 0x0,
}

impl From<u32> for SubrecordType {
    fn from(num: u32) -> Self {
        num::FromPrimitive::from_u32(num).unwrap_or(Self::Unknown)
    }
}

#[derive(Debug, Readable)]
#[subrecord_header(true)]
#[size_var(header, size)]
pub struct Subrecord<Data>
where
    Data: Readable,
{
    pub header: SubrecordHeader,
    pub data: Data,
}
