use crate::{parser::Expression, nscript::{AnyValue, Environment, AnyType}};

pub fn add<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_add(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator add of incompatible types"))
}

pub fn sub<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_sub(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator sub of incompatible types"))
}

pub fn mul<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_mul(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator mul of incompatible types"))
}

pub fn div<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_div(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator div of incompatible types"))
}

pub fn modulo<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_modulo(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator modulo of incompatible types"))
}

pub fn power<'ctx>(env: &Environment<'ctx>, left: &Expression, right: &Expression) -> AnyValue<'ctx> {
  let left = left.codegen(env).deref(env);
  let right = right.codegen(env).deref(env);

  AnyType::common_type(env, &left, &right)
    .map(|type_| type_.op_power(env, &left, &right))
    .flatten()
    .unwrap_or_else(|| panic!("Parser error: Operator power of incompatible types"))
}

pub fn minus<'ctx>(env: &Environment<'ctx>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env).deref(env);

  value.get_type().op_minus(env, &value)
    .unwrap_or_else(|| panic!("Parser error: Operator minus of incompatible types"))
}

pub fn plus<'ctx>(env: &Environment<'ctx>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env).deref(env);

  value.get_type().op_plus(env, &value)
    .unwrap_or_else(|| panic!("Parser error: Operator plus of incompatible types"))
}