use inkwell::values::BasicValueEnum;

use crate::{parser::expressions::Expression, nscript::{any_value::AnyValue, environment::Environment}};

pub fn call<'ctx>(env: &mut Environment<'ctx>, name: &String, args: &[Expression]) -> AnyValue<'ctx> {
  let mut fn_args = Vec::new();
  for arg in args {
    let arg = arg.codegen(env);

    match arg {
      AnyValue::Integer(value) => fn_args.push(value.into()),
      AnyValue::Number(value) => fn_args.push(value.into()),
      AnyValue::Boolean(value) => fn_args.push(value.into()),
      AnyValue::Null => fn_args.push(env.context.i32_type().const_int(0, false).into()),
      _ => panic!("Parser error: invalid argument")
    }
  }

  if let Some(function) = env.state.get_function(&name) {
    let res = env.builder.build_call(function.clone(), fn_args.as_slice(), &name);
    let res = res.try_as_basic_value();
    // TODO: get correct return type
    if let Some(_) = res.right() {
      AnyValue::Null
    } else {
      match res.left().unwrap() {
        BasicValueEnum::IntValue(value) => AnyValue::Integer(value),
        BasicValueEnum::FloatValue(value) => AnyValue::Number(value),
        _ => panic!("Parser error: invalid return type")
      }
    }
  }
  else {
    panic!("Parser error: function not found");
  }
}