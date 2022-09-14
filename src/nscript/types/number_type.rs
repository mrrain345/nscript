use std::fmt::Display;

use inkwell::{values::FloatValue, types::{BasicTypeEnum, FloatType}};

use crate::nscript::{values::Number, Environment};

use super::type_::Type;

#[derive(Clone, Copy)]
pub struct NumberType;

impl<'ctx> NumberType {
  pub fn create_const(&self, env: &Environment<'ctx>, value: f64) -> Number<'ctx> {
    Number {
      value: env.borrow().context.f64_type().const_float(value)
    }
  }
}

impl<'ctx> Type<'ctx> for NumberType {
  type LLVMType = FloatType<'ctx>;
  type LLVMValue = FloatValue<'ctx>;
  type Value = Number<'ctx>;
  
  fn llvm_type(&self, env: &Environment<'ctx>) -> Self::LLVMType {
    env.borrow().context.f64_type()
  }

  fn llvm_basic_type(&self, env: &Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    Some(env.borrow().context.f64_type().into())
  }

  fn create_value(&self, env: &Environment<'ctx>, value: Self::LLVMValue) -> Self::Value {
    Number { value }
  }
}

impl Display for NumberType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Number")
  }
}