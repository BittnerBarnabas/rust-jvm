use crate::core::jvm::JvmValue;
use crate::core::opcode;

struct EvaluationStack {
    stack: Vec<JvmValue>
}

impl EvaluationStack {
    pub fn new() -> EvaluationStack {
        EvaluationStack {
            stack: Vec::new()
        }
    }

    pub fn add(&mut self) -> () {
        let rhs = self.stack.pop().expect("A value is expected in the stack!");
        let lhs = self.stack.pop().expect("A value is expected in the stack!");

        match (lhs, rhs) {
            (JvmValue::Int { val: lhsVal }, JvmValue::Int { val: rhsVal }) => {
                self.stack.push(JvmValue::Int { val: lhsVal + rhsVal })
            }
            (lhs, rhs) => panic!(format!("Cannot add 2 values of type: {} {}", lhs, rhs))
        }
    }

    pub fn i_constant(&mut self, constant: i32) -> () {
        self.stack.push(JvmValue::Int {
            val: constant
        })
    }

    pub fn pop(&mut self) -> JvmValue {
        self.stack.pop().expect("Cannot pop from empty stack!")
    }

    pub fn push(&mut self, value: JvmValue) {
        self.stack.push(value);
    }
}

pub fn interpret(byte_codes: &Vec<u8>) -> Option<JvmValue> {
    let mut current = 0;
    let mut eval_stack = EvaluationStack::new();
    let mut local_variables: Vec<JvmValue> = Vec::new();
    loop {
        match byte_codes.get(current) {
            Some(byte_code) => {
                match byte_code {
                    &opcode::IADD => eval_stack.add(),
                    &opcode::ICONST_M1 => eval_stack.i_constant(-1),
                    &opcode::ICONST_0 => eval_stack.i_constant(0),
                    &opcode::ICONST_1 => eval_stack.i_constant(1),
                    &opcode::ICONST_2 => eval_stack.i_constant(2),
                    &opcode::ICONST_3 => eval_stack.i_constant(3),
                    &opcode::ICONST_4 => eval_stack.i_constant(4),
                    &opcode::ICONST_5 => eval_stack.i_constant(5),
                    &opcode::ISTORE_0 => store_local_variable(&mut local_variables, eval_stack.pop(), 0),
                    &opcode::ISTORE_1 => store_local_variable(&mut local_variables, eval_stack.pop(), 1),
                    &opcode::ISTORE_2 => store_local_variable(&mut local_variables, eval_stack.pop(), 2),
                    &opcode::ISTORE_3 => store_local_variable(&mut local_variables, eval_stack.pop(), 3),
                    &opcode::ILOAD_0 => load_local_variable(&mut local_variables, &mut eval_stack, 0),
                    &opcode::ILOAD_1 => load_local_variable(&mut local_variables, &mut eval_stack, 1),
                    &opcode::ILOAD_2 => load_local_variable(&mut local_variables, &mut eval_stack, 2),
                    &opcode::ILOAD_3 => load_local_variable(&mut local_variables, &mut eval_stack, 3),
                    &opcode::IRETURN => {
                        match eval_stack.pop() {
                            intValue @ JvmValue::Int { val: _ } => return Some(intValue),
                            _ => panic!("Non-int value was found on top of stack when executing IRETURN"),
                        }
                    }
                    &opcode::RETURN => return None,
                    _ => panic!(format!("Unsupported byte code: {}", byte_code))
                }
            }
            None => panic!("Malformed array of byte codes! Should have been terminated with Return")
        }
        current += 1;
    }
}

fn store_local_variable(local_variables: &mut Vec<JvmValue>, variable: JvmValue, index: usize) {
    if local_variables.len() <= index {
        local_variables.reserve(1);
        local_variables.resize(index + 1, JvmValue::ObjRef { val: 0 });
    }
    local_variables[index] = variable;
}

fn load_local_variable(local_variables: &mut Vec<JvmValue>, eval_stack: &mut EvaluationStack, index: usize) {
    if local_variables.len() <= index {
        panic!(format!("There's no local variable at index: {}", index))
    }
    eval_stack.push(local_variables[index].clone());
}

