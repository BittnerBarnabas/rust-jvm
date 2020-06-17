#[derive(Clone)]
pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Clone)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Clone)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Clone)]
pub enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame {
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemFrameExtended {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    ChopFrame {
        offset_delta: u16,
    },
    SameFrameExtended {
        offset_delta: u16,
    },
    AppendFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Clone)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    UninitializedVariable { offset: u16 },
    Long,
    Double,
}

#[derive(Clone)]
pub struct NAME {}

#[derive(Clone)]
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
    LocalVariableTable {
        local_variable_table: Vec<LocalVariable>,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    StackMapTable {
        entries: Vec<StackMapFrame>,
    },
    Exceptions {},
    InnerClasses {},
    EnclosingMethod {},
    Synthetic {},
    SourceDebugExtension {},
    LocalVariableTypeTable {},
    Deprecated {},
    RuntimeVisibleAnnotations {},
    RuntimeInvisibleAnnotations {},
    RuntimeVisibleParameterAnnotations {},
    RuntimeInvisibleParameterAnnotations {},
    RuntimeVisibleTypeAnnotations {},
    RuntimeInvisibleTypeAnnotations {},
    AnnotationDefault {},
    BootstrapMethods {},
    MethodParameters {},
    Signature {
        signature_index: u16,
    },
    Custom {
        attribute_name_index: u16,
        attribute_length: u32,
        info: Vec<u8>,
    },
}
