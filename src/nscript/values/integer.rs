use inkwell::{values::{IntValue, PointerValue, BasicValueEnum}, types::{AnyTypeEnum, BasicTypeEnum}};

use crate::nscript::{Environment, AnyType, types::IntegerType, Type};

use super::{value::Value, AnyValue, Number};

#[derive(Debug, Clone)]
pub struct Integer<'ctx> {
  pub value: IntValue<'ctx>
}

impl<'ctx> Value<'ctx> for Integer<'ctx> {
  type Type = IntegerType;
  type LLVMValue = IntValue<'ctx>;

  // Memory

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    env.borrow_mut().builder.build_alloca(self.value.get_type(), "Integer")
  }

  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {
    env.borrow_mut().builder.build_store(ptr, self.value);
  }

  // Cast

  fn silent_cast(&self, env: &Environment<'ctx>, type_: &AnyType) -> Option<AnyValue<'ctx>> {
    match type_ {
      // cast Integer as Number
      AnyType::Number => Some(Number {
        value: env.borrow_mut().builder.build_signed_int_to_float(self.value, env.borrow().context.f64_type(), "cast")
      }.into()),
      _ => None,
    }
  }

  // Arithmetic

  // fn op_add(&self, env: &Environment<'ctx>, other: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
  //   match other {
  //     AnyValue::Integer(value) => {
  //       let val = env.borrow_mut().builder.build_int_add(self.value, value.value, "add");
  //       Some(IntegerType.create_value(env, val).into())
  //     },
  //     _ => None,
  //   }
  // }

  // Types

  fn get_type(&self) -> AnyType<'ctx> {
    AnyType::Integer
  }

  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<BasicValueEnum<'ctx>> {
    Some(self.value.into())
  }

  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue {
    self.value
  }
}

impl<'ctx> Into<AnyValue<'ctx>> for Integer<'ctx> {
  fn into(self) -> AnyValue<'ctx> {
    AnyValue::Integer(self)
  }
}

impl<'ctx> From<AnyValue<'ctx>> for Integer<'ctx> {
  fn from(value: AnyValue<'ctx>) -> Self {
    match value {
      AnyValue::Integer(value) => value,
      _ => panic!("Invalid type"),
    }
  }
}