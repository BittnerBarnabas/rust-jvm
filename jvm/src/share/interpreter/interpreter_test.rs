use std::sync::Arc;

use mockall::*;
use mockall::predicate::*;
use mockall_double::double;

use crate::share::interpreter::interpreter::interpret;
#[double]
use crate::share::interpreter::local_variables::JvmLocalVariableStore;
use crate::share::interpreter::opcode;
#[double]
use crate::share::memory::heap::Heap;
#[double]
use crate::share::runtime::stack_frame::JvmStackFrame;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::{JvmValue, ObjectRef};
use crate::share::utilities::jvm_value::JvmValue::ObjRef;
use crate::share::memory::oop::Oop::ObjectOop;

fn run_interpreter(code: Vec<u8>) -> Result<JvmValue, JvmException> {
    let mut store = JvmLocalVariableStore::new();
    let frame = JvmStackFrame::new();

    let result = interpret(&frame, &code, &mut store);
    result
}

#[test]
pub fn noop() {
    let result = run_interpreter(vec![opcode::NOP, opcode::RETURN]);
    assert_eq!(result, Ok(JvmValue::Void {}))
}

#[test]
pub fn bipush_int_values_then_ireturn() {
    let result = run_interpreter(vec![0x10, opcode::BIPUSH, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 16 }))
}

#[test]
pub fn iconst_m1_then_ireturn() {
    let result = run_interpreter(vec![opcode::ICONST_M1, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: -1 }))
}

#[test]
pub fn iconst_1_then_ireturn() {
    let result = run_interpreter(vec![opcode::ICONST_1, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 1 }))
}

#[test]
pub fn iconst_2_then_ireturn() {
    let result = run_interpreter(vec![opcode::ICONST_2, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 2 }))
}

#[test]
pub fn iconst_3_then_ireturn() {
    let result = run_interpreter(vec![opcode::ICONST_3, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 3 }))
}

#[test]
pub fn iconst_4_then_ireturn() {
    let result = run_interpreter(vec![opcode::ICONST_4, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 4 }))
}

#[test]
pub fn iconst_5_then_ireturn() {
    let result = run_interpreter(vec![opcode::ICONST_5, opcode::IRETURN]);
    assert_eq!(result, Ok(JvmValue::Int { val: 5 }))
}

#[test]
pub fn iload_with_correct_index() {
    let mut store = JvmLocalVariableStore::new();
    let frame = JvmStackFrame::new();
    let code = vec![opcode::ILOAD, 0x10, opcode::IRETURN];

    store
        .expect_load()
        .with(eq(16))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}

#[test]
#[should_panic(expected = "Test Exception")]
pub fn iload_with_incorrect_index() {
    let mut store = JvmLocalVariableStore::new();
    let frame = JvmStackFrame::new();
    let code = vec![opcode::ILOAD, 0x02, opcode::IRETURN];

    store
        .expect_load()
        .with(eq(2))
        .times(1)
        .returning(|_| panic!("Test Exception"));

    interpret(&frame, &code, &mut store);
}

#[test]
pub fn iload0_with_correct_index() {
    let mut store = JvmLocalVariableStore::new();
    let frame = JvmStackFrame::new();
    let code = vec![opcode::ILOAD_0, opcode::IRETURN];

    store
        .expect_load()
        .with(eq(0))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}

#[test]
pub fn iload1_with_correct_index() {
    let mut store = JvmLocalVariableStore::new();
    let frame = JvmStackFrame::new();
    let code = vec![opcode::ILOAD_1, opcode::IRETURN];

    store
        .expect_load()
        .with(eq(1))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}

#[test]
pub fn iload2_with_correct_index() {
    let mut store = JvmLocalVariableStore::new();
    let frame = JvmStackFrame::new();
    let code = vec![opcode::ILOAD_2, opcode::IRETURN];

    store
        .expect_load()
        .with(eq(2))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}

#[test]
pub fn iload3_with_correct_index() {
    let mut store = JvmLocalVariableStore::new();
    let frame = JvmStackFrame::new();
    let code = vec![opcode::ILOAD_3, opcode::IRETURN];

    store
        .expect_load()
        .with(eq(3))
        .times(1)
        .returning(|_| JvmValue::Int { val: 12 });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(Ok(JvmValue::Int { val: 12 }), result)
}


fn test_conditional_compare_to_zero(test_value: u8, cond_opcode: u8, expected_return_value: i32) {
    let code = vec![test_value,
                    cond_opcode, 0x0, 0x05,
                    opcode::ICONST_0, opcode::IRETURN,
                    opcode::ICONST_1, opcode::IRETURN];

    let actual_return = run_interpreter(code);
    assert_eq!(actual_return, Ok(JvmValue::Int { val: expected_return_value }))
}

#[test]
pub fn ifeq() {
    test_conditional_compare_to_zero(opcode::ICONST_0, opcode::IFEQ, 1);
    test_conditional_compare_to_zero(opcode::ICONST_1, opcode::IFEQ, 0);
}

#[test]
pub fn ifne() {
    test_conditional_compare_to_zero(opcode::ICONST_0, opcode::IFNE, 0);
    test_conditional_compare_to_zero(opcode::ICONST_1, opcode::IFNE, 1);
}

//TODO Add the remained of the conditionals

fn test_conditional_compare_to_value(test_value: u8, test_value_compare_to: u8, cond_opcode: u8, expected_return_value: i32) {
    let code = vec![test_value, test_value_compare_to,
                    cond_opcode, 0x0, 0x05,
                    opcode::ICONST_0, opcode::IRETURN,
                    opcode::ICONST_1, opcode::IRETURN];

    let actual_return = run_interpreter(code);
    assert_eq!(actual_return, Ok(JvmValue::Int { val: expected_return_value }))
}

#[test]
pub fn if_icmpeq() {
    test_conditional_compare_to_value(opcode::ICONST_0, opcode::ICONST_0, opcode::IF_ICMPEQ, 1);
    test_conditional_compare_to_value(opcode::ICONST_0, opcode::ICONST_1, opcode::IF_ICMPEQ, 0);
}

//TODO Add the remained of the conditionals

#[test]
fn goto() {
    let code = vec![opcode::ICONST_0, opcode::ICONST_1, opcode::GOTO, 0x0, 0x5,
                    opcode::ICONST_2, opcode::ICONST_3, opcode::IRETURN];

    let actual_return = run_interpreter(code);
    assert_eq!(actual_return, Ok(JvmValue::Int { val: 1 }))
}

#[test]
fn i_return() {
    let code = vec![opcode::ICONST_5, opcode::IRETURN];

    let actual_return = run_interpreter(code);
    assert_eq!(actual_return, Ok(JvmValue::Int { val: 5 }))
}

#[test]
fn aa_store() {
    let code = vec![
        opcode::ALOAD_3,
        opcode::ILOAD_2,
        opcode::ILOAD_1,
        opcode::AASTORE,
        opcode::RETURN
    ];

    let mut store = JvmLocalVariableStore::new();
    let mut frame = JvmStackFrame::new();

    store.expect_load()
        .with(eq(3))
        .times(1)
        .returning(|_| JvmValue::ObjRef(ObjectRef::Null));

    store.expect_load()
        .with(eq(2))
        .times(1)
        .returning(|_| JvmValue::Int { val: 5 });

    store.expect_load()
        .with(eq(1))
        .times(1)
        .returning(|_| JvmValue::Int { val: 8 });

    frame.expect_heap()
        .times(1)
        .returning(|| {
            let mut mock_heap = Heap::default();

            mock_heap.expect_store_in_array()
                .with(eq(ObjectRef::Null),  eq(5), eq(JvmValue::Int {val:8}))
                .times(1)
                .returning(|_, _, _| Ok(()));

            Arc::new(mock_heap)
        });

    let result = interpret(&frame, &code, &mut store);

    assert_eq!(result, Ok(JvmValue::Void {}))
}