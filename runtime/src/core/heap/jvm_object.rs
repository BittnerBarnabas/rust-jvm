use crate::core::jvm_value::JvmValue;
use crate::core::klass::klass::Klass;
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
            .map(|f| f.get_default())
            .collect();

        Oop::ObjectOop {
            klass,
            instance_data,
        }
    }
}
