use crate::{parser::expressions::Expression, nscript::{any_value::AnyValue, environment::Environment}};

pub fn mul<'ctx>(env: &mut Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env);
  let right = right.codegen(env);

  match (left, right) {
    // Integer * Integer
    (AnyValue::Integer(left), AnyValue::Integer(right)) => {
      let value = env.builder.build_int_mul(left, right, "mul");
      AnyValue::Integer(value)
    },
    // Number * Number
    (AnyValue::Number(left), AnyValue::Number(right)) => {
      let value = env.builder.build_float_mul(left, right, "mul");
      AnyValue::Number(value)
    },
    // Integer * Number
    (AnyValue::Integer(left), AnyValue::Number(right)) => {
      let left = env.builder.build_signed_int_to_float(left, env.context.f64_type(), "int_to_float");
      let value = env.builder.build_float_mul(left, right, "mul");
      AnyValue::Number(value)
    },
    // Number * Integer
    (AnyValue::Number(left), AnyValue::Integer(right)) => {
      let right = env.builder.build_signed_int_to_float(right, env.context.f64_type(), "int_to_float");
      let value = env.builder.build_float_mul(left, right, "mul");
      AnyValue::Number(value)
    },
    _ => panic!("Parser error: Mul of incompatible types")
  }
}