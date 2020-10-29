use crate::file::read::{EspReader, Readable};
use byteorder::{LittleEndian, ReadBytesExt};
use esplugin_derive::*;
use std::ffi::CString;
use std::io;
use std::io::{BufRead, Read};
use std::mem::size_of;

#[derive(Debug)]
pub struct VariantBytes {
    bytes: [u8; 4],
}

impl Readable for VariantBytes {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut buf = [0u8; 4];
        reader.buf_reader.read_exact(&mut buf)?;
        reader.progress(4i64);
        
        Ok(Self { bytes: buf })
    }
}

impl VariantBytes {
    pub fn boolean(&self) -> bool {
        u32::from_be_bytes(self.bytes.clone()) > 0
    }

    pub fn int(&self) -> u32 {
        u32::from_be_bytes(self.bytes.clone())
    }

    pub fn float(&self) -> f32 {
        f32::from_be_bytes(self.bytes.clone())
    }

    pub fn lstring(&self) -> u32 {
        u32::from_be_bytes(self.bytes.clone())
    }
}

#[derive(Debug, Readable)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Default, Readable)]
pub struct VersionControlInfo {
    pub vc_day: u8,
    pub vc_month: u8,
    pub vc_previous_id: u8,
    pub vc_current_id: u8,
}

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
            Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid ZString"))
        }
    }
}

impl Readable for char {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let byte = u8::read(reader)?;
        Ok(byte.into())
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
