use crate::share::classfile::access_flags;
use crate::share::classfile::access_flags::{ACC_NATIVE, ACC_STATIC};
use crate::share::classfile::attribute::AttributeInfo;
use crate::share::classfile::klass::Klass;
use crate::share::native::native_methods::NativeMethod;
use crate::share::parser::descriptors::{
    MethodDescriptor, MethodDescriptorParser, ReturnDescriptor,
};
use crate::share::parser::parser::Parser;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue;
use std::borrow::BorrowMut;
use std::cell::Cell;
use std::fmt;
use std::fmt::Formatter;
use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex, RwLock, Weak};

pub struct MethodReference {
    pub class_name: String,
    pub method_name: String,
}

pub struct MethodInfo {
    access_flags: u16,
    name: String,
    raw_descriptor: String,
    descriptor: MethodDescriptor,
    attributes: Vec<AttributeInfo>,
    native_method: RwLock<Option<NativeMethod>>,
    code: Option<CodeInfo>,
    klass: RwLock<Option<Weak<Klass>>>,
}

impl fmt::Display for MethodInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}:{}",
            self.get_klass().qualified_name(),
            self.name,
            self.raw_descriptor
        )
    }
}

#[derive(Clone)]
pub struct CodeInfo {
    bytes: Vec<u8>,
    local_variables: u16,
}

impl CodeInfo {
    pub fn new(bytes: Vec<u8>, local_variables: u16) -> Self {
        CodeInfo {
            bytes,
            local_variables,
        }
    }

    pub fn bytes(&self) -> &Vec<u8> {
        return &self.bytes;
    }

    pub fn local_variables(&self) -> u16 {
        return self.local_variables.clone();
    }
}

impl MethodInfo {
    pub fn from(
        access_flags: u16,
        name: String,
        raw_descriptor: String,
        attributes: Vec<AttributeInfo>,
    ) -> Result<MethodInfo, Error> {
        let code = MethodInfo::resolve_code(&attributes);

        let descriptor = MethodDescriptorParser::new()
            .parse(raw_descriptor.as_str())
            .map_err(|err| Error::new(ErrorKind::Other, format!("{:?}", err)))?;

        Ok(MethodInfo {
            access_flags,
            name,
            raw_descriptor,
            descriptor,
            attributes,
            native_method: RwLock::new(None),
            code,
            klass: RwLock::new(None),
        })
    }

    fn resolve_code(attributes: &Vec<AttributeInfo>) -> Option<CodeInfo> {
        let code = attributes.iter().find(|att| match att {
            AttributeInfo::Code { .. } => true,
            _ => false,
        });

        return match code {
            Some(AttributeInfo::Code {
                code, max_locals, ..
            }) => Some(CodeInfo::new(code.clone(), max_locals.clone())),
            _ => None,
        };
    }

    pub fn name_desc(&self) -> String {
        format!("{}{}", self.name, self.raw_descriptor)
    }

    pub fn code_info(&self) -> &Option<CodeInfo> {
        &self.code
    }

    pub fn is_native(&self) -> bool {
        access_flags::flag_matches(self.access_flags, ACC_NATIVE)
    }

    pub fn is_static(&self) -> bool {
        access_flags::flag_matches(self.access_flags, ACC_STATIC)
    }

    pub fn set_native_method(&self, method: NativeMethod) {
        *self.native_method.write().unwrap() = Some(method)
    }

    pub fn native_method(&self) -> Option<NativeMethod> {
        *self.native_method.read().unwrap()
    }

    pub fn set_klass(&self, klass: Weak<Klass>) {
        *self.klass.write().unwrap() = Some(klass)
    }

    pub fn get_klass(&self) -> Arc<Klass> {
        self.klass
            .read()
            .unwrap()
            .as_ref()
            .expect("set_klass should be called before get_klass is called!")
            .upgrade()
            .expect("Klass has been unloaded but it's been referenced from code!")
    }

    pub fn number_of_parameters(&self) -> u8 {
        self.descriptor.parameters.len() as u8
    }

    pub fn is_void(&self) -> bool {
        self.descriptor.return_descriptor == ReturnDescriptor::Void
    }
}
