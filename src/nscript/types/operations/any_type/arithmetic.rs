use crate::nscript::{AnyType, Environment, AnyValue, types::{*, operations::traits::ArithmeticOps}};

impl<'ctx> AnyType<'ctx> {
  pub fn op_add(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_add(env, left, right),
      AnyType::Number => NumberType.op_add(env, left, right),
      AnyType::Boolean => BooleanType.op_add(env, left, right),
      AnyType::Null => NullType.op_add(env, left, right),
      AnyType::Object(obj) => obj.op_add(env, left, right),
      AnyType::Ref(ref_) => ref_.op_add(env, left, right),
    }
  }

  pub fn op_sub(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_sub(env, left, right),
      AnyType::Number => NumberType.op_sub(env, left, right),
      AnyType::Boolean => BooleanType.op_sub(env, left, right),
      AnyType::Null => NullType.op_sub(env, left, right),
      AnyType::Object(obj) => obj.op_sub(env, left, right),
      AnyType::Ref(ref_) => ref_.op_sub(env, left, right),
    }
  }
  
  pub fn op_mul(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_mul(env, left, right),
      AnyType::Number => NumberType.op_mul(env, left, right),
      AnyType::Boolean => BooleanType.op_mul(env, left, right),
      AnyType::Null => NullType.op_mul(env, left, right),
      AnyType::Object(obj) => obj.op_mul(env, left, right),
      AnyType::Ref(ref_) => ref_.op_mul(env, left, right),
    }
  }
  
  pub fn op_div(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_div(env, left, right),
      AnyType::Number => NumberType.op_div(env, left, right),
      AnyType::Boolean => BooleanType.op_div(env, left, right),
      AnyType::Null => NullType.op_div(env, left, right),
      AnyType::Object(obj) => obj.op_div(env, left, right),
      AnyType::Ref(ref_) => ref_.op_div(env, left, right),
    }
  }
  
  pub fn op_modulo(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_modulo(env, left, right),
      AnyType::Number => NumberType.op_modulo(env, left, right),
      AnyType::Boolean => BooleanType.op_modulo(env, left, right),
      AnyType::Null => NullType.op_modulo(env, left, right),
      AnyType::Object(obj) => obj.op_modulo(env, left, right),
      AnyType::Ref(ref_) => ref_.op_modulo(env, left, right),
    }
  }
  
  pub fn op_power(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_power(env, left, right),
      AnyType::Number => NumberType.op_power(env, left, right),
      AnyType::Boolean => BooleanType.op_power(env, left, right),
      AnyType::Null => NullType.op_power(env, left, right),
      AnyType::Object(obj) => obj.op_power(env, left, right),
      AnyType::Ref(ref_) => ref_.op_power(env, left, right),
    }
  }
  
  pub fn op_plus(&self, env: &Environment<'ctx>, value: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_plus(env, value),
      AnyType::Number => NumberType.op_plus(env, value),
      AnyType::Boolean => BooleanType.op_plus(env, value),
      AnyType::Null => NullType.op_plus(env, value),
      AnyType::Object(obj) => obj.op_plus(env, value),
      AnyType::Ref(ref_) => ref_.op_plus(env, value),
    }
  }
  
  pub fn op_minus(&self, env: &Environment<'ctx>, value: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_minus(env, value),
      AnyType::Number => NumberType.op_minus(env, value),
      AnyType::Boolean => BooleanType.op_minus(env, value),
      AnyType::Null => NullType.op_minus(env, value),
      AnyType::Object(obj) => obj.op_minus(env, value),
      AnyType::Ref(ref_) => ref_.op_minus(env, value),
    }
  }
}