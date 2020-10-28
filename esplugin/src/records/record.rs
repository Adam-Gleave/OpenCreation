use crate::file::read::{EspReader, Readable};
use crate::records::header::RecordHeader;
use esplugin_derive::*;
use std::fmt::Debug;

#[derive(Debug, Readable)]
#[record_header(true)]
#[size_var(header, size)]
pub struct Record<HeaderFlags, Data>
where
    HeaderFlags: Readable,
    Data: Readable,
{
    pub header: RecordHeader<HeaderFlags>,
    pub data: Data,
}
