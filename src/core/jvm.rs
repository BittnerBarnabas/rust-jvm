use crate::core::constant_pool::{ConstantPool, CpInfo};
use crate::core::jvm::AccessFlag::{
    Abstract, Annotation, Enum, Final, Interface, Module, Public, Super, Synthetic,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum JvmValue {
    Boolean { val: bool },
    Byte { val: i8 },
    Short { val: i16 },
    Int { val: i32 },
    Long { val: i64 },
    Float { val: f32 },
    Double { val: f64 },
    Char { val: char },
    ObjRef { val: usize },
    ReturnAddr { val: usize },
}

impl JvmValue {
    pub fn null_obj() -> JvmValue {
        JvmValue::ObjRef { val: 0 }
    }
}

impl fmt::Display for JvmValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JvmValue::Boolean { val: v } => write!(f, "{}", v),
            JvmValue::Byte { val: v } => write!(f, "{}", v),
            JvmValue::Short { val: v } => write!(f, "{}", v),
            JvmValue::Int { val: v } => write!(f, "{}", v),
            JvmValue::Long { val: v } => write!(f, "{}", v),
            JvmValue::Float { val: v } => write!(f, "{}", v),
            JvmValue::Double { val: v } => write!(f, "{}", v),
            JvmValue::Char { val: v } => write!(f, "{}", v),
            JvmValue::ObjRef { val: v } => write!(f, "{}", v),
            JvmValue::ReturnAddr { val: v } => write!(f, "{}", v),
        }
    }
}

const ACC_PUBLIC: u16 = 0x0001;
const ACC_FINAL: u16 = 0x0010;
const ACC_SUPER: u16 = 0x0020;
const ACC_INTERFACE: u16 = 0x0200;
const ACC_ABSTRACT: u16 = 0x0400;
const ACC_SYNTHETIC: u16 = 0x1000;
const ACC_ANNOTATION: u16 = 0x2000;
const ACC_ENUM: u16 = 0x4000;
const ACC_MODULE: u16 = 0x8000;

pub enum AccessFlag {
    Public,
    Final,
    Super,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
    Enum,
    Module,
}

impl AccessFlag {
    pub fn unmask_u16(value: u16) -> Vec<AccessFlag> {
        let mut access_flags: Vec<AccessFlag> = Vec::new();

        if value & ACC_PUBLIC != 0 {
            access_flags.push(Public);
        }
        if value & ACC_FINAL != 0 {
            access_flags.push(Final)
        }
        if value & ACC_SUPER != 0 {
            access_flags.push(Super)
        }
        if value & ACC_INTERFACE != 0 {
            access_flags.push(Interface)
        }
        if value & ACC_ABSTRACT != 0 {
            access_flags.push(Abstract)
        }
        if value & ACC_SYNTHETIC != 0 {
            access_flags.push(Synthetic)
        }
        if value & ACC_ANNOTATION != 0 {
            access_flags.push(Annotation)
        }
        if value & ACC_ENUM != 0 {
            access_flags.push(Enum)
        }
        if value & ACC_MODULE != 0 {
            access_flags.push(Module)
        }
        return access_flags;
    }
}

pub enum AttributeInfo {
    ConstantValue {},
    Custom {
        attribute_name_index: u16,
        attribute_length: u32,
        info: Vec<u8>,
    },
}

pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

pub struct Klass {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: Vec<AccessFlag>,
    pub this_class: CpInfo,
    pub super_class: Option<CpInfo>,
    pub interfaces: Vec<CpInfo>,
}
