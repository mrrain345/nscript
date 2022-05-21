use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn plus<'ctx>(env: &mut Environment<'ctx>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env);

  match value {
    // Integer
    AnyValue::Integer(value) => {
      AnyValue::Integer(value)
    },
    // Number
    AnyValue::Number(value) => {
      AnyValue::Number(value)
    },
    _ => panic!("Parser error: Plus of incompatible types")
  }
}