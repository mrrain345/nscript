use crate::{parser::Expression, nscript::{AnyValue, Environment, Operator, AnyType}};

pub fn add<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, left, right)
    .map(|type_| type_.op_add(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Add of incompatible types"))
}