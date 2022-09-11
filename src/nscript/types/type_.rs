use std::fmt::Display;

use inkwell::{types::BasicTypeEnum};

use crate::nscript::{Environment, values::Value};

pub trait Type<'ctx>: Display + Clone {
  type LLVMType : inkwell::types::AnyType<'ctx>;
  type LLVMValue : inkwell::values::AnyValue<'ctx>;
  type Value : Value<'ctx>;

  fn create_value(env: &Environment, value: Self::LLVMValue) -> Self::Value;

  fn llvm_type(&self, env: &Environment<'ctx>) -> Self::LLVMType;
  fn llvm_basic_type(&self, env: &Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>>;
}