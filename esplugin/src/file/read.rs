use std::fs::File;
use std::io;
use std::io::BufReader;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use crate::records::header::RecordType;
use crate::subrecords::header::SubrecordType;

pub struct EspReader {
    buf_reader: BufReader<File>,
    record_to_read: i64,
}

impl EspReader {
    pub fn new(file: File) -> Self {
        Self { 
            buf_reader: BufReader::new(file), 
            record_to_read: 0,
        }
    }

    pub fn next_record_data(&mut self, data_size: u32) {
        self.record_to_read = data_size as i64;
    }

    pub fn record_left(&self) -> i64 {
        self.record_to_read
    }

    pub fn read_record_type(&mut self) -> io::Result<RecordType> {
        let code = self.buf_reader.read_u32::<BigEndian>()?;
        self.record_to_read -= std::mem::size_of::<u32>() as i64;
        Ok(RecordType::from(code))
    }

        pub fn read_subrecord_type(&mut self) -> io::Result<SubrecordType> {
        let code = self.buf_reader.read_u32::<BigEndian>()?;
        self.record_to_read -= std::mem::size_of::<u32>() as i64;
        Ok(SubrecordType::from(code))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        self.buf_reader.read_u32::<LittleEndian>()
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        self.buf_reader.read_u16::<LittleEndian>()
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        self.buf_reader.read_u8()
    }

    pub fn read_i32(&mut self) -> io::Result<i32> {
        self.buf_reader.read_i32::<LittleEndian>()
    }

    pub fn read_f32(&mut self) -> io::Result<f32> {
        self.buf_reader.read_f32::<LittleEndian>()
    }
}

pub trait Readable where Self: Sized {
    fn read(reader: &mut EspReader) -> io::Result<Self>;
}