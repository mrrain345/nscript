use crate::{parser::Expression, nscript::{AnyValue, Environment, AnyType}};

pub fn equal<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_equal(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator equal of incompatible types"))
}

pub fn not_equal<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_not_equal(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator not_equal of incompatible types"))
}

pub fn less_than<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_less_than(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator less_than of incompatible types"))
}

pub fn greater_than<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_greater_than(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator greater_than of incompatible types"))
}

pub fn less_or_equal<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_less_or_equal(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator less_or_equal of incompatible types"))
}

pub fn greater_or_equal<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_greater_or_equal(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator greater_or_equal of incompatible types"))
}