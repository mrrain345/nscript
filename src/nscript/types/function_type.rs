use std::fmt::Display;

use inkwell::types::BasicTypeEnum;

use crate::nscript::{values::Function, Environment};

use super::type_::Type;

#[derive(Clone, Copy)]
pub struct FunctionType;

impl<'ctx> Type<'ctx> for FunctionType {
  type LLVMType = inkwell::types::FunctionType<'ctx>;
  type LLVMValue = inkwell::values::FunctionValue<'ctx>;
  type Value = Function<'ctx>;

  fn llvm_type(&self, env: &Environment<'ctx>) -> Self::LLVMType {
    env.borrow().context.void_type().fn_type(&[], false)
  }

  fn llvm_basic_type(&self, _env: &Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    None
  }

  fn create_value(env: &Environment, value: Self::LLVMValue) -> Self::Value {
    todo!()
  }
}

impl Display for FunctionType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Fn")
  }
}