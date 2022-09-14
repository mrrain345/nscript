use crate::nscript::{AnyType, Environment, AnyValue, types::{*, operations::traits::LogicalOps}};

impl<'ctx> AnyType<'ctx> {
  pub fn op_and(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_and(env, left, right),
      AnyType::Number => NumberType.op_and(env, left, right),
      AnyType::Boolean => BooleanType.op_and(env, left, right),
      AnyType::Null => NullType.op_and(env, left, right),
      AnyType::Object(obj) => obj.op_and(env, left, right),
      AnyType::Ref(ref_) => ref_.op_and(env, left, right),
    }
  }

  pub fn op_or(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_or(env, left, right),
      AnyType::Number => NumberType.op_or(env, left, right),
      AnyType::Boolean => BooleanType.op_or(env, left, right),
      AnyType::Null => NullType.op_or(env, left, right),
      AnyType::Object(obj) => obj.op_or(env, left, right),
      AnyType::Ref(ref_) => ref_.op_or(env, left, right),
    }
  }
  
  pub fn op_not(&self, env: &Environment<'ctx>, value: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_not(env, value),
      AnyType::Number => NumberType.op_not(env, value),
      AnyType::Boolean => BooleanType.op_not(env, value),
      AnyType::Null => NullType.op_not(env, value),
      AnyType::Object(obj) => obj.op_not(env, value),
      AnyType::Ref(ref_) => ref_.op_not(env, value),
    }
  }
}