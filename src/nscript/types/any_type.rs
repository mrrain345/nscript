use std::fmt::Display;

use inkwell::values::AnyValueEnum;

use crate::nscript::{values::AnyValue, Environment};

use super::{object_type::ObjectType, integer_type::IntegerType, Type, number_type::NumberType, boolean_type::BooleanType, null_type::NullType, RefType};

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
  Ref(RefType<'ctx>),
}

impl<'ctx> AnyType<'ctx> {
  pub fn from_string(env: &Environment<'ctx>, string: &str) -> Option<AnyType<'ctx>> {
    match string {
      "Integer" => Some(AnyType::Integer),
      "Number" => Some(AnyType::Number),
      "Boolean" => Some(AnyType::Boolean),
      "null" => Some(AnyType::Null),
      _ => None,
    }
  }

  /// Convert LLVM value to AnyValue
  pub fn create_value(&self, env: &Environment<'ctx>, value: AnyValueEnum<'ctx>) -> AnyValue<'ctx> {
    match self {
      AnyType::Integer => IntegerType.create_value(env, value.into_int_value()).into(),
      AnyType::Number => NumberType.create_value(env, value.into_float_value()).into(),
      // AnyType::String => StringType.create_value(env, value),
      AnyType::Boolean => BooleanType.create_value(env, value.into_int_value()).into(),
      AnyType::Null => NullType.create_value(env, value.into_pointer_value()).into(),
      AnyType::Object(object) => object.create_value(env, value.into_pointer_value()).into(),
      // AnyType::Function => FunctionType::create_value(env, value),
      AnyType::Ref(ref_) => ref_.create_value(env, value.into_pointer_value()).into(),
    }
  }

  pub fn common_type(env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyType<'ctx>> {
    if left.get_type() == right.get_type() { return Some(left.get_type()); }

    let cast = right.silent_cast(env, &left.get_type());
    if cast.is_some() { return Some(cast.unwrap().get_type()); }
    let cast = left.silent_cast(env, &right.get_type());
    if cast.is_some() { return Some(cast.unwrap().get_type()); }

    None
  }


  /// Get LLVM type
  pub fn llvm_type(&self, env: &Environment<'ctx>) -> inkwell::types::AnyTypeEnum<'ctx> {
    match self {
      AnyType::Integer => IntegerType.llvm_type(env).into(),
      AnyType::Number => NumberType.llvm_type(env).into(),
      AnyType::Boolean => BooleanType.llvm_type(env).into(),
      AnyType::Null => NullType.llvm_type(env).into(),
      AnyType::Object(object) => object.llvm_type(env).into(),
      AnyType::Ref(ref_) => ref_.llvm_type(env).into(),
    }
  }

  /// Get LLVM basic type
  pub fn llvm_basic_type(&self, env: &Environment<'ctx>) -> Option<inkwell::types::BasicTypeEnum<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.llvm_basic_type(env),
      AnyType::Number => NumberType.llvm_basic_type(env),
      AnyType::Boolean => BooleanType.llvm_basic_type(env),
      AnyType::Null => NullType.llvm_basic_type(env),
      AnyType::Object(object) => object.llvm_basic_type(env),
      AnyType::Ref(ref_) => ref_.llvm_basic_type(env),
    }
  }

  
  pub fn is_integer(&self) -> bool {
    matches!(self, AnyType::Integer)
  }

  pub fn is_number(&self) -> bool {
    matches!(self, AnyType::Number)
  }

  pub fn is_boolean(&self) -> bool {
    matches!(self, AnyType::Boolean)
  }

  pub fn is_null(&self) -> bool {
    matches!(self, AnyType::Null)
  }

  pub fn is_object(&self) -> bool {
    matches!(self, AnyType::Object(_))
  }

  pub fn is_ref(&self) -> bool {
    matches!(self, AnyType::Ref(_))
  }


  pub fn into_integer(self) -> Option<IntegerType> {
    if let AnyType::Integer = self { Some(IntegerType) } else { None }
  }

  pub fn into_number(self) -> Option<NumberType> {
    if let AnyType::Number = self { Some(NumberType) } else { None }
  }

  pub fn into_boolean(self) -> Option<BooleanType> {
    if let AnyType::Boolean = self { Some(BooleanType) } else { None }
  }

  pub fn into_null(self) -> Option<NullType> {
    if let AnyType::Null = self { Some(NullType) } else { None }
  }

  pub fn into_object(self) -> Option<ObjectType<'ctx>> {
    if let AnyType::Object(object) = self { Some(object) } else { None }
  }

  pub fn into_ref(self) -> Option<RefType<'ctx>> {
    if let AnyType::Ref(ref_) = self { Some(ref_) } else { None }
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
      AnyType::Ref(ref_) => ref_.fmt(f),
    }
  }
}