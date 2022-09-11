use std::fmt::Display;

use inkwell::{values::{AnyValueEnum, IntValue}, types::{AnyTypeEnum, BasicTypeEnum, IntType}};

use crate::nscript::{values::{Boolean, AnyValue}, Environment};

use super::type_::Type;

#[derive(Clone, Copy)]
pub struct BooleanType;

impl<'ctx> Type<'ctx> for BooleanType {
  type LLVMType = IntType<'ctx>;
  type LLVMValue = IntValue<'ctx>;
  type Value = Boolean<'ctx>;

  fn llvm_type(&self, env: &Environment<'ctx>) -> Self::LLVMType {
    env.borrow().context.i8_type()
  }

  fn llvm_basic_type(&self, env: &Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    Some(env.borrow().context.i8_type().into())
  }

  fn create_value(env: &Environment, value: Self::LLVMValue) -> Self::Value {
    Boolean { value }
  }
}

impl Display for BooleanType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Boolean")
  }
}