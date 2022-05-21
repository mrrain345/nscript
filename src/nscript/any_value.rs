use combine::{Parser, Stream};
use inkwell::{values::{IntValue, FloatValue, FunctionValue, PointerValue, BasicValueEnum, StructValue}, types::{BasicTypeEnum, StructType}};

use super::{type_::Type, Property, PropertyValue, Environment, any_type::AnyType, Class, Object};

#[derive(Debug, Clone, PartialEq)]
pub enum AnyValue<'ctx> {
  Integer(IntValue<'ctx>),
  Number(FloatValue<'ctx>),
  Boolean(IntValue<'ctx>),
  Null,
  Fn { fn_: FunctionValue<'ctx>, name: String, args: Vec<(String, Type)> },
  Ptr { ptr: PointerValue<'ctx>, type_: AnyType<'ctx> },
  Class(&'ctx Class<'ctx>),
  Object(Box<Object<'ctx>>),
}

impl<'ctx, Input> Parser<Input> for AnyValue<'ctx> 
where
  Input: Stream {
  
  type Output = AnyValue<'ctx>;
  type PartialState = ();
}

impl<'ctx> AnyValue<'ctx> {
  pub fn allocate(&self, env: &mut Environment<'ctx>) -> PointerValue<'ctx> {
    match self {
      AnyValue::Integer(value) => env.builder.build_alloca(value.get_type(), "Integer"),
      AnyValue::Number(value) => env.builder.build_alloca(value.get_type(), "Number"),
      AnyValue::Boolean(value) => env.builder.build_alloca(value.get_type(), "Boolean"),
      AnyValue::Null => env.builder.build_alloca(env.context.bool_type(), "null"),
      AnyValue::Ptr { ptr, .. } => env.builder.build_alloca(ptr.get_type(), "ptr"),
      AnyValue::Object(object) => env.builder.build_alloca(object.class().struct_type(), "Object"),
      _ => panic!("Invalid type")
    }
  }

  pub fn get_type(&self) -> AnyType {
    match self {
      AnyValue::Integer(_) => AnyType::Integer,
      AnyValue::Number(_) => AnyType::Number,
      AnyValue::Boolean(_) => AnyType::Boolean,
      AnyValue::Null => AnyType::Null,
      AnyValue::Fn { .. } => AnyType::Function,
      AnyValue::Ptr { type_, .. } => type_.clone(),
      AnyValue::Class(class) => AnyType::Class(class),
      AnyValue::Object(object) => AnyType::Object(object.class()),
    }
  }

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

  pub fn is_object(&self) -> bool {
    if let AnyValue::Object { .. } = self {true} else {false}
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

  pub fn into_class(self) -> &'ctx Class<'ctx> {
    match self {
      AnyValue::Class(class) => class,
      _ => panic!("Invalid type")
    }
  }

  pub fn into_object(self) -> Box<Object<'ctx>> {
    match self {
      AnyValue::Object(object) => object,
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