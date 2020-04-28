#[cfg(test)]
mod tests {
    use runtime::core::class_parser::ClassParser;
    use runtime::core::jvm_value::JvmValue;
    use runtime::core::stack_frame::StackFrame;
    use runtime::core::{interpreter, opcode};

    #[test]
    pub fn storing_and_adding_local_integers() {
        let opcodes: Vec<u8> = vec![
            opcode::BIPUSH,
            0x10,
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

        let frame = StackFrame::new();
        let interpreter_result = interpreter::interpret(&frame, &opcodes).ok();
        assert_eq!(interpreter_result, Some(JvmValue::Int { val: 20 }));
    }

    #[test]
    pub fn test() {
        let class_file_in_bytes =
            std::fs::read("../resources/tests/Main2.class").expect("File Not Found");

        let parser = ClassParser::from(class_file_in_bytes);
        let result = parser
            .parse_class()
            .expect("Class file couldn't be parsed!");

        let cl_init_code = result.methods[4].get_code().expect("Code is not found!");
        let frame = StackFrame::new();
        let result1 = interpreter::interpret(&frame, cl_init_code);

        println!("ABC");
    }
}
