use inkwell::types::AnyTypeEnum;

use crate::{parser::Expression, nscript::{AnyValue, Environment, AnyType, values::Function}};

pub fn fn_<'ctx>(env: &Environment<'ctx>, name: &String, args: &[(String, String)], return_type: &String, body: &[Expression]) -> AnyValue<'ctx> {
  // Get the return type
  let return_type = AnyType::from_string(env, return_type).unwrap();
  let args = args.iter().map(|(name, type_)| (name.to_owned(), AnyType::from_string(env, type_).unwrap()) ).collect::<Vec<_>>();
  
  // Get the parameters types
  let arg_types = args.iter().map(|(_, type_)| {
    type_.llvm_basic_type(env)
      .expect(format!("Parser error: Invalid parameter type `{type_:?}`").as_str()).into()
  }).collect::<Vec<_>>();

  // Create the function signature
  let fn_type = match return_type.llvm_type(env) {
    AnyTypeEnum::ArrayType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::FloatType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::IntType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::PointerType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::StructType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::VectorType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::VoidType(type_) => type_.fn_type(&arg_types[..], false),
    AnyTypeEnum::FunctionType(_) => unreachable!("Parser error: Function type not allowed as the return type"),
  };
  
  let previous_block = env.current_block();

  let function = {
    let function = {
      let mut env = env.borrow_mut();
      
      // Create the function
      let function_value = env.module.add_function(name, fn_type, None);
      let function_block = env.context.append_basic_block(function_value, "entry");
      env.state.set_current_block(function_block);
      let function = Function::new(function_value, Some(name.clone()), args.clone(), return_type.clone());

      // Add the function to the environment
      let function = env.state.add(name.clone(), function.into())
        .expect("Parser error: Function already exists");

      // Create the function scope
      env.state.push_scope();
      env.builder.position_at_end(function_block);
      
      function.into_function().unwrap()
    };

    // Add the parameters to the environment
    function.function_value().get_params()
      .iter()
      .zip(args.iter())
      .for_each(|(param, (name, type_))| {
        env.add(name.clone(), type_.create_value(env, (*param).into()));
      });

    function
  };

  // Compile the function body
  for expr in body {
    expr.codegen(env);
  }
  
  {
    let mut env = env.borrow_mut();

    // Return null if the function is a void function
    if return_type.is_null() {
      env.builder.build_return(None);
    }

    // Finish the function
    env.state.pop_scope();
    env.state.set_current_block(previous_block);
    env.builder.position_at_end(previous_block);
  }

  // Return the function
  AnyValue::Function(function)
}