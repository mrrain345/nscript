use crate::{parser::expressions::{Expression, Type}, nscript::{any_value::AnyValue, environment::Environment}};

pub fn let_<'ctx>(env: &mut Environment<'ctx>, name: &String, type_: &Option<Type>, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env);

  // TODO: Check if type is compatible with value
  env.state.add_label(name.into(), value)
    .expect(format!("Label `{name}` already exists").as_str())
}