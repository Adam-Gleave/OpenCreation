use crate::file::read::{EspReader, Readable};
use crate::file::types::*;
use crate::records::record::{Record, RecordFlags};
use crate::subrecords::common::EDIDData;
use crate::subrecords::subrecord::{Subrecord, SubrecordType};
use bitflags::bitflags;
use esplugin_derive::*;
use std::io::Read;
use std::io;

pub type CLASRecord = Record<RecordFlags, CLASData>;

pub type EDIDSubrecord = Subrecord<EDIDData>;
pub type FULLSubrecord = Subrecord<FULLData>;
pub type DESCSubrecord = Subrecord<DESCData>;
pub type ICONSubrecord = Subrecord<ICONData>;
pub type DATASubrecord = Subrecord<DATAData>;

bitflags! {
    #[derive(Default)]
    pub struct ClasFlags: u8 {
        const GUARD = 0x01;
    }
}

impl Readable for ClasFlags {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(ClasFlags::from_bits(u8::read(reader)?).unwrap_or(Default::default()))
    }
}

#[derive(Debug, Readable)]
pub struct FULLData {
    pub name: LString,
}

#[derive(Debug, Readable)]
pub struct DESCData {
    pub description: LString,
}

#[derive(Debug, Readable)]
pub struct ICONData {
    pub image: ZString,
}

#[derive(Debug)]
pub struct DATAData {
    pub unknown: u32,
    pub training_skill: u8,
    pub training_level: u8,
    pub skill_weights: [u8; 18],
    pub bleedout_default: f32,
    pub voice_points: u32,
    pub health_weight: u8,
    pub magicka_weight: u8,
    pub stamina_weight: u8,
    pub flags: ClasFlags,
}

impl Readable for DATAData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        Ok(Self {
            unknown: u32::read(reader)?,
            training_skill: u8::read(reader)?,
            training_level: u8::read(reader)?,
            skill_weights: {
                let mut buf = [0u8; 18];
                reader.buf_reader.read_exact(&mut buf)?;
                reader.progress(18i64);
                buf
            },
            bleedout_default: f32::read(reader)?,
            voice_points: u32::read(reader)?,
            health_weight: u8::read(reader)?,
            magicka_weight: u8::read(reader)?,
            stamina_weight: u8::read(reader)?,
            flags: ClasFlags::read(reader)?,
        })
    }
}

#[derive(Debug, Default)]
pub struct CLASData {
    pub edid: Option<EDIDSubrecord>,
    pub full: Option<FULLSubrecord>,
    pub desc: Option<DESCSubrecord>,
    pub icon: Option<ICONSubrecord>,
    pub data: Option<DATASubrecord>,
}

impl Readable for CLASData {
    fn read(reader: &mut EspReader) -> io::Result<Self> {
        let mut record: CLASData = Default::default();
        
        while reader.record_left() > 0 {
            let subrecord_type = reader.read_subrecord_type()?;

            match subrecord_type {
                SubrecordType::EDID => record.edid = Some(EDIDSubrecord::read(reader)?),
                SubrecordType::FULL => record.full = Some(FULLSubrecord::read(reader)?),
                SubrecordType::DESC => record.desc = Some(DESCSubrecord::read(reader)?),
                SubrecordType::ICON => record.icon = Some(ICONSubrecord::read(reader)?),
                SubrecordType::DATA => record.data = Some(DATASubrecord::read(reader)?),
                _ => {
                    let msg = format!("Unexpected subrecord code {:#?} found in CLAS", subrecord_type);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
                },
            }
        }
        
        Ok(record)
    }
}
