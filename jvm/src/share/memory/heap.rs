use crate::share::memory::jvm_object::Oop;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue;
use std::cell::{Cell, RefCell};

pub struct JvmHeap {
    object_count: Cell<usize>,
    heap: RefCell<Vec<Oop>>,
}

impl JvmHeap {
    pub fn new() -> JvmHeap {
        JvmHeap {
            object_count: Cell::new(0),
            heap: RefCell::new(Vec::new()),
        }
    }

    pub fn store(&self, oop: Oop) -> Result<JvmValue, JvmException> {
        //TODO check size and throw OutOfMemoryErrors if alloc wouldn't succeed
        let object_reference = JvmValue::ObjRef {
            val: self.object_count.get().clone(),
        };
        self.object_count.replace(self.object_count.get() + 1);
        self.heap.borrow_mut().push(oop);
        Ok(object_reference)
    }
}
