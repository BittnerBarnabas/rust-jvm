use crate::share::parser::descriptors::{FieldDescriptorParser, MethodDescriptorParser};
use crate::share::parser::parser::Parser;

#[test]
fn test_base_type_parsing() {
    let parser = FieldDescriptorParser::new();
    assert_eq!(
        format!("{:?}", parser.parse("Z").unwrap()),
        "FieldDescriptor(BaseType(Boolean))"
    );
    assert_eq!(
        format!("{:?}", parser.parse("B").unwrap()),
        "FieldDescriptor(BaseType(Byte))"
    );
    assert_eq!(
        format!("{:?}", parser.parse("S").unwrap()),
        "FieldDescriptor(BaseType(Short))"
    );
    assert_eq!(
        format!("{:?}", parser.parse("C").unwrap()),
        "FieldDescriptor(BaseType(Char))"
    );
    assert_eq!(
        format!("{:?}", parser.parse("I").unwrap()),
        "FieldDescriptor(BaseType(Int))"
    );
    assert_eq!(
        format!("{:?}", parser.parse("J").unwrap()),
        "FieldDescriptor(BaseType(Long))"
    );
    assert_eq!(
        format!("{:?}", parser.parse("F").unwrap()),
        "FieldDescriptor(BaseType(Float))"
    );
    assert_eq!(
        format!("{:?}", parser.parse("D").unwrap()),
        "FieldDescriptor(BaseType(Double))"
    );
}

#[test]
fn test_object_type_parsing() {
    let parser = FieldDescriptorParser::new();
    assert_eq!(
        format!("{:?}", parser.parse("Ljava/lang/Object;").unwrap()),
        r#"FieldDescriptor(ObjectType("java/lang/Object"))"#,
    );
    assert_eq!(
        format!("{:?}", parser.parse("Ljava/lang/Object   ;").unwrap()),
        r#"FieldDescriptor(ObjectType("java/lang/Object"))"#,
    );
    assert_eq!(
        format!("{:?}", parser.parse("Ljava/lang/Object$ABC123;").unwrap()),
        r#"FieldDescriptor(ObjectType("java/lang/Object$ABC123"))"#,
    );
    assert_eq!(
        format!("{:?}", parser.parse("LLLObject;").unwrap()),
        r#"FieldDescriptor(ObjectType("LLObject"))"#,
    );
    assert_eq!(
        format!("{:?}", parser.parse("LL;").unwrap()),
        r#"FieldDescriptor(ObjectType("L"))"#,
    );

    assert!(parser.parse("L java/lang/Object;").is_err());
    assert!(parser.parse("Ljava/lang/Object").is_err());
    assert!(parser.parse("java/lang/Object").is_err());
    assert!(parser.parse("L;").is_err());
}

#[test]
fn test_array_type_parsing() {
    let parser = FieldDescriptorParser::new();
    assert_eq!(
        format!("{:?}", parser.parse("[Ljava/lang/Object;").unwrap()),
        r#"FieldDescriptor(ArrayType(ComponentType(ObjectType("java/lang/Object"))))"#
    );
    assert_eq!(
        format!("{:?}", parser.parse("[Z").unwrap()),
        r#"FieldDescriptor(ArrayType(ComponentType(BaseType(Boolean))))"#
    );
    assert_eq!(
        format!("{:?}", parser.parse("[[Z").unwrap()),
        r#"FieldDescriptor(ArrayType(ComponentType(ArrayType(ComponentType(BaseType(Boolean))))))"#
    );
}

#[test]
fn test_method_descriptor_parsing() {
    let parser = MethodDescriptorParser::new();
    assert_eq!(format!("{:?}", parser.parse("()V").unwrap()), r#"([])Void"#);
    assert_eq!(
        format!("{:?}", parser.parse("(Z)I").unwrap()),
        r#"([ParameterDescriptor(BaseType(Boolean))])Type(BaseType(Int))"#
    );
    assert_eq!(
        format!("{:?}", parser.parse("(ZI)V").unwrap()),
        r#"([ParameterDescriptor(BaseType(Boolean)), ParameterDescriptor(BaseType(Int))])Void"#
    );
    assert_eq!(
        format!("{:?}", parser.parse("([Ljava/lang/String;)V").unwrap()),
        r#"([ParameterDescriptor(ArrayType(ComponentType(ObjectType("java/lang/String"))))])Void"#
    );
}
