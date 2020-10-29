use crate::file::read::{EspReader, Readable};
use crate::groups::header::GroupHeader;
use std::io;
use std::fmt::Debug;

pub const CODE: u32 = 0x47525550;   // "GRUP"

#[derive(Debug, Default)]
pub struct Group<Child>
where
    Child: Readable,
{
    pub header: GroupHeader,
    pub data: Vec<Child>,
}

impl<Child> Readable for Group<Child>
where 
    Child: Readable,
{
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let header = GroupHeader::read(reader)?;
        reader.next_group_data(header.size);

        let mut data = vec![];
        while reader.group_left() > 0 {
            reader.read_record_type()?;
            data.push(Child::read(reader)?);
        }

        Ok(Self { header, data })
    }
}
