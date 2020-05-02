use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::core::class_loader::ClassLoader;
use crate::core::jvm_exception::JvmException;
use crate::core::klass::attribute::AttributeInfo;
use crate::core::klass::constant_pool::{ConstantPool, CpInfo, Qualifier};
use crate::core::klass::field::FieldInfo;
use crate::core::klass::method::{MethodInfo, MethodReference};

#[derive(Clone)]
pub struct Klass {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl Klass {
    pub fn new(
        minor_version: u16,
        major_version: u16,
        constant_pool: ConstantPool,
        access_flags: u16,
        this_class: String,
        super_class: Option<String>,
        interfaces: Vec<String>,
        fields: Vec<FieldInfo>,
        methods: Vec<MethodInfo>,
        attributes: Vec<AttributeInfo>,
    ) -> Klass {
        Klass {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }

    pub fn get_qualified_name(&self) -> String {
        self.this_class.clone()
    }

    pub fn get_method_by_qualified_name(&self, qualified_name: Qualifier) -> Option<MethodInfo> {
        return match qualified_name {
            Qualifier::MethodRef {
                class_name,
                name,
                descriptor,
            } => self
                .get_method_by_name_desc(format!("{}{}", name, descriptor))
                .cloned(),
            _ => None,
        };
    }

    pub fn get_method_by_name_desc(&self, name_desc: String) -> Option<&MethodInfo> {
        self.methods
            .iter()
            .find(|method| name_desc == method.get_name_desc())
    }
}
