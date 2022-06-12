use inkwell::{types::{StructType, PointerType}, AddressSpace};

use super::{AnyType, Environment};

#[derive(Debug, PartialEq)]
pub struct Class<'ctx> {
  name: Option<String>,
  struct_type: StructType<'ctx>,
  properties: Vec<Property<'ctx>>,
}

impl<'ctx> Class<'ctx> {
  pub fn new(env: &Environment<'ctx>, name: Option<String>, properties: Vec<Property<'ctx>>) -> Self {
    let struct_type = env.borrow().context.struct_type(
      &properties
        .iter()
        .map(|p| p.type_.into_llvm_basic_type(env).expect(format!("Failed to get basic type `{:?}`", p.type_).as_str()))
        .collect::<Vec<_>>(),
      false,
    );

    Class {
      name,
      struct_type,
      properties,
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

  pub fn get_property(&self, index: usize) -> &Property {
    &self.properties[index]
  }

  pub fn properties_len(&self) -> usize {
    self.properties.len()
  }

  pub fn properties(&self) -> &[Property] {
    &self.properties
  }
}

#[derive(Debug, PartialEq)]
pub struct Property<'ctx> {
  pub name: String,
  pub type_: AnyType<'ctx>,
  pub modifiers: Option<Vec<String>>,
}