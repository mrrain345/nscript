use crate::{parser::expressions::{Expression, Type}, nscript::{any_value::AnyValue, environment::Environment}};

pub fn let_<'ctx>(env: &mut Environment<'ctx>, name: &String, type_: &Option<Type>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env);

  if value.is_none() {
    panic!("Parser error: invalid expression");
  }

  let value = value.unwrap();
  // TODO: Check if type is compatible with value
  let value = env.state.add_label(name.into(), value).into_option()
    .expect(format!("Label `{}` already exists", name).as_str());

  Some(value).into()
}