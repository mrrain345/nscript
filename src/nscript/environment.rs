use inkwell::{context::Context, module::Module, builder::Builder, values::{AnyValueEnum, IntValue, FloatValue}};

use super::state::State;

#[derive(Debug)]
pub struct Environment<'ctx> {
  pub context: &'ctx Context,
  pub module: Module<'ctx>,
  pub builder: Builder<'ctx>,
  pub state: State<'ctx>,
}

impl<'ctx> Environment<'ctx> {
  pub fn new(context: &'ctx Context) -> Self {
    Environment {
      context,
      module: context.create_module("main"),
      builder: context.create_builder(),
      state: State::new(),
    }
  }

  pub fn integer(&self, value: i32) -> IntValue {
    return self.context.i32_type().const_int(value as u64, false);
  }

  pub fn number(&self, value: f64) -> FloatValue {
    return self.context.f64_type().const_float(value);
  }

  pub fn boolean(&self, value: bool) -> IntValue {
    return self.context.bool_type().const_int(value as u64, false);
  }

  pub fn null(&self) -> Option<AnyValueEnum> {
    return None
  }
}
