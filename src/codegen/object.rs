use crate::{nscript::{AnyValue, Environment, Object, AnyType}, parser::PropertyValue};

pub fn object<'ctx>(env: &Environment<'ctx>, class_name: &str, properties: &[PropertyValue]) -> AnyValue<'ctx> {
  // Get the object's Class
  let class = env.get_class(class_name).expect("Class not found").into_class();

  // Set the object's properties
  let mut props = Vec::with_capacity(properties.len());
  
  for property in class.properties().iter() {
    let property_name = property.name.as_str();
    let prop = properties.iter().find(|p| p.name == property_name)
      .expect(format!("Property `{property_name}` not found in class {class_name}").as_str());

    let value = prop.value.codegen(env);
    
    let type_ = &prop.type_;
    let type_ = if let Some(type_) = type_ {
      AnyType::from_string(env, &type_).unwrap()
    } else {
      value.get_type()
    };

    let value = value.silent_cast(env, &type_).unwrap();
    props.push(value);
  }

  // Create the object
  let object = Object::new(env, class, props);

  // Return the object
  AnyValue::Object(Box::new(object))
}