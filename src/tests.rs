#[cfg(test)]
mod tests {
    use crate::core::jvm::JvmValue;
    use crate::core::opcode;
    use crate::core::{class_parser, interpreter};

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

        let interpreter_result = interpreter::interpret(&opcodes);
        assert_eq!(interpreter_result, Some(JvmValue::Int { val: 20 }));
    }

    #[test]
    pub fn test() {
        let class_file_in_bytes =
            std::fs::read("/home/barnab/tmp/Main2.class").expect("File Not Found");

        let result =
            class_parser::parse_class(class_file_in_bytes).expect("Class file couldn't be parsed!");

        println!("ABC");
    }
}
