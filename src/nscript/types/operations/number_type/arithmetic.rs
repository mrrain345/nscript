use crate::nscript::{types::{operations::traits::ArithmeticOps, NumberType}, Environment, AnyValue, Type};

impl<'ctx> ArithmeticOps<'ctx> for NumberType {
  fn op_add(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Number(left), AnyValue::Number(right)) => {
        let value = env.borrow_mut().builder.build_float_add(left.value, right.value, "add");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Number(left), AnyValue::Integer(right)) => {
        let right = env.borrow_mut().builder.build_signed_int_to_float(right.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_add(left.value, right, "add");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Integer(left), AnyValue::Number(right)) => {
        let left = env.borrow_mut().builder.build_signed_int_to_float(left.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_add(left, right.value, "add");
        Some(NumberType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_sub(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Number(left), AnyValue::Number(right)) => {
        let value = env.borrow_mut().builder.build_float_sub(left.value, right.value, "sub");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Number(left), AnyValue::Integer(right)) => {
        let right = env.borrow_mut().builder.build_signed_int_to_float(right.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_sub(left.value, right, "sub");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Integer(left), AnyValue::Number(right)) => {
        let left = env.borrow_mut().builder.build_signed_int_to_float(left.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_sub(left, right.value, "sub");
        Some(NumberType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_mul(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Number(left), AnyValue::Number(right)) => {
        let value = env.borrow_mut().builder.build_float_mul(left.value, right.value, "mul");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Number(left), AnyValue::Integer(right)) => {
        let right = env.borrow_mut().builder.build_signed_int_to_float(right.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_mul(left.value, right, "mul");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Integer(left), AnyValue::Number(right)) => {
        let left = env.borrow_mut().builder.build_signed_int_to_float(left.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_mul(left, right.value, "mul");
        Some(NumberType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_div(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Number(left), AnyValue::Number(right)) => {
        let value = env.borrow_mut().builder.build_float_div(left.value, right.value, "div");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Number(left), AnyValue::Integer(right)) => {
        let right = env.borrow_mut().builder.build_signed_int_to_float(right.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_div(left.value, right, "div");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Integer(left), AnyValue::Number(right)) => {
        let left = env.borrow_mut().builder.build_signed_int_to_float(left.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_div(left, right.value, "div");
        Some(NumberType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  fn op_modulo(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match (left, right) {
      (AnyValue::Number(left), AnyValue::Number(right)) => {
        let value = env.borrow_mut().builder.build_float_rem(left.value, right.value, "modulo");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Number(left), AnyValue::Integer(right)) => {
        let right = env.borrow_mut().builder.build_signed_int_to_float(right.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_rem(left.value, right, "modulo");
        Some(NumberType.create_value(env, value).into())
      },
      (AnyValue::Integer(left), AnyValue::Number(right)) => {
        let left = env.borrow_mut().builder.build_signed_int_to_float(left.value, NumberType.llvm_type(env), "int_to_float");
        let value = env.borrow_mut().builder.build_float_rem(left, right.value, "modulo");
        Some(NumberType.create_value(env, value).into())
      },
      _ => None,
    }
  }

  // fn op_power(&self, env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
  //   match (left, right) {
  //     (AnyValue::Number(left), AnyValue::Number(right)) => {
  //       let value = env.borrow_mut().builder.build_float_mul(left.value, right.value, "power");
  //       Some(NumberType.create_value(env, value).into())
  //     },
  //     _ => None,
  //   }
  // }

  fn op_plus(&self, env: &Environment<'ctx>, value: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match value {
      AnyValue::Number(value) => {
        Some(value.clone().into())
      },
      _ => None,
    }
  }

  fn op_minus(&self, env: &Environment<'ctx>, value: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    match value {
      AnyValue::Number(val) => {
        let value = env.borrow_mut().builder.build_float_neg(val.value, "minus");
        Some(NumberType.create_value(env, value).into())
      },
      _ => None,
    }
  }
}