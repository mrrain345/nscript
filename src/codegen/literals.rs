use crate::nscript::{AnyValue, Environment, types::{IntegerType, NumberType, BooleanType}};

pub fn integer<'ctx>(env: &Environment<'ctx>, value: i32) -> AnyValue<'ctx> {
  IntegerType.create_const(env, value).into()
}

pub fn number<'ctx>(env: &Environment<'ctx>, value: f64) -> AnyValue<'ctx> {
  NumberType.create_const(env, value).into()
}

pub fn string<'ctx>(env: &Environment<'ctx>, value: &str) -> AnyValue<'ctx> {
  todo!()
}

pub fn boolean<'ctx>(env: &Environment<'ctx>, value: bool) -> AnyValue<'ctx> {
  BooleanType.create_const(env, value).into()
}

pub fn null<'ctx>(_env: &Environment<'ctx>) -> AnyValue<'ctx> {
  AnyValue::Null
}

pub fn identifier<'ctx>(env: &Environment<'ctx>, name: &str) -> AnyValue<'ctx> {
  if let Some(value) = env.borrow_mut().state.get(name) {
    value
  } else { panic!("Parser error: label `{name}` not found") }
}