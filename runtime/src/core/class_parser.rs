use std::io::{Cursor, Error, ErrorKind};

use byteorder::{BigEndian, ReadBytesExt};

use utils::ResultIterator;

use crate::core::klass::attribute::{AttributeInfo, ExceptionHandler, LineNumber, LocalVariable};
use crate::core::klass::constant_pool::{ConstantPool, CpInfo};
use crate::core::klass::field::FieldInfo;
use crate::core::klass::klass::Klass;
use crate::core::klass::method::MethodInfo;

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
    fn parse(&mut self) -> Result<Klass, Error> {
        let access_flags = self.cursor.read_u16::<BigEndian>()?;
        let this_class = self
            .parse_class_pointer()?
            .expect("this_class is not found!");
        let super_class = self.parse_class_pointer()?;
        let interfaces = self.parse_interfaces()?;
        let fields = self.parse_fields()?;
        let methods = self.parse_methods()?;
        let attributes = self.parse_attributes()?;

        Ok(Klass {
            minor_version: self.minor_version,
            major_version: self.major_version,
            constant_pool: self.constant_pool.clone(),
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }

    fn parse_class_pointer(&mut self) -> Result<Option<String>, Error> {
        let ind = self.cursor.read_u16::<BigEndian>()?;
        if ind == 0 {
            return Ok(None);
        }

        return match self.constant_pool.get(ind as usize) {
            CpInfo::Class { name_index: index } => {
                self.get_utf8_from_pool(index.clone()).map(|str| Some(str))
            }
            other => Err(Error::new(
                ErrorKind::Other,
                format!("constant_pool[{}] should point to Class info!", ind),
            )),
        };
    }

    fn parse_interfaces(&mut self) -> Result<Vec<String>, Error> {
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
        let name = self.get_utf8_from_pool(name_index)?;
        let descriptor_index = self.cursor.read_u16::<BigEndian>()?;
        let attributes = self.parse_attributes()?;

        return Ok(FieldInfo {
            access_flags,
            name,
            descriptor: self.get_utf8_from_pool(descriptor_index)?,
            attributes,
        });
    }

    fn parse_methods(&mut self) -> Result<Vec<MethodInfo>, Error> {
        let mut methods: Vec<MethodInfo> = Vec::new();
        let method_count = self.cursor.read_u16::<BigEndian>()?;
        for _i in 0..method_count {
            methods.push(self.parse_method()?);
        }
        return Ok(methods);
    }

    fn parse_method(&mut self) -> Result<MethodInfo, Error> {
        let access_flags = self.cursor.read_u16::<BigEndian>()?;
        let name_index = self.cursor.read_u16::<BigEndian>()?;
        let descriptor_index = self.cursor.read_u16::<BigEndian>()?;
        let attributes = self.parse_attributes()?;
        return MethodInfo::from(
            access_flags,
            self.get_utf8_from_pool(name_index)?,
            self.get_utf8_from_pool(descriptor_index)?,
            attributes,
        );
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
        let attribute_name = self.get_utf8_from_pool(name_index)?;
        let attribute_length = self.cursor.read_u32::<BigEndian>()?;

        return match attribute_name.as_str() {
            "ConstantValue" => Ok(AttributeInfo::ConstantValue {
                constant_value_index: self.cursor.read_u16::<BigEndian>()?,
            }),
            "Code" => {
                let max_stack = self.cursor.read_u16::<BigEndian>()?;
                let max_locals = self.cursor.read_u16::<BigEndian>()?;

                let code_length = self.cursor.read_u32::<BigEndian>()?;
                let code = (0..code_length)
                    .map(|_| self.cursor.read_u8())
                    .collect_to_result()?;

                let exception_table_length = self.cursor.read_u16::<BigEndian>()?;

                let exception_table = (0..exception_table_length)
                    .map(|_| -> Result<ExceptionHandler, Error> {
                        Ok(ExceptionHandler {
                            start_pc: self.cursor.read_u16::<BigEndian>()?,
                            end_pc: self.cursor.read_u16::<BigEndian>()?,
                            handler_pc: self.cursor.read_u16::<BigEndian>()?,
                            catch_type: self.cursor.read_u16::<BigEndian>()?,
                        })
                    })
                    .collect_to_result()?;

                let attributes = self.parse_attributes()?;

                Ok(AttributeInfo::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                })
            }
            "LineNumberTable" => {
                let table_length = self.cursor.read_u16::<BigEndian>()?;
                let line_number_table = (0..table_length)
                    .map(|_| -> Result<LineNumber, Error> {
                        Ok(LineNumber {
                            start_pc: self.cursor.read_u16::<BigEndian>()?,
                            line_number: self.cursor.read_u16::<BigEndian>()?,
                        })
                    })
                    .collect_to_result()?;

                Ok(AttributeInfo::LineNumberTable { line_number_table })
            }
            "LocalVariableTable" => {
                let table_length = self.cursor.read_u16::<BigEndian>()?;
                let local_variable_table = (0..table_length)
                    .map(|_| -> Result<LocalVariable, Error> {
                        Ok(LocalVariable {
                            start_pc: self.cursor.read_u16::<BigEndian>()?,
                            length: self.cursor.read_u16::<BigEndian>()?,
                            name_index: self.cursor.read_u16::<BigEndian>()?,
                            descriptor_index: self.cursor.read_u16::<BigEndian>()?,
                            index: self.cursor.read_u16::<BigEndian>()?,
                        })
                    })
                    .collect_to_result()?;

                Ok(AttributeInfo::LocalVariableTable {
                    local_variable_table,
                })
            }
            "SourceFile" => Ok(AttributeInfo::SourceFile {
                sourcefile_index: self.cursor.read_u16::<BigEndian>()?,
            }),
            "Signature" => Ok(AttributeInfo::Signature {
                signature_index: self.cursor.read_u16::<BigEndian>()?,
            }),
            unimplemented_attribute => {
                eprintln!(
                    "Unimplemented attribute: {} wrapping in custom attribute",
                    unimplemented_attribute
                );

                let info = (0..attribute_length)
                    .map(|_| self.cursor.read_u8())
                    .collect_to_result()?;

                Ok(AttributeInfo::Custom {
                    attribute_name_index: name_index,
                    attribute_length,
                    info,
                })
            }
        };
    }

    fn get_utf8_from_pool(&self, index: u16) -> Result<String, Error> {
        return if let CpInfo::Utf8 { string: str } = self.constant_pool.get(index as usize) {
            Ok(str.clone())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("Index: {} must point to UTF8 constant!", index),
            ))
        };
    }
}
