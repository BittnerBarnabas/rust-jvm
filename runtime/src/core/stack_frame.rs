use crate::core::interpreter;
use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;
use crate::core::klass::method::MethodInfo;

pub struct StackFrame<'a> {
    previous: Option<&'a StackFrame<'a>>,
}

impl<'a> StackFrame<'a> {
    pub fn new() -> StackFrame<'a> {
        StackFrame { previous: None }
    }

    pub fn execute_method(&self, method: &MethodInfo) -> Result<JvmValue, JvmException> {
        let next_frame = StackFrame {
            previous: Some(self),
        };

        match method.get_code() {
            Some(code) => {
                let result = interpreter::interpret(&next_frame, code);
                return result;
            }
            _ => Err(JvmException::new()),
        }
    }
}
