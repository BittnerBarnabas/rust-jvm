use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::{JvmValue, ObjectRef};

#[cfg(test)]
#[path = "./evaluation_stack_test.rs"]
mod evaluation_stack_test;

pub struct EvaluationStack {
    stack: Vec<JvmValue>,
}

#[cfg_attr(test, mockall::automock)]
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
            (lhs, rhs) => panic!(format!("Cannot add 2 values of type: {:?} {:?}", lhs, rhs)),
        }
    }

    pub fn mul(&mut self) -> () {
        let rhs = self.stack.pop().expect("A value is expected in the stack!");
        let lhs = self.stack.pop().expect("A value is expected in the stack!");

        match (lhs, rhs) {
            (JvmValue::Int { val: lhs_val }, JvmValue::Int { val: rhs_val }) => {
                self.stack.push(JvmValue::Int {
                    val: lhs_val * rhs_val,
                })
            }
            (lhs, rhs) => panic!(format!("Cannot add 2 values of type: {:?} {:?}", lhs, rhs)),
        }
    }

    pub fn i_constant(&mut self, constant: i32) -> () {
        self.stack.push(JvmValue::Int { val: constant })
    }

    pub fn pop(&mut self) -> JvmValue {
        self.stack.pop().expect("Cannot pop from empty stack!")
    }

    pub fn pop_int(&mut self) -> Result<i32, JvmException> {
        match self.pop() {
            JvmValue::Int { val } => Ok(val),
            other => Err(JvmException::from(format!(
                "JvmValue::Int expected but got: {:?}",
                other
            ))),
        }
    }

    pub fn pop_ref(&mut self) -> Result<ObjectRef, JvmException> {
        if let JvmValue::ObjRef(object_ref) = self.pop() {
            Ok(object_ref)
        } else {
            Err(JvmException::from("Non-object ref value was found on top of stack!"))
        }
    }

    pub fn push(&mut self, value: JvmValue) {
        self.stack.push(value);
    }

    pub fn stack(&self) -> &Vec<JvmValue> {
        &self.stack
    }
}

#[cfg(test)]
impl EvaluationStack {
    pub fn stack_mut(&mut self) -> &mut Vec<JvmValue> {
        &mut self.stack
    }
}
