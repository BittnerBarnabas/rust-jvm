mod core {
    pub mod class_loader;
    pub mod class_parser;
    pub mod constant_pool;
    pub mod interpreter;
    pub mod jvm;
    pub mod opcode;
}

mod tests;

use crate::core::interpreter;
use crate::core::jvm::StackFrame;
use crate::core::opcode;

fn main() {}
