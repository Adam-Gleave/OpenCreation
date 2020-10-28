use crate::file::read::{EspReader, Readable};
use byteorder::{LittleEndian, ReadBytesExt};
use std::ffi::CString;
use std::io;
use std::io::BufRead;
use std::mem::size_of;

#[derive(Debug, Eq, PartialEq)]
pub struct ZString {
    pub value: String,
}

impl Readable for ZString {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut buf = vec![];
        reader.buf_reader.read_until(0u8, &mut buf)?;
        buf.pop();

        reader.progress((buf.len() + 1) as i64);
        let result = CString::new(buf);

        if let Ok(cstring) = result {
            Ok(Self {
                value: cstring.to_str().unwrap().to_owned(),
            })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid ZString",
            ))
        }
    }
}

impl Readable for u64 {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let result = reader.buf_reader.read_u64::<LittleEndian>()?;
        reader.progress(size_of::<u64>() as i64);
        Ok(result)
    }
}

impl Readable for u32 {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let result = reader.buf_reader.read_u32::<LittleEndian>()?;
        reader.progress(size_of::<u32>() as i64);
        Ok(result)
    }
}

impl Readable for u16 {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let result = reader.buf_reader.read_u16::<LittleEndian>()?;
        reader.progress(size_of::<u16>() as i64);
        Ok(result)
    }
}

impl Readable for u8 {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let result = reader.buf_reader.read_u8()?;
        reader.progress(size_of::<u8>() as i64);
        Ok(result)
    }
}

impl Readable for i32 {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let result = reader.buf_reader.read_i32::<LittleEndian>()?;
        reader.progress(size_of::<i32>() as i64);
        Ok(result)
    }
}

impl Readable for f32 {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let result: f32 = reader.buf_reader.read_f32::<LittleEndian>()?;
        reader.progress(size_of::<f32>() as i64);
        Ok(result)
    }
}

impl<T> Readable for Vec<T>
where
    T: Readable,
{
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut result = Vec::<T>::new();

        while reader.subrecord_left() > 0 {
            result.push(T::read(reader)?);
        }

        Ok(result)
    }
}
