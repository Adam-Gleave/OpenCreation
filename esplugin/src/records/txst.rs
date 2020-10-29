use crate::file::read::{EspReader, Readable};
use crate::file::types::*;
use crate::records::record::{Record, RecordFlags};
use crate::subrecords::common::EDIDData;
use crate::subrecords::subrecord::{Subrecord, SubrecordType};
use bitflags::bitflags;
use esplugin_derive::*;
use std::io;

pub type TXSTRecord = Record<RecordFlags, TXSTData>;

pub type EDIDSubrecord = Subrecord<EDIDData>;
pub type OBNDSubrecord = Subrecord<OBNDData>;
pub type TX00Subrecord = Subrecord<TX00Data>;
pub type TX01Subrecord = Subrecord<TX01Data>;
pub type TX02Subrecord = Subrecord<TX02Data>;
pub type TX03Subrecord = Subrecord<TX03Data>;
pub type TX04Subrecord = Subrecord<TX04Data>;
pub type TX05Subrecord = Subrecord<TX05Data>;
pub type TX06Subrecord = Subrecord<TX06Data>;
pub type TX07Subrecord = Subrecord<TX07Data>;
pub type DODTSubrecord = Subrecord<DODTData>;
pub type DNAMSubrecord = Subrecord<DNAMData>;

bitflags! {
    #[derive(Default)]
    pub struct DecalFlags: u8 {
        const PARALLAX          = 0x01;
        const ALPHA_BLENDING    = 0x02;
        const ALPHA_TESTING     = 0x04;
        const NOT_4_SUBTEXTURES = 0x08;
    }
}

impl Readable for DecalFlags {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(DecalFlags::from_bits(u8::read(reader)?).unwrap_or(Default::default()))
    }
}

bitflags! {
    #[derive(Default)]
    pub struct TextureFlags: u16 {
        const HAS_NO_SPECULAR        = 0x01;
        const FACEGEN_TEXTURES       = 0x02;
        const HAS_MODEL_SPACE_NORMAL = 0x04;
    }
}

impl Readable for TextureFlags {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(TextureFlags::from_bits(u16::read(reader)?).unwrap_or(Default::default()))
    }
}

#[derive(Debug, Readable)]
pub struct OBNDData {
    pub unknown: Vec<u8>,
}

#[derive(Debug, Readable)]
pub struct TX00Data {
    pub texture_00: ZString,
}

#[derive(Debug, Readable)]
pub struct TX01Data {
    pub texture_01: ZString,
}

#[derive(Debug, Readable)]
pub struct TX02Data {
    pub texture_02: ZString,
}

#[derive(Debug, Readable)]
pub struct TX03Data {
    pub texture_03: ZString,
}

#[derive(Debug, Readable)]
pub struct TX04Data {
    pub texture_04: ZString,
}

#[derive(Debug, Readable)]
pub struct TX05Data {
    pub texture_05: ZString,
}

#[derive(Debug, Readable)]
pub struct TX06Data {
    pub texture_06: ZString,
}

#[derive(Debug, Readable)]
pub struct TX07Data {
    pub texture_07: ZString,
}

#[derive(Debug, Readable)]
pub struct DODTData {
    min_width: f32,
    max_width: f32,
    min_height: f32,
    max_height: f32,
    depth: f32,
    shininess: f32,
    parallax_scale: f32,
    parallax_passes: u8,
    flags: DecalFlags,
    unknown: u16,
    color: RGB,
}

#[derive(Debug, Readable)]
pub struct DNAMData {
    pub flags: TextureFlags,
}

#[derive(Debug, Default)]
pub struct TXSTData {
    pub edid: Option<EDIDSubrecord>,
    pub obnd: Option<OBNDSubrecord>,
    pub tx00: Option<TX00Subrecord>,
    pub tx01: Option<TX01Subrecord>,
    pub tx02: Option<TX02Subrecord>,
    pub tx03: Option<TX03Subrecord>,
    pub tx04: Option<TX04Subrecord>,
    pub tx05: Option<TX05Subrecord>,
    pub tx06: Option<TX06Subrecord>,
    pub tx07: Option<TX07Subrecord>,
    pub dodt: Option<DODTSubrecord>,
    pub dnam: Option<DNAMSubrecord>,
}

impl Readable for TXSTData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: TXSTData = Default::default();
        
        while reader.record_left() > 0 {
            let subrecord_type = reader.read_subrecord_type()?;

            match subrecord_type {
                SubrecordType::EDID => record.edid = Some(EDIDSubrecord::read(reader)?),
                SubrecordType::OBND => record.obnd = Some(OBNDSubrecord::read(reader)?),
                SubrecordType::TX00 => record.tx00 = Some(TX00Subrecord::read(reader)?),
                SubrecordType::TX01 => record.tx01 = Some(TX01Subrecord::read(reader)?),
                SubrecordType::TX02 => record.tx02 = Some(TX02Subrecord::read(reader)?),
                SubrecordType::TX03 => record.tx03 = Some(TX03Subrecord::read(reader)?),
                SubrecordType::TX04 => record.tx04 = Some(TX04Subrecord::read(reader)?),
                SubrecordType::TX05 => record.tx05 = Some(TX05Subrecord::read(reader)?),
                SubrecordType::TX06 => record.tx06 = Some(TX06Subrecord::read(reader)?),
                SubrecordType::TX07 => record.tx07 = Some(TX07Subrecord::read(reader)?),
                SubrecordType::DODT => record.dodt = Some(DODTSubrecord::read(reader)?),
                SubrecordType::DNAM => record.dnam = Some(DNAMSubrecord::read(reader)?),
                _ => {
                    let msg = format!("Unexpected subrecord {:#?} found in TXST", subrecord_type.num());
                    return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
                },
            }
        }

        Ok(record)
    }
}
