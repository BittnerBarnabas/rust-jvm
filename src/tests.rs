#[cfg(test)]
mod tests {
    use crate::core::opcode;
    use crate::core::interpreter;

    #[test]
    fn storing_and_adding_local_integers() {
        let opcodes: Vec<u8> = vec![
            opcode::ICONST_2,
            opcode::ISTORE_1,
            opcode::ICONST_4,
            opcode::ISTORE_2,
            opcode::ILOAD_1,
            opcode::ILOAD_2,
            opcode::IADD,
            opcode::ISTORE_3,
            opcode::ILOAD_3,
            opcode::IRETURN
        ];

        let interpreter_result = interpreter::interpret(&opcodes);
        println!("Result: {:?}", &interpreter_result);
        assert_eq!(2 + 2, 4);
    }
}