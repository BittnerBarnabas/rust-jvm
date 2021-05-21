use core::sync::atomic::Ordering;
use std::collections::HashMap;
use std::iter::Map;
use std::ops::Deref;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::AtomicUsize;

use crate::share::classfile::klass::Klass;
use crate::share::memory::oop::Oop;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::{JvmValue, ObjectRef, PrimitiveType};
use crate::share::utilities::jvm_value::JvmValue::ObjRef;
use crate::share::memory::oop::oops::{PrimitiveArrayOopDesc, ObjectOopDesc, ArrayOopDesc};
use crate::share::memory::oop::Oop::{ArrayOop, PrimitiveArrayOop, ObjectOop};

#[cfg_attr(test, mockall::automock)]
pub trait Heap: Send + Sync {
    fn allocate_object(&self, klass: Arc<Klass>) -> Result<ObjectOopDesc, JvmException>;
    fn allocate_array(&self, klass: Arc<Klass>, size: i32) -> Result<ArrayOopDesc, JvmException>;
    fn allocate_primitive_array(&self, primitive_type: PrimitiveType, size: i32) -> Result<PrimitiveArrayOopDesc, JvmException>;
}

pub struct JvmHeap {
    object_count: AtomicUsize,
    heap: Mutex<HashMap<HeapWordKey, HeapWord>>,
}

impl JvmHeap {
    pub fn new() -> JvmHeap {
        JvmHeap {
            object_count: AtomicUsize::new(0),
            heap: Mutex::new(HashMap::new()),
        }
    }

    fn store(&self, heap_word: HeapWord) -> Result<(), JvmException> {
        self.heap.lock().unwrap().insert(heap_word.key(), heap_word);
        Ok(())
    }

    fn build_default_object(klass: Arc<Klass>) -> HeapWord {
        let instance_data: Vec<JvmValue> = klass
            .instance_fields()
            .iter()
            .map(|f| f.default())
            .collect();

        HeapWord::new(instance_data)
    }

    fn allocate_obj_array(klass: Arc<Klass>, size: i32) -> HeapWord {
        assert!(
            size >= 0,
            "Cannot build array OOP with negative size! {}",
            size
        );
        let instance_data = vec![JvmValue::null_obj(); size as usize];

        HeapWord::new(instance_data)
    }

    fn allocate_primitive_array(primitive_type: PrimitiveType, size: i32) -> HeapWord {
        assert!(
            size >= 0,
            "Cannot build array OOP with negative size! {}",
            size
        );

        let instance_data = vec![JvmValue::from(primitive_type); size as usize];

        HeapWord::new(instance_data)
    }

    fn set_field(oop: &HeapWord, field_offset: usize, value: JvmValue) -> Result<(), JvmException> {
        oop.data.write().unwrap()[field_offset] = value;
        Ok(())
    }

    fn get_field(oop: &HeapWord, field_offset: usize) -> Result<JvmValue, JvmException> {
        Ok(oop.data.read().unwrap()[field_offset].clone())
    }
}

impl Heap for JvmHeap {
    fn allocate_object(&self, klass: Arc<Klass>) -> Result<ObjectOopDesc, JvmException> {
        let new_obj = JvmHeap::build_default_object(klass.clone());
        self.store(new_obj.clone())?;
        Ok(ObjectOopDesc::new(klass, new_obj))
    }

    fn allocate_array(&self, klass: Arc<Klass>, size: i32) -> Result<ArrayOopDesc, JvmException> {
        let new_obj = JvmHeap::allocate_obj_array(klass.clone(), size.clone());
        self.store(new_obj.clone())?;
        Ok(
            ArrayOopDesc {
                klass,
                size,
                instance_data: new_obj,
            }
        )
    }

    fn allocate_primitive_array(&self, primitive_type: PrimitiveType, size: i32) -> Result<PrimitiveArrayOopDesc, JvmException> {
        let new_obj = JvmHeap::allocate_primitive_array(primitive_type.clone(), size.clone());
        self.store(new_obj.clone())?;
        Ok(
            PrimitiveArrayOopDesc {
                inner_type: primitive_type,
                size,
                instance_data: new_obj,
            }
        )
    }
}

type HeapWordKey = usize;

#[derive(Debug, Clone)]
pub struct HeapWord {
    data: Arc<RwLock<Vec<JvmValue>>>,
}

impl PartialEq for HeapWord {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl HeapWord {
    //TODO: Make sure that only Heap can create heapwords
    pub fn new(data: Vec<JvmValue>) -> HeapWord {
        HeapWord {
            data: Arc::new(RwLock::new(data))
        }
    }

    pub fn key(&self) -> HeapWordKey {
        ((self.data.read().unwrap().as_ptr()) as usize)
    }

    pub fn put_field(&self, field_offset: usize, value: JvmValue) -> Result<(), JvmException> {
        self.data.write().unwrap()[field_offset] = value;
        Ok(())
    }

    pub fn get_field(&self, field_offset: usize) -> Result<JvmValue, JvmException> {
        Ok(self.data.read().unwrap()[field_offset].clone())
    }
    pub fn data(&self) -> Arc<RwLock<Vec<JvmValue>>> {
        self.data.clone()
    }
}
