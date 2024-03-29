use std::sync::{Arc, Mutex};

use crate::share::classfile::klass::Klass;
use crate::share::memory::heap::HeapWord;
use crate::share::memory::oop::oops::{ObjectOopDesc, PrimitiveArrayOopDesc, ArrayOopDesc};
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::{JvmValue, PrimitiveType};

#[derive(Debug, Clone, PartialEq)]
pub enum Oop {
    ObjectOop(ObjectOopDesc),
    ArrayOop(ArrayOopDesc),
    PrimitiveArrayOop(PrimitiveArrayOopDesc),
}

impl Oop {
    pub fn instance_data(&self) -> &HeapWord {
        match self {
            Self::ObjectOop(object) => object.instance_data(),
            Self::ArrayOop(ArrayOopDesc { instance_data, .. }) => {
                instance_data
            }
            Self::PrimitiveArrayOop(PrimitiveArrayOopDesc { instance_data, .. }) => {
                instance_data
            }
        }
    }

    pub fn java_klass_or_fail(&self) -> Arc<Klass> {
        match self {
            Oop::ObjectOop(oops) => oops.klass(),
            Oop::ArrayOop(oops) => oops.klass(),
            Oop::PrimitiveArrayOop(_) => panic!("Can only get the klass from Reference Types!")
        }
    }
}

pub mod oops {
    use std::sync::Arc;

    use crate::share::classfile::klass::Klass;
    use crate::share::memory::heap::{HeapWord, Heap};
    use crate::share::utilities::jvm_value::{PrimitiveType, JvmValue};
    use crate::share::utilities::jvm_exception::JvmException;
    use std::borrow::BorrowMut;

    type KlassPointer = Arc<Klass>;

    #[derive(Debug, Clone, PartialEq)]
    pub struct PrimitiveArrayOopDesc {
        pub inner_type: PrimitiveType,
        pub size: i32,
        pub instance_data: HeapWord,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ArrayOopDesc {
        pub klass: KlassPointer,
        pub size: i32,
        pub instance_data: HeapWord,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ObjectOopDesc {
        // mark: usize,
        //should make this more compact
        klass: KlassPointer,
        instance_data: HeapWord,
    }

    impl ObjectOopDesc {
        pub fn new(klass: KlassPointer, instance_data: HeapWord) -> ObjectOopDesc {
            ObjectOopDesc {
                klass,
                instance_data,
            }
        }

        pub fn instance_data(&self) -> &HeapWord {
            &self.instance_data
        }

        pub fn klass(&self) -> KlassPointer {
            self.klass.clone()
        }
    }

    impl ArrayOopDesc {
        pub fn klass(&self) -> KlassPointer {
            self.klass.clone()
        }
    }

    impl PrimitiveArrayOopDesc {
        pub fn copy_bytes(&self, heap: &dyn Heap, bytes: Vec<u8>) -> Result<(), JvmException> {
            let arc = self.instance_data.data();
            let mut data = arc.write().unwrap();
            for i in 0..bytes.len() {
                data[i] = JvmValue::from(bytes[i] as i8)
            }
            Ok(())
        }
    }
}

