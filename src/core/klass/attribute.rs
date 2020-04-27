pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

pub enum AttributeInfo {
    ConstantValue {
        constant_value_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionHandler>,
        attributes: Vec<AttributeInfo>,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumber>,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    Signature {
        signature_index: u16,
    },
    Custom {
        attribute_name_index: u16,
        attribute_length: u32,
        info: Vec<u8>,
    },
}
