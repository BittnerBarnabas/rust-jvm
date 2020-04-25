use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use cesu8::from_java_cesu8;
use std::borrow::Borrow;
use std::fs::File;
use std::io;
use std::io::{Cursor, Error, ErrorKind};

const CONSTANT_Utf8: u8 = 0x01;
const CONSTANT_Integer: u8 = 0x03;
const CONSTANT_Float: u8 = 0x04;
const CONSTANT_Long: u8 = 0x05;
const CONSTANT_Double: u8 = 0x06;
const CONSTANT_Class: u8 = 0x07;
const CONSTANT_String: u8 = 0x08;
const CONSTANT_Fieldref: u8 = 0x09;
const CONSTANT_Methodref: u8 = 0x0A;
const CONSTANT_InterfaceMethodref: u8 = 0x0B;
const CONSTANT_NameAndType: u8 = 0x0C;
const CONSTANT_MethodHandle: u8 = 0x0F;
const CONSTANT_MethodType: u8 = 0x10;
const CONSTANT_Dynamic: u8 = 0x11;
const CONSTANT_InvokeDynamic: u8 = 0x12;
const CONSTANT_Module: u8 = 0x13;
const CONSTANT_Package: u8 = 0x14;

pub struct ConstantPool {
    pool: Vec<CpInfo>,
}

impl ConstantPool {
    pub fn from(constants: Vec<CpInfo>) -> ConstantPool {
        ConstantPool { pool: constants }
    }

    pub fn create(cursor: &mut Cursor<Vec<u8>>, count: usize) -> Result<ConstantPool, Error> {
        let mut constant_pool: Vec<CpInfo> = Vec::new();
        for _i in 1..count {
            let cp_info = CpInfo::create(cursor)?;
            constant_pool.push(cp_info);
        }
        Ok(ConstantPool::from(constant_pool))
    }

    pub fn get(&self, ind: usize) -> &CpInfo {
        &self.pool[ind - 1]
    }
}

#[derive(Clone)]
pub enum CpInfo {
    Utf8 {
        string: String,
    },
    Integer {
        bytes: u32,
    },
    Float {
        bytes: u32,
    },
    Long {
        high_bytes: u32,
        low_bytes: u32,
    },
    Double {
        high_bytes: u32,
        low_bytes: u32,
    },
    Class {
        name_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
}

impl CpInfo {
    pub fn create(cursor: &mut Cursor<Vec<u8>>) -> Result<CpInfo, Error> {
        match cursor.read_u8()? {
            CONSTANT_Utf8 => {
                let length = cursor.read_u16::<BigEndian>()?;
                let mut bytes: Vec<u8> = Vec::new();
                for _n in 0..length {
                    bytes.push(cursor.read_u8()?)
                }

                let decoded_string =
                    cesu8::from_java_cesu8(&bytes).expect("CESU-8 Decoding Failed!");

                Ok(CpInfo::Utf8 {
                    string: decoded_string.into_owned(),
                })
            }
            CONSTANT_Integer => Ok(CpInfo::Integer {
                bytes: cursor.read_u32::<BigEndian>()?,
            }),
            CONSTANT_Float => Ok(CpInfo::Float {
                bytes: cursor.read_u32::<BigEndian>()?,
            }),
            CONSTANT_Double => Ok(CpInfo::Double {
                high_bytes: cursor.read_u32::<BigEndian>()?,
                low_bytes: cursor.read_u32::<BigEndian>()?,
            }),
            CONSTANT_Long => Ok(CpInfo::Long {
                high_bytes: cursor.read_u32::<BigEndian>()?,
                low_bytes: cursor.read_u32::<BigEndian>()?,
            }),
            CONSTANT_Class => Ok(CpInfo::Class {
                name_index: cursor.read_u16::<BigEndian>()?,
            }),
            CONSTANT_Fieldref => Ok(CpInfo::FieldRef {
                class_index: cursor.read_u16::<BigEndian>()?,
                name_and_type_index: cursor.read_u16::<BigEndian>()?,
            }),
            CONSTANT_Methodref => Ok(CpInfo::MethodRef {
                class_index: cursor.read_u16::<BigEndian>()?,
                name_and_type_index: cursor.read_u16::<BigEndian>()?,
            }),
            CONSTANT_NameAndType => Ok(CpInfo::NameAndType {
                name_index: cursor.read_u16::<BigEndian>()?,
                descriptor_index: cursor.read_u16::<BigEndian>()?,
            }),
            CONSTANT_MethodHandle => Ok(CpInfo::MethodHandle {
                reference_kind: cursor.read_u8()?,
                reference_index: cursor.read_u16::<BigEndian>()?,
            }),
            other => Err(Error::new(
                ErrorKind::Other,
                format!("Unknown cp_tag: {}", other),
            )),
        }
    }
}
