use crate::core::class_loader::ClassLoader;
use crate::core::interpreter::interpreter;
use crate::core::interpreter::local_variables::LocalVariableStore;
use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;
use crate::core::klass::klass::Klass;
use crate::core::klass::method::MethodInfo;
use mockall::*;
use std::rc::Rc;

const DEFAULT_LOCAL_VARIABLE_STORE_SIZE: usize = 128;

pub trait JvmStackFrame {
    fn class_loader(&self) -> &ClassLoader;
    fn current_class(&self) -> &Klass;
    fn execute_method(
        &self,
        method: Rc<MethodInfo>,
        klass: &Klass,
    ) -> Result<JvmValue, JvmException>;
}

pub struct StackFrame<'a> {
    previous: Option<&'a StackFrame<'a>>,
    class_loader: &'a ClassLoader,
    current_class: &'a Klass,
    current_method: Option<Rc<MethodInfo>>,
}

impl<'a> StackFrame<'a> {
    pub fn new(class_loader: &'a ClassLoader, current_class: &'a Klass) -> StackFrame<'a> {
        StackFrame {
            previous: None,
            class_loader,
            current_class,
            current_method: None,
        }
    }
}

#[automock]
impl JvmStackFrame for StackFrame<'_> {
    fn class_loader(&self) -> &ClassLoader {
        self.class_loader
    }

    fn current_class(&self) -> &Klass {
        self.current_class
    }

    fn execute_method(
        &self,
        method: Rc<MethodInfo>,
        klass: &Klass,
    ) -> Result<JvmValue, JvmException> {
        let next_frame = StackFrame {
            previous: Some(self),
            class_loader: self.class_loader,
            current_class: klass,
            current_method: Some(method.clone()),
        };

        if method.is_native() {
            let native_fn = method.get_native_method().ok_or(JvmException::new())?;
            return native_fn();
        }

        match method.get_code() {
            Some(code) => {
                let mut local_variables: LocalVariableStore =
                    LocalVariableStore::new(DEFAULT_LOCAL_VARIABLE_STORE_SIZE);
                let result = interpreter::interpret(&next_frame, code, &mut local_variables);
                return result;
            }
            _ => Err(JvmException::new()),
        }
    }
}
