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
