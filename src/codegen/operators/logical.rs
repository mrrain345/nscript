use crate::{parser::Expression, nscript::{AnyValue, Environment, AnyType}};

pub fn and<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_and(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator and of incompatible types"))
}

pub fn or<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_or(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator or of incompatible types"))
}

pub fn not<'ctx>(env: &Environment<'ctx>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env).deref(env);

  value.get_type().op_not(env, &value)
    .unwrap_or_else(|| panic!("Parser error: Operator not of incompatible types"))
}