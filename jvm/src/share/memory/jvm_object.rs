use crate::share::classfile::klass::Klass;
use crate::share::utilities::jvm_value::JvmValue;
use std::sync::Arc;
use std::fmt;
use std::fmt::Formatter;

pub enum Oop {
    ObjectOop {
        // mark: usize,
        //should make this more compact
        klass: Arc<Klass>,
        instance_data: Vec<JvmValue>,
    },
    ArrayOop {
        klass: Arc<Klass>,
        instance_data: Vec<JvmValue>,
    },
}

impl fmt::Display for Oop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Oop::ObjectOop { klass, .. } => {
                write!(f, "ObjectOop({})", klass.qualified_name())
            }
            Oop::ArrayOop { klass, .. } => {
                write!(f, "ArrayOop({})", klass.qualified_name())
            }
        }
    }
}


impl Oop {
    pub fn build_default_object(klass: Arc<Klass>) -> Oop {
        let instance_data: Vec<JvmValue> = klass
            .instance_fields()
            .iter()
            .map(|f| f.default())
            .collect();

        Oop::ObjectOop {
            klass,
            instance_data,
        }
    }

    pub fn build_array(klass: Arc<Klass>, size: i32) -> Oop {
        assert!(
            size >= 0,
            "Cannot build array OOP with negative size! {}",
            size
        );
        let instance_data: Vec<JvmValue> = (0..size).map(|_| JvmValue::null_obj()).collect();

        Oop::ArrayOop {
            klass,
            instance_data,
        }
    }
}
