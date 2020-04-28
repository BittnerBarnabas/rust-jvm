use std::collections::HashMap;

use crate::core::jvm_exception::JvmException;
use crate::core::klass::klass::Klass;

type ClassKey = String;

pub struct ClassLoader {
    lookup_table: HashMap<ClassKey, Klass>,
}

impl ClassLoader {
    pub fn new() -> ClassLoader {
        ClassLoader {
            lookup_table: HashMap::new(),
        }
    }

    pub fn bootstrap(&mut self, main_class: String) -> Result<(), JvmException> {}
}
