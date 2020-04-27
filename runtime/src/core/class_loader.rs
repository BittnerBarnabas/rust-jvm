use std::collections::HashMap;

use crate::core::klass::klass::Klass;

type ClassKey = String;

pub struct ClassLoader {
    lookup_table: HashMap<ClassKey, Klass>,
}
