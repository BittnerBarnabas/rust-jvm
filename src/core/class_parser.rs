use crate::core::constant_pool;
use crate::core::constant_pool::{ConstantPool, CpInfo};
use crate::core::jvm::{AccessFlag, FieldInfo};
use crate::core::jvm::{AttributeInfo, Klass};
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io;
use std::io::{Cursor, Error, ErrorKind};
use utils::ResultIterator;

const CLASS_MAGIC_NUMBER: u32 = 0xCAFEBABE;

pub fn parse_class(bytes: Vec<u8>) -> Result<Klass, Error> {
    let mut cursor = Cursor::new(bytes);
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

    let minor_version = cursor.read_u16::<BigEndian>()?;
    let major_version = cursor.read_u16::<BigEndian>()?;

    let constant_pool_count = cursor.read_u16::<BigEndian>()?;

    let mut constant_pool: ConstantPool =
        ConstantPool::create(&mut cursor, constant_pool_count as usize)?;

    let access_flags = AccessFlag::unmask_u16(cursor.read_u16::<BigEndian>()?);

    let this_class = resolve_class(&mut cursor, &constant_pool)?.expect("this_class is not found!");

    let super_class = resolve_class(&mut cursor, &constant_pool)?;

    let interfaces_count = cursor.read_u16::<BigEndian>()?;
    let mut interfaces: Vec<CpInfo> = Vec::new();
    for _i in 0..interfaces_count {
        interfaces.push(resolve_class(&mut cursor, &constant_pool)?.expect("Interface not found"))
    }

    Ok(Klass {
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
    })
}

fn resolve_class(
    cursor: &mut Cursor<Vec<u8>>,
    constant_pool: &ConstantPool,
) -> Result<Option<CpInfo>, Error> {
    let ind = cursor.read_u16::<BigEndian>()?;
    if ind == 0 {
        return Ok(None);
    }

    return match constant_pool.get(ind as usize) {
        class_info @ CpInfo::Class { name_index: _ } => Ok(Some(class_info.clone())),
        other => Err(Error::new(
            ErrorKind::Other,
            format!("constant_pool[{}] should point to Class info!", ind),
        )),
    };
}

fn resolve_fields(cursor: &mut Cursor<Vec<u8>>) -> Result<Vec<FieldInfo>, Error> {
    let mut fields: Vec<FieldInfo> = Vec::new();
    let fields_count = cursor.read_u16::<BigEndian>()?;
    for _i in 0..fields_count {
        fields.push(resolve_field(cursor)?);
    }
    return Ok(fields);
}

fn resolve_field(cursor: &mut Cursor<Vec<u8>>) -> Result<FieldInfo, Error> {
    let access_flags = cursor.read_u16::<BigEndian>()?;
    let name_index = cursor.read_u16::<BigEndian>()?;
    let descriptor_index = cursor.read_u16::<BigEndian>()?;
    let attributes = parse_attributes(cursor)?;
    return Ok(FieldInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    });
}

fn parse_attributes(cursor: &mut Cursor<Vec<u8>>) -> Result<Vec<AttributeInfo>, Error> {
    let mut attribute_info: Vec<AttributeInfo> = Vec::new();
    let attributes_count = cursor.read_u16::<BigEndian>()?;

    let attributes = (0..attributes_count)
        .map(|_| parse_attribute(cursor))
        .collect_to_result()?;

    return Ok(attributes);
}

fn parse_attribute(cursor: &mut Cursor<Vec<u8>>) -> Result<AttributeInfo, Error> {
    let name_index = cursor.read_u16::<BigEndian>()?;
    return Ok(attribute_info);
}
