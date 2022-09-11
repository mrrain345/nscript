use inkwell::{values::PointerValue, types::{AnyTypeEnum, BasicTypeEnum}};

use crate::nscript::{Environment, AnyType, types::NullType, Type};

use super::{value::Value, AnyValue};

#[derive(Debug, Clone)]
pub struct Ref<'ctx> {
  pub type_: AnyType<'ctx>,
  pub ptr: PointerValue<'ctx>,
}

impl<'ctx> Ref<'ctx> {
  pub fn new(type_: AnyType<'ctx>, ptr: PointerValue<'ctx>) -> Self {
    Self { type_, ptr }
  }
}

impl<'ctx> Ref<'ctx> {
  pub fn deref(&self, env: &Environment<'ctx>) -> AnyValue<'ctx> {
    let value = env.borrow_mut().builder.build_load(self.ptr, "deref");
    self.type_.create_value(env, value.into())
  }
}

impl<'ctx> Value<'ctx> for Ref<'ctx> {
  type Type = NullType; // TODO
  type LLVMValue = PointerValue<'ctx>;

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    env.borrow_mut().builder.build_alloca(self.ptr.get_type(), "Ref")
  }

  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {
    env.borrow_mut().builder.build_store(ptr, self.ptr);
  }

  fn get_type(&self) -> AnyType<'ctx> {
    todo!()
  }

  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue {
    self.ptr
  }

  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<inkwell::values::BasicValueEnum<'ctx>> {
    Some(self.ptr.into())
  }
}

impl<'ctx> Into<AnyValue<'ctx>> for Ref<'ctx> {
  fn into(self) -> AnyValue<'ctx> {
    AnyValue::Ref(self)
  }
}

impl<'ctx> From<AnyValue<'ctx>> for Ref<'ctx> {
  fn from(value: AnyValue<'ctx>) -> Self {
    match value {
      AnyValue::Ref(value) => value,
      _ => panic!("Invalid type"),
    }
  }
}