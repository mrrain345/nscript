use crate::nscript::{Environment, Class, AnyType};

#[derive(Debug, Clone, PartialEq)]
pub struct Type(pub String);

impl Type {

  pub fn is_null(&self) -> bool {
    self.0 == "null"
  }

  pub fn is_number(&self) -> bool {
    self.0 == "Number"
  }

  pub fn is_string(&self) -> bool {
    self.0 == "String"
  }

  pub fn is_integer(&self) -> bool {
    self.0 == "Integer"
  }

  pub fn is_boolean(&self) -> bool {
    self.0 == "Boolean"
  }

  pub fn is_class(&self, env: &mut Environment) -> bool {
    env.get_class(&self.0).is_some()
  }

  pub fn into_class<'ctx>(&self, env: &mut Environment<'ctx>) -> Option<&'ctx Class<'ctx>> {
    env.get_class(&self.0).map(|c| c.into_class())
  }

  pub fn into_type<'ctx>(&self, env: &mut Environment<'ctx>) -> Option<AnyType<'ctx>> {
    let type_ = match self.0.as_str() {
      "null" => Some(AnyType::Null),
      "Integer" => Some(AnyType::Integer),
      "Number" => Some(AnyType::Number),
      "String" => Some(AnyType::String),
      "Boolean" => Some(AnyType::Boolean),
      _ => None,
    };

    if type_.is_some() { return type_; }

    if let Some(class) = self.into_class(env) {
      Some(AnyType::Object(class))
    } else {
      None
    }
  }
}