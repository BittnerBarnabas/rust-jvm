use crate::share::classfile::class_loader::ClassLoader;
use crate::share::classfile::klass::Klass;
use crate::share::classfile::method::MethodInfo;
use crate::share::interpreter::interpreter;
use crate::share::interpreter::local_variables::LocalVariableStore;
use crate::share::memory::heap::JvmHeap;
use crate::share::native::native_methods::NativeMethodArgs;
use crate::share::utilities::context::GlobalContext;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue;
use mockall::*;
use std::rc::Rc;

pub trait JvmStackFrame {
    fn class_loader(&self) -> Rc<dyn ClassLoader>;
    fn heap(&self) -> Rc<JvmHeap>;
    fn current_class(&self) -> Rc<Klass>;
    fn execute_method(
        &self,
        method: Rc<MethodInfo>,
        klass: Rc<Klass>,
    ) -> Result<JvmValue, JvmException>;
}

pub struct StackFrame<'a> {
    previous: Option<&'a StackFrame<'a>>,
    context: &'a GlobalContext,
    current_class: Rc<Klass>,
    current_method: Option<Rc<MethodInfo>>,
}

impl<'a> StackFrame<'a> {
    pub fn new(context: &'a GlobalContext, current_class: Rc<Klass>) -> StackFrame<'a> {
        StackFrame {
            previous: None,
            context,
            current_class,
            current_method: None,
        }
    }
}

#[automock]
impl JvmStackFrame for StackFrame<'_> {
    fn class_loader(&self) -> Rc<dyn ClassLoader> {
        self.context.class_loader().clone()
    }

    fn heap(&self) -> Rc<JvmHeap> {
        self.context.heap().clone()
    }

    fn current_class(&self) -> Rc<Klass> {
        self.current_class.clone()
    }

    fn execute_method(
        &self,
        method: Rc<MethodInfo>,
        klass: Rc<Klass>,
    ) -> Result<JvmValue, JvmException> {
        let next_frame = StackFrame {
            previous: Some(self),
            context: self.context.clone(),
            current_class: klass.clone(),
            current_method: Some(method.clone()),
        };

        if method.is_native() {
            let native_fn = method
                .native_method()
                .ok_or(JvmException::from_string(format!(
                    "Native method is not linked for: {}",
                    method.name_desc()
                )))?;
            return native_fn(NativeMethodArgs::new(&klass, &self.context));
        }

        match method.code_info() {
            Some(code_info) => {
                let mut local_variables: LocalVariableStore =
                    LocalVariableStore::new(code_info.local_variables() as usize);
                let result = crate::share::interpreter::interpreter::interpret(
                    &next_frame,
                    code_info.bytes(),
                    &mut local_variables,
                );
                return result;
            }
            _ => Err(JvmException::new()),
        }
    }
}
