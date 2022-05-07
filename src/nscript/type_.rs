use inkwell::types::{AnyTypeEnum, BasicMetadataTypeEnum};

use super::{environment::Environment, any_value::AnyType};

#[derive(Debug, Clone, PartialEq)]
pub struct Type(pub String);

impl Type {
  pub fn is_null(&self) -> bool {
    self.0 == "null"
  }

  pub fn is_number(&self) -> bool {
    self.0 == "Number"
  }

  pub fn is_string(&self) -> bool {
    self.0 == "String"
  }

  pub fn is_integer(&self) -> bool {
    self.0 == "Integer"
  }

  pub fn is_boolean(&self) -> bool {
    self.0 == "Boolean"
  }
  
  pub fn into_llvm_type<'ctx>(&self, env: &mut Environment<'ctx>) -> Option<AnyTypeEnum<'ctx>> {
    match self.0.as_str() {
      "null" => Some(AnyTypeEnum::VoidType(env.context.void_type().into())),
      "Integer" => Some(AnyTypeEnum::IntType(env.context.i32_type().into())),
      "Number" => Some(AnyTypeEnum::FloatType(env.context.f64_type().into())),
      // "String" => Some(AnyTypeEnum::PointerType(env.context.i8_type().ptr_type(env.context.i8_type().into()).into())),
      "Boolean" => Some(AnyTypeEnum::IntType(env.context.bool_type().into())),
      _ => None,
    }
  }

  pub fn into_llvm_basic_type<'ctx>(&self, env: &mut Environment<'ctx>) -> Option<BasicMetadataTypeEnum<'ctx>> {
    match self.0.as_str() {
      "null" => None,
      "Integer" => Some(BasicMetadataTypeEnum::IntType(env.context.i32_type().into())),
      "Number" => Some(BasicMetadataTypeEnum::FloatType(env.context.f64_type().into())),
      // "String" => Some(BasicMetadataTypeEnum::PointerType(env.context.i8_type().ptr_type(env.context.i8_type().into()).into())),
      "Boolean" => Some(BasicMetadataTypeEnum::IntType(env.context.bool_type().into())),
      _ => None,
    }
  }

  pub fn to_type(&self) -> AnyType {
    match self.0.as_str() {
      "null" => AnyType::Null,
      "Integer" => AnyType::Integer,
      "Number" => AnyType::Number,
      "String" => AnyType::String,
      "Boolean" => AnyType::Boolean,
      _ => panic!("Unknown type: {}", self.0),
    }
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ParamsList(pub Vec<(String, Type)>);