use inkwell::{types::{AnyTypeEnum, BasicMetadataTypeEnum, BasicTypeEnum}};

use super::{Environment, AnyType, Class};

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

  pub fn is_class(&self, env: &mut Environment) -> bool {
    env.get_class(&self.0).is_some()
  }

  pub fn into_class<'ctx>(&self, env: &mut Environment<'ctx>) -> Option<&'ctx Class<'ctx>> {
    env.get_class(&self.0).map(|c| c.into_class())
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

  pub fn into_llvm_basic_type<'ctx>(&self, env: &mut Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    match self.0.as_str() {
      "null" => None,
      "Integer" => Some(BasicTypeEnum::IntType(env.context.i32_type().into())),
      "Number" => Some(BasicTypeEnum::FloatType(env.context.f64_type().into())),
      // "String" => Some(BasicTypeEnum::PointerType(env.context.i8_type().ptr_type(env.context.i8_type().into()).into())),
      "Boolean" => Some(BasicTypeEnum::IntType(env.context.bool_type().into())),
      _ => None,
    }
  }

  pub fn into_llvm_basic_metadata_type<'ctx>(&self, env: &mut Environment<'ctx>) -> Option<BasicMetadataTypeEnum<'ctx>> {
    match self.0.as_str() {
      "null" => None,
      "Integer" => Some(BasicMetadataTypeEnum::IntType(env.context.i32_type().into())),
      "Number" => Some(BasicMetadataTypeEnum::FloatType(env.context.f64_type().into())),
      // "String" => Some(BasicMetadataTypeEnum::PointerType(env.context.i8_type().ptr_type(env.context.i8_type().into()).into())),
      "Boolean" => Some(BasicMetadataTypeEnum::IntType(env.context.bool_type().into())),
      _ => None,
    }
  }

  pub fn into_type<'ctx>(&self, env: &mut Environment<'ctx>) -> Option<AnyType<'ctx>> {
    let type_ = match self.0.as_str() {
      "null" => Some(AnyType::Null),
      "Integer" => Some(AnyType::Integer),
      "Number" => Some(AnyType::Number),
      "String" => Some(AnyType::String),
      "Boolean" => Some(AnyType::Boolean),
      _ => None,
    };

    if type_.is_some() { return type_; }

    if let Some(class) = self.into_class(env) {
      Some(AnyType::Object(class))
    } else {
      None
    }
  }
}