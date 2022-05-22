use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn or<'ctx>(env: &mut Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  match (left, right) {
    // Boolean || Boolean
    (AnyValue::Boolean(left), AnyValue::Boolean(right)) => {
      let value = env.builder.build_or(left, right, "or");
      AnyValue::Boolean(value)
    },
    _ => panic!("Parser error: And of incompatible types")
  }
}