use crate::share::utilities::jvm_value::JvmValue;
use std::sync::{Arc, Mutex};
use crate::share::classfile::klass::Klass;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::memory::heap::HeapWord;

type KlassPointer = Arc<Klass>;

#[derive(Debug, Clone)]
pub enum Oop {
    ObjectOop {
        // mark: usize,
        //should make this more compact
        klass: KlassPointer,
        instance_data: HeapWord,
    },
    ArrayOop {
        klass: KlassPointer,
        size: i32,
        instance_data: HeapWord,
    },
}

impl Oop {
    pub fn instance_data(&self) -> HeapWord {
        match self {
            Oop::ObjectOop { instance_data, .. } => {
                instance_data.clone()
            }
            Oop::ArrayOop { instance_data, .. } => {
                instance_data.clone()
            }
        }
    }
}