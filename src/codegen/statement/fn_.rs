use inkwell::types::AnyTypeEnum;

use crate::{parser::Expression, nscript::{AnyValue, Environment, Type, Function}};

pub fn fn_<'ctx>(env: &mut Environment<'ctx>, name: &String, args: &[(String, Type)], return_type: &Type, body: &[Expression]) -> AnyValue<'ctx> {
  // Get the return type
  let return_type = return_type.into_type(env).unwrap();
  let args = args.iter().map(|(name, type_)| (name.to_owned(), type_.into_type(env).unwrap()) ).collect::<Vec<_>>();

  print!("fn {}(", name);
  for (i, (name, type_)) in args.iter().enumerate() {
    if i > 0 { print!(", "); }
    print!("{}: {}", name, type_);
  }
  println!(") -> {}\n", return_type);
  
  // Get the parameters types
  let arg_types = args.iter().map(|(_, type_)| {
    type_.into_llvm_basic_type(env)
      .expect(format!("Parser error: Invalid parameter type `{type_:?}`").as_str()).into()
  }).collect::<Vec<_>>();

  // Create the function signature
  let fn_type = match return_type.into_llvm_type(env) {
    AnyTypeEnum::ArrayType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::FloatType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::IntType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::PointerType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::StructType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::VectorType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::VoidType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::FunctionType(_) => unreachable!("Parser error: Function type not allowed as the return type"),
  };

  // Create the function
  let previous_block = env.current_block();
  let function_value = env.module.add_function(name, fn_type, None);
  let function_block = env.context.append_basic_block(function_value, "entry");
  env.set_current_block(function_block);
  let function = Function::new(function_value, Some(name.clone()), args.clone(), return_type);

  // Add the function to the environment
  let fn_ = env.add_function(name.clone(), function)
    .expect("Parser error: Function already exists");

  // Create the function scope
  env.state.push_scope();
  env.builder.position_at_end(function_block);

  // Add the parameters to the environment
  function_value.get_params()
    .iter()
    .zip(args.iter())
    .for_each(|(param, (name, type_))| {
      env.add_label(name.clone(), AnyValue::from_basic_value(type_, *param));
    });

  // Compile the function body
  for expr in body {
    expr.codegen(env);
  }
  
  // Return null if the function is a void function
  if return_type.is_null() {
    env.builder.build_return(None);
  }

  // Finish the function
  env.state.pop_scope();
  env.set_current_block(previous_block);
  env.builder.position_at_end(previous_block);

  // Return the function
  AnyValue::Fn(fn_)
}