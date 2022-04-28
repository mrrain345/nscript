use crate::{parser::expressions::Expression, nscript::{any_value::AnyValue, environment::Environment}};

pub fn minus<'ctx>(env: &mut Environment<'ctx>, expr: &Expression) -> AnyValue<'ctx> {
  let expr = expr.codegen(env);

  if let Some(expr) = expr.into_option() {
    if expr.is_int_value() {
      // Integer
      let expr = expr.into_int_value();
      let result = env.builder.build_int_neg(expr, "neg");
      result.into()
    }
    else if expr.is_float_value() {
      // Number
      let expr = expr.into_float_value();
      let result = env.builder.build_float_neg(expr, "neg");
      result.into()
    }
    else { panic!("Parser error: Negation of incompatible type") }
  }
  else { panic!("Parser error: invalid expression") }
}