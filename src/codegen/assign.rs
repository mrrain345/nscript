use crate::{parser::expressions::Expression, nscript::{any_value::AnyValue, environment::Environment}};

pub fn assign<'ctx>(env: &mut Environment<'ctx>, name: &String, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env);

  if value.is_none() {
    panic!("Parser error: invalid expression");
  }

  let value = env.state.set_variable(name.into(), value.unwrap());
  
  if value.is_none() {
    panic!("Parser error: variable `{}` not found", name);
  }

  value
}