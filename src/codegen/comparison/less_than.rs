use inkwell::{IntPredicate, FloatPredicate};

use crate::{nscript::{AnyValue, Environment}, parser::Expression};

pub fn less_than<'ctx>(env: &mut Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env);
  let right = right.codegen(env);

  match (left, right) {
    // Integer < Integer
    (AnyValue::Integer(left), AnyValue::Integer(right)) => {
      let value = env.builder.build_int_compare(IntPredicate::SLT, left, right, "less_than");
      AnyValue::Boolean(value)
    },
    // Number < Number
    (AnyValue::Number(left), AnyValue::Number(right)) => {
      let value = env.builder.build_float_compare(FloatPredicate::OLT, left, right, "less_than");
      AnyValue::Boolean(value)
    },
    // Integer < Number
    (AnyValue::Integer(left), AnyValue::Number(right)) => {
      let left = env.builder.build_signed_int_to_float(left, env.context.f64_type(), "int_to_float");
      let value = env.builder.build_float_compare(FloatPredicate::OLT, left, right, "less_than");
      AnyValue::Boolean(value)
    },
    // Number < Integer
    (AnyValue::Number(left), AnyValue::Integer(right)) => {
      let right = env.builder.build_signed_int_to_float(right, env.context.f64_type(), "int_to_float");
      let value = env.builder.build_float_compare(FloatPredicate::OLT, left, right, "less_than");
      AnyValue::Boolean(value)
    },
    _ => panic!("Parser error: Less than of incompatible types")
  }
}