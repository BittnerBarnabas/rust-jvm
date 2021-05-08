use crate::share::interpreter::evaluation_stack::EvaluationStack;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::{JvmValue, ObjectRef};

#[test]
pub fn push_and_then_pop() {
    let value_1 = JvmValue::Int { val: 1 };
    let value_2 = JvmValue::Int { val: 2 };
    let expected_value_1 = JvmValue::Int { val: 2 };
    let expected_value_2 = JvmValue::Int { val: 1 };

    let mut stack_under_test = EvaluationStack::new();

    stack_under_test.push(value_1);
    stack_under_test.push(value_2);

    assert_eq!(expected_value_1, stack_under_test.pop());
    assert_eq!(expected_value_2, stack_under_test.pop());
}

#[test]
pub fn push_i_constants() {
    let mut stack_under_test = EvaluationStack::new();
    stack_under_test.i_constant(1);
    stack_under_test.i_constant(2);

    assert_eq!(JvmValue::Int { val: 2 }, stack_under_test.pop());
    assert_eq!(JvmValue::Int { val: 1 }, stack_under_test.pop());
}

#[test]
pub fn pop_i_constants() {
    let mut stack_under_test = EvaluationStack::new();
    stack_under_test.i_constant(1);
    stack_under_test.i_constant(2);

    assert_eq!(Ok(2), stack_under_test.pop_int());
    assert_eq!(Ok(1), stack_under_test.pop_int());
}

#[test]
pub fn pop_i_constants_not_int_in_stack() {
    let mut stack_under_test = EvaluationStack::new();
    stack_under_test.push(JvmValue::Double { val: 1.0 });

    assert_eq!(Err(JvmException::from("JvmValue::Int expected but got: Double { val: 1.0 }")), stack_under_test.pop_int());
}

#[test]
pub fn add_i_constants() {
    let mut stack_under_test = EvaluationStack::new();
    stack_under_test.i_constant(1);
    stack_under_test.i_constant(2);
    stack_under_test.i_constant(3);

    stack_under_test.add();

    assert_eq!(Ok(5), stack_under_test.pop_int());
    assert_eq!(Ok(1), stack_under_test.pop_int());
}


#[test]
#[should_panic(expected = "Cannot pop from empty stack!")]
pub fn pop_from_empty_stack() {
    let mut stack_under_test = EvaluationStack::new();
    stack_under_test.pop();
}

