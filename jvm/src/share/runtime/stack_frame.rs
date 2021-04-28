use std::sync::Arc;

use crate::share::classfile::class_loader::ClassLoader;
use crate::share::classfile::constant_pool::ConstantPool;
use crate::share::classfile::klass::Klass;
use crate::share::classfile::method::MethodInfo;
use crate::share::interpreter::local_variables::{JvmLocalVariableStore, LocalVariableStore};
use crate::share::memory::heap::Heap;
use crate::share::native::native_methods::NativeMethodArgs;
use crate::share::utilities::context::GlobalContext;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue;

#[cfg_attr(test, mockall::automock)]
pub trait JvmStackFrame {
    fn class_loader(&self) -> Arc<dyn ClassLoader>;
    fn heap(&self) -> Arc<dyn Heap>;
    fn current_class(&self) -> Arc<Klass>;
    fn constant_pool(&self) -> &ConstantPool;
    fn execute_method(
        &self,
        method: Arc<MethodInfo>,
        args: Vec<JvmValue>,
    ) -> Result<JvmValue, JvmException>;
}

pub struct StackFrame<'a> {
    previous: Option<&'a StackFrame<'a>>,
    context: &'a GlobalContext,
    current_class: Arc<Klass>,
    current_method: Option<Arc<MethodInfo>>,
}

impl<'a> StackFrame<'a> {
    pub fn new(context: &'a GlobalContext, current_class: Arc<Klass>) -> StackFrame<'a> {
        StackFrame {
            previous: None,
            context,
            current_class,
            current_method: None,
        }
    }
}

impl JvmStackFrame for StackFrame<'_> {
    fn class_loader(&self) -> Arc<dyn ClassLoader> {
        self.context.class_loader().clone()
    }

    fn heap(&self) -> Arc<dyn Heap> {
        self.context.heap().clone()
    }

    fn current_class(&self) -> Arc<Klass> {
        self.current_class.clone()
    }

    fn constant_pool(&self) -> &ConstantPool {
        self.current_class.constant_pool()
    }

    fn execute_method(
        &self,
        method: Arc<MethodInfo>,
        args: Vec<JvmValue>,
    ) -> Result<JvmValue, JvmException> {
        log::trace!("Method to execute: {}", method);
        let next_frame = StackFrame {
            previous: Some(self),
            context: self.context.clone(),
            current_class: method.get_klass(),
            current_method: Some(method.clone()),
        };

        if method.is_native() {
            let native_fn = method.native_method().ok_or(JvmException::from(format!(
                "Native method is not linked for: {}",
                method.name_desc()
            )))?;
            return native_fn(NativeMethodArgs::new(
                &next_frame.current_class,
                &next_frame.context,
            ));
        }

        //Method is Byte-Code implemented only
        match method.code_info() {
            Some(code_info) => {
                let mut local_variables: LocalVariableStore =
                    LocalVariableStore::new(code_info.local_variables() as usize);

                for i in 0..args.len() {
                    local_variables.store(*args.get(i).expect("Should not happen."), i as u8)
                }

                let result = crate::share::interpreter::interpreter::interpret(
                    &next_frame,
                    code_info.bytes(),
                    &mut local_variables,
                );
                log::trace!("Returning from byte-code method: {}", method);
                return result;
            }
            _ => Err(JvmException::new()),
        }
    }
}
