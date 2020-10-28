use crate::file::read::{EspReader, Readable};
use esplugin_derive::*;
use std::fmt::Debug;

#[derive(Debug, Readable)]
pub struct Record<H, D>
where
    D: Readable,
    H: Readable,
{
    pub header: H,
    pub data: D,
}
