use crate::share::classfile::access_flags::ACC_STATIC;
use crate::share::classfile::attribute::AttributeInfo;
use crate::share::utilities::jvm_value::JvmValue;
use std::sync::Mutex;

// #[derive(Clone)]
pub struct FieldInfo {
    access_flags: u16,
    name: String,
    descriptor: String,
    attributes: Vec<AttributeInfo>,
    static_value: Mutex<Option<JvmValue>>,
}

impl FieldInfo {
    pub fn new(
        access_flags: u16,
        name: String,
        descriptor: String,
        attributes: Vec<AttributeInfo>,
    ) -> FieldInfo {
        FieldInfo {
            access_flags,
            name,
            descriptor,
            attributes,
            static_value: Mutex::new(None),
        }
    }

    pub fn matches_name_and_type(&self, name: &String, type_descriptor: &String) -> bool {
        &self.name == name && &self.descriptor == type_descriptor
    }

    pub fn is_static(&self) -> bool {
        crate::share::classfile::access_flags::flag_matches(self.access_flags, ACC_STATIC)
    }

    /// Will return the stored static value, it's only valid on static fields.
    /// Should check before, using `FieldInfo::is_static`
    pub fn static_value(&self) -> JvmValue {
        assert!(self.is_static());
        self.static_value
            .lock()
            .unwrap()
            .as_ref()
            .expect("Should not happen.")
            .clone()
    }

    pub fn set_static_value(&self, value: JvmValue) {
        assert!(self.is_static());
        self.static_value.lock().unwrap().replace(value);
    }

    pub fn default(&self) -> JvmValue {
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
