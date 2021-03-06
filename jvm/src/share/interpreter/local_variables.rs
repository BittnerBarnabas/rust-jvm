use crate::share::utilities::jvm_value::JvmValue;

#[cfg(test)]
#[path = "./local_variables_test.rs"]
mod local_variables_test;

#[cfg_attr(test, mockall::automock)]
pub trait JvmLocalVariableStore {
    fn store(&mut self, var: JvmValue, ind: u8);
    fn load(&self, ind: u8) -> JvmValue;
}

pub struct LocalVariableStore {
    store: Vec<JvmValue>,
}

impl LocalVariableStore {
    pub fn new(init_size: usize) -> LocalVariableStore {
        let mut store: Vec<JvmValue> = Vec::with_capacity(init_size);
        store.resize(init_size, JvmValue::null_obj());

        LocalVariableStore { store }
    }
}

impl JvmLocalVariableStore for LocalVariableStore {
    fn store(&mut self, var: JvmValue, ind: u8) {
        self.store[ind as usize] = var;
    }

    fn load(&self, ind: u8) -> JvmValue {
        self.store[ind as usize].clone()
    }
}
