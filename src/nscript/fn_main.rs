use crate::parser::Expression;

use super::{Environment};

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