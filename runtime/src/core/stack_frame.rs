use crate::core::class_loader::ClassLoader;
use crate::core::heap::heap::JvmHeap;
use crate::core::interpreter;
use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;
use crate::core::klass::klass::Klass;
use crate::core::klass::method::MethodInfo;
use std::rc::Rc;

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

    pub fn class_loader(&self) -> &ClassLoader {
        self.class_loader
    }

    pub fn current_class(&self) -> &Klass {
        self.current_class
    }

    pub fn execute_method(
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
                let result = interpreter::interpret(&next_frame, code);
                return result;
            }
            _ => Err(JvmException::new()),
        }
    }
}
