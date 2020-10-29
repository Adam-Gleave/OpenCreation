use crate::records::record::RecordType;
use crate::subrecords::subrecord::SubrecordType;
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::mem::size_of;

pub struct EspReader {
    pub buf_reader: BufReader<File>,
    group_to_read: i64,
    record_to_read: i64,
    subrecord_to_read: i64,
}

impl EspReader {
    pub fn new(file: File) -> Self {
        Self {
            buf_reader: BufReader::new(file),
            group_to_read: 0,
            record_to_read: 0,
            subrecord_to_read: 0,
        }
    }

    pub fn next_group_data(&mut self, data_size: u32) {
        self.group_to_read = (data_size - 24) as i64;
    }

    pub fn group_left(&self) -> i64 {
        self.group_to_read
    }

    pub fn next_record_data(&mut self, data_size: u32) {
        self.record_to_read = data_size as i64;
    }

    pub fn record_left(&self) -> i64 {
        self.record_to_read
    }

    pub fn next_subrecord_data(&mut self, data_size: u16) {
        self.subrecord_to_read = data_size as i64;
    }

    pub fn subrecord_left(&self) -> i64 {
        self.subrecord_to_read
    }

    pub fn read_record_type(&mut self) -> io::Result<RecordType> {
        let code = self.buf_reader.read_u32::<BigEndian>()?;
        self.progress(size_of::<u32>() as i64);
        Ok(RecordType::from(code))
    }

    pub fn read_subrecord_type(&mut self) -> io::Result<SubrecordType> {
        let code = self.buf_reader.read_u32::<BigEndian>()?;
        self.progress(size_of::<u32>() as i64);
        Ok(SubrecordType::from(code))
    }

    pub fn progress(&mut self, num_bytes: i64) {
        self.group_to_read -= num_bytes;
        self.record_to_read -= num_bytes;
        self.subrecord_to_read -= num_bytes;
    }
}

pub trait Readable
where
    Self: Sized,
{
    fn read(reader: &mut EspReader) -> io::Result<Self>;
}
