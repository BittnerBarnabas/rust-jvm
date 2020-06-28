use regex::Error;

use crate::share::parser::parser::Parser;
use crate::share::utilities::jvm_exception::JvmException;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[cfg(test)]
#[path = "./descriptors_tests.rs"]
mod descriptors_tests;

#[derive(Debug)]
pub enum BaseType {
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Char,
}

#[derive(Debug)]
pub enum FieldType {
    BaseType(BaseType),
    ObjectType(String),
    ArrayType(Box<ComponentType>),
}

#[derive(Debug)]
pub enum ComponentType {
    ComponentType(FieldType),
}

#[derive(Debug)]
pub enum FieldDescriptor {
    FieldDescriptor(FieldType),
}

pub struct MethodDescriptor {
    pub parameters: Vec<ParameterDescriptor>,
    pub return_descriptor: ReturnDescriptor,
}

impl fmt::Debug for MethodDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}){:?}", self.parameters, self.return_descriptor)
    }
}

#[derive(Debug)]
pub enum ParameterDescriptor {
    ParameterDescriptor(FieldType),
}

#[derive(Debug)]
pub enum ReturnDescriptor {
    Type(FieldType),
    Void,
}

lalrpop_util::lalrpop_mod!(pub descriptors, "/share/parser/grammar/descriptors.rs"); // synthesized by LALRPOP

pub struct FieldDescriptorParser {
    parser: descriptors::FieldDescriptorParser,
}

impl FieldDescriptorParser {
    pub fn new() -> FieldDescriptorParser {
        FieldDescriptorParser {
            parser: descriptors::FieldDescriptorParser::new(),
        }
    }
}

impl Parser<FieldDescriptor> for FieldDescriptorParser {
    fn parse(&self, input: &str) -> Result<FieldDescriptor, JvmException> {
        self.parser
            .parse(input)
            .map_err(|err| JvmException::from(format!("{:?}", err)))
    }
}

pub struct MethodDescriptorParser {
    parser: descriptors::MethodDescriptorParser,
}

impl MethodDescriptorParser {
    pub fn new() -> MethodDescriptorParser {
        MethodDescriptorParser {
            parser: descriptors::MethodDescriptorParser::new(),
        }
    }
}

impl Parser<MethodDescriptor> for MethodDescriptorParser {
    fn parse(&self, input: &str) -> Result<MethodDescriptor, JvmException> {
        self.parser
            .parse(input)
            .map_err(|err| JvmException::from(format!("{:?}", err)))
    }
}
