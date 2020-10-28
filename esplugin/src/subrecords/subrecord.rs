use crate::file::read::{EspReader, Readable};
use crate::subrecords::header::SubrecordHeader;
use esplugin_derive::*;
use std::fmt::Debug;

#[derive(Debug, Readable)]
pub struct Subrecord<D>
where
    D: Readable,
{
    pub header: SubrecordHeader,
    pub data: D,
}
