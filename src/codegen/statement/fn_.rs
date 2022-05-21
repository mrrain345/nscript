use inkwell::types::AnyTypeEnum;

use crate::{parser::expressions::Expression, nscript::{AnyValue, Environment, Type, ParamsList}};

pub fn fn_<'ctx>(env: &mut Environment<'ctx>, name: &String, args: &ParamsList, return_type: &Type, body: &[Expression]) -> AnyValue<'ctx> {
  // Get the return type
  let return_type = return_type.into_llvm_type(env)
    .expect("Parser error: Invalid return type");
  
  // Get the parameters types
  let params = args.0.iter().map(|(_, type_)| {
    type_.into_llvm_basic_metadata_type(env)
      .expect("Parser error: Invalid parameter type")
  }).collect::<Vec<_>>();

  // Create the function signature
  let fn_type = match return_type {
    AnyTypeEnum::VoidType(type_) => type_.fn_type(&params[..], false),
    AnyTypeEnum::IntType(type_) => type_.fn_type(&params[..], false),
    AnyTypeEnum::FloatType(type_) => type_.fn_type(&params[..], false),
    _ => panic!("Parser error: Invalid return type"),
  };

  // Create the function
  let previous_block = env.current_block();
  let function = env.module.add_function(name, fn_type, None);
  let function_block = env.context.append_basic_block(function, "entry");
  env.set_current_block(function_block);

  // Add the function to the environment
  let fn_ = env.add_function(name.to_string(), function, args.0.clone())
    .expect("Parser error: Function already exists");

  // Create the function scope
  env.state.push_scope();
  env.builder.position_at_end(function_block);

  // Add the parameters to the environment
  function.get_params()
    .iter()
    .zip(args.0.iter())
    .for_each(|(param, (name, type_))| {
      env.add_label(name.to_string(), (*param).into());
    });

  // Compile the function body
  for expr in body {
    expr.codegen(env);
  }
  
  // Return null if the function is a void function
  if let AnyTypeEnum::VoidType(_) = return_type {
    env.builder.build_return(None);
  }

  // Finish the function
  env.state.pop_scope();
  env.set_current_block(previous_block);
  env.builder.position_at_end(previous_block);

  // Return the function
  fn_
}