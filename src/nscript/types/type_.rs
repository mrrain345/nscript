use std::fmt::Display;

use inkwell::{types::BasicTypeEnum};

use crate::nscript::{Environment, values::Value, AnyValue};

pub trait Type<'ctx>: Display + Clone {
  type LLVMType : inkwell::types::AnyType<'ctx>;
  type LLVMValue : inkwell::values::AnyValue<'ctx>;
  type Value : Value<'ctx>;

  fn create_value(&self, env: &Environment<'ctx>, value: Self::LLVMValue) -> Self::Value;

  // Arithmetic

  // fn op_add(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  // fn op_sub(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  // fn op_mul(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  // fn op_div(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  // fn op_modulo(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  // fn op_power(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  // fn op_plus(&self, env: &Environment<'ctx>, value: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  // fn op_minus(&self, env: &Environment<'ctx>, value: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }

  // Types

  fn llvm_type(&self, env: &Environment<'ctx>) -> Self::LLVMType;
  fn llvm_basic_type(&self, env: &Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>>;
}