use inkwell::values::BasicValueEnum;

use crate::nscript::{AnyValue, AnyType, Environment, StateType, Object};

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
  if let Some((value, type_, ..)) = env.state.get(name, None) {
    // Get variable
    if type_ == StateType::Variable {
      if let AnyValue::Ptr{ ptr, type_} = value {

        // Load value or return pointer
        let res = if !type_.is_object() {
          env.builder.build_load(ptr, "load")
        } else {
          BasicValueEnum::PointerValue(ptr)
        };

        // Return value
        match type_ {
          AnyType::Integer => AnyValue::Integer(res.into_int_value()),
          AnyType::Number => AnyValue::Number(res.into_float_value()),
          AnyType::Boolean => AnyValue::Boolean(res.into_int_value()),
          AnyType::Null => AnyValue::Null,
          AnyType::Object(class) => AnyValue::Ptr { ptr: res.into_pointer_value(), type_: AnyType::Object(class) },
          _ => panic!("Parser error: invalid type `{type_:?}`")
        }
        
      } else { panic!("Parser error: invalid type `{type_:?}`") }
    } else {
      // Get label
      value
    }
  } else { panic!("Parser error: label `{name}` not found") }
}