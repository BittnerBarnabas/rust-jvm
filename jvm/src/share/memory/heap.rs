use crate::share::memory::jvm_object::Oop;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue;
use std::cell::{Cell, RefCell};
use std::sync::Mutex;

pub struct JvmHeap {
    object_count: Mutex<usize>,
    heap: Mutex<Vec<Oop>>,
}

impl JvmHeap {
    pub fn new() -> JvmHeap {
        JvmHeap {
            object_count: Mutex::new(0),
            heap: Mutex::new(Vec::new()),
        }
    }

    pub fn store(&self, oop: Oop) -> Result<JvmValue, JvmException> {
        let mut object_count = *self.object_count.lock().unwrap();
        //TODO check size and throw OutOfMemoryErrors if alloc wouldn't succeed
        let object_reference = JvmValue::ObjRef {
            val: object_count.clone(),
        };
        object_count += 1;
        self.heap.lock().unwrap().push(oop);
        Ok(object_reference)
    }
}
