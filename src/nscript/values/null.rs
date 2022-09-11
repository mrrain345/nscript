use inkwell::{types::{PointerType, AnyTypeEnum, BasicTypeEnum, VoidType}, values::{PointerValue, BasicValueEnum}};

use crate::nscript::{Environment, AnyType, types::NullType, Type};

use super::{value::Value, AnyValue};

#[derive(Debug, Clone)]
pub struct Null;

impl<'ctx> Null {
  pub fn llvm_null_type(env: &Environment<'ctx>) -> PointerType<'ctx> {
    env.borrow().context.struct_type(&[], true).ptr_type(inkwell::AddressSpace::Const).into()
  }

  pub fn llvm_null_value(env: &Environment<'ctx>) -> PointerValue<'ctx> {
    Null::llvm_null_type(env).const_null()
  }
}

impl<'ctx> Value<'ctx> for Null {
  type Type = NullType;
  type LLVMValue = PointerValue<'ctx>;

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    Null::llvm_null_value(env)
  }

  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {}

  fn get_type(&self) -> AnyType<'ctx> {
    AnyType::Null
  }

  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue {
    Null::llvm_null_value(env)
  }

  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<BasicValueEnum<'ctx>> {
    Some(Null::llvm_null_value(env).into())
  }
}

impl<'ctx> Into<AnyValue<'ctx>> for Null {
  fn into(self) -> AnyValue<'ctx> {
    AnyValue::Null
  }
}

impl<'ctx> From<AnyValue<'ctx>> for Null {
  fn from(value: AnyValue<'ctx>) -> Self {
    match value {
      AnyValue::Null => Null,
      _ => panic!("Invalid type"),
    }
  }
}