use std::{rc::Rc, ops::Deref};

use inkwell::{types::{StructType, PointerType, BasicTypeEnum}, AddressSpace, values::{PointerValue, BasicValueEnum}};

use crate::nscript::{Environment, AnyType, types::ObjectType, Type};

use super::{value::Value, AnyValue};

#[derive(Debug, PartialEq)]
struct ClassData<'ctx> {
  name: Option<String>,
  struct_type: StructType<'ctx>,
  properties: Vec<Property<'ctx>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class<'ctx> {
  data: Rc<ClassData<'ctx>>,
}

impl<'ctx> Class<'ctx> {
  pub fn new(env: &Environment<'ctx>, name: Option<String>, properties: Vec<Property<'ctx>>) -> Self {
    let struct_type = env.borrow().context.struct_type(
      &properties
        .iter()
        .map(|p| p.type_.llvm_basic_type(env).expect(format!("Failed to get basic type `{:?}`", p.type_).as_str()))
        .collect::<Vec<_>>(),
      false,
    );

    Class {
      data: Rc::new(ClassData {
        name,
        struct_type,
        properties,
      })
    }
  }

  pub fn name(&self) -> Option<&str> {
    self.name.as_deref()
  }

  pub fn name_or_default(&self) -> &str {
    self.name.as_deref().unwrap_or("<class>")
  }

  pub fn struct_type(&self) -> StructType<'ctx> {
    self.struct_type.clone()
  }

  pub fn ptr_type(&self) -> PointerType<'ctx> {
    self.struct_type.ptr_type(AddressSpace::Generic)
  }

  pub fn position(&self, name: &str) -> Option<usize> {
    self.properties.iter().position(|prop| prop.name == name)
  }

  pub fn property(&self, name: &str) -> Option<&Property> {
    self.properties.iter().find(|prop| prop.name == name)
  }

  pub fn get_property(&self, index: usize) -> &Property<'ctx> {
    &self.properties[index]
  }

  pub fn properties_len(&self) -> usize {
    self.properties.len()
  }

  pub fn properties(&self) -> &[Property] {
    &self.properties
  }
}

impl<'ctx> Value<'ctx> for Class<'ctx> {
  type Type = ObjectType<'ctx>;
  type LLVMValue = PointerValue<'ctx>;

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    todo!()
  }

  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {
    todo!()
  }

  fn get_type(&self) -> AnyType<'ctx> {
    todo!()
  }

  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue {
    todo!()
  }

  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<BasicValueEnum<'ctx>> {
    None
  }
}

impl<'ctx> Into<AnyValue<'ctx>> for Class<'ctx> {
  fn into(self) -> AnyValue<'ctx> {
    AnyValue::Class(self)
  }
}

impl<'ctx> From<AnyValue<'ctx>> for Class<'ctx> {
  fn from(value: AnyValue<'ctx>) -> Self {
    match value {
      AnyValue::Class(value) => value,
      _ => panic!("Invalid type"),
    }
  }
}

impl<'ctx> Deref for Class<'ctx> {
  type Target = ClassData<'ctx>;

  fn deref(&self) -> &Self::Target {
    self.data.as_ref()
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property<'ctx> {
  pub name: String,
  pub type_: AnyType<'ctx>,
  pub modifiers: Option<Vec<String>>,
}