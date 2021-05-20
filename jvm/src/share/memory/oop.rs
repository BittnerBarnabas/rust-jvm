use std::sync::{Arc, Mutex};

use crate::share::classfile::klass::Klass;
use crate::share::memory::heap::HeapWord;
use crate::share::memory::oop::oops::{ObjectOopDesc, PrimitiveArrayOopDesc, ArrayOopDesc};
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::{JvmValue, PrimitiveType};

#[derive(Debug, Clone)]
pub enum Oop {
    ObjectOop(ObjectOopDesc),
    ArrayOop(ArrayOopDesc),
    PrimitiveArrayOop(PrimitiveArrayOopDesc),
}

impl Oop {
    pub fn instance_data(&self) -> &HeapWord {
        match self {
            Self::ObjectOop(ObjectOopDesc { instance_data, .. }) => {
                instance_data
            }
            Self::ArrayOop(ArrayOopDesc { instance_data, .. }) => {
                instance_data
            }
            Self::PrimitiveArrayOop(PrimitiveArrayOopDesc { instance_data, .. }) => {
                instance_data
            }
        }
    }
}

pub mod oops {
    use std::sync::Arc;

    use crate::share::classfile::klass::Klass;
    use crate::share::memory::heap::HeapWord;
    use crate::share::utilities::jvm_value::PrimitiveType;

    type KlassPointer = Arc<Klass>;

    #[derive(Debug, Clone)]
    pub struct PrimitiveArrayOopDesc {
        pub inner_type: PrimitiveType,
        pub size: i32,
        pub instance_data: HeapWord,
    }

    #[derive(Debug, Clone)]
    pub struct ArrayOopDesc {
        pub klass: KlassPointer,
        pub size: i32,
        pub instance_data: HeapWord,
    }

    #[derive(Debug, Clone)]
    pub struct ObjectOopDesc {
        // mark: usize,
        //should make this more compact
        pub klass: KlassPointer,
        pub instance_data: HeapWord,
    }
}

