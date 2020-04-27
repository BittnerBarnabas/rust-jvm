use crate::core::klass::attribute::AttributeInfo;
use crate::core::klass::constant_pool::{ConstantPool, CpInfo};
use crate::core::klass::field::FieldInfo;
use crate::core::klass::method::MethodInfo;

pub struct Klass {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: CpInfo,
    pub super_class: Option<CpInfo>,
    pub interfaces: Vec<CpInfo>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}
