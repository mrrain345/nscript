use crate::{parser::expressions::Expression, nscript::{any_value::AnyValue, environment::Environment}};

pub fn modulo<'ctx>(env: &mut Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env);
  let right = right.codegen(env);

  if let (Some(left), Some(right)) = (left.into_option(), right.into_option()) {
    if left.is_int_value() && right.is_int_value() {
      // Integer % Integer
      let left = left.into_int_value();
      let right = right.into_int_value();
      let result = env.builder.build_int_signed_rem(left, right, "mod");
      result.into()
    }
    else { panic!("Parser error: Modulo of incompatible types") }
  }
  else { panic!("Parser error: invalid expression") }
}