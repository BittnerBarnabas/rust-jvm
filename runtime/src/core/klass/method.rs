use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;
use crate::core::klass::access_flags::ACC_NATIVE;
use crate::core::klass::attribute::AttributeInfo;
use std::cell::Cell;
use std::io::{Error, ErrorKind};

pub struct MethodReference {
    pub class_name: String,
    pub method_name: String,
}

pub struct MethodInfo {
    access_flags: u16,
    name: String,
    descriptor: String,
    attributes: Vec<AttributeInfo>,
    native_method: Cell<Option<fn() -> Result<JvmValue, JvmException>>>,
    code: Option<Vec<u8>>,
}

impl MethodInfo {
    pub fn from(
        access_flags: u16,
        name: String,
        descriptor: String,
        attributes: Vec<AttributeInfo>,
    ) -> Result<MethodInfo, Error> {
        let code = MethodInfo::resolve_code(&attributes);
        Ok(MethodInfo {
            access_flags,
            name,
            descriptor,
            attributes,
            native_method: Cell::new(None),
            code,
        })
    }

    fn resolve_code(attributes: &Vec<AttributeInfo>) -> Option<Vec<u8>> {
        let code = attributes.iter().find(|att| match att {
            AttributeInfo::Code { .. } => true,
            _ => false,
        });

        return match code {
            Some(AttributeInfo::Code { code, .. }) => Some(code.clone()),
            _ => None,
        };
    }

    pub fn get_name_desc(&self) -> String {
        format!("{}{}", self.name, self.descriptor)
    }

    pub fn get_code(&self) -> &Option<Vec<u8>> {
        &self.code
    }

    pub fn is_native(&self) -> bool {
        self.access_flags & ACC_NATIVE != 0
    }

    pub fn set_native_method(&self, method: fn() -> Result<JvmValue, JvmException>) {
        self.native_method.set(Some(method))
    }
}
