use mockall::predicate::*;
use mockall::*;

use crate::core::interpreter::interpreter::interpret;
use crate::core::interpreter::local_variables::MockLocalVariableStore as LocalVariableStore;
// use crate::core::stack_frame::StackFrame;
use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;
// use crate::core::interpreter::local_variables::LocalVariableStore;
use crate::core::opcode;
use crate::core::stack_frame::MockStackFrame as StackFrame;

fn run_interpreter_only(code: Vec<u8>) -> Result<JvmValue, JvmException> {
    let mut store = LocalVariableStore::default();
    let frame = StackFrame::new();

    let result = interpret(&frame, &code, &mut store);
    result
}

#[test]
pub fn noop() {
    let result = run_interpreter_only(vec![opcode::NOP, opcode::RETURN]);
    assert_eq!(result, Ok(JvmValue::Void {}))
}

#[test]
pub fn bipush_int_values_then_ireturn() {
    let result = run_interpreter_only(vec![0x10, opcode::BIPUSH, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 16 }))
}

#[test]
pub fn iconst_m1_then_ireturn() {
    let result = run_interpreter_only(vec![opcode::ICONST_M1, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: -1 }))
}

#[test]
pub fn iconst_1_then_ireturn() {
    let result = run_interpreter_only(vec![opcode::ICONST_1, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 1 }))
}

#[test]
pub fn iconst_2_then_ireturn() {
    let result = run_interpreter_only(vec![opcode::ICONST_2, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 2 }))
}

#[test]
pub fn iconst_3_then_ireturn() {
    let result = run_interpreter_only(vec![opcode::ICONST_3, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 3 }))
}

#[test]
pub fn iconst_4_then_ireturn() {
    let result = run_interpreter_only(vec![opcode::ICONST_4, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 4 }))
}

#[test]
pub fn iconst_5_then_ireturn() {
    let result = run_interpreter_only(vec![opcode::ICONST_5, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 5 }))
}

#[test]
pub fn iload_with_correct_index() {
    let mut store = LocalVariableStore::default();
    let frame = StackFrame::new();
    let code = vec![opcode::ILOAD, 0x10, opcode::IRETURN];

    store
        .expect_load()
        .with(predicate::eq(16))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}

#[test]
#[should_panic(expected = "Test Exception")]
pub fn iload_with_incorrect_index() {
    let mut store = LocalVariableStore::default();
    let frame = StackFrame::new();
    let code = vec![opcode::ILOAD, 0x02, opcode::IRETURN];

    store
        .expect_load()
        .with(predicate::eq(2))
        .times(1)
        .returning(|_| panic!("Test Exception"));

    interpret(&frame, &code, &mut store);
}

#[test]
pub fn iload0_with_correct_index() {
    let mut store = LocalVariableStore::default();
    let frame = StackFrame::new();
    let code = vec![opcode::ILOAD_0, opcode::IRETURN];

    store
        .expect_load()
        .with(predicate::eq(0))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}
#[test]
pub fn iload1_with_correct_index() {
    let mut store = LocalVariableStore::default();
    let frame = StackFrame::new();
    let code = vec![opcode::ILOAD_1, opcode::IRETURN];

    store
        .expect_load()
        .with(predicate::eq(1))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}
#[test]
pub fn iload2_with_correct_index() {
    let mut store = LocalVariableStore::default();
    let frame = StackFrame::new();
    let code = vec![opcode::ILOAD_2, opcode::IRETURN];

    store
        .expect_load()
        .with(predicate::eq(2))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}
#[test]
pub fn iload3_with_correct_index() {
    let mut store = LocalVariableStore::default();
    let frame = StackFrame::new();
    let code = vec![opcode::ILOAD_3, opcode::IRETURN];

    store
        .expect_load()
        .with(predicate::eq(3))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}
