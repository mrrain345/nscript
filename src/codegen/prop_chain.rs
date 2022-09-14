use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn prop_chain<'ctx>(env: &Environment<'ctx>, object: &Expression, chain: &[String]) -> AnyValue<'ctx> {
  // Get the object
  let object = object.codegen(env);

  // If chain is empty, return the object
  if chain.is_empty() { return object; }

  let mut ref_ = object.into_ref().unwrap();

  let env = env.borrow_mut();
  
  for prop_name in chain {
    let class = ref_.type_.clone().into_object().expect("It's not an object").class;
    
    // Get the property
    let position = class.position(prop_name)
      .expect(format!("Parser error: invalid property name `{prop_name}` in object `{type_}`", type_=ref_.type_).as_str());
    let property = class.get_property(position);
    
    // Get the property's pointer
    let obj_ptr = env.builder.build_load(ref_.ptr, prop_name).into_pointer_value();
    ref_.ptr = env.builder.build_struct_gep(obj_ptr, position as u32, prop_name)
      .expect(format!("Parser error: property `{prop_name}` [{position}] doesn't exist in object of type `{type_}`", type_=ref_.type_.clone()).as_str());
    ref_.type_ = property.type_.clone();
  }

  // Return a pointer to the property
  ref_.into()
}