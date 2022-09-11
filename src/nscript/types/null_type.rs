use std::fmt::Display;

use inkwell::{values::PointerValue, types::{BasicTypeEnum, PointerType}};

use crate::nscript::{values::Null, Environment};

use super::type_::Type;

#[derive(Clone, Copy)]
pub struct NullType;

impl<'ctx> Type<'ctx> for NullType {
  type LLVMType = PointerType<'ctx>;
  type LLVMValue = PointerValue<'ctx>;
  type Value = Null;

  fn llvm_type(&self, env: &Environment<'ctx>) -> Self::LLVMType {
    Null::llvm_null_type(env)
  }

  fn llvm_basic_type(&self, _env: &Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    None
  }

  fn create_value(env: &Environment, value: Self::LLVMValue) -> Self::Value {
    Null
  }
}

impl Display for NullType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "null")
  }
}