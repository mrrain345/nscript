use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn not<'ctx>(env: &mut Environment<'ctx>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env).deref(env);

  match value {
    // Boolean
    AnyValue::Boolean(value) => {
      let value = env.builder.build_not(value, "not");
      AnyValue::Boolean(value)
    },
    _ => panic!("Parser error: And of incompatible types")
  }
}