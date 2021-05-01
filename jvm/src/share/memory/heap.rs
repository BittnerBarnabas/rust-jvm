use crate::share::classfile::klass::Klass;
use crate::share::memory::jvm_object::Oop;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::{JvmValue, ObjectRef};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;

#[cfg_attr(test, mockall::automock)]
pub trait Heap: Send + Sync {
    fn store_object(&self, klass: Arc<Klass>) -> Result<JvmValue, JvmException>;
    fn store_array(&self, klass: Arc<Klass>, size: i32) -> Result<JvmValue, JvmException>;
    fn store_in_array(&self, ref_to_array: ObjectRef, index: usize, value: JvmValue) -> Result<(), JvmException>;
    fn array_length(&self, ref_to_array: ObjectRef) -> Result<usize, JvmException>;
}

pub struct JvmHeap {
    object_count: AtomicUsize,
    heap: Mutex<Vec<Oop>>,
}

impl JvmHeap {
    pub fn new() -> JvmHeap {
        JvmHeap {
            object_count: AtomicUsize::new(0),
            heap: Mutex::new(Vec::new()),
        }
    }

    fn store(&self, oop: Oop) -> Result<JvmValue, JvmException> {
        let object_reference = self.next_obj_ref();
        self.heap.lock().unwrap().push(oop);
        Ok(object_reference)
    }

    fn next_obj_ref(&self) -> JvmValue {
        let object_count = self.object_count.fetch_add(1, Ordering::AcqRel);
        //TODO check size and throw OutOfMemoryErrors if alloc wouldn't succeed
        let object_reference = JvmValue::ObjRef(ObjectRef::from(object_count));
        object_reference
    }
}

impl Heap for JvmHeap {
    fn store_object(&self, klass: Arc<Klass>) -> Result<JvmValue, JvmException> {
        let new_obj = Oop::build_default_object(klass);
        self.store(new_obj)
    }

    fn store_array(&self, klass: Arc<Klass>, size: i32) -> Result<JvmValue, JvmException> {
        let oop = Oop::build_array(klass, size);
        self.store(oop)
    }

    fn store_in_array(
        &self,
        ref_to_array: ObjectRef,
        index: usize,
        value: JvmValue,
    ) -> Result<(), JvmException> {
        let mut guard = self.heap.lock().unwrap();

        let oop = guard
            .get_mut(ref_to_array.get_ref())
            .ok_or(JvmException::from(format!("Could not get object lock for ref: {:?}", ref_to_array)))?;

        match oop {
            Oop::ArrayOop {
                klass: _,
                instance_data,
            } => {
                instance_data[index] = value;
                Ok(())
            }
            oop => Err(JvmException::from(format!("ObjectRef {:?} should point to array on heap but it was {}", ref_to_array, oop))),
        }
    }

    fn array_length(&self, ref_to_array: ObjectRef) -> Result<usize, JvmException> {
        let guard = self.heap.lock().unwrap();

        let oop = guard
            .get(ref_to_array.get_ref())
            .ok_or(JvmException::from(format!("Could not get object lock for ref: {:?}", ref_to_array)))?;

        match oop {
            Oop::ArrayOop {
                klass: _,
                instance_data,
            } => Ok(instance_data.len()),
            incorrect_oop => Err(JvmException::from(format!("ArrayOOP Expected, but got: {}", incorrect_oop))),
        }
    }
}
