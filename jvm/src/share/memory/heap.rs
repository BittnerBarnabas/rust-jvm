use crate::share::classfile::klass::Klass;
use crate::share::memory::oop::Oop;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::{JvmValue, ObjectRef, PrimitiveType};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
use crate::share::utilities::jvm_value::JvmValue::ObjRef;
use std::iter::Map;
use std::collections::HashMap;
use crate::share::memory::oop::Oop::{ObjectOop, ArrayOop, PrimitiveArrayOop};
use std::ops::Deref;


#[cfg_attr(test, mockall::automock)]
pub trait Heap: Send + Sync {
    fn allocate_object(&self, klass: Arc<Klass>) -> Result<JvmValue, JvmException>;
    fn put_object_field(&self, ref_to_object: ObjectRef, field_offset: usize, value: JvmValue) -> Result<(), JvmException>;
    fn load_object_field(&self, ref_to_object: ObjectRef, field_offset: usize) -> Result<JvmValue, JvmException>;
    fn allocate_array(&self, klass: Arc<Klass>, size: i32) -> Result<JvmValue, JvmException>;
    fn allocate_primitive_array(&self, primitive_type: PrimitiveType, size: i32) -> Result<JvmValue, JvmException>;
    fn store_in_array(&self, ref_to_array: ObjectRef, index: i32, value: JvmValue) -> Result<(), JvmException>;
    fn load_from_array(&self, ref_to_array: ObjectRef, index: i32) -> Result<JvmValue, JvmException>;
    fn array_length(&self, ref_to_array: ObjectRef) -> Result<i32, JvmException>;
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
    fn allocate_object(&self, klass: Arc<Klass>) -> Result<JvmValue, JvmException> {
        let new_obj = JvmHeap::build_default_object(klass.clone());
        self.store(new_obj.clone())?;
        Ok(
            ObjRef(ObjectRef::Oop(ObjectOop {
                klass,
                instance_data: new_obj,
            }))
        )
    }

    fn put_object_field(&self, ref_to_object: ObjectRef, field_offset: usize, value: JvmValue) -> Result<(), JvmException> {
        match ref_to_object {
            ObjectRef::Null => Err(JvmException::from("ref_to_array was null! NPE")),
            ObjectRef::Oop(oop) => {
                match oop {
                    Oop::ObjectOop { instance_data, .. } => {
                        let mut guard = self.heap.lock().unwrap();

                        let oop = guard
                            .get(&instance_data.key())
                            .ok_or(JvmException::from(format!("Could not get object lock for ref: {:?}", instance_data)))?;

                        JvmHeap::set_field(oop, field_offset, value)
                    }
                    Oop::ArrayOop { instance_data, size, .. } => {
                        let mut guard = self.heap.lock().unwrap();

                        let oop = guard
                            .get(&instance_data.key())
                            .ok_or(JvmException::from(format!("Could not get object lock for array-ref: {:?}", instance_data)))?;

                        JvmHeap::set_field(oop, field_offset, value)
                    }
                    Oop::PrimitiveArrayOop { .. } => Err(JvmException::from("ref_to_array was PrimitiveArrayOop!"))
                }
            }
        }
    }

    fn load_object_field(&self, ref_to_object: ObjectRef, field_offset: usize) -> Result<JvmValue, JvmException> {
        match &ref_to_object {
            ObjectRef::Null => {
                Err(JvmException::from("ref_to_array was null! NPE"))
            }
            ObjectRef::Oop(obj_oop) => {
                let mut guard = self.heap.lock().unwrap();

                let heap_word = obj_oop.instance_data();
                let oop = guard
                    .get(&heap_word.key())
                    .ok_or(JvmException::from(format!("Could not get object lock for ref: {:?}", ref_to_object)))?;

                JvmHeap::get_field(oop, field_offset)
            }
        }
    }

    fn allocate_array(&self, klass: Arc<Klass>, size: i32) -> Result<JvmValue, JvmException> {
        let new_obj = JvmHeap::allocate_obj_array(klass.clone(), size.clone());
        self.store(new_obj.clone())?;
        Ok(
            ObjRef(ObjectRef::Oop(
                ArrayOop {
                    klass,
                    size,
                    instance_data: new_obj,
                }
            ))
        )
    }

    fn allocate_primitive_array(&self, primitive_type: PrimitiveType, size: i32) -> Result<JvmValue, JvmException> {
        let new_obj = JvmHeap::allocate_primitive_array(primitive_type.clone(), size.clone());
        self.store(new_obj.clone())?;
        Ok(
            ObjRef(ObjectRef::Oop(
                PrimitiveArrayOop {
                    inner_type: primitive_type,
                    size,
                    instance_data: new_obj,
                }
            ))
        )
    }

    fn store_in_array(
        &self,
        ref_to_array: ObjectRef,
        index: i32,
        value: JvmValue,
    ) -> Result<(), JvmException> {
        match &ref_to_array {
            ObjectRef::Null => {
                Err(JvmException::from("ref_to_array was null! NPE"))
            }
            ObjectRef::Oop(array_oop) => {
                let mut guard = self.heap.lock().unwrap();

                let heap_word = array_oop.instance_data();
                let oop = guard
                    .get(&heap_word.key())
                    .ok_or(JvmException::from(format!("Could not get object lock for ref: {:?}", ref_to_array)))?;

                JvmHeap::set_field(oop, index as usize, value)
            }
        }
    }

    fn load_from_array(&self, ref_to_array: ObjectRef, index: i32) -> Result<JvmValue, JvmException> {
        match &ref_to_array {
            ObjectRef::Null => {
                Err(JvmException::from("ref_to_array was null! NPE"))
            }
            ObjectRef::Oop(array_oop) => {
                let mut guard = self.heap.lock().unwrap();

                let heap_word = array_oop.instance_data();
                let oop = guard
                    .get(&heap_word.key())
                    .ok_or(JvmException::from(format!("Could not get object lock for ref: {:?}", ref_to_array)))?;

                JvmHeap::get_field(oop, index as usize)
            }
        }
    }

    fn array_length(&self, ref_to_array: ObjectRef) -> Result<i32, JvmException> {
        match ref_to_array {
            ObjectRef::Null => {
                Err(JvmException::from("ref_to_array was null! NPE"))
            }
            ObjectRef::Oop(oop) => {
                match oop {
                    Oop::ArrayOop {
                        size, ..
                    } => Ok(size),
                    incorrect_oop => Err(JvmException::from(format!("ArrayOOP Expected, but got: {:?}", incorrect_oop))),
                }
            }
        }
    }
}


type HeapWordKey = usize;

#[derive(Debug, Clone)]
pub struct HeapWord {
    data: Arc<RwLock<Vec<JvmValue>>>,
}

impl HeapWord {
    pub fn new(data: Vec<JvmValue>) -> HeapWord {
        HeapWord {
            data: Arc::new(RwLock::new(data))
        }
    }

    pub fn key(&self) -> HeapWordKey {
        ((self.data.read().unwrap().as_ptr()) as usize)
    }
}
