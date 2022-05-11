use crate::{parser::expressions::{Expression}, nscript::{AnyValue, Environment, Type, Property}};

pub fn class<'ctx>(env: &mut Environment<'ctx>, name: &String, properties: &[Property]) -> AnyValue<'ctx> {
  env.state.add_class(name.into(), properties.into())
    .expect("Failed to add class")
}