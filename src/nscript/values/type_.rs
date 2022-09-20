use inkwell::{values::PointerValue, types::{AnyTypeEnum, BasicTypeEnum}};

use crate::nscript::{Environment, AnyType, types::{NullType}};

use super::{value::Value, AnyValue, Null};

#[derive(Debug, Clone)]
pub struct Type<'ctx> {
  pub type_: AnyType<'ctx>,
}

impl<'ctx> Type<'ctx> {
  pub fn new(type_: AnyType<'ctx>, ptr: PointerValue<'ctx>) -> Self {
    Self { type_ }
  }
}

impl<'ctx> Value<'ctx> for Type<'ctx> {
  type Type = NullType; // TODO
  type LLVMValue = PointerValue<'ctx>;

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    Null.llvm_value(env)
  }

  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {
    
  }

  fn get_type(&self) -> AnyType<'ctx> {
    todo!()
  }

  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue {
    Null.llvm_value(env)
  }

  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<inkwell::values::BasicValueEnum<'ctx>> {
    None
  }
}

impl<'ctx> Into<AnyValue<'ctx>> for Type<'ctx> {
  fn into(self) -> AnyValue<'ctx> {
    AnyValue::Type(self)
  }
}

impl<'ctx> From<AnyValue<'ctx>> for Type<'ctx> {
  fn from(value: AnyValue<'ctx>) -> Self {
    match value {
      AnyValue::Type(value) => value,
      _ => panic!("Invalid type"),
    }
  }
}