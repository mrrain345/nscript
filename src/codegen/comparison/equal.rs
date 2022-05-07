use inkwell::{FloatPredicate, IntPredicate};

use crate::{parser::expressions::Expression, nscript::{AnyValue, Environment}};

pub fn equal<'ctx>(env: &mut Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env);
  let right = right.codegen(env);

  match (left, right) {
    // Integer == Integer
    (AnyValue::Integer(left), AnyValue::Integer(right)) => {
      let value = env.builder.build_int_compare(IntPredicate::EQ, left, right, "equal");
      AnyValue::Boolean(value)
    },
    // Number == Number
    (AnyValue::Number(left), AnyValue::Number(right)) => {
      let value = env.builder.build_float_compare(FloatPredicate::UEQ, left, right, "equal");
      AnyValue::Boolean(value)
    },
    // Integer == Number
    (AnyValue::Integer(left), AnyValue::Number(right)) => {
      let left = env.builder.build_signed_int_to_float(left, env.context.f64_type(), "int_to_float");
      let value = env.builder.build_float_compare(FloatPredicate::UEQ, left, right, "equal");
      AnyValue::Boolean(value)
    },
    // Number == Integer
    (AnyValue::Number(left), AnyValue::Integer(right)) => {
      let right = env.builder.build_signed_int_to_float(right, env.context.f64_type(), "int_to_float");
      let value = env.builder.build_float_compare(FloatPredicate::UEQ, left, right, "equal");
      AnyValue::Boolean(value)
    },
    // Boolean == Boolean
    (AnyValue::Boolean(left), AnyValue::Boolean(right)) => {
      let value = env.builder.build_int_compare(IntPredicate::EQ, left, right, "equal");
      AnyValue::Boolean(value)
    },
    // Null == Null
    (AnyValue::Null, AnyValue::Null) => {
      AnyValue::Boolean(env.context.bool_type().const_int(1, false))
    },
    // Null == any
    (AnyValue::Null, _) => {
      AnyValue::Boolean(env.context.bool_type().const_int(0, false))
    },
    // any == Null
    (_, AnyValue::Null) => {
      AnyValue::Boolean(env.context.bool_type().const_int(0, false))
    },
    _ => panic!("Parser error: Equal of incompatible types")
  }
}