use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn return_<'ctx>(env: &Environment<'ctx>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env);
  let mut env = env.borrow_mut();

  let _ = match value {
    AnyValue::Null => env.builder.build_return(None),
    AnyValue::Boolean(value) => env.builder.build_return(Some(&value)),
    AnyValue::Integer(value) => env.builder.build_return(Some(&value)),
    AnyValue::Number(value) => env.builder.build_return(Some(&value)),
    _ => panic!("Parser error: Return of incompatible types"),
  };

  value
}