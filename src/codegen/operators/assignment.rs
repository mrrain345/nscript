use crate::{nscript::{Environment, AnyValue}, parser::Expression};

pub fn assign<'ctx>(env: &Environment<'ctx>, ptr: &Expression, value: &Expression) -> AnyValue<'ctx> {
  let ptr = ptr.codegen(env);
  let value = value.codegen(env);

  let ref_ = ptr.into_ref().unwrap();

  let value = if value.get_type() != ref_.type_ {
    value.silent_cast(env, &ref_.type_).unwrap()
  } else {
    value
  };
  
  value.store(env, ref_.ptr);

  // match value {
  //   AnyValue::Integer(value) => {
  //     env.builder().build_store(ptr, value);
  //   },
  //   AnyValue::Number(value) => {
  //     env.builder().build_store(ptr, value);
  //   },
  //   AnyValue::Boolean(value) => {
  //     env.builder().build_store(ptr, value);
  //   },
  //   _ => panic!("Parser error: invalid type `{value}`")
  // }

  value

  // env.set_variable(name.into(), ptr, type_)
  //   .expect(format!("Variable `{name}` doesn't exist").as_str())
}