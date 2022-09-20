use inkwell::IntPredicate;

use crate::nscript::{types::{operations::traits::ComparisonOps, BooleanType, IntegerType}, Environment, AnyValue, Type};

impl<'ctx> ComparisonOps<'ctx> for IntegerType {
  fn op_equal(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_compare(IntPredicate::EQ, left.value, right.value, "add");
        Some(BooleanType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_not_equal(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_compare(IntPredicate::NE, left.value, right.value, "sub");
        Some(BooleanType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_less_than(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_compare(IntPredicate::SLT, left.value, right.value, "mul");
        Some(BooleanType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_greater_than(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_compare(IntPredicate::SGT, left.value, right.value, "div");
        Some(BooleanType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_less_or_equal(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_compare(IntPredicate::SLE, left.value, right.value, "modulo");
        Some(BooleanType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_greater_or_equal(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_compare(IntPredicate::SGE, left.value, right.value, "modulo");
        Some(BooleanType.create_value(env, value).into())
      },
      _ => None,
    }
  }
}