use std::io;
use std::fmt::Debug;
use crate::file::read::{EspReader, Readable};
use crate::subrecords::header::SubrecordHeader;

#[derive(Debug)]
pub struct Subrecord<D> where D: Readable + Debug
{
    pub header: SubrecordHeader,
    pub data: D,
}

impl<D> Readable for Subrecord<D> where D: Readable + Debug {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self{
            header: SubrecordHeader::read(reader)?,
            data: D::read(reader)?,
        })
    }
}