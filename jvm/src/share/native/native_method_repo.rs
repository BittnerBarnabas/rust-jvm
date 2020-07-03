use crate::share::classfile::method::MethodInfo;
use crate::share::native::native_methods::NativeMethod;
use crate::share::utilities::global_symbols::{
    java_lang_Object_hashCode, java_lang_Object_registerNatives,
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
            crate::share::native::native_methods::register_natives,
        );
        store.insert(
            java_lang_Object_hashCode.clone(),
            crate::share::native::native_methods::hash_code,
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
