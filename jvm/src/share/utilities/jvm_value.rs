use crate::share::memory::oop::Oop;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue::ObjRef;
use crate::share::memory::oop::oops::{ObjectOopDesc, ArrayOopDesc, PrimitiveArrayOopDesc};
use crate::share::memory::oop::Oop::{ObjectOop, ArrayOop, PrimitiveArrayOop};

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectRef {
    Null,
    Ref(Oop),
}

impl ObjectRef {
    pub fn dereference(&self) -> Result<Oop, JvmException> {
        match self {
            ObjectRef::Null => Err(JvmException::from("NPE!!!")),
            ObjectRef::Ref(oop) => Ok(oop.clone())
        }
    }
}

impl Default for ObjectRef {
    fn default() -> Self {
        ObjectRef::Null
    }
}

#[derive(Debug, Clone, PartialEq)]
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

impl From<Oop> for JvmValue {
    fn from(oop: Oop) -> Self {
        JvmValue::ObjRef(ObjectRef::Ref(oop))
    }
}

impl From<ObjectOopDesc> for JvmValue {
    fn from(oop: ObjectOopDesc) -> Self {
        JvmValue::from(ObjectOop(oop))
    }
}

impl From<ArrayOopDesc> for JvmValue {
    fn from(oop: ArrayOopDesc) -> Self {
        JvmValue::from(ArrayOop(oop))
    }
}


impl From<PrimitiveArrayOopDesc> for JvmValue {
    fn from(oop: PrimitiveArrayOopDesc) -> Self {
        JvmValue::from(PrimitiveArrayOop(oop))
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

#[derive(Debug, Clone, PartialEq)]
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

