use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn minus<'ctx>(env: &Environment<'ctx>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env).deref(env);
  let mut env = env.borrow_mut();

  match value {
    // Integer
    AnyValue::Integer(value) => {
      let value = env.builder.build_int_neg(value, "minus");
      AnyValue::Integer(value)
    },
    // Number
    AnyValue::Number(value) => {
      let value = env.builder.build_float_neg(value, "minus");
      AnyValue::Number(value)
    },
    _ => panic!("Parser error: Minus of incompatible types")
  }
}