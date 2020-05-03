use crate::core::jvm_value::JvmValue;
use crate::core::klass::access_flags;
use crate::core::klass::access_flags::ACC_STATIC;
use crate::core::klass::attribute::AttributeInfo;

#[derive(Clone)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn is_static(&self) -> bool {
        access_flags::flag_matches(self.access_flags, ACC_STATIC)
    }

    pub fn get_default(&self) -> JvmValue {
        match self.descriptor.as_str() {
            "Z" => JvmValue::Boolean { val: false },
            "B" => JvmValue::Byte { val: 0 },
            "S" => JvmValue::Short { val: 0 },
            "C" => JvmValue::Char { val: '\0' },
            "I" => JvmValue::Int { val: 0 },
            "J" => JvmValue::Long { val: 0 },
            "F" => JvmValue::Float { val: 0.0 },
            "D" => JvmValue::Double { val: 0.0 },
            _ => JvmValue::null_obj(),
        }
    }
}
