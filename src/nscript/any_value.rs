use combine::{Parser, Stream};
use inkwell::{values::{IntValue, FloatValue, FunctionValue, PointerValue, BasicValueEnum}, types::BasicTypeEnum};

use super::type_::Type;

#[derive(Debug, Clone, Copy)]
pub enum AnyType {
  Integer,
  Number,
  String,
  Boolean,
  Null,
}

#[derive(Debug, Clone)]
pub enum AnyValue<'ctx> {
  Integer(IntValue<'ctx>),
  Number(FloatValue<'ctx>),
  Boolean(IntValue<'ctx>),
  Null,
  Fn { fn_: FunctionValue<'ctx>, name: String, args: Vec<(String, Type)> },
  Ptr { ptr: PointerValue<'ctx>, type_: AnyType },
}

impl<'ctx, Input> Parser<Input> for AnyValue<'ctx> 
where
  Input: Stream {
  
  type Output = AnyValue<'ctx>;
  type PartialState = ();
}

impl<'ctx> AnyValue<'ctx> {
  pub fn is_integer(&self) -> bool {
    if let AnyValue::Integer(_) = self {true} else {false}
  }

  pub fn is_number(&self) -> bool {
    if let AnyValue::Number(_) = self {true} else {false}
  }

  pub fn is_boolean(&self) -> bool {
    if let AnyValue::Boolean(_) = self {true} else {false}
  }

  pub fn is_null(&self) -> bool {
    if let AnyValue::Null = self {true} else {false}
  }

  pub fn is_function(&self) -> bool {
    if let AnyValue::Fn {..} = self {true} else {false}
  }

  pub fn into_integer(self) -> IntValue<'ctx> {
    match self {
      AnyValue::Integer(value) => value,
      _ => panic!("Invalid type")
    }
  }

  pub fn into_number(self) -> FloatValue<'ctx> {
    match self {
      AnyValue::Number(value) => value,
      _ => panic!("Invalid type")
    }
  }

  pub fn into_boolean(self) -> IntValue<'ctx> {
    match self {
      AnyValue::Boolean(value) => value,
      _ => panic!("Invalid type")
    }
  }

  pub fn into_function(self) -> FunctionValue<'ctx> {
    match self {
      AnyValue::Fn{fn_, ..} => fn_,
      _ => panic!("Invalid type")
    }
  }
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for AnyValue<'ctx> {
  fn into(self) -> BasicValueEnum<'ctx> {
    match self {
      AnyValue::Integer(value) => BasicValueEnum::IntValue(value),
      AnyValue::Number(value) => BasicValueEnum::FloatValue(value),
      AnyValue::Boolean(value) => BasicValueEnum::IntValue(value),
      AnyValue::Ptr {ptr, ..} => BasicValueEnum::PointerValue(ptr),
      _ => panic!("Invalid type")
    }
  }
}

impl<'ctx> From<BasicValueEnum<'ctx>> for AnyValue<'ctx> {
  fn from(value: BasicValueEnum<'ctx>) -> AnyValue<'ctx> {
    match value {
      BasicValueEnum::IntValue(value) if value.get_type().get_bit_width() == 32 => AnyValue::Integer(value),
      BasicValueEnum::IntValue(value) if value.get_type().get_bit_width() == 1 => AnyValue::Boolean(value),
      BasicValueEnum::FloatValue(value) => AnyValue::Number(value),
      _ => panic!("Invalid type")
    }
  }
}