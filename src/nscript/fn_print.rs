use inkwell::{AddressSpace, module::Linkage};

use super::{environment::Environment, AnyType, values::Function};

pub fn fn_print<'ctx>(env: &Environment<'ctx>) -> Option<()> {
  let (print_int, print_num, print_bool) = {
    let mut env = env.borrow_mut();

    // Types
    let i8_type = env.context.i8_type();
    let i32_type = env.context.i32_type();
    let f64_type = env.context.f64_type();
    let bool_type = env.context.bool_type();
    let void_type = env.context.void_type();

    // Declare the printf function (fn printf(cstr: i8*, ...) -> i32)
    let printf_type = i32_type.fn_type(&[i8_type.ptr_type(AddressSpace::Generic) .into()], true);
    let printf =  env.module.add_function("printf", printf_type, Some(Linkage::AvailableExternally));


    // Create printInt function
    let fn_type = void_type.fn_type(&[i32_type.into()], false);
    let print_int = env.module.add_function("printInt", fn_type, None);

    // Create printNum function
    let fn_type = void_type.fn_type(&[f64_type.into()], false);
    let print_num = env.module.add_function("printNum", fn_type, None);

    // Create printBool function
    let fn_type = void_type.fn_type(&[bool_type.into()], false);
    let print_bool = env.module.add_function("printBool", fn_type, None);


    // Create the printInt function
    let entry_block = env.context.append_basic_block(print_int, "entry");
    env.builder.position_at_end(entry_block);
    let format = env.builder.build_global_string_ptr("%d\n", "format");
    let arg = print_int.get_nth_param(0)?.into_int_value();
    let args = [format.as_pointer_value().into(), arg.into()];
    env.builder.build_call(printf, &args, "printf");
    env.builder.build_return(None);

    // Create the printNum function
    let entry_block = env.context.append_basic_block(print_num, "entry");
    env.builder.position_at_end(entry_block);
    let format = env.builder.build_global_string_ptr("%f\n", "format");
    let arg = print_num.get_nth_param(0)?.into_float_value();
    let args = [format.as_pointer_value().into(), arg.into()];
    env.builder.build_call(printf, &args, "printf");
    env.builder.build_return(None);

    // Create the printBool function
    let entry_block = env.context.append_basic_block(print_bool, "entry");
    let true_block = env.context.append_basic_block(print_bool, "true");
    let false_block = env.context.append_basic_block(print_bool, "false");
    env.builder.position_at_end(entry_block);
    let str_true = env.builder.build_global_string_ptr("true\n", "true");
    let str_false = env.builder.build_global_string_ptr("false\n", "false");
    let arg = print_bool.get_nth_param(0)?.into_int_value();
    env.builder.build_conditional_branch(arg, true_block, false_block);

    env.builder.position_at_end(true_block);
    let args = [str_true.as_pointer_value().into()];
    env.builder.build_call(printf, &args, "printf");
    env.builder.build_return(None);

    env.builder.position_at_end(false_block);
    let args = [str_false.as_pointer_value().into()];
    env.builder.build_call(printf, &args, "printf");
    env.builder.build_return(None);
    
    // Return functions out of lock scope
    (print_int, print_num, print_bool)
  };

  // Create function objects
  let print_int = Function::new(print_int, Some("printInt".to_owned()), vec![("value".into(), AnyType::Integer)], AnyType::Null);
  let print_num = Function::new(print_num, Some("printNum".to_owned()), vec![("value".into(), AnyType::Number)], AnyType::Null);
  let print_bool = Function::new(print_bool, Some("printBool".to_owned()), vec![("value".into(), AnyType::Boolean)], AnyType::Null);
  
  // Add print function to the environment
  env.add_function("print".into(), print_int.clone());
  env.add_function("printInt".into(), print_int);
  env.add_function("printNum".into(), print_num);
  env.add_function("printBool".into(), print_bool);

  Some(())
}