use combine::{Parser, Stream};
use inkwell::values::{IntValue, FloatValue, FunctionValue, PointerValue};

#[derive(Debug, Clone, Copy)]
pub enum AnyType {
  Integer,
  Number,
  String,
  Boolean,
  Null,
}

#[derive(Debug, Clone, Copy)]
pub enum AnyValue<'ctx> {
  Integer(IntValue<'ctx>),
  Number(FloatValue<'ctx>),
  Boolean(IntValue<'ctx>),
  Null,
  Fn(FunctionValue<'ctx>),
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
    if let AnyValue::Fn(_) = self {true} else {false}
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
      AnyValue::Fn(value) => value,
      _ => panic!("Invalid type")
    }
  }
}