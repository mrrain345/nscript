use std::{sync::RwLock, rc::Rc};

use inkwell::values::{StructValue, PointerValue};

use super::{Class, AnyValue, Environment, AnyType};

#[derive(Debug)]
pub struct Object<'ctx> {
  class: &'ctx Class<'ctx>,
  struct_ptr: PointerValue<'ctx>,
  properties: Rc<RwLock<Vec<AnyValue<'ctx>>>>,
}

impl<'ctx> Object<'ctx> {
  pub fn new(env: &mut Environment<'ctx>, class: &'ctx Class<'ctx>, properties: Vec<AnyValue<'ctx>>) -> Self {
    
    // Create the struct
    let struct_ptr = env.builder.build_malloc(class.struct_type(), class.name_or_default()).unwrap();

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
        AnyValue::Object(ref object) => {
          let value = env.builder.build_load(object.struct_ptr(), property_name);
          env.builder.build_store(ptr, value)
        },
        _ => panic!("Parser error: invalid type of property `{property_name:?}` in class `{name:?}`", name=class.name_or_default()),
      };
    }

    // Return the object
    let obj = Object {
      class,
      struct_ptr,
      properties: Rc::new(RwLock::new(properties)),
    };

    env.gc.add(obj.clone());

    obj
  }

  // pub fn from_struct_value(env: &mut Environment<'ctx>, class: &'ctx Class<'ctx>, struct_value: StructValue<'ctx>) -> Self {
    
  //   let properties = class.properties()
  //     .iter()
  //     .enumerate()
  //     .map(|(index, property)| {
  //       let property_name = class.get_property(index).name.as_ref();
  //       let property_value = env.builder.build_extract_value(struct_value, index as u32, property_name).unwrap();

  //       match property.type_ {
  //         AnyType::Integer => AnyValue::Integer(property_value.into_int_value()),
  //         AnyType::Number => AnyValue::Number(property_value.into_float_value()),
  //         AnyType::Boolean => AnyValue::Boolean(property_value.into_int_value()),
  //         _ => panic!("Parser error: invalid type of property `{property_name:?}` in class `{name:?}`", name=class.name_or_default()),
  //       }
  //     })
  //     .collect();

  //   Object::new(env, class, properties)
  // }

  pub fn class(&self) -> &'ctx Class<'ctx> {
    self.class
  }

  pub fn set_property(&mut self, name: &str, value: AnyValue<'ctx>) {
    let index = self.class.position(name);
    index.map(|index| {
      self.properties.write().unwrap()[index] = value;
    });
  }

  pub fn get_property(&self, name: &str) -> Option<AnyValue<'ctx>> {
    let index = self.class.position(name);
    index.map(|index| self.properties.read().unwrap()[index].clone())
  }

  pub fn struct_ptr(&self) -> PointerValue<'ctx> {
    self.struct_ptr
  }
}

impl<'ctx> Clone for Object<'ctx> {
  fn clone(&self) -> Self {
    Object {
      class: self.class,
      struct_ptr: self.struct_ptr,
      properties: Rc::clone(&self.properties),
    }
  }
}