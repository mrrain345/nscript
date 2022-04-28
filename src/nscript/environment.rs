use inkwell::{context::Context, module::Module, builder::Builder};

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
}
