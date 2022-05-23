use crate::parser::Expression;

use super::{Environment, AnyType, function, Function};

pub fn fn_main<'ctx>(env: &mut Environment<'ctx>, expressions: &[Expression]) -> Option<()> {
  let name = "main";

  // Types
  let void_type = env.context.void_type();

  // Create the main function (fn main() -> null)
  let fn_type = void_type.fn_type(&[], false);
  let function = env.module.add_function(name, fn_type, None);
  
  // Create blocks
  let entry_block = env.context.append_basic_block(function, "entry");
  env.set_current_block(entry_block);
  
  // Get print function
  let print = env.module.get_function("print")?;

  // Create the function object
  let function = Function::new(
    print,
    Some("print".to_owned()),
    vec![("value".into(), AnyType::Integer)],
    AnyType::Null
  );

  // Add print function to the environment
  env.add_function("print".into(), function);
  
  // --- Entry block ---
  env.builder.position_at_end(entry_block);

  // Compile the AST
  for expr in expressions {
    expr.codegen(env);
  }

  // Return
  env.builder.build_return(None);
  Some(())
}