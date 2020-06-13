use crate::share::classfile::klass::Klass;
use crate::share::utilities::jvm_value::JvmValue;
use std::rc::Rc;

pub enum Oop {
    ObjectOop {
        // mark: usize,
        //should make this more compact
        klass: Rc<Klass>,
        instance_data: Vec<JvmValue>,
    },
}

impl Oop {
    pub fn build_default_object(klass: Rc<Klass>) -> Oop {
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
}
