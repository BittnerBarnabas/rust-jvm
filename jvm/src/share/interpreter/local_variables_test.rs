use crate::share::interpreter::local_variables::{LocalVariableStore, JvmLocalVariableStore};
use crate::share::utilities::jvm_value::JvmValue;

#[test]
pub fn store_and_then_load() {
    let mut local_variables = LocalVariableStore::new(4);
    local_variables.store(JvmValue::Int {val: 1}, 3);
    local_variables.store(JvmValue::Int {val: 4}, 1);

    assert_eq!(JvmValue::Int {val: 1}, local_variables.load(3));
    assert_eq!(JvmValue::Int {val: 4}, local_variables.load(1));
}