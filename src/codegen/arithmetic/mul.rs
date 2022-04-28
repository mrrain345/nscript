use crate::{parser::expressions::Expression, nscript::{any_value::AnyValue, environment::Environment}};

pub fn mul<'ctx>(env: &mut Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env);
  let right = right.codegen(env);

  if let (Some(left), Some(right)) = (left.into_option(), right.into_option()) {
    if left.is_int_value() && right.is_int_value() {
      // Integer * Integer
      let left = left.into_int_value();
      let right = right.into_int_value();
      let result = env.builder.build_int_mul(left, right, "mul");
      result.into()
    }
    else if left.is_float_value() && right.is_float_value() {
      // Number * Number
      let left = left.into_float_value();
      let right = right.into_float_value();
      let result = env.builder.build_float_mul(left, right, "mul");
      result.into()
    }
    else if left.is_int_value() && right.is_float_value() {
      // Integer * Number
      let left = left.into_int_value();
      let left = env.builder.build_signed_int_to_float(left, env.context.f64_type(), "int_to_float");
      let right = right.into_float_value();
      let result = env.builder.build_float_mul(left, right, "mul");
      result.into()
    }
    else if left.is_float_value() && right.is_int_value() {
      // Number * Integer
      let right = right.into_int_value();
      let right = env.builder.build_signed_int_to_float(right, env.context.f64_type(), "int_to_float");
      let left = left.into_float_value();
      let result = env.builder.build_float_mul(left, right, "mul");
      result.into()
    }
    else { panic!("Parser error: Multiplication of incompatible types") }
  }
  else { panic!("Parser error: invalid expression") }
}