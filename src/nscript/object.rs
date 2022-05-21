use inkwell::values::{StructValue, PointerValue};

use super::{PropertyValue, Class, AnyValue, Environment, AnyType};

#[derive(Debug, Clone, PartialEq)]
pub struct Object<'ctx> {
  class: &'ctx Class<'ctx>,
  struct_ptr: PointerValue<'ctx>,
  properties: Vec<AnyValue<'ctx>>,
}

impl<'ctx> Object<'ctx> {
  pub fn new(env: &mut Environment<'ctx>, class: &'ctx Class<'ctx>, properties: Vec<AnyValue<'ctx>>) -> Self {
    
    // Create the struct
    let struct_ptr = env.builder.build_alloca(class.struct_type(), class.name_or_default());

    // Set the struct's properties
    for (index, property) in properties.iter().enumerate() {

      // Get the property's name and pointer
      let property_name = class.get_property(index).name.as_ref();
      let ptr = env.builder.build_struct_gep(struct_ptr, index as u32, property_name).unwrap();

      // Set the property's value
      match *property {
        AnyValue::Integer(value) => env.builder.build_store(ptr, value),
        AnyValue::Number(value) => env.builder.build_store(ptr, value),
        AnyValue::Boolean(value) => env.builder.build_store(ptr, value),
        _ => panic!("Parser error: invalid type of property `{property_name:?}` in class `{name:?}`", name=class.name_or_default()),
      };
    }

    // Return the object
    Object {
      class,
      struct_ptr,
      properties,
    }
  }

  pub fn from_struct_value(env: &mut Environment<'ctx>, class: &'ctx Class<'ctx>, struct_value: StructValue<'ctx>) -> Self {
    
    let properties = class.properties()
      .iter()
      .enumerate()
      .map(|(index, property)| {
        let property_name = class.get_property(index).name.as_ref();
        let property_value = env.builder.build_extract_value(struct_value, index as u32, property_name).unwrap();

        match property.type_.into_type().unwrap() {
          AnyType::Integer => AnyValue::Integer(property_value.into_int_value()),
          AnyType::Number => AnyValue::Number(property_value.into_float_value()),
          AnyType::Boolean => AnyValue::Boolean(property_value.into_int_value()),
          _ => panic!("Parser error: invalid type of property `{property_name:?}` in class `{name:?}`", name=class.name_or_default()),
        }
      })
      .collect();

    Object::new(env, class, properties)
  }

  pub fn class(&self) -> &'ctx Class<'ctx> {
    self.class
  }

  pub fn set_property(&mut self, name: &str, value: AnyValue<'ctx>) {
    let index = self.class.position(name);
    index.map(|index| self.properties[index] = value);
  }

  pub fn get_property(&self, name: &str) -> Option<&AnyValue<'ctx>> {
    let index = self.class.position(name);
    index.map(|index| &self.properties[index])
  }

  pub fn struct_ptr(&self) -> PointerValue<'ctx> {
    self.struct_ptr
  }
}