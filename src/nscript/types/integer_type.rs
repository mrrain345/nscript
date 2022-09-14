use std::fmt::Display;

use inkwell::{types::{BasicTypeEnum, IntType}, values::IntValue};

use crate::nscript::{values::Integer, Environment};

use super::type_::Type;

#[derive(Clone, Copy)]
pub struct IntegerType;

impl<'ctx> IntegerType {
  pub fn create_const(&self, env: &Environment<'ctx>, value: i32) -> Integer<'ctx> {
    Integer {
      value: env.borrow().context.i32_type().const_int(value as u64, false)
    }
  }
}

impl<'ctx> Type<'ctx> for IntegerType {
  type LLVMType = IntType<'ctx>;
  type LLVMValue = IntValue<'ctx>;
  type Value = Integer<'ctx>;
  
  fn create_value(&self, env: &Environment<'ctx>, value: Self::LLVMValue) -> Self::Value {
    Integer { value }
  }

  fn llvm_type(&self, env: &Environment<'ctx>) -> Self::LLVMType {
    env.borrow().context.i32_type()
  }

  fn llvm_basic_type(&self, env: &Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    Some(env.borrow().context.i32_type().into())
  }
}

impl Display for IntegerType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Integer")
  }
}