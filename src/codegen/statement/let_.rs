use crate::{parser::Expression, nscript::{AnyValue, Environment, AnyType}};

pub fn let_<'ctx>(env: &Environment<'ctx>, name: &String, type_: &Option<String>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env);

  // Check if the type is compatible with the value
  let value = if let Some(type_) = type_ {
    let type_ = AnyType::from_string(env, type_).unwrap();
    value.silent_cast(env, &type_).expect("Invalid type")
  } else {
    value
  };

  // TODO: Check if type is compatible with value
  env.add_label(name.into(), value)
    .expect(format!("Label `{name}` already exists").as_str())
}