use crate::nscript::{types::{operations::traits::ArithmeticOps, IntegerType}, Environment, AnyValue, Type};

impl<'ctx> ArithmeticOps<'ctx> for IntegerType {
  fn op_add(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_add(left.value, right.value, "add");
        Some(IntegerType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_sub(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_sub(left.value, right.value, "sub");
        Some(IntegerType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_mul(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_mul(left.value, right.value, "mul");
        Some(IntegerType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_div(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_signed_div(left.value, right.value, "div");
        Some(IntegerType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_modulo(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Integer(left), AnyValue::Integer(right)) => {
        let value = env.borrow_mut().builder.build_int_signed_rem(left.value, right.value, "modulo");
        Some(IntegerType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  // fn op_power(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
  //   match (left, right) {
  //     (AnyValue::Integer(left), AnyValue::Integer(right)) => {
  //       let value = env.borrow_mut().builder.build_int_mul(left.value, right.value, "power");
  //       Some(IntegerType.create_value(env, value).into())
  //     },
  //     _ => None,
  //   }
  // }

  fn op_plus(&self, env: &Environment<'ctx>, value: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match value {
      AnyValue::Integer(value) => {
        Some(value.clone().into())
      },
      _ => None,
    }
  }

  fn op_minus(&self, env: &Environment<'ctx>, value: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match value {
      AnyValue::Integer(val) => {
        let value = env.borrow_mut().builder.build_int_neg(val.value, "minus");
        Some(IntegerType.create_value(env, value).into())
      },
      _ => None,
    }
  }
}