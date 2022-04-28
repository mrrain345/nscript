use crate::{parser::expressions::Expression, nscript::{any_value::AnyValue, environment::Environment}};

pub fn plus<'ctx>(env: &mut Environment<'ctx>, expr: &Expression) -> AnyValue<'ctx> {
  let expr = expr.codegen(env);

  if let Some(expr) = expr.into_option() {
    if expr.is_int_value() {
      // Integer
      let expr = expr.into_int_value();
      expr.into()
    }
    else if expr.is_float_value() {
      // Number
      let expr = expr.into_float_value();
      expr.into()
    }
    else { panic!("Parser error: Incompatible type") }
  }
  else { panic!("Parser error: invalid expression") }
}