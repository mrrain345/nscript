use crate::{nscript::{AnyValue, Environment, PropertyValue, Object}};

pub fn object<'ctx>(env: &mut Environment<'ctx>, class_name: &String, properties: &[PropertyValue]) -> AnyValue<'ctx> {
  // Get the object's Class
  let class = env.get_class(class_name).expect("Class not found").into_class();

  // TODO: Check if name and type is compatible with value
  // Set the object's properties
  let mut props = vec![];
  for property in class.properties().iter() {
    let property_name = property.name.as_str();
    let prop = properties.iter().find(|p| p.name == property_name)
      .expect(format!("Property `{property_name}` not found in class {class_name}").as_str());

    let value = prop.value.codegen(env);
    props.push(value);
  }

  // Create the object
  let object = Object::new(env, class, props);

  // Return the object
  AnyValue::Object(Box::new(object))
}