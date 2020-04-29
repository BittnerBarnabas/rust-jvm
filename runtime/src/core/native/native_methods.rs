use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;

pub fn hash_code() -> Result<JvmValue, JvmException> {
    //TODO Implement this
    Ok(JvmValue::Int { val: 1 })
}

pub fn register_natives() -> Result<JvmValue, JvmException> {
    Ok(JvmValue::Void {})
}
