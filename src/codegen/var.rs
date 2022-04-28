use crate::{parser::expressions::{Expression, Type}, nscript::{any_value::AnyValue, environment::Environment}};

pub fn var<'ctx>(env: &mut Environment<'ctx>, name: &String, type_: &Option<Type>, value: &Option<Box<Expression>>) -> AnyValue<'ctx> {
  // if value.is_none() {
  //   if type_.is_none() {
  //     panic!("Parser error: you must specify a type or a value for variable `{}`", name);
  //   }
  //   let value = match type_.as_ref().unwrap().0.as_str() {
  //     "Integer" => env.integer(0).into(),
  //     "Number" => env.number(0.0).into(),
  //     "Boolean" => env.boolean(false).into(),
  //     _ => panic!("Parser error: invalid type `{}`", type_.as_ref().unwrap().0)
  //   };
  //   let value = env.state.add_variable(name.into(), value).into_option()
  //     .expect(format!("Variable `{}` already exists", name).as_str());

  //   return Some(value).into();
  // }

  let value = value.as_ref().unwrap().codegen(env);

  if value.is_none() {
    panic!("Parser error: invalid expression");
  }

  let value = value.unwrap();
  // TODO: Check if type is compatible with value
  env.state.add_variable(name.into(), value);

  Some(value).into()
}