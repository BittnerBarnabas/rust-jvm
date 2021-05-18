use crate::share::memory::oop::Oop;
use crate::share::utilities::jvm_value::JvmValue::ObjRef;

#[derive(Debug, Clone)]
pub enum ObjectRef {
    Null,
    Oop(Oop),
}

impl Default for ObjectRef {
    fn default() -> Self {
        ObjectRef::Null
    }
}

#[derive(Debug, Clone)]
pub enum JvmValue {
    Boolean { val: bool },
    Byte { val: i8 },
    Short { val: i16 },
    Int { val: i32 },
    Long { val: i64 },
    Float { val: f32 },
    Double { val: f64 },
    Char { val: char },
    ObjRef(ObjectRef),
    ReturnAddr { val: usize },
    Void {},
}

impl JvmValue {
    pub fn null_obj() -> JvmValue {
        JvmValue::ObjRef(ObjectRef::default())
    }
}

impl From<PrimitiveType> for JvmValue {
    fn from(val: PrimitiveType) -> Self {
        match val {
            PrimitiveType::Boolean => JvmValue::Boolean { val: false },
            PrimitiveType::Byte => JvmValue::Byte { val: 0 },
            PrimitiveType::Short => JvmValue::Short { val: 0 },
            PrimitiveType::Int => JvmValue::Int { val: 0 },
            PrimitiveType::Long => JvmValue::Long { val: 0 },
            PrimitiveType::Float => JvmValue::Float { val: 0.0 },
            PrimitiveType::Double => JvmValue::Double { val: 0.0 },
            PrimitiveType::Char => JvmValue::Char { val: '\0' },
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Char,
}

impl From<i32> for PrimitiveType {
    fn from(val: i32) -> Self {
        match val {
            4 => PrimitiveType::Boolean,
            5 => PrimitiveType::Char,
            6 => PrimitiveType::Float,
            7 => PrimitiveType::Double,
            8 => PrimitiveType::Byte,
            9 => PrimitiveType::Short,
            10 => PrimitiveType::Int,
            11 => PrimitiveType::Long,
            _ => panic!("Primitive descriptor is not valid: {}", val)
        }
    }
}

