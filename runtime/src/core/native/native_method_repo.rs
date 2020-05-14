use crate::core::klass::method::MethodInfo;
use crate::core::native::native_methods::NativeMethod;
use std::collections::HashMap;
use std::iter::Map;
use std::rc::Rc;

pub struct NativeMethodRepo {
    store: HashMap<String, NativeMethod>,
}

impl NativeMethodRepo {
    pub fn new() -> NativeMethodRepo {
        let mut store: HashMap<String, NativeMethod> = HashMap::new();
        store.insert(
            String::from("registerNatives()V"),
            crate::core::native::native_methods::register_natives,
        );
        store.insert(
            String::from("hashCode()I"),
            crate::core::native::native_methods::hash_code,
        );

        NativeMethodRepo { store }
    }

    pub fn find_method(&self, method: Rc<MethodInfo>) -> Option<NativeMethod> {
        match self.store.get(method.name_desc().as_str()) {
            Some(native_method) => Some(*native_method),
            _ => None,
        }
    }
}
