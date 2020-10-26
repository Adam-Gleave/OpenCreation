use crate::file::read::{EspReader, Readable};
use std::fmt::Debug;
use std::io;

#[derive(Debug)]
pub struct Record<H, D>
where
    D: Readable + Debug,
    H: Readable + Debug,
{
    pub header: H,
    pub data: D,
}

impl<H, D> Readable for Record<H, D>
where
    D: Readable + Debug,
    H: Readable + Debug,
{
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            header: H::read(reader)?,
            data: D::read(reader)?,
        })
    }
}
