use inkwell::types::{AnyTypeEnum, BasicTypeEnum};

use super::{Environment, AnyValue, Class};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum AnyType<'ctx> {
  Integer,
  Number,
  String,
  Boolean,
  Null,
  Object(&'ctx Class<'ctx>),
  Function,
  Class(&'ctx Class<'ctx>),
}

impl<'ctx> AnyType<'ctx> {
  pub fn is_null(&self) -> bool {
    if let AnyType::Null = *self { true } else { false }
  }

  pub fn is_boolean(&self) -> bool {
    if let AnyType::Boolean = *self { true } else { false }
  }

  pub fn is_number(&self) -> bool {
    if let AnyType::Number = *self { true } else { false }
  }

  pub fn is_integer(&self) -> bool {
    if let AnyType::Integer = *self { true } else { false }
  }

  pub fn is_string(&self) -> bool {
    if let AnyType::String = *self { true } else { false }
  }

  pub fn is_object(&self) -> bool {
    if let AnyType::Object(..) = *self { true } else { false }
  }

  pub fn is_function(&self) -> bool {
    if let AnyType::Function = *self { true } else { false }
  }

  pub fn is_class(&self) -> bool {
    if let AnyType::Class(..) = *self { true } else { false }
  }

  pub fn into_class(self) -> &'ctx Class<'ctx> {
    match self {
      AnyType::Class(class) => class,
      _ => panic!("Invalid type"),
    }
  }

  pub fn into_object(self) -> &'ctx Class<'ctx> {
    match self {
      AnyType::Object(class) => class,
      _ => panic!("Invalid type"),
    }
  }

  pub fn into_llvm_type(self, env: &mut Environment<'ctx>) -> AnyTypeEnum<'ctx> {
    match self {
      AnyType::Integer => env.context.i32_type().into(),
      AnyType::Number => env.context.f64_type().into(),
      AnyType::String => todo!(),
      AnyType::Boolean => env.context.i8_type().into(),
      AnyType::Null => env.context.void_type().into(),
      AnyType::Class(class) => class.struct_type().into(),
      AnyType::Object(class) => class.struct_type().into(),
      _ => panic!("Unsupported type"),
    }
  }

  pub fn into_llvm_basic_type(self, env: &mut Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    match self {
      AnyType::Integer => Some(env.context.i32_type().into()),
      AnyType::Number => Some(env.context.f64_type().into()),
      AnyType::String => todo!(),
      AnyType::Boolean => Some(env.context.i8_type().into()),
      AnyType::Object(class) => Some(class.struct_type().into()),
      AnyType::Class(class) => Some(class.struct_type().into()),
      _ => None,
    }
  }

  pub fn is_compatible(&self, other: &AnyType) -> bool {
    match (self, other) {
      (AnyType::Number, AnyType::Integer) => true,

      (AnyType::Integer, AnyType::Integer) => true,
      (AnyType::Number, AnyType::Number) => true,
      (AnyType::String, AnyType::String) => true,
      (AnyType::Boolean, AnyType::Boolean) => true,
      (AnyType::Null, AnyType::Null) => true,
      (AnyType::Object(class1), AnyType::Object(class2)) => class1 == class2,
      (AnyType::Class(class1), AnyType::Class(class2)) => class1 == class2,
      _ => false,
    }
  }

  pub fn default(&self, env: &mut Environment<'ctx>) -> AnyValue<'ctx> {
    match self {
      AnyType::Integer => AnyValue::Integer(env.integer(0)),
      AnyType::Number => AnyValue::Number(env.number(0.0)),
      // AnyType::String => AnyValue::String(env.string("")),
      AnyType::Boolean => AnyValue::Boolean(env.boolean(false)),
      AnyType::Null => AnyValue::Null,
      _ => panic!("default called on non-primitive type"),
    }
  }

  pub fn into_value(self, env: &mut Environment<'ctx>, value: AnyValue<'ctx>) -> AnyValue<'ctx> {
    match (self, value) {
      (AnyType::Integer, AnyValue::Integer(value)) => AnyValue::Integer(value),
      (AnyType::Number, AnyValue::Number(value)) => AnyValue::Number(value),
      // (AnyType::String, AnyValue::String(value)) => AnyValue::String(value),
      (AnyType::Boolean, AnyValue::Boolean(value)) => AnyValue::Boolean(value),
      (AnyType::Null, AnyValue::Null) => AnyValue::Null,
      // (AnyType::Object(class), AnyValue::Object(value)) => {
      //   if class.is_instance(value) {
      //     AnyValue::Object(value)
      //   } else {
      //     panic!("Invalid object type")
      //   }
      // }
      // (AnyType::Class(class), AnyValue::Object(value)) => {
      //   if class.is_instance(value) {
      //     AnyValue::Object(value)
      //   } else {
      //     panic!("Invalid object type")
      //   }
      // }
      _ => panic!("Invalid type"),
    }
  }
}