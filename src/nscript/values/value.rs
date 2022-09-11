use std::fmt::Debug;

use inkwell::{values::{PointerValue, BasicValueEnum}};

use crate::nscript::{Environment, AnyType, Type};

use super::AnyValue;

pub trait Value<'ctx>: Debug + Clone + Into<AnyValue<'ctx>> + From<AnyValue<'ctx>> {
  type Type : Type<'ctx>;
  type LLVMValue : inkwell::values::AnyValue<'ctx>;

  // Memory

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx>;
  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>);

  // Cast

  fn silent_cast(&self, env: &Environment<'ctx>, type_: &AnyType) -> Option<AnyValue<'ctx>> { None }
  fn cast(&self, env: &Environment<'ctx>, type_: &AnyType) -> Option<AnyValue<'ctx>> { self.silent_cast(env, type_) }

  // Arithmetic operators

  fn op_add(&self, env: &Environment<'ctx>, other: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  fn op_sub(&self, env: &Environment<'ctx>, other: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  fn op_mul(&self, env: &Environment<'ctx>, other: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  fn op_div(&self, env: &Environment<'ctx>, other: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  fn op_modulo(&self, env: &Environment<'ctx>, other: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  fn op_power(&self, env: &Environment<'ctx>, other: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> { None }
  fn op_plus(&self, env: &Environment<'ctx>) -> Option<AnyValue<'ctx>> { None }
  fn op_minus(&self, env: &Environment<'ctx>) -> Option<AnyValue<'ctx>> { None }

  // Types

  fn get_type(&self) -> AnyType<'ctx>;
  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue;
  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<BasicValueEnum<'ctx>>;
}