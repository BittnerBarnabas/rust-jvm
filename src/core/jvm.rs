use crate::core::constant_pool::{ConstantPool, CpInfo};
use crate::core::interpreter;
use crate::core::interpreter::interpret;
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
    Void {},
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
            JvmValue::Void {} => write!(f, "Void"),
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

pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

pub enum AttributeInfo {
    ConstantValue {
        constant_value_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionHandler>,
        attributes: Vec<AttributeInfo>,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumber>,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    Signature {
        signature_index: u16,
    },
    Custom {
        attribute_name_index: u16,
        attribute_length: u32,
        info: Vec<u8>,
    },
}

pub struct FieldInfo {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<AttributeInfo>,
}

pub struct MethodInfo {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn get_code(&self) -> Option<&Vec<u8>> {
        let code = self.attributes.iter().find(|att| match att {
            AttributeInfo::Code { .. } => true,
            _ => false,
        });

        return match code {
            Some(AttributeInfo::Code { code, .. }) => Some(code),
            _ => None,
        };
    }
}

pub struct Klass {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: Vec<AccessFlag>,
    pub this_class: CpInfo,
    pub super_class: Option<CpInfo>,
    pub interfaces: Vec<CpInfo>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

pub struct JvmException {}

pub struct StackFrame<'a> {
    previous: Option<&'a StackFrame<'a>>,
}

impl<'a> StackFrame<'a> {
    pub fn new() -> StackFrame<'a> {
        StackFrame { previous: None }
    }

    pub fn execute_method(&self, method: &MethodInfo) -> Result<JvmValue, JvmException> {
        let next_frame = StackFrame {
            previous: Some(self),
        };

        match method.get_code() {
            Some(code) => {
                let result = interpreter::interpret(&next_frame, code);
                return result;
            }
            _ => Err(JvmException {}),
        }
    }
}
