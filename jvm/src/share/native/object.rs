use crate::share::native::native_methods::NativeMethodArgs;
use crate::share::utilities::jvm_value::JvmValue;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::native::native_methods;

pub fn register_natives(args: NativeMethodArgs) -> Result<JvmValue, JvmException> {
    native_methods::register_natives(args);
    Ok(JvmValue::Void {})
}

pub fn hash_code(_args: NativeMethodArgs) -> Result<JvmValue, JvmException> {
    //TODO Implement this
    Ok(JvmValue::Int { val: 1 })
}