mod core {
    pub mod class_parser;
    pub mod constant_pool;
    pub mod interpreter;
    pub mod jvm;
    pub mod opcode;
}
mod tests;

use crate::core::interpreter;
use crate::core::opcode;

fn main() {
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
        opcode::IRETURN,
    ];

    let interpreter_result = interpreter::interpret(&opcodes);
    println!("Result: {:?}", &interpreter_result);
}
