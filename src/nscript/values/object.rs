use inkwell::values::PointerValue;

use crate::nscript::{Environment, AnyType, types::ObjectType};

use super::{value::Value, Class, AnyValue};

#[derive(Debug, Clone)]
pub struct Object<'ctx> {
  class: Class<'ctx>,
  struct_ptr: PointerValue<'ctx>,
}

impl<'ctx> Object<'ctx> {
  pub fn new(env: &Environment<'ctx>, class: &'ctx Class<'ctx>, properties: Vec<AnyValue<'ctx>>) -> Self {
    
    // Create the struct
    let struct_ptr = env.borrow_mut().builder.build_malloc(class.struct_type(), class.name_or_default()).unwrap();

    // Set the struct's properties
    for (index, property) in properties.iter().enumerate() {

      // Get the property's name and pointer
      let property_name = class.get_property(index).name.as_ref();
      let ptr = env.borrow_mut().builder.build_struct_gep(struct_ptr, index as u32, property_name).unwrap();

      // Set the property's value
      property.store(env, ptr);
    }

    // Return the object
    let obj = Object {
      class: class.clone(),
      struct_ptr,
    };

    env.borrow_mut().gc.add(obj.clone());

    obj
  }

  pub fn class(&self) -> Class<'ctx> {
    self.class.clone()
  }

  pub fn set_property(&mut self, env: &Environment<'ctx>, name: &str, value: AnyValue<'ctx>) {
    // get property index
    let index = self.class.position(name).unwrap_or_else(|| panic!("Property {name} doesn't exist in object {object}", object=self.class.name_or_default()));

    // get property
    let propPtr = env.borrow_mut().builder.build_struct_gep(self.struct_ptr, index as u32, name).unwrap();
    let propType = self.class.get_property(index).type_;

    // set property
    let propValue = value.silent_cast(env, &propType).unwrap_or_else(|| panic!("Value {value} is not castable to {propType}"));
    propValue.store(env, propPtr);
  }

  pub fn get_property(&self, env: &Environment<'ctx>, name: &str) -> AnyValue<'ctx> {
    // get property index
    let index = self.class.position(name).unwrap_or_else(|| panic!("Property {name} doesn't exist in object {object}", object=self.class.name_or_default()));

    // get property
    let propPtr = env.borrow_mut().builder.build_struct_gep(self.struct_ptr, index as u32, name).unwrap();
    let propType = self.class.get_property(index).type_;
    
    let prop = env.borrow_mut().builder.build_load(propPtr, name);
    propType.create_value(env, prop.into())
  }

  pub fn struct_ptr(&self) -> PointerValue<'ctx> {
    self.struct_ptr
  }
}

impl<'ctx> Value<'ctx> for Object<'ctx> {
  type Type = ObjectType<'ctx>;
  type LLVMValue = PointerValue<'ctx>;

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    env.borrow_mut().builder.build_alloca(self.struct_ptr.get_type(), "Object")
  }

  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {
    env.borrow_mut().builder.build_store(ptr, self.struct_ptr);
  }

  fn get_type(&self) -> AnyType<'ctx> {
    AnyType::Object(ObjectType{ class: self.class.clone() })
  }

  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue {
    self.struct_ptr
  }

  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<inkwell::values::BasicValueEnum<'ctx>> {
    Some(self.struct_ptr.into())
  }
}

impl<'ctx> Into<AnyValue<'ctx>> for Object<'ctx> {
  fn into(self) -> AnyValue<'ctx> {
    AnyValue::Object(self)
  }
}

impl<'ctx> From<AnyValue<'ctx>> for Object<'ctx> {
  fn from(value: AnyValue<'ctx>) -> Self {
    match value {
      AnyValue::Object(value) => value,
      _ => panic!("Invalid type"),
    }
  }
}