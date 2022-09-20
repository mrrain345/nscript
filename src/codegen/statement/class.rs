use crate::nscript::AnyType;
use crate::nscript::values::{Class, Property};
use crate::parser;

use crate::{nscript::{AnyValue, Environment}};

pub fn class<'ctx>(env: &Environment<'ctx>, name: &String, properties: &[parser::Property]) -> AnyValue<'ctx> {
  
  let mut props = Vec::new();
  for prop in properties {
    let property = Property {
      name: prop.name.clone(),
      type_: AnyType::from_string(env, prop.type_.as_str()).expect(format!("Failed to get type `{:?}`", prop.type_).as_str()),
      modifiers: prop.modifiers.clone(),
    };

    props.push(property);
  }
  
  let class = Class::new(
    env,
    Some(name.clone()),
    props,
  );

  env.add(name.into(), class.into())
    .expect("Class already exists")
}