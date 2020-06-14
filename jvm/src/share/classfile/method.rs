use crate::share::classfile::access_flags;
use crate::share::classfile::access_flags::ACC_NATIVE;
use crate::share::classfile::attribute::AttributeInfo;
use crate::share::native::native_methods::NativeMethod;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue;
use std::cell::Cell;
use std::io::Error;
use std::sync::Mutex;

pub struct MethodReference {
    pub class_name: String,
    pub method_name: String,
}

pub struct MethodInfo {
    access_flags: u16,
    name: String,
    descriptor: String,
    attributes: Vec<AttributeInfo>,
    native_method: Mutex<Option<NativeMethod>>,
    code: Option<CodeInfo>,
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
        descriptor: String,
        attributes: Vec<AttributeInfo>,
    ) -> Result<MethodInfo, Error> {
        let code = MethodInfo::resolve_code(&attributes);
        Ok(MethodInfo {
            access_flags,
            name,
            descriptor,
            attributes,
            native_method: Mutex::new(None),
            code,
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
        format!("{}{}", self.name, self.descriptor)
    }

    pub fn code_info(&self) -> &Option<CodeInfo> {
        &self.code
    }

    pub fn is_native(&self) -> bool {
        crate::share::classfile::access_flags::flag_matches(self.access_flags, ACC_NATIVE)
    }

    pub fn set_native_method(&self, method: NativeMethod) {
        *self.native_method.lock().unwrap() = Some(method)
    }

    pub fn native_method(&self) -> Option<NativeMethod> {
        *self.native_method.lock().unwrap()
    }
}