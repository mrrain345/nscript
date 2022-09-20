use inkwell::values::FunctionValue;

use super::{environment::Environment, AnyType, values::Function};

pub extern fn print_int(val: i32) {
  println!("{val}");
}

pub extern fn print_num(val: f64) {
  println!("{val}");
}

pub extern fn print_bool(val: bool) {
  if val {
    println!("true");
  } else {
    println!("false");
  }
}

pub fn fn_print<'ctx>(env: &Environment<'ctx>) -> (FunctionValue<'ctx>, FunctionValue<'ctx>, FunctionValue<'ctx>) {

  let (fn_print_int, fn_print_num, fn_print_bool) = {
    let env = env.borrow_mut();

    // printInt
    let fn_type = env.context.void_type().fn_type(&[env.context.i32_type().into()], false);
    let fn_print_int = env.module.add_function("printInt", fn_type, None);

    // printNum
    let fn_type = env.context.void_type().fn_type(&[env.context.f64_type().into()], false);
    let fn_print_num = env.module.add_function("printNum", fn_type, None);

    // printBool
    let fn_type = env.context.void_type().fn_type(&[env.context.bool_type().into()], false);
    let fn_print_bool = env.module.add_function("printBool", fn_type, None);

    (fn_print_int, fn_print_num, fn_print_bool)
  };

  // Create function objects
  let print_int = Function::new(fn_print_int, Some("printInt".into()), vec![("value".into(), AnyType::Integer)], AnyType::Null);
  let print_num = Function::new(fn_print_num, Some("printNum".into()), vec![("value".into(), AnyType::Number)], AnyType::Null);
  let print_bool = Function::new(fn_print_bool, Some("printBool".into()), vec![("value".into(), AnyType::Boolean)], AnyType::Null);
  
  // Add print function to the environment
  env.add("print".into(), print_int.clone().into());
  env.add("printInt".into(), print_int.into());
  env.add("printNum".into(), print_num.into());
  env.add("printBool".into(), print_bool.into());

  (fn_print_int, fn_print_num, fn_print_bool)
}