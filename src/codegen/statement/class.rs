use crate::{nscript::{AnyValue, Environment, Property, Class}};

pub fn class<'ctx>(env: &mut Environment<'ctx>, name: &String, properties: &[Property]) -> AnyValue<'ctx> {
  let class = Class::new(
    env,
    Some(name.clone()),
    properties.to_vec()
  );

  env.add_class(name.into(), class)
    .expect("Class already exists")
}