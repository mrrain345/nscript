use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn prop_chain<'ctx>(env: &mut Environment<'ctx>, object: &Expression, chain: &[String]) -> AnyValue<'ctx> {
  // Get the object
  let object = object.codegen(env);

  // If chain is empty, return the object
  if chain.is_empty() { return object; }
  
  // Get the property
  chain.iter().fold(object, |object, property| {

    // Check if object is a pointer
    if !object.is_ptr() {
      panic!("Parser error: object `{object}` is not a pointer");
    }
    
    let (ptr, type_) = object.into_ptr();
    
    // Check if object is an object
    if !type_.is_object() {
      panic!("Property `{property}` has invalid type: `{type_}`");
    }

    let class = type_.into_object();

    // Get property's position
    let prop_positon = class.position(property)
      .expect(format!("Property `{property}` not found").as_str());
    
    // Get property's type
    let prop_type = class.get_property(prop_positon).type_;

    let struct_ptr = env.builder.build_struct_gep(ptr, prop_positon as u32, &property).unwrap();
    
    // If property is an object, dereference a pointer
    if prop_type.is_object() {
      let struct_ptr = env.builder.build_load(struct_ptr, &property).into_pointer_value();
      AnyValue::Ptr { ptr: struct_ptr, type_: prop_type }
    } else {
      AnyValue::Ptr { ptr: struct_ptr, type_: prop_type }
    }
  })
}