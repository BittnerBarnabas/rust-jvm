use std::io::{Cursor, Error, ErrorKind};

use byteorder::{BigEndian, ReadBytesExt};

use utils::ResultIterator;

use crate::core::constant_pool::{ConstantPool, CpInfo};
use crate::core::jvm::{AccessFlag, FieldInfo};
use crate::core::jvm::{AttributeInfo, Klass};

const CLASS_MAGIC_NUMBER: u32 = 0xCAFEBABE;

pub struct ClassParser {
    bytes: Vec<u8>,
}

struct ClassParserImpl {
    cursor: Cursor<Vec<u8>>,
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstantPool,
}

impl ClassParser {
    pub fn from(bytes: Vec<u8>) -> ClassParser {
        ClassParser { bytes }
    }

    pub fn parse_class(&self) -> Result<Klass, Error> {
        let mut cursor = Cursor::new(self.bytes.clone());

        ClassParser::validate_magic(&mut cursor)?;
        let minor_version = cursor.read_u16::<BigEndian>()?;
        let major_version = cursor.read_u16::<BigEndian>()?;

        let constant_pool_count = cursor.read_u16::<BigEndian>()?;

        let mut constant_pool = ConstantPool::create(&mut cursor, constant_pool_count as usize)?;

        ClassParserImpl {
            cursor,
            major_version,
            minor_version,
            constant_pool,
        }
        .parse()
    }

    fn validate_magic(cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let magic = cursor.read_u32::<BigEndian>()?;
        if magic != CLASS_MAGIC_NUMBER {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Incorrect magic number: {}, maybe not .class format?",
                    magic
                ),
            ));
        }
        return Ok(());
    }
}

impl ClassParserImpl {
    pub fn parse(&mut self) -> Result<Klass, Error> {
        let access_flags = AccessFlag::unmask_u16(self.cursor.read_u16::<BigEndian>()?);
        let this_class = self
            .parse_class_pointer()?
            .expect("this_class is not found!");
        let super_class = self.parse_class_pointer()?;
        let interfaces = self.parse_interfaces()?;

        Ok(Klass {
            minor_version: self.minor_version,
            major_version: self.major_version,
            constant_pool: self.constant_pool.clone(),
            access_flags,
            this_class,
            super_class,
            interfaces,
        })
    }

    fn parse_class_pointer(&mut self) -> Result<Option<CpInfo>, Error> {
        let ind = self.cursor.read_u16::<BigEndian>()?;
        if ind == 0 {
            return Ok(None);
        }

        return match self.constant_pool.get(ind as usize) {
            class_info @ CpInfo::Class { name_index: _ } => Ok(Some(class_info.clone())),
            other => Err(Error::new(
                ErrorKind::Other,
                format!("constant_pool[{}] should point to Class info!", ind),
            )),
        };
    }

    fn parse_interfaces(&mut self) -> Result<Vec<CpInfo>, Error> {
        let interfaces_count = self.cursor.read_u16::<BigEndian>()?;
        let interfaces = (0..interfaces_count)
            .map(|_| {
                self.parse_class_pointer().and_then(|intf| {
                    intf.ok_or(Error::new(ErrorKind::Other, "Interface not found"))
                })
            })
            .collect_to_result()?;
        Ok(interfaces)
    }

    fn parse_fields(&mut self) -> Result<Vec<FieldInfo>, Error> {
        let mut fields: Vec<FieldInfo> = Vec::new();
        let fields_count = self.cursor.read_u16::<BigEndian>()?;
        for _i in 0..fields_count {
            fields.push(self.parse_field()?);
        }
        return Ok(fields);
    }

    fn parse_field(&mut self) -> Result<FieldInfo, Error> {
        let access_flags = self.cursor.read_u16::<BigEndian>()?;
        let name_index = self.cursor.read_u16::<BigEndian>()?;
        let descriptor_index = self.cursor.read_u16::<BigEndian>()?;
        let attributes = self.parse_attributes()?;
        return Ok(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        });
    }

    fn parse_attributes(&mut self) -> Result<Vec<AttributeInfo>, Error> {
        let attributes_count = self.cursor.read_u16::<BigEndian>()?;

        let attributes = (0..attributes_count)
            .map(|_| self.parse_attribute())
            .collect_to_result()?;

        return Ok(attributes);
    }

    fn parse_attribute(&mut self) -> Result<AttributeInfo, Error> {
        let name_index = self.cursor.read_u16::<BigEndian>()?;
        return Ok(AttributeInfo::ConstantValue {});
    }
}
