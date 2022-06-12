use crate::nscript::{AnyValue, Environment};

pub fn integer<'ctx>(env: &Environment<'ctx>, value: i32) -> AnyValue<'ctx> {
  AnyValue::Integer(env.borrow().context.i32_type().const_int(value as u64, false))
}

pub fn number<'ctx>(env: &Environment<'ctx>, value: f64) -> AnyValue<'ctx> {
  AnyValue::Number(env.borrow().context.f64_type().const_float(value))
}

pub fn string<'ctx>(env: &Environment<'ctx>, value: &str) -> AnyValue<'ctx> {
  todo!()
}

pub fn boolean<'ctx>(env: &Environment<'ctx>, value: bool) -> AnyValue<'ctx> {
  AnyValue::Boolean(env.borrow().context.bool_type().const_int(value as u64, false))
}

pub fn null<'ctx>(_env: &Environment<'ctx>) -> AnyValue<'ctx> {
  AnyValue::Null
}

pub fn identifier<'ctx>(env: &Environment<'ctx>, name: &str) -> AnyValue<'ctx> {
  if let Some((value, ..)) = env.borrow_mut().state.get(name, None) {
    value
  } else { panic!("Parser error: label `{name}` not found") }
}