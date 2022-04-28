use crate::{parser::expressions::Expression, nscript::{any_value::{AnyValue, AnyType}, environment::Environment}};

pub fn assign<'ctx>(env: &mut Environment<'ctx>, name: &String, value: &Expression) -> AnyValue<'ctx> {
  let value = value.codegen(env);

  let (ptr, type_) = match env.state.get_variable(name) {
    Some(AnyValue::Ptr{ ptr, type_ }) => (ptr, type_),
    _ => panic!("Parser error: variable `{name}` not found"),
  };

  match value {
    AnyValue::Integer(value) => {
      env.builder.build_store(ptr, value);
    },
    AnyValue::Number(value) => {
      env.builder.build_store(ptr, value);
    },
    AnyValue::Boolean(value) => {
      env.builder.build_store(ptr, value);
    },
    _ => panic!("Parser error: invalid type `{value:?}`")
  }

  env.state.set_variable(name.into(), ptr, type_)
    .expect(format!("Variable `{name}` doesn't exist").as_str())
}