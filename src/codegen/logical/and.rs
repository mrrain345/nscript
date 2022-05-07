use crate::{parser::expressions::Expression, nscript::{AnyValue, Environment}};

pub fn and<'ctx>(env: &mut Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env);
  let right = right.codegen(env);

  match (left, right) {
    // Boolean && Boolean
    (AnyValue::Boolean(left), AnyValue::Boolean(right)) => {
      let value = env.builder.build_and(left, right, "and");
      AnyValue::Boolean(value)
    },
    _ => panic!("Parser error: And of incompatible types")
  }
}