use crate::core::klass::attribute::AttributeInfo;
use crate::core::klass::constant_pool::{ConstantPool, CpInfo};
use crate::core::klass::field::FieldInfo;
use crate::core::klass::method::MethodInfo;

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
    pub fn get_qualified_name(&self) -> String {
        self.this_class.clone()
    }
}
