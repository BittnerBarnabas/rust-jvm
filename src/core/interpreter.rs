use crate::core::jvm::JvmValue;
use crate::core::opcode;

const DEFAULT_LOCAL_VARIABLE_STORE_SIZE: usize = 128;

struct EvaluationStack {
    stack: Vec<JvmValue>,
}

impl EvaluationStack {
    pub fn new() -> EvaluationStack {
        EvaluationStack { stack: Vec::new() }
    }

    pub fn add(&mut self) -> () {
        let rhs = self.stack.pop().expect("A value is expected in the stack!");
        let lhs = self.stack.pop().expect("A value is expected in the stack!");

        match (lhs, rhs) {
            (JvmValue::Int { val: lhs_val }, JvmValue::Int { val: rhs_val }) => {
                self.stack.push(JvmValue::Int {
                    val: lhs_val + rhs_val,
                })
            }
            (lhs, rhs) => panic!(format!("Cannot add 2 values of type: {} {}", lhs, rhs)),
        }
    }

    pub fn i_constant(&mut self, constant: i32) -> () {
        self.stack.push(JvmValue::Int { val: constant })
    }

    pub fn pop(&mut self) -> JvmValue {
        self.stack.pop().expect("Cannot pop from empty stack!")
    }

    pub fn push(&mut self, value: JvmValue) {
        self.stack.push(value);
    }
}

struct LocalVariableStore {
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

pub fn interpret(byte_codes: &Vec<u8>) -> Option<JvmValue> {
    let mut current = 0;
    let mut eval_stack = EvaluationStack::new();
    let mut local_variables: LocalVariableStore =
        LocalVariableStore::new(DEFAULT_LOCAL_VARIABLE_STORE_SIZE);
    loop {
        match byte_codes.get(current) {
            Some(byte_code) => match byte_code {
                &opcode::IADD => eval_stack.add(),
                &opcode::ICONST_M1 => eval_stack.i_constant(-1),
                &opcode::ICONST_0 => eval_stack.i_constant(0),
                &opcode::ICONST_1 => eval_stack.i_constant(1),
                &opcode::ICONST_2 => eval_stack.i_constant(2),
                &opcode::ICONST_3 => eval_stack.i_constant(3),
                &opcode::ICONST_4 => eval_stack.i_constant(4),
                &opcode::ICONST_5 => eval_stack.i_constant(5),
                &opcode::ISTORE => {
                    current += 1;
                    local_variables.store(eval_stack.pop(), byte_codes[current])
                }
                &opcode::ISTORE_0 => local_variables.store(eval_stack.pop(), 0),
                &opcode::ISTORE_1 => local_variables.store(eval_stack.pop(), 1),
                &opcode::ISTORE_2 => local_variables.store(eval_stack.pop(), 2),
                &opcode::ISTORE_3 => local_variables.store(eval_stack.pop(), 3),
                &opcode::ILOAD => {
                    current += 1;
                    eval_stack.push(local_variables.load(byte_codes[current]))
                }
                &opcode::ILOAD_0 => eval_stack.push(local_variables.load(0)),
                &opcode::ILOAD_1 => eval_stack.push(local_variables.load(1)),
                &opcode::ILOAD_2 => eval_stack.push(local_variables.load(2)),
                &opcode::ILOAD_3 => eval_stack.push(local_variables.load(3)),
                &opcode::IRETURN => match eval_stack.pop() {
                    java_int @ JvmValue::Int { val: _ } => return Some(java_int),
                    _ => panic!("Non-int value was found on top of stack when executing IRETURN"),
                },
                &opcode::RETURN => return None,
                _ => panic!(format!("Unsupported byte code: {}", byte_code)),
            },
            None => {
                panic!("Malformed array of byte codes! Should have been terminated with Return")
            }
        }
        current += 1;
    }
}
