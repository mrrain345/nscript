use std::fmt::Display;

use inkwell::values::AnyValueEnum;

use crate::nscript::{values::AnyValue, Environment};

use super::{object_type::ObjectType, integer_type::IntegerType, Type, number_type::NumberType, boolean_type::BooleanType, null_type::NullType};

#[derive(Debug, Clone, PartialEq)]
pub enum AnyType<'ctx> {
  Integer,
  Number,
  // String,
  Boolean,
  Null,
  Object(ObjectType<'ctx>),
  // Function,
  // Class(ClassType<'ctx>),
}

impl<'ctx> AnyType<'ctx> {
  /// Convert LLVM value to AnyValue
  pub fn create_value(&self, env: &Environment, value: AnyValueEnum<'ctx>) -> AnyValue<'ctx> {
    match self {
      AnyType::Integer => IntegerType::create_value(env, value.into_int_value()).into(),
      AnyType::Number => NumberType::create_value(env, value.into_float_value()).into(),
      // AnyType::String => StringType::create_value(env, value),
      AnyType::Boolean => BooleanType::create_value(env, value.into_int_value()).into(),
      AnyType::Null => NullType::create_value(env, value.into_pointer_value()).into(),
      AnyType::Object(_) => ObjectType::create_value(env, value.into_pointer_value()).into(),
      // AnyType::Function => FunctionType::create_value(env, value),
    }
  }

  /// Get LLVM type
  pub fn llvm_type(&self, env: &Environment<'ctx>) -> inkwell::types::AnyTypeEnum<'ctx> {
    match self {
      AnyType::Integer => IntegerType.llvm_type(env).into(),
      AnyType::Number => NumberType.llvm_type(env).into(),
      AnyType::Boolean => BooleanType.llvm_type(env).into(),
      AnyType::Null => NullType.llvm_type(env).into(),
      AnyType::Object(object) => ObjectType::llvm_type(object, env).into(),
    }
  }

  /// Get LLVM basic type
  pub fn llvm_basic_type(&self, env: &Environment<'ctx>) -> Option<inkwell::types::BasicTypeEnum<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.llvm_basic_type(env),
      AnyType::Number => NumberType.llvm_basic_type(env),
      AnyType::Boolean => BooleanType.llvm_basic_type(env),
      AnyType::Null => NullType.llvm_basic_type(env),
      AnyType::Object(object) => ObjectType::llvm_basic_type(object, env),
    }
  }
}

impl<'ctx> Display for AnyType<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AnyType::Integer => IntegerType.fmt(f),
      AnyType::Number => NumberType.fmt(f),
      // AnyType::String => StringType.fmt(f),
      AnyType::Boolean => BooleanType.fmt(f),
      AnyType::Null => NullType.fmt(f),
      AnyType::Object(object) => object.fmt(f),
      // AnyType::Function => FunctionType.fmt(f),
    }
  }
}