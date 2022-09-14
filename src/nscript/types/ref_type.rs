use std::{fmt::Display, ops::Deref};

use inkwell::{values::{AnyValueEnum, PointerValue}, types::{PointerType, BasicTypeEnum}};

use crate::nscript::{values::{Class, AnyValue, Ref, Null}, Environment};

use super::{type_::Type, AnyType, IntegerType, NumberType, BooleanType, NullType};

#[derive(Debug, Clone, PartialEq)]
pub struct RefType<'ctx> {
  pub type_: Box<AnyType<'ctx>>
}

impl<'ctx> Type<'ctx> for RefType<'ctx> {
  type LLVMType = PointerType<'ctx>;
  type LLVMValue = PointerValue<'ctx>;
  type Value = Ref<'ctx>;

  fn llvm_type(&self, env: &Environment<'ctx>) -> Self::LLVMType {
    match self.type_.deref() {
      AnyType::Integer => IntegerType.llvm_type(env).ptr_type(inkwell::AddressSpace::Generic),
      AnyType::Number => NumberType.llvm_type(env).ptr_type(inkwell::AddressSpace::Generic),
      AnyType::Boolean => BooleanType.llvm_type(env).ptr_type(inkwell::AddressSpace::Generic),
      AnyType::Null => NullType.llvm_type(env),
      AnyType::Object(object) => object.llvm_type(env),
      AnyType::Ref(ref_) => ref_.llvm_type(env),
    }
  }

  fn llvm_basic_type(&self, env: &Environment<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    let type_ = match self.type_.deref() {
      AnyType::Integer => IntegerType.llvm_type(env).ptr_type(inkwell::AddressSpace::Generic),
      AnyType::Number => NumberType.llvm_type(env).ptr_type(inkwell::AddressSpace::Generic),
      AnyType::Boolean => BooleanType.llvm_type(env).ptr_type(inkwell::AddressSpace::Generic),
      AnyType::Null => NullType.llvm_type(env),
      AnyType::Object(object) => object.llvm_type(env),
      AnyType::Ref(ref_) => ref_.llvm_type(env),
    };

    Some(type_.into())
  }

  fn create_value(&self, env: &Environment<'ctx>, value: Self::LLVMValue) -> Self::Value {
    todo!()
  }
}

impl<'ctx> Display for RefType<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("ref ");
    self.type_.fmt(f)
  }
}