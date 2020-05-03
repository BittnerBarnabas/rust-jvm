use crate::core::jvm_value::JvmValue;

pub struct EvaluationStack {
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
