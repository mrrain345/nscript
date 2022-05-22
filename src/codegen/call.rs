use inkwell::values::BasicValueEnum;

use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn call<'ctx>(env: &mut Environment<'ctx>, name: &String, args: &[Expression]) -> AnyValue<'ctx> {
  // Get the function
  let function = env.get_function(name);
  if function.is_none() {
    panic!("Parser error: Function {name} does not exist");
  }
  let function = function.unwrap();

  // Get the function info
  let (function, arguments) = if let AnyValue::Fn { fn_, name: _, args } = function {
    (fn_, args)
  } else {
    panic!("Parser error: {name} is not a function");
  };

  // Check if the number of arguments is correct
  if args.len() != arguments.len() {
    panic!("Parser error: {} takes {} arguments, but {} were given", name, arguments.len(), args.len());
  }

  // Get the function arguments
  let mut fn_args = Vec::new();

  for (arg, (arg_name, type_)) in args.iter().zip(arguments.iter()) {
    let arg = arg.codegen(env);
    let type_ = &type_.0;

    // If argument is a pointer, get the value
    let arg = if arg.is_ptr() {
      let (ptr, type_) = arg.into_ptr();
      AnyValue::from_basic_value(type_, env.builder.build_load(ptr, arg_name))
    } else {
      arg
    };

    match arg {
      AnyValue::Integer(value) if type_ == "Integer" => fn_args.push(value.into()),
      AnyValue::Number(value) if type_ == "Number" => fn_args.push(value.into()),
      AnyValue::Boolean(value) if type_ == "Boolean" => fn_args.push(value.into()),
      AnyValue::Null if type_ == "null" => fn_args.push(env.context.i32_type().const_int(0, false).into()),
      _ => panic!("Parser error: invalid argument `{arg:?}`, type: {type_}")
    }
  }

  let res = env.builder.build_call(function, fn_args.as_slice(), &name);
  let res = res.try_as_basic_value();

  // TODO: get correct return type
  if let Some(_) = res.right() {
    AnyValue::Null
  } else {
    match res.left().unwrap() {
      BasicValueEnum::IntValue(value) if value.get_type().get_bit_width() == 32 => AnyValue::Integer(value),
      BasicValueEnum::IntValue(value) if value.get_type().get_bit_width() == 1 => AnyValue::Boolean(value),
      BasicValueEnum::FloatValue(value) => AnyValue::Number(value),
      _ => panic!("Parser error: invalid return type")
    }
  }
}