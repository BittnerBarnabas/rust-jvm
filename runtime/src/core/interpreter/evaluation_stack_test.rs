use crate::core::interpreter::evaluation_stack::MockEvaluationStack as EvaluationStack;
use crate::core::jvm_value::JvmValue;

#[test]
pub fn myTest() {
    let mut mymock = EvaluationStack::default();
    mymock
        .expect_pop()
        .returning(|| JvmValue::Boolean { val: false });
    let x = mymock.pop();
    println!("{}", x);
}
