use std::ops::Deref;

use combine::{Parser, Stream};
use inkwell::values::{IntValue, FloatValue, FunctionValue, PointerValue, BasicValueEnum, BasicMetadataValueEnum};

use super::{Environment, AnyType, Class, Object, Function};

#[derive(Debug, Clone, PartialEq)]
pub enum AnyValue<'ctx> {
  Integer(IntValue<'ctx>),
  Number(FloatValue<'ctx>),
  Boolean(IntValue<'ctx>),
  Null,
  Fn(Box<Function<'ctx>>),
  Ptr { ptr: PointerValue<'ctx>, type_: AnyType<'ctx> },
  Object(Box<Object<'ctx>>),
  Class(&'ctx Class<'ctx>),
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

  pub fn deref(self, env: &mut Environment<'ctx>) -> AnyValue<'ctx> {
    if self.is_ptr() {
      let (ptr, type_) = self.into_ptr();
      let value = env.builder.build_load(ptr, "deref");
      AnyValue::from_basic_value(&type_, value)
    } else {
      self
    }
  }

  pub fn silent_cast(&self, env: &mut Environment<'ctx>, to: &AnyType) -> Option<AnyValue<'ctx>> {
    match (self, *to) {
      (AnyValue::Integer(value), AnyType::Integer) => Some(AnyValue::Integer(*value)),
      (AnyValue::Integer(value), AnyType::Number) => Some(AnyValue::Number(env.builder.build_signed_int_to_float(*value, env.context.f64_type(), "cast"))),
      (AnyValue::Number(value), AnyType::Number) => Some(AnyValue::Number(*value)),
      (AnyValue::Boolean(value), AnyType::Boolean) => Some(AnyValue::Boolean(*value)),
      (AnyValue::Null, AnyType::Null) => Some(AnyValue::Null),
      (AnyValue::Class(class), AnyType::Class(to_class)) => {
        if *class == to_class {
          Some(AnyValue::Class(class))
        } else {
          None
        }
      },
      (AnyValue::Object(object), AnyType::Object(class)) => {
        if object.class() == class {
          Some(AnyValue::Object(object.clone()))
        } else {
          None
        }
      },
      _ => None
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

  pub fn is_class(&self) -> bool {
    if let AnyValue::Class(_) = self {true} else {false}
  }

  pub fn is_ptr(&self) -> bool {
    if let AnyValue::Ptr {..} = self {true} else {false}
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

  pub fn into_function(self) -> Box<Function<'ctx>> {
    match self {
      AnyValue::Fn(function) => function,
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

  pub fn into_ptr(self) -> (PointerValue<'ctx>, AnyType<'ctx>) {
    match self {
      AnyValue::Ptr { ptr, type_ } => (ptr, type_),
      _ => panic!("Invalid type")
    }
  }

  pub fn from_basic_value(type_: &AnyType<'ctx>, value: BasicValueEnum<'ctx>) -> AnyValue<'ctx> {
    match type_ {
      AnyType::Integer => AnyValue::Integer(value.into_int_value()),
      AnyType::Number => AnyValue::Number(value.into_float_value()),
      AnyType::Boolean => AnyValue::Boolean(value.into_int_value()),
      AnyType::Null => AnyValue::Null,
      _ => panic!("Invalid type `{type_:?}`\nvalue: {value:?}")
    }
  }

  pub fn into_llvm_basic_value(&self) -> BasicValueEnum<'ctx> {
    match self {
      AnyValue::Integer(value) => BasicValueEnum::IntValue(*value),
      AnyValue::Number(value) => BasicValueEnum::FloatValue(*value),
      AnyValue::Boolean(value) => BasicValueEnum::IntValue(*value),
      AnyValue::Ptr {ptr, ..} => BasicValueEnum::PointerValue(*ptr),
      _ => panic!("Invalid type")
    }
  }
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for AnyValue<'ctx> {
  fn into(self) -> BasicValueEnum<'ctx> {
    self.into_llvm_basic_value()
  }
}

impl<'ctx> Into<BasicMetadataValueEnum<'ctx>> for AnyValue<'ctx> {
  fn into(self) -> BasicMetadataValueEnum<'ctx> {
    self.into_llvm_basic_value().into()
  }
}