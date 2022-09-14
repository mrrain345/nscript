use inkwell::{values::{FloatValue, PointerValue, BasicValueEnum}, types::{AnyTypeEnum, BasicTypeEnum}};

use crate::nscript::{Environment, AnyType, types::NumberType, Type};

use super::{value::Value, AnyValue, Integer};

#[derive(Debug, Clone)]
pub struct Number<'ctx> {
  pub value: FloatValue<'ctx>
}

impl<'ctx> Value<'ctx> for Number<'ctx> {
  type Type = NumberType;
  type LLVMValue = FloatValue<'ctx>;

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    env.borrow_mut().builder.build_alloca(self.value.get_type(), "Number")
  }

  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {
    env.borrow_mut().builder.build_store(ptr, self.value);
  }

  fn cast(&self, env: &Environment<'ctx>, type_: &AnyType) -> Option<AnyValue<'ctx>> {
    match type_ {
      // cast Number as Integer
      AnyType::Number => Some(Integer {
        value: env.borrow_mut().builder.build_float_to_signed_int(self.value, env.borrow().context.i32_type(), "cast")
      }.into()),
      _ => None,
    }
  }

  // fn op_add(&self, env: &Environment<'ctx>, other: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
  //   match other {
  //     AnyValue::Number(value) => {
  //       let val = env.borrow_mut().builder.build_float_add(self.value, value.value, "add");
  //       Some(NumberType.create_value(env, val).into())
  //     },
  //     _ => None,
  //   }
  // }

  fn get_type(&self) -> AnyType<'ctx> {
    AnyType::Number
  }

  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<BasicValueEnum<'ctx>> {
    Some(self.value.into())
  }

  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue {
    self.value
  }
}

impl<'ctx> Into<AnyValue<'ctx>> for Number<'ctx> {
  fn into(self) -> AnyValue<'ctx> {
    AnyValue::Number(self)
  }
}

impl<'ctx> From<AnyValue<'ctx>> for Number<'ctx> {
  fn from(value: AnyValue<'ctx>) -> Self {
    match value {
      AnyValue::Number(value) => value,
      _ => panic!("Invalid type"),
    }
  }
}