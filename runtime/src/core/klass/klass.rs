use std::rc::Rc;

use crate::core::klass::attribute::AttributeInfo;
use crate::core::klass::constant_pool::{ConstantPool, Qualifier};
use crate::core::klass::field::FieldInfo;
use crate::core::klass::method::MethodInfo;

#[derive(Clone)]
pub struct Klass {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub instance_fields: Vec<Rc<FieldInfo>>,
    pub static_fields: Vec<Rc<FieldInfo>>,
    pub methods: Vec<Rc<MethodInfo>>,
    pub attributes: Vec<AttributeInfo>,
}

impl Klass {
    pub fn new(
        minor_version: u16,
        major_version: u16,
        constant_pool: ConstantPool,
        access_flags: u16,
        this_class: String,
        super_class: Option<String>,
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
            super_class,
            interfaces,
            instance_fields,
            static_fields,
            methods,
            attributes,
        }
    }

    pub fn get_instance_fields(&self) -> Vec<Rc<FieldInfo>> {
        self.instance_fields.iter().map(|f| f.clone()).collect()
    }

    pub fn get_qualified_name(&self) -> String {
        self.this_class.clone()
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
            .find(|method| name_desc == method.get_name_desc())
            .map(|method| Rc::clone(method))
    }
}
