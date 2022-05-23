use crate::{parser::Expression, nscript::{AnyValue, Environment}};

pub fn call<'ctx>(env: &mut Environment<'ctx>, name: &String, args: &[Expression]) -> AnyValue<'ctx> {
  // Get the function
  let function = env.get_function(name)
    .expect(format!("Parser error: Function {name} does not exist").as_str());

  // Evaluate the arguments
  let args = args.iter().map(|expr| expr.codegen(env)).collect::<Vec<_>>();

  // Call the function
  function.call(env, &args)
}