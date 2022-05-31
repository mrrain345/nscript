use crate::nscript::{AnyValue, Environment};

pub fn integer<'ctx>(env: &mut Environment<'ctx>, value: i32) -> AnyValue<'ctx> {
  AnyValue::Integer(env.context.i32_type().const_int(value as u64, false))
}

pub fn number<'ctx>(env: &mut Environment<'ctx>, value: f64) -> AnyValue<'ctx> {
  AnyValue::Number(env.context.f64_type().const_float(value))
}

pub fn string<'ctx>(env: &mut Environment<'ctx>, value: &str) -> AnyValue<'ctx> {
  todo!()
}

pub fn boolean<'ctx>(env: &mut Environment<'ctx>, value: bool) -> AnyValue<'ctx> {
  AnyValue::Boolean(env.context.bool_type().const_int(value as u64, false))
}

pub fn null<'ctx>(_env: &mut Environment<'ctx>) -> AnyValue<'ctx> {
  AnyValue::Null
}

pub fn identifier<'ctx>(env: &mut Environment<'ctx>, name: &str) -> AnyValue<'ctx> {
  if let Some((value, ..)) = env.state.get(name, None) {
    value
  } else { panic!("Parser error: label `{name}` not found") }
}