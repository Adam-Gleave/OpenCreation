use crate::file::read::{EspReader, Readable};
use crate::subrecords::header::SubrecordHeader;
use esplugin_derive::*;
use std::fmt::Debug;

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
