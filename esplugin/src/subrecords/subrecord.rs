use crate::file::read::{EspReader, Readable};
use crate::subrecords::header::SubrecordHeader;
use esplugin_derive::*;
use num_derive::FromPrimitive;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
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
    OBND = 0x4f424e44,
    TX00 = 0x54583030,
    TX01 = 0x54583031,
    TX02 = 0x54583032,
    TX03 = 0x54583033,
    TX04 = 0x54583034,
    TX05 = 0x54583035,
    TX06 = 0x54583036,
    TX07 = 0x54583037,
    DODT = 0x444f4454,
    DNAM = 0x444e414d,
    FNAM = 0x464e414d,
    FLTV = 0x464c5456,
    FULL = 0x46554c4c,
    DESC = 0x44455343,
    ICON = 0x49434f4e,
    Unknown = 0,
}

impl SubrecordType {
    #[allow(unused)]
    pub fn num(&self) -> u32 { *self as u32 }
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
