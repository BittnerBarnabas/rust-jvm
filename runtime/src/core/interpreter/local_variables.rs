use crate::core::jvm_value::JvmValue;

pub struct LocalVariableStore {
    store: Vec<JvmValue>,
}

impl LocalVariableStore {
    pub fn new(init_size: usize) -> LocalVariableStore {
        let mut store: Vec<JvmValue> = Vec::with_capacity(init_size);
        store.resize(init_size, JvmValue::null_obj());

        LocalVariableStore { store }
    }

    pub fn store(&mut self, var: JvmValue, ind: u8) {
        self.store[ind as usize] = var;
    }

    pub fn load(&self, ind: u8) -> JvmValue {
        self.store[ind as usize].clone()
    }
}
