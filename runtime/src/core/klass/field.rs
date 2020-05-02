use crate::core::klass::attribute::AttributeInfo;

#[derive(Clone)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<AttributeInfo>,
}
