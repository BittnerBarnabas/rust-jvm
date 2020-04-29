pub mod core {
    pub mod native {
        pub mod native_methods;
    }
    pub mod klass {
        pub mod access_flags;
        pub mod attribute;
        pub mod constant_pool;
        pub mod field;
        pub mod klass;
        pub mod method;
    }

    pub mod class_loader;
    pub mod class_parser;
    pub mod interpreter;
    pub mod jvm_exception;
    pub mod jvm_value;
    pub mod opcode;
    pub mod stack_frame;
}
