use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn return_<'ctx>(env: &Environment<'ctx>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env);
  let env = env.borrow_mut();

  match value {
    AnyValue::Null => env.builder.build_return(None),
    AnyValue::Boolean(ref value) => env.builder.build_return(Some(&value.value)),
    AnyValue::Integer(ref value) => env.builder.build_return(Some(&value.value)),
    AnyValue::Number(ref value) => env.builder.build_return(Some(&value.value)),
    _ => panic!("Parser error: Return of incompatible types"),
  };

  value
}