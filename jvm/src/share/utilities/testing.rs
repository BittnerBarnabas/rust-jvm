use crate::share::classfile::klass::Klass;
use crate::share::classfile::class_parser::ClassParser;
use std::sync::Arc;
use crate::share::memory::heap::HeapWord;
use crate::share::utilities::jvm_value::JvmValue;
use crate::share::utilities::jvm_value::ObjectRef::Ref;
use crate::share::memory::oop::Oop::ObjectOop;
use crate::share::memory::oop::oops::ObjectOopDesc;
use crate::share::memory::oop::Oop;

pub fn test_class() -> Arc<Klass> {
    let absolute_path = format!("{}/{}", "/home/barnab/projects/rust-jvm/resources/tests/unit", "UnitTestClass.class");
    log::trace!("Reading absolute file: {}", absolute_path);

    ClassParser::from(std::fs::read(absolute_path.clone()).unwrap()).parse_class().unwrap()
}

pub fn test_object_oop() -> Oop {
    ObjectOop(ObjectOopDesc::new(test_class(), HeapWord::new(vec![])))
}

pub fn test_object_ref() -> JvmValue {
    JvmValue::ObjRef(Ref(test_object_oop()))
}
