#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ObjectRef {
    val: usize,
}

impl ObjectRef {
    pub fn get_ref(&self) -> usize {
        self.val.clone()
    }
}

impl From<usize> for ObjectRef {
    fn from(val: usize) -> Self {
        ObjectRef { val }
    }
}

impl Default for ObjectRef {
    fn default() -> Self {
        ObjectRef::from(0)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
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
