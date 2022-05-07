use crate::{parser::expressions::{Expression}, nscript::{AnyValue, AnyType, Type, Environment}};

pub fn var<'ctx>(env: &mut Environment<'ctx>, name: &String, type_: &Option<Type>, value: &Option<Box<Expression>>) -> AnyValue<'ctx> {
  // if value.is_none() {
  //   if type_.is_none() {
  //     panic!("Parser error: you must specify a type or a value for variable `{name}`");
  //   }
  //   let value = match type_.as_ref().unwrap().0.as_str() {
  //     "Integer" => env.integer(0).into(),
  //     "Number" => env.number(0.0).into(),
  //     "Boolean" => env.boolean(false).into(),
  //     _ => panic!("Parser error: invalid type `{}`", type_.as_ref().unwrap().0)
  //   };
  //   let value = env.state.add_variable(name.into(), value).into_option()
  //     .expect(format!("Variable `{name}` already exists").as_str());

  //   return Some(value).into();
  // }

  let value = value.as_ref().unwrap().codegen(env);

  // TODO: Check if type is compatible with value
  let (ptr, type_) = match value {
    AnyValue::Integer(value) => {
      let ptr = env.builder.build_alloca(env.context.i32_type(), name.as_str());
      env.builder.build_store(ptr, value);
      (ptr, AnyType::Integer)
    },
    AnyValue::Number(value) => {
      let ptr = env.builder.build_alloca(env.context.f64_type(), name.as_str());
      env.builder.build_store(ptr, value);
      (ptr, AnyType::Number)
    },
    AnyValue::Boolean(value) => {
      let ptr = env.builder.build_alloca(env.context.bool_type(), name.as_str());
      env.builder.build_store(ptr, value);
      (ptr, AnyType::Boolean)
    },
    _ => panic!("Parser error: invalid type `{type_:?}`")
  };

  env.state.add_variable(name.into(), ptr, type_)
    .expect(format!("Variable `{name}` already exists").as_str())
}