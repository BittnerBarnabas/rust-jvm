use std::collections::HashMap;

use crate::core::class_parser::ClassParser;
use crate::core::jvm_exception::JvmException;
use crate::core::klass::klass::Klass;
use std::fs::File;
use std::io::Error;

static RESOURCES_PATH: &str = "../resources/";

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

    pub fn bootstrap(&mut self) -> Result<(), JvmException> {
        let object_klass = ClassLoader::read_and_parse_class(&String::from("java/lang/Object"))
            .map_err(|err| JvmException::from_string(err.to_string()))?;

        Ok(())
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
