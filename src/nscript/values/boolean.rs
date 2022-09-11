use inkwell::{values::{IntValue, PointerValue, BasicValueEnum}, types::BasicTypeEnum};

use crate::nscript::{Environment, AnyType, types::BooleanType, Type};

use super::{value::Value, AnyValue};

#[derive(Debug, Clone)]
pub struct Boolean<'ctx> {
  pub value: IntValue<'ctx>
}

impl<'ctx> Value<'ctx> for Boolean<'ctx> {
  type Type = BooleanType;
  type LLVMValue = IntValue<'ctx>;

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    env.borrow_mut().builder.build_alloca(self.value.get_type(), "Boolean")
  }

  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {
    env.borrow_mut().builder.build_store(ptr, self.value);
  }

  fn get_type(&self) -> AnyType<'ctx> {
    AnyType::Boolean
  }

  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<BasicValueEnum<'ctx>> {
    Some(self.value.into())
  }

  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue {
    self.value
  }
}

impl<'ctx> Into<AnyValue<'ctx>> for Boolean<'ctx> {
  fn into(self) -> AnyValue<'ctx> {
    AnyValue::Boolean(self)
  }
}

impl<'ctx> From<AnyValue<'ctx>> for Boolean<'ctx> {
  fn from(value: AnyValue<'ctx>) -> Self {
    match value {
      AnyValue::Boolean(value) => value,
      _ => panic!("Invalid type"),
    }
  }
}