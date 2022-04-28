use crate::nscript::{any_value::{AnyValue, AnyType}, environment::Environment, state::StateType};

pub fn integer<'ctx>(env: &mut Environment<'ctx>, value: i32) -> AnyValue<'ctx> {
  AnyValue::Integer(env.integer(value))
}

pub fn number<'ctx>(env: &mut Environment<'ctx>, value: f64) -> AnyValue<'ctx> {
  AnyValue::Number(env.number(value))
}

pub fn string<'ctx>(env: &mut Environment<'ctx>, value: &str) -> AnyValue<'ctx> {
  todo!()
}

pub fn boolean<'ctx>(env: &mut Environment<'ctx>, value: bool) -> AnyValue<'ctx> {
  AnyValue::Boolean(env.boolean(value))
}

pub fn null<'ctx>(env: &mut Environment<'ctx>) -> AnyValue<'ctx> {
  AnyValue::Null
}

pub fn identifier<'ctx>(env: &mut Environment<'ctx>, name: &str) -> AnyValue<'ctx> {
  if let Some((value, type_)) = env.state.get(name, None) {
    // Get variable
    if type_ == StateType::Variable {
      if let AnyValue::Ptr{ ptr, type_} = value {
        let res = env.builder.build_load(ptr, "load");

        match type_ {
          AnyType::Integer => AnyValue::Integer(res.into_int_value()),
          AnyType::Number => AnyValue::Number(res.into_float_value()),
          AnyType::Boolean => AnyValue::Boolean(res.into_int_value()),
          _ => panic!("Parser error: invalid type `{type_:?}`")
        }
        
      } else {
        panic!("Parser error: invalid type `{type_:?}`")
      }
    } else {
      // Get label
      value
    }
  } else {
    panic!("Parser error: label `{name}` not found");
  }
}