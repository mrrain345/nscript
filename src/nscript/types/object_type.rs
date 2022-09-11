use std::fmt::Display;

use inkwell::{values::{AnyValueEnum, PointerValue}, types::{PointerType, BasicTypeEnum}};

use crate::nscript::{values::{Class, AnyValue, Object}, Environment};

use super::type_::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType<'ctx> {
  pub class: Class<'ctx>
}

impl<'ctx> Type<'ctx> for ObjectType<'ctx> {
  type LLVMType = PointerType<'ctx>;
  type LLVMValue = PointerValue<'ctx>;
  type Value = Object<'ctx>;

  fn llvm_type(&self, _env: &Environment<'ctx>) -> Self::LLVMType {
    self.class.ptr_type().into()
  }

  fn llvm_basic_type(&self, _env: &Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    Some(self.class.ptr_type().into())
  }

  fn create_value(env: &Environment, value: Self::LLVMValue) -> Self::Value {
    todo!()
  }
}

impl<'ctx> Display for ObjectType<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.class.name().unwrap_or("Object"))
  }
}