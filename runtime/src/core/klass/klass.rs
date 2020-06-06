use std::rc::Rc;

use crate::core::klass::attribute::AttributeInfo;
use crate::core::klass::constant_pool::{ConstantPool, Qualifier};
use crate::core::klass::field::FieldInfo;
use crate::core::klass::klass::ClassLoadingStatus::{Initialized, Linked, Loaded, Mentioned};
use crate::core::klass::method::MethodInfo;
use std::cell::{Cell, Ref, RefCell};
use std::slice::Iter;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ClassLoadingStatus {
    Mentioned,
    Loaded,
    Linked,
    Initialized,
}

pub struct Klass {
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class: String,
    super_class_name: Option<String>,
    super_class: RefCell<Option<Rc<Klass>>>,
    interfaces: Vec<String>,
    instance_fields: Vec<Rc<FieldInfo>>,
    static_fields: Vec<Rc<FieldInfo>>,
    methods: Vec<Rc<MethodInfo>>,
    attributes: Vec<AttributeInfo>,
    status: Cell<ClassLoadingStatus>,
}

impl Klass {
    pub fn new(
        minor_version: u16,
        major_version: u16,
        constant_pool: ConstantPool,
        access_flags: u16,
        this_class: String,
        super_class_name: Option<String>,
        interfaces: Vec<String>,
        fields: Vec<FieldInfo>,
        methods: Vec<MethodInfo>,
        attributes: Vec<AttributeInfo>,
    ) -> Klass {
        let methods: Vec<Rc<MethodInfo>> = methods.iter().map(|m| Rc::new(m.clone())).collect();
        let instance_fields: Vec<Rc<FieldInfo>> = fields
            .iter()
            .filter(|field| !field.is_static())
            .map(|f| Rc::new(f.clone()))
            .collect();
        let static_fields: Vec<Rc<FieldInfo>> = fields
            .iter()
            .filter(|field| field.is_static())
            .map(|f| Rc::new(f.clone()))
            .collect();
        Klass {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class_name,
            super_class: RefCell::new(None),
            interfaces,
            instance_fields,
            static_fields,
            methods,
            attributes,
            status: Cell::new(Mentioned),
        }
    }

    pub fn qualified_name(&self) -> String {
        self.this_class.clone()
    }

    pub fn qualified_super_name(&self) -> Option<String> {
        self.super_class_name.clone()
    }

    pub fn super_class(&self) -> Ref<Option<Rc<Klass>>> {
        self.super_class.borrow()
    }

    pub fn set_super_class(&self, super_class: Rc<Klass>) {
        self.super_class.borrow_mut().replace(super_class);
    }

    pub fn interfaces(&self) -> Vec<String> {
        self.interfaces.iter().cloned().collect()
    }

    pub fn instance_fields(&self) -> &Vec<Rc<FieldInfo>> {
        &self.instance_fields
    }

    pub fn initialize_static_fields(&self) {
        self.static_fields
            .iter()
            .for_each(|f| f.set_static_value(f.default()))
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }

    pub fn referenced_classes(&self) -> Vec<String> {
        self.super_class_name
            .as_ref()
            .map_or(Vec::new(), |name| vec![name.clone()])
    }

    pub fn is_mentioned(&self) -> bool {
        self.status.get() >= Mentioned
    }

    pub fn is_loaded(&self) -> bool {
        self.status.get() >= Loaded
    }

    pub fn is_linked(&self) -> bool {
        self.status.get() >= Linked
    }

    pub fn is_initialized(&self) -> bool {
        self.status.get() >= Initialized
    }

    pub fn set_status(&self, status: ClassLoadingStatus) {
        self.status.replace(status);
    }

    pub fn get_method_by_qualified_name(
        &self,
        qualified_name: Qualifier,
    ) -> Option<Rc<MethodInfo>> {
        return match qualified_name {
            Qualifier::MethodRef {
                class_name: _,
                name,
                descriptor,
            } => self.get_method_by_name_desc(format!("{}{}", name, descriptor)),
            _ => None,
        };
    }

    pub fn get_method_by_name_desc(&self, name_desc: String) -> Option<Rc<MethodInfo>> {
        self.methods
            .iter()
            .find(|method| name_desc == method.name_desc())
            .map(|method| Rc::clone(method))
    }

    pub fn register_natives(&self) {
        self.get_method_by_name_desc("registerNatives()V".to_string())
            .map(|method| {
                method.set_native_method(crate::core::native::native_methods::register_natives)
            });
    }

    pub fn methods(&self) -> &Vec<Rc<MethodInfo>> {
        &self.methods
    }
}
