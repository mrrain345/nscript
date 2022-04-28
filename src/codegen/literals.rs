use crate::nscript::{any_value::AnyValue, environment::Environment};

pub fn integer<'ctx>(env: &mut Environment<'ctx>, value: i32) -> AnyValue<'ctx> {
  env.context.i32_type().const_int(value as u64, false).into()
}

pub fn number<'ctx>(env: &mut Environment<'ctx>, value: f64) -> AnyValue<'ctx> {
  env.context.f64_type().const_float(value).into()
}

pub fn identifier<'ctx>(env: &mut Environment<'ctx>, name: &str) -> AnyValue<'ctx> {
  if let Some(value) = env.state.get(name, None).into_option() {
    Some(value).into()
  } else {
    panic!("Parser error: label not found")
  }
}