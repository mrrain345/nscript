use crate::nscript::{AnyType, Environment, AnyValue, types::{*, operations::traits::ComparisonOps}};

impl<'ctx> AnyType<'ctx> {
  pub fn op_equal(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_equal(env, left, right),
      AnyType::Number => NumberType.op_equal(env, left, right),
      AnyType::Boolean => BooleanType.op_equal(env, left, right),
      AnyType::Null => NullType.op_equal(env, left, right),
      AnyType::Object(obj) => obj.op_equal(env, left, right),
      AnyType::Ref(ref_) => ref_.op_equal(env, left, right),
    }
  }

  pub fn op_not_equal(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_not_equal(env, left, right),
      AnyType::Number => NumberType.op_not_equal(env, left, right),
      AnyType::Boolean => BooleanType.op_not_equal(env, left, right),
      AnyType::Null => NullType.op_not_equal(env, left, right),
      AnyType::Object(obj) => obj.op_not_equal(env, left, right),
      AnyType::Ref(ref_) => ref_.op_not_equal(env, left, right),
    }
  }
  
  pub fn op_less_than(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_less_than(env, left, right),
      AnyType::Number => NumberType.op_less_than(env, left, right),
      AnyType::Boolean => BooleanType.op_less_than(env, left, right),
      AnyType::Null => NullType.op_less_than(env, left, right),
      AnyType::Object(obj) => obj.op_less_than(env, left, right),
      AnyType::Ref(ref_) => ref_.op_less_than(env, left, right),
    }
  }
  
  pub fn op_greater_than(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_greater_than(env, left, right),
      AnyType::Number => NumberType.op_greater_than(env, left, right),
      AnyType::Boolean => BooleanType.op_greater_than(env, left, right),
      AnyType::Null => NullType.op_greater_than(env, left, right),
      AnyType::Object(obj) => obj.op_greater_than(env, left, right),
      AnyType::Ref(ref_) => ref_.op_greater_than(env, left, right),
    }
  }
  
  pub fn op_less_or_equal(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_less_or_equal(env, left, right),
      AnyType::Number => NumberType.op_less_or_equal(env, left, right),
      AnyType::Boolean => BooleanType.op_less_or_equal(env, left, right),
      AnyType::Null => NullType.op_less_or_equal(env, left, right),
      AnyType::Object(obj) => obj.op_less_or_equal(env, left, right),
      AnyType::Ref(ref_) => ref_.op_less_or_equal(env, left, right),
    }
  }
  
  pub fn op_greater_or_equal(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match self {
      AnyType::Integer => IntegerType.op_greater_or_equal(env, left, right),
      AnyType::Number => NumberType.op_greater_or_equal(env, left, right),
      AnyType::Boolean => BooleanType.op_greater_or_equal(env, left, right),
      AnyType::Null => NullType.op_greater_or_equal(env, left, right),
      AnyType::Object(obj) => obj.op_greater_or_equal(env, left, right),
      AnyType::Ref(ref_) => ref_.op_greater_or_equal(env, left, right),
    }
  }
}