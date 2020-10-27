use crate::records::header::RecordType;
use crate::subrecords::header::SubrecordType;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::mem::size_of;

pub struct EspReader {
    buf_reader: BufReader<File>,
    record_to_read: i64,
    subrecord_to_read: i64,
}

impl EspReader {
    pub fn new(file: File) -> Self {
        Self {
            buf_reader: BufReader::new(file),
            record_to_read: 0,
            subrecord_to_read: 0,
        }
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

    pub fn read_u64(&mut self) -> io::Result<u64> {
        let result = self.buf_reader.read_u64::<LittleEndian>()?;
        self.progress(size_of::<u64>() as i64);
        Ok(result)
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let result = self.buf_reader.read_u32::<LittleEndian>()?;
        self.progress(size_of::<u32>() as i64);
        Ok(result)
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        let result = self.buf_reader.read_u16::<LittleEndian>()?;
        self.progress(size_of::<u16>() as i64);
        Ok(result)
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        let result = self.buf_reader.read_u8()?;
        self.progress(size_of::<u8> as i64);
        Ok(result)
    }

    pub fn read_i32(&mut self) -> io::Result<i32> {
        let result = self.buf_reader.read_i32::<LittleEndian>()?;
        self.progress(size_of::<i32>() as i64);
        Ok(result)
    }

    pub fn read_f32(&mut self) -> io::Result<f32> {
        let result: f32 = self.buf_reader.read_f32::<LittleEndian>()?;
        self.progress(size_of::<f32>() as i64);
        Ok(result)
    }

    pub fn read_zstring(&mut self) -> io::Result<String> {
        let mut buf = vec![];
        self.buf_reader.read_until(0u8, &mut buf)?;
        buf.pop();

        self.progress((buf.len() + 1) as i64);
        let result = CString::new(buf);

        if let Ok(cstring) = result {
            Ok(cstring.to_str().unwrap().to_owned())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid ZString",
            ))
        }
    }

    fn progress(&mut self, num_bytes: i64) {
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

pub trait Coded<T> {
    fn code() -> T;
}
