use crate::share::classfile::klass::Klass;
use crate::share::memory::jvm_object::Oop;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};

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

    pub fn store_object(&self, klass: Arc<Klass>) -> Result<JvmValue, JvmException> {
        let new_obj = Oop::build_default_object(klass);
        self.store(new_obj)
    }

    pub fn store_array(&self, klass: Arc<Klass>, size: i32) -> Result<JvmValue, JvmException> {
        let oop = Oop::build_array(klass, size);
        self.store(oop)
    }

    fn store(&self, oop: Oop) -> Result<JvmValue, JvmException> {
        let object_reference = self.next_obj_ref();
        self.heap.lock().unwrap().push(oop);
        Ok(object_reference)
    }

    fn next_obj_ref(&self) -> JvmValue {
        let mut object_count = *self.object_count.lock().unwrap();
        //TODO check size and throw OutOfMemoryErrors if alloc wouldn't succeed
        let object_reference = JvmValue::ObjRef {
            val: object_count.clone(),
        };
        object_count += 1;
        object_reference
    }
}
