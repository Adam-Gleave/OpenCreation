use crate::file::read::{EspReader, Readable};
use esplugin_derive::*;

#[derive(Debug, Readable)]
pub struct SubrecordHeader {
    pub size: u16,
}
