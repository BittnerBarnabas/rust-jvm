#[cfg(test)]
mod tests {
    use crate::core::interpreter;
    use crate::core::jvm::JvmValue;
    use crate::core::opcode;

    #[test]
    fn storing_and_adding_local_integers() {
        let opcodes: Vec<u8> = vec![
            opcode::ICONST_2,
            opcode::ISTORE_1,
            opcode::ICONST_4,
            opcode::ISTORE,
            0x05,
            opcode::ILOAD_1,
            opcode::ILOAD,
            0x05,
            opcode::IADD,
            opcode::ISTORE_3,
            opcode::ILOAD_3,
            opcode::IRETURN,
        ];

        let interpreter_result = interpreter::interpret(&opcodes);
        assert_eq!(interpreter_result, Some(JvmValue::Int { val: 6 }));
    }
}
