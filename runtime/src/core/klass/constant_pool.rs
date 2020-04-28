use std::io::{Cursor, Error, ErrorKind};

use byteorder::{BigEndian, ReadBytesExt};

const CONSTANT_UTF8: u8 = 0x01;
const CONSTANT_INTEGER: u8 = 0x03;
const CONSTANT_FLOAT: u8 = 0x04;
const CONSTANT_LONG: u8 = 0x05;
const CONSTANT_DOUBLE: u8 = 0x06;
const CONSTANT_CLASS: u8 = 0x07;
const CONSTANT_STRING: u8 = 0x08;
const CONSTANT_FIELDREF: u8 = 0x09;
const CONSTANT_METHODREF: u8 = 0x0A;
const CONSTANT_INTERFACE_METHODREF: u8 = 0x0B;
const CONSTANT_NAME_AND_TYPE: u8 = 0x0C;
const CONSTANT_METHOD_HANDLE: u8 = 0x0F;
const CONSTANT_METHOD_TYPE: u8 = 0x10;
const CONSTANT_DYNAMIC: u8 = 0x11;
const CONSTANT_INVOKE_DYNAMIC: u8 = 0x12;
const CONSTANT_MODULE: u8 = 0x13;
const CONSTANT_PACKAGE: u8 = 0x14;

#[derive(Clone)]
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
    String {
        string_index: u16,
    },
}

impl CpInfo {
    pub fn create(cursor: &mut Cursor<Vec<u8>>) -> Result<CpInfo, Error> {
        match cursor.read_u8()? {
            CONSTANT_UTF8 => {
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
            CONSTANT_INTEGER => Ok(CpInfo::Integer {
                bytes: cursor.read_u32::<BigEndian>()?,
            }),
            CONSTANT_FLOAT => Ok(CpInfo::Float {
                bytes: cursor.read_u32::<BigEndian>()?,
            }),
            CONSTANT_DOUBLE => Ok(CpInfo::Double {
                high_bytes: cursor.read_u32::<BigEndian>()?,
                low_bytes: cursor.read_u32::<BigEndian>()?,
            }),
            CONSTANT_LONG => Ok(CpInfo::Long {
                high_bytes: cursor.read_u32::<BigEndian>()?,
                low_bytes: cursor.read_u32::<BigEndian>()?,
            }),
            CONSTANT_CLASS => Ok(CpInfo::Class {
                name_index: cursor.read_u16::<BigEndian>()?,
            }),
            CONSTANT_FIELDREF => Ok(CpInfo::FieldRef {
                class_index: cursor.read_u16::<BigEndian>()?,
                name_and_type_index: cursor.read_u16::<BigEndian>()?,
            }),
            CONSTANT_METHODREF => Ok(CpInfo::MethodRef {
                class_index: cursor.read_u16::<BigEndian>()?,
                name_and_type_index: cursor.read_u16::<BigEndian>()?,
            }),
            CONSTANT_NAME_AND_TYPE => Ok(CpInfo::NameAndType {
                name_index: cursor.read_u16::<BigEndian>()?,
                descriptor_index: cursor.read_u16::<BigEndian>()?,
            }),
            CONSTANT_METHOD_HANDLE => Ok(CpInfo::MethodHandle {
                reference_kind: cursor.read_u8()?,
                reference_index: cursor.read_u16::<BigEndian>()?,
            }),
            CONSTANT_STRING => Ok(CpInfo::String {
                string_index: cursor.read_u16::<BigEndian>()?,
            }),
            other => Err(Error::new(
                ErrorKind::Other,
                format!("Unknown cp_tag: {}", other),
            )),
        }
    }
}
