pub use crate::file::read::{EspReader, Readable};
pub use crate::file::types::*;
pub use esplugin_derive::*;

#[derive(Debug, Readable)]
pub struct EDIDData {
    pub editor_id: ZString,
}

#[derive(Debug, Readable)]
pub struct CNAMData {
    pub rgb: RGB,
}
