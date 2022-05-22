use inkwell::values::BasicValueEnum;

use crate::nscript::{AnyValue, AnyType, Environment, StateType, Object};

pub fn integer<'ctx>(env: &mut Environment<'ctx>, value: i32) -> AnyValue<'ctx> {
  AnyValue::Integer(env.integer(value))
}

pub fn number<'ctx>(env: &mut Environment<'ctx>, value: f64) -> AnyValue<'ctx> {
  AnyValue::Number(env.number(value))
}

pub fn string<'ctx>(env: &mut Environment<'ctx>, value: &str) -> AnyValue<'ctx> {
  todo!()
}

pub fn boolean<'ctx>(env: &mut Environment<'ctx>, value: bool) -> AnyValue<'ctx> {
  AnyValue::Boolean(env.boolean(value))
}

pub fn null<'ctx>(env: &mut Environment<'ctx>) -> AnyValue<'ctx> {
  AnyValue::Null
}

pub fn identifier<'ctx>(env: &mut Environment<'ctx>, name: &str) -> AnyValue<'ctx> {
  if let Some((value, ..)) = env.state.get(name, None) {
    value
  } else { panic!("Parser error: label `{name}` not found") }
}