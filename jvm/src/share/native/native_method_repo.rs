use crate::share::classfile::method::MethodInfo;
use crate::share::native::native_methods::NativeMethod;
use crate::share::utilities::global_symbols::{
    java_lang_Object_hashCode,
    java_lang_Object_registerNatives,
    java_lang_Class_registerNatives,
};
use std::collections::HashMap;
use std::rc::Rc;

pub struct NativeMethodRepo {
    store: HashMap<String, NativeMethod>,
}

impl NativeMethodRepo {
    pub fn new() -> NativeMethodRepo {
        let mut store: HashMap<String, NativeMethod> = HashMap::new();
        store.insert(
            java_lang_Object_registerNatives.clone(),
            crate::share::native::object::register_natives,
        );
        store.insert(
            java_lang_Object_hashCode.clone(),
            crate::share::native::object::hash_code,
        );
        store.insert(
            java_lang_Class_registerNatives.clone(),
            crate::share::native::class::register_natives,
        );

        NativeMethodRepo { store }
    }

    pub fn find_method(&self, method: &MethodInfo) -> Option<NativeMethod> {
        let key = format!("{}_{}", method.get_klass().qualified_name(), method.name_desc());
        match self.store.get(key.as_str()) {
            Some(native_method) => Some(*native_method),
            _ => None,
        }
    }
}
