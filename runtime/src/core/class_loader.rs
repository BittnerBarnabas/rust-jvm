use std::collections::HashMap;

use crate::core::class_parser::ClassParser;
use crate::core::context::GlobalContext;
use crate::core::heap::heap::JvmHeap;
use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;
use crate::core::klass::constant_pool::Qualifier;
use crate::core::klass::klass::ClassLoadingStatus::{
    BeingInitialized, Initialized, Linked, Loaded,
};
use crate::core::klass::klass::Klass;
use crate::core::klass::method::MethodInfo;
use crate::core::native::native_method_repo::NativeMethodRepo;
use crate::core::stack_frame::{JvmStackFrame, StackFrame};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::io::Error;
use std::rc::Rc;
use utils::ResultIterator;

static RESOURCES_PATH: &str = "resources/";

type ClassKey = String;

pub trait ClassLoader {
    fn lookup_static_method(
        &self,
        qualified_name: Qualifier,
    ) -> Result<Rc<MethodInfo>, JvmException>;

    fn load_class(&self, qualified_name: String) -> Result<Rc<Klass>, JvmException>;

    fn load_and_init_class(&self, qualified_name: String) -> Result<Rc<Klass>, JvmException>;
}

type ClassLoaderKey = String;

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
    lookup_table: RefCell<HashMap<ClassKey, Rc<Klass>>>,
    resource_locator: ResourceLocator,
    context: Rc<GlobalContext>,
}

impl ClassLoader for BootstrapClassLoader {
    fn lookup_static_method(
        &self,
        qualified_name: Qualifier,
    ) -> Result<Rc<MethodInfo>, JvmException> {
        match &qualified_name {
            Qualifier::MethodRef {
                class_name,
                name: _,
                descriptor: _,
            } => {
                let klass = self.load_class(class_name.clone())?;

                if !klass.is_being_initialized() && !klass.is_initialized() {
                    self.load_and_init_class(class_name.clone())?;
                }
                assert!(klass.is_initialized() || klass.is_being_initialized());

                klass
                    .get_method_by_qualified_name(qualified_name)
                    .ok_or(JvmException::new())
            }
            _ => Err(JvmException::new()),
        }
    }

    fn load_class(&self, qualified_name: String) -> Result<Rc<Klass>, JvmException> {
        self.load_class(qualified_name)
    }

    fn load_and_init_class(&self, qualified_name: String) -> Result<Rc<Klass>, JvmException> {
        let class = self.load_class(qualified_name)?;
        self.link_class(class.clone())?;
        self.initialize_class(class.clone())?;
        Ok(class)
    }
}

impl BootstrapClassLoader {
    pub fn new(
        resource_locator: ResourceLocator,
        context: Rc<GlobalContext>,
    ) -> BootstrapClassLoader {
        BootstrapClassLoader {
            lookup_table: RefCell::new(HashMap::new()),
            resource_locator,
            context,
        }
    }

    pub fn bootstrap(&self) -> Result<(), JvmException> {
        self.load_and_init_class(String::from("java/lang/Object"))?;

        Ok(())
    }

    /// Loads a class from the bootstrap classpath or returns a `JvmException` if the class lookup
    /// or the parsing fails.
    ///
    /// 5.3.1 Section of JVM Specification
    pub fn load_class(&self, class_name: String) -> Result<Rc<Klass>, JvmException> {
        let class_lookup = self.lookup_table.borrow().get(class_name.as_str()).cloned();
        match class_lookup {
            Some(class) => {
                if !class.is_loaded() {
                    self.do_load(&class_name)
                } else {
                    Ok(class.clone())
                }
            }
            None => self.do_load(&class_name),
        }
    }

    fn do_load(&self, class_name: &String) -> Result<Rc<Klass>, JvmException> {
        let raw_class = self
            .resource_locator
            .read_from_resource(&class_name)
            .map_err(|err| JvmException::from_string(err.to_string()))?;
        //TODO: ClassNotFoundException
        let derived_class = self.derive_class(raw_class)?;
        derived_class.set_status(Loaded);

        //record the resolved class in the cache
        self.lookup_table
            .borrow_mut()
            .insert(class_name.clone(), derived_class.clone());

        //return a pointer to it
        Ok(derived_class)
    }

    /// Tries to parse a class from the given bytes. If succeeds returns a `Klass` wrapped in an `Rc`,
    /// otherwise will return the appropriate `JvmException`.  
    fn derive_class(&self, class_to_derive: Vec<u8>) -> Result<Rc<Klass>, JvmException> {
        let klass = ClassParser::from(class_to_derive)
            .parse_class()
            .map_err(|err| JvmException::from_string(err.to_string()))?;
        //TODO: LinkageError, ClassFormatError and likes

        //we've got the class, now need to check its superclass
        match klass.qualified_super_name() {
            Some(super_name) => {
                self.load_class(super_name)?;
                //TODO: 5.4.3.1 steps are missing to load Reference Type for array classes
                //as well as applying access control to this class and its superclass

                //TODO check some constraints, superclass is not an interface and superclass
                // is not this class
            }
            None => {
                assert_eq!(
                    klass.qualified_name(),
                    String::from("java/lang/Object"),
                    "Only java/lang/Object permitted to not have a super class, but class {} doesn't have any!",
                    klass.qualified_name()
                );
            }
        }

        //loading up all direct interfaces
        let _loaded_interfaces = klass
            .interfaces()
            .iter()
            .map(|interface| self.load_class(interface.clone()))
            .collect_to_result()?;

        //TODO add extra checks over resolved interfaces: IncompatibleClassChangeError, and ClassCircularityError

        Ok(Rc::new(klass))
    }

    fn link_class(&self, class_to_link: Rc<Klass>) -> Result<(), JvmException> {
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

    fn verify_class(&self, class_to_link: Rc<Klass>) -> Result<(), JvmException> {
        //TODO do some bytecode verification
        Ok(())
    }

    fn prepare_class(&self, class_to_prepare: Rc<Klass>) -> Result<(), JvmException> {
        //initialize static fields to their default values
        class_to_prepare.initialize_static_fields();

        // Register bootstrap native method. Probably non-standard... Will need to check
        class_to_prepare.register_natives();
        Ok(())
    }

    fn initialize_class(&self, class_to_init: Rc<Klass>) -> Result<(), JvmException> {
        assert!(
            class_to_init.is_linked(),
            "Class should be linked before calling init!"
        );

        if !class_to_init.is_initialized() {
            class_to_init.set_status(BeingInitialized);

            class_to_init
                .get_method_by_name_desc("<clinit>()V".to_string())
                .map(|init: Rc<MethodInfo>| -> Result<(), JvmException> {
                    let frame = StackFrame::new(&self.context, class_to_init.clone());
                    let result: JvmValue = frame.execute_method(init, class_to_init.clone())?;
                    Ok(())
                })
                .unwrap_or_else(|| Ok(()))?;
            class_to_init.set_status(Initialized);
        }
        Ok(())
    }
}
