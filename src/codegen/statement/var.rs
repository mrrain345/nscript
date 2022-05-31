use crate::{parser::Expression, nscript::{AnyValue, AnyType, Environment}};

pub fn var<'ctx>(env: &mut Environment<'ctx>, name: &String, type_: &Option<String>, value: Option<&Expression>) -> AnyValue<'ctx> {
  let value = value.as_ref().unwrap().codegen(env).deref(env);

  // If value is a pointer to a primitive type, dereference it
  let value = if value.is_primitive_ptr() {
    value.deref(env)
  } else {
    value
  };

  // Check if the type is compatible with the value
  let value = if let Some(type_) = type_ {
    let type_ = AnyType::from_string(env, type_).unwrap();
    value.silent_cast(env, &type_).expect("Invalid type")
  } else {
    value
  };

  // Allocate and store the value
  let ptr = value.allocate(env);
  value.store(env, ptr);

  // let (ptr, type_) = match value {
  //   AnyValue::Integer(val) => {
  //     let ptr = value.allocate(env);
  //     env.builder.build_store(ptr, val);
  //     (ptr, AnyType::Integer)
  //   },
  //   AnyValue::Number(val) => {
  //     let ptr = value.allocate(env);
  //     env.builder.build_store(ptr, val);
  //     (ptr, AnyType::Number)
  //   },
  //   AnyValue::Boolean(val) => {
  //     let ptr = value.allocate(env);
  //     env.builder.build_store(ptr, val);
  //     (ptr, AnyType::Boolean)
  //   },
  //   AnyValue::Object(object) => {
  //     // let value = env.builder.build_load(object.struct_ptr(), object.class().name_or_default()).into_struct_value();
  //     (object.struct_ptr(), AnyType::Object(object.class()))
  //   },
  //   _ => panic!("Parser error: invalid type `{type_:?}`, value: `{value}`")
  // };

  env.add_variable(name.into(), ptr, value.get_type())
    .expect(format!("Variable `{name}` already exists").as_str())
}