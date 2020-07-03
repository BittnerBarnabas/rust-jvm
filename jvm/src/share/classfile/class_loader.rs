use std::collections::HashMap;

use crate::share::classfile::class_parser::ClassParser;
use crate::share::classfile::constant_pool::Qualifier;
use crate::share::classfile::klass::ClassLoadingStatus::{
    BeingInitialized, Initialized, Linked, Loaded,
};
use crate::share::classfile::klass::Klass;
use crate::share::classfile::method::MethodInfo;
use crate::share::runtime::stack_frame::{JvmStackFrame, StackFrame};
use crate::share::utilities::context::GlobalContext;
use crate::share::utilities::global_symbols::{java_lang_Object, java_lang_String};
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue;
use std::borrow::BorrowMut;
use std::io::Error;
use std::sync::{Arc, Mutex};
use utils::ResultIterator;

static RESOURCES_PATH: &str = "resources/";

type ClassKey = String;

pub trait ClassLoader: Send + Sync {
    fn lookup_static_method(
        &self,
        qualified_name: Qualifier,
    ) -> Result<Arc<MethodInfo>, JvmException>;

    fn lookup_instance_method(
        &self,
        qualified_name: Qualifier,
    ) -> Result<Arc<MethodInfo>, JvmException>;

    fn load_class(&self, qualified_name: &Qualifier) -> Result<Arc<Klass>, JvmException>;

    fn load_and_init_class(&self, qualified_name: &String) -> Result<Arc<Klass>, JvmException>;

    fn bootstrap(&self) -> Result<(), JvmException>;
}

pub struct ResourceLocator {
    resource_root: String,
}

impl ResourceLocator {
    pub fn new(root: String) -> ResourceLocator {
        log::trace!("ClassLoader initialized.");
        ResourceLocator {
            resource_root: root,
        }
    }

    fn read_from_resource(&self, class_name: &String) -> Result<Vec<u8>, Error> {
        let path = ResourceLocator::class_name_to_path(class_name);
        let absolute_path = format!("{}/{}", self.resource_root, path);
        log::trace!("Reading absolute file: {}", absolute_path);
        std::fs::read(absolute_path)
    }

    fn class_name_to_path(class_name: &String) -> String {
        format!("{}.class", class_name)
    }
}

pub struct LinkResolver {}

impl LinkResolver {}

pub struct BootstrapClassLoader {
    lookup_table: Mutex<HashMap<ClassKey, Arc<Klass>>>,
    resource_locator: ResourceLocator,
    context: Arc<GlobalContext>,
}

impl ClassLoader for BootstrapClassLoader {
    fn lookup_static_method(
        &self,
        qualified_name: Qualifier,
    ) -> Result<Arc<MethodInfo>, JvmException> {
        match &qualified_name {
            Qualifier::MethodRef {
                class_name,
                name: _,
                descriptor: _,
            } => {
                let klass = self.load_class(class_name)?;

                if !klass.is_being_initialized() && !klass.is_initialized() {
                    self.load_and_init_class(class_name)?;
                }
                assert!(klass.is_initialized() || klass.is_being_initialized());

                klass
                    .get_method_by_qualified_name(qualified_name)
                    .ok_or(JvmException::new())
            }
            _ => Err(JvmException::new()),
        }
    }

    fn lookup_instance_method(
        &self,
        qualified_name: Qualifier,
    ) -> Result<Arc<MethodInfo>, JvmException> {
        self.lookup_static_method(qualified_name)
    }

    fn load_class(&self, qualified_name: &Qualifier) -> Result<Arc<Klass>, JvmException> {
        let qualified_klass_name = match qualified_name {
            Qualifier::Class { name } => name,
            _ => return Err(JvmException::new()),
        };
        self.load_class(qualified_klass_name)
    }

    fn load_and_init_class(&self, qualified_name: &String) -> Result<Arc<Klass>, JvmException> {
        let class = self.load_class(qualified_name)?;
        self.link_class(class.clone())?;
        self.initialize_class(class.clone())?;
        Ok(class)
    }

    fn bootstrap(&self) -> Result<(), JvmException> {
        self.load_and_init_class(&*java_lang_Object)?;
        // self.load_and_init_class(&*java_lang_String)?;

        Ok(())
    }
}

impl BootstrapClassLoader {
    pub fn new(
        resource_locator: ResourceLocator,
        context: Arc<GlobalContext>,
    ) -> BootstrapClassLoader {
        BootstrapClassLoader {
            lookup_table: Mutex::new(HashMap::new()),
            resource_locator,
            context,
        }
    }

    /// Loads a class from the bootstrap classpath or returns a `JvmException` if the class lookup
    /// or the parsing fails.
    ///
    /// 5.3.1 Section of JVM Specification
    pub fn load_class(&self, class_name: &String) -> Result<Arc<Klass>, JvmException> {
        let class_lookup = self
            .lookup_table
            .lock()
            .unwrap()
            .get(class_name.as_str())
            .cloned();
        match class_lookup {
            Some(class) => {
                if !class.is_loaded() {
                    self.do_load(class_name)
                } else {
                    Ok(class.clone())
                }
            }
            None => self.do_load(class_name),
        }
    }

    fn do_load(&self, class_name: &String) -> Result<Arc<Klass>, JvmException> {
        let raw_class = self
            .resource_locator
            .read_from_resource(&class_name)
            .map_err(|err| JvmException::from(err.to_string()))?;
        //TODO: ClassNotFoundException
        let derived_class = self.derive_class(raw_class)?;
        derived_class.set_status(Loaded);

        //record the resolved class in the cache
        self.lookup_table
            .lock()
            .unwrap()
            .borrow_mut()
            .insert(class_name.clone(), derived_class.clone());

        //return a pointer to it
        Ok(derived_class)
    }

    /// Tries to parse a class from the given bytes. If succeeds returns a `Klass` wrapped in an `Arc`,
    /// otherwise will return the appropriate `JvmException`.  
    fn derive_class(&self, class_to_derive: Vec<u8>) -> Result<Arc<Klass>, JvmException> {
        let klass = ClassParser::from(class_to_derive)
            .parse_class()
            .map_err(|err| JvmException::from(err.to_string()))?;
        //TODO: LinkageError, ClassFormatError and likes

        //we've got the class, now need to check its superclass
        match klass.qualified_super_name() {
            Some(super_name) => {
                self.load_class(&super_name)?;
                //TODO: 5.4.3.1 steps are missing to load Reference Type for array classes
                //as well as applying access control to this class and its superclass

                //TODO check some constraints, superclass is not an interface and superclass
                // is not this class
            }
            None => {
                assert_eq!(
                    klass.qualified_name(),
                    *java_lang_Object,
                    "Only {} permitted to not have a super class, but class {} doesn't have any!",
                    *java_lang_Object,
                    klass.qualified_name()
                );
            }
        }

        //loading up all direct interfaces
        let _loaded_interfaces = klass
            .interfaces()
            .iter()
            .map(|interface| self.load_class(interface))
            .collect_to_result()?;

        //TODO add extra checks over resolved interfaces: IncompatibleClassChangeError, and ClassCircularityError

        Ok(klass)
    }

    fn link_class(&self, class_to_link: Arc<Klass>) -> Result<(), JvmException> {
        assert!(
            class_to_link.is_loaded(),
            "Class should be loaded before being linked!"
        );

        if !class_to_link.is_linked() {
            self.verify_class(class_to_link.clone())?;
            self.prepare_class(class_to_link.clone())?;
            class_to_link.set_status(Linked);
        }
        Ok(())
    }

    fn verify_class(&self, _class_to_link: Arc<Klass>) -> Result<(), JvmException> {
        //TODO do some bytecode verification
        Ok(())
    }

    fn prepare_class(&self, class_to_prepare: Arc<Klass>) -> Result<(), JvmException> {
        //initialize static fields to their default values
        class_to_prepare.initialize_static_fields();

        // Register bootstrap native method. Probably non-standard... Will need to check
        class_to_prepare.register_natives();
        Ok(())
    }

    fn initialize_class(&self, class_to_init: Arc<Klass>) -> Result<(), JvmException> {
        assert!(
            class_to_init.is_linked(),
            "Class should be linked before calling init!"
        );

        if !class_to_init.is_initialized() {
            class_to_init.set_status(BeingInitialized);

            class_to_init
                .get_method_by_name_desc("<clinit>()V".to_string())
                .map(|init: Arc<MethodInfo>| -> Result<(), JvmException> {
                    let frame = StackFrame::new(&self.context, class_to_init.clone());
                    frame.execute_method(init, Vec::new())?;
                    Ok(())
                })
                .unwrap_or_else(|| Ok(()))?;
            class_to_init.set_status(Initialized);
        }
        Ok(())
    }
}
