use crate::{parser::Expression, nscript::{AnyValue, AnyType, Environment}};

pub fn var<'ctx>(env: &mut Environment<'ctx>, name: &String, type_: &Option<String>, value: Option<&Expression>) -> AnyValue<'ctx> {
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

  let value = value.as_ref().unwrap().codegen(env).deref(env);

  // If value is a pointer to a primitive type, dereference it
  let value = if value.is_primitive_ptr() {
    value.deref(env)
  } else {
    value
  };

  // TODO: Check if type is compatible with value
  let (ptr, type_) = match value {
    AnyValue::Integer(val) => {
      let ptr = value.allocate(env);
      env.builder.build_store(ptr, val);
      (ptr, AnyType::Integer)
    },
    AnyValue::Number(val) => {
      let ptr = value.allocate(env);
      env.builder.build_store(ptr, val);
      (ptr, AnyType::Number)
    },
    AnyValue::Boolean(val) => {
      let ptr = value.allocate(env);
      env.builder.build_store(ptr, val);
      (ptr, AnyType::Boolean)
    },
    AnyValue::Object(object) => {
      // let value = env.builder.build_load(object.struct_ptr(), object.class().name_or_default()).into_struct_value();
      (object.struct_ptr(), AnyType::Object(object.class()))
    },
    _ => panic!("Parser error: invalid type `{type_:?}`, value: `{value}`")
  };

  env.add_variable(name.into(), ptr, type_)
    .expect(format!("Variable `{name}` already exists").as_str())
}