use crate::nscript::AnyType;
use crate::parser;

use crate::{nscript::{AnyValue, Environment, Class, Property}};

pub fn class<'ctx>(env: &mut Environment<'ctx>, name: &String, properties: &[parser::Property]) -> AnyValue<'ctx> {
  
  let props = properties.iter().map(|prop| {
    Property {
      name: prop.name.clone(),
      type_: AnyType::from_string(env, prop.type_.as_str()).expect(format!("Failed to get type `{:?}`", prop.type_).as_str()),
      modifiers: prop.modifiers.clone(),
    }
  }).collect();
  
  let class = Class::new(
    env,
    Some(name.clone()),
    props,
  );

  env.add_class(name.into(), class)
    .expect("Class already exists")
}