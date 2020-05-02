use std::collections::HashMap;

use crate::core::class_parser::ClassParser;
use crate::core::interpreter;
use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;
use crate::core::klass::constant_pool::Qualifier;
use crate::core::klass::klass::Klass;
use crate::core::klass::method::MethodInfo;
use crate::core::native::native_methods::register_natives;
use crate::core::stack_frame::StackFrame;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::fs::File;
use std::io::Error;
use std::rc::Rc;

static RESOURCES_PATH: &str = "../resources/";

type ClassKey = String;

pub struct ClassLoader {
    lookup_table: RefCell<HashMap<ClassKey, Rc<Klass>>>,
}

impl ClassLoader {
    pub fn new() -> ClassLoader {
        ClassLoader {
            lookup_table: RefCell::new(HashMap::new()),
        }
    }

    pub fn lookup_class(&self, qualified_name: String) -> Result<Rc<Klass>, JvmException> {
        if let Some(klass_ref) = self.lookup_table.borrow().get(&qualified_name) {
            return Ok(klass_ref.clone());
        }

        self.load_class(qualified_name.clone())?;
        //if load_class is successful, second call should definitely resolve the class
        return self.lookup_class(qualified_name);
    }

    pub fn load_class(&self, qualified_name: String) -> Result<(), JvmException> {
        let klass = ClassLoader::read_and_parse_class(&qualified_name)
            .map_err(|err| JvmException::from_string(err.to_string()))?;

        self.call_cl_init(&klass);

        self.lookup_table
            .borrow_mut()
            .insert(klass.get_qualified_name(), Rc::new(klass));
        Ok(())
    }

    fn bootstrap_class(&self, qualified_name: String) -> Result<(), JvmException> {
        let klass = ClassLoader::read_and_parse_class(&qualified_name)
            .map_err(|err| JvmException::from_string(err.to_string()))?;

        klass
            .get_method_by_name_desc("registerNatives()V".to_string())
            .map(|method| {
                method.set_native_method(crate::core::native::native_methods::register_natives)
            });

        self.call_cl_init(&klass);

        self.lookup_table
            .borrow_mut()
            .insert(klass.get_qualified_name(), Rc::new(klass));
        Ok(())
    }

    pub fn lookup_method(&self, qualified_name: Qualifier) -> Result<MethodInfo, JvmException> {
        match &qualified_name {
            Qualifier::MethodRef {
                class_name,
                name,
                descriptor,
            } => {
                let klass = self.lookup_class(class_name.clone())?;
                klass
                    .get_method_by_qualified_name(qualified_name)
                    .ok_or(JvmException::new())
            }
            _ => Err(JvmException::new()),
        }
    }

    pub fn bootstrap(&self) -> Result<(), JvmException> {
        self.bootstrap_class(String::from("java/lang/Object"));
        Ok(())
    }

    fn call_cl_init(&self, klass: &Klass) -> Option<Result<JvmValue, JvmException>> {
        klass
            .get_method_by_name_desc("<clinit>()V".to_string())
            .map(|init| {
                let frame = StackFrame::new(self, klass);
                frame.execute_method(init, klass)
            })
    }

    fn read_and_parse_class(class_name: &String) -> Result<Klass, Error> {
        let class_in_bytes = ClassLoader::read_from_resources(class_name)?;
        ClassParser::from(class_in_bytes).parse_class()
    }

    fn read_from_resources(class_name: &String) -> Result<Vec<u8>, Error> {
        let path = ClassLoader::class_name_to_path(class_name);
        std::fs::read(format!("{}{}", RESOURCES_PATH, path))
    }

    fn class_name_to_path(class_name: &String) -> String {
        format!("{}.class", class_name)
    }
}
