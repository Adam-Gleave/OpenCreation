use crate::file::read::{EspReader, Readable};
use crate::subrecords::header::SubrecordHeader;
use std::fmt::Debug;
use std::io;

#[derive(Debug)]
pub struct Subrecord<D>
where
    D: Readable,
{
    pub header: SubrecordHeader,
    pub data: D,
}

impl<D> Readable for Subrecord<D>
where
    D: Readable,
{
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            header: SubrecordHeader::read(reader)?,
            data: D::read(reader)?,
        })
    }
}
