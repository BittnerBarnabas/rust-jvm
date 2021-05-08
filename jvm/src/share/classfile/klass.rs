use crate::share::classfile::attribute::AttributeInfo;
use crate::share::classfile::constant_pool::{ConstantPool, Qualifier};
use crate::share::classfile::field::FieldInfo;
use crate::share::classfile::klass::ClassLoadingStatus::{
    BeingInitialized, Initialized, Linked, Loaded, Mentioned,
};
use crate::share::classfile::method::MethodInfo;
use std::sync::{Arc, Mutex};
use crate::share::native::native_method_repo::NativeMethodRepo;
use crate::share::parser::descriptors::FieldDescriptor;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ClassLoadingStatus {
    Mentioned,
    Loaded,
    Linked,
    BeingInitialized,
    Initialized,
}

pub struct Klass {
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class: String,
    super_class_name: Option<String>,
    super_class: Mutex<Option<Arc<Klass>>>,
    interfaces: Vec<String>,
    instance_fields: Vec<Arc<FieldInfo>>,
    static_fields: Vec<Arc<FieldInfo>>,
    methods: Vec<Arc<MethodInfo>>,
    attributes: Vec<AttributeInfo>,
    status: Mutex<ClassLoadingStatus>,
}

impl Debug for Klass {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.this_class)
    }
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
        mut fields: Vec<FieldInfo>,
        mut methods: Vec<MethodInfo>,
        attributes: Vec<AttributeInfo>,
    ) -> Klass {
        let methods: Vec<Arc<MethodInfo>> = methods.drain(0..).map(|m| Arc::new(m)).collect();

        let mut instance_fields: Vec<Arc<FieldInfo>> = Vec::new();
        let mut static_fields: Vec<Arc<FieldInfo>> = Vec::new();
        fields.drain(0..).for_each(|elem| {
            if elem.is_static() {
                static_fields.push(Arc::new(elem))
            } else {
                instance_fields.push(Arc::new(elem))
            }
        });

        Klass {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class_name,
            super_class: Mutex::new(None),
            interfaces,
            instance_fields,
            static_fields,
            methods,
            attributes,
            status: Mutex::new(Mentioned),
        }
    }

    pub fn qualified_name(&self) -> String {
        self.this_class.clone()
    }

    pub fn qualified_super_name(&self) -> Option<String> {
        self.super_class_name.clone()
    }

    // pub fn super_class(&self) -> Ref<Option<Arc<Klass>>> {
    //     self.super_class.borrow()
    // }

    pub fn set_super_class(&self, super_class: Arc<Klass>) {
        self.super_class.lock().unwrap().replace(super_class);
    }

    pub fn interfaces(&self) -> Vec<String> {
        self.interfaces.iter().cloned().collect()
    }

    pub fn instance_fields(&self) -> &Vec<Arc<FieldInfo>> {
        &self.instance_fields
    }

    pub fn initialize_static_fields(&self) {
        self.static_fields
            .iter()
            .for_each(|f| f.set_static_value(f.default()))
    }

    pub fn get_static_field_by_name_and_type(&self, name: &String, type_descriptor: &String) -> Option<Arc<FieldInfo>> {
        self.static_fields
            .iter()
            .find(|f| f.matches_name_and_type(name, type_descriptor))
            .map(|f| f.clone())
    }

    pub fn get_instance_field_offset(&self, name: &String, type_descriptor: &String) -> Option<usize> {
        for i in 0..self.instance_fields.len() {
            if self.instance_fields[i].matches_name_and_type(name, type_descriptor) {
                return Some(i);
            }
        }
        None
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
        *self.status.lock().unwrap() >= Mentioned
    }

    pub fn is_loaded(&self) -> bool {
        *self.status.lock().unwrap() >= Loaded
    }

    pub fn is_linked(&self) -> bool {
        *self.status.lock().unwrap() >= Linked
    }

    pub fn is_being_initialized(&self) -> bool {
        *self.status.lock().unwrap() == BeingInitialized
    }

    pub fn is_initialized(&self) -> bool {
        *self.status.lock().unwrap() == Initialized
    }

    pub fn set_status(&self, status: ClassLoadingStatus) {
        *self.status.lock().unwrap() = status;
    }

    pub fn get_method_by_qualified_name(
        &self,
        qualified_name: &Qualifier,
    ) -> Option<Arc<MethodInfo>> {
        return match qualified_name {
            Qualifier::MethodRef {
                class_name: _,
                name,
                descriptor,
            } => self.get_method_by_name_desc(format!("{}{}", name, descriptor)),
            _ => None,
        };
    }

    pub fn get_method_by_name_desc(&self, name_desc: String) -> Option<Arc<MethodInfo>> {
        self.methods
            .iter()
            .find(|method| name_desc == method.name_desc())
            .map(|method| Arc::clone(method))
    }

    pub fn register_natives(&self, native_method_repo: &NativeMethodRepo) {
        self.get_method_by_name_desc("registerNatives()V".to_string())
            .map(|method| {
                native_method_repo.find_method(method.as_ref())
                    .map(|native_method| {
                        method.set_native_method(native_method)
                    })
            });
    }

    pub fn methods(&self) -> &Vec<Arc<MethodInfo>> {
        &self.methods
    }
}
