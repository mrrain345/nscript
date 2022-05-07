use crate::{nscript::{Environment, AnyValue}, parser::expressions::Expression};

pub fn if_<'ctx>(env: &mut Environment<'ctx>, condition: &Expression, then: &[Expression], else_: &[Expression]) -> AnyValue<'ctx> {
  let condition = condition.codegen(env);

  if !condition.is_boolean() && !condition.is_null() {
    panic!("Parser error: Condition of if statement must be a boolean or null");
  }
  
  // Create the then block
  let function_block = env.state.current_block.unwrap();
  let then_block = env.context.insert_basic_block_after(function_block, "then");

  // Create the else block
  let else_block = if else_.len() != 0 {
    Some(env.context.insert_basic_block_after(then_block, "else"))
  } else {
    None
  };

  // Create the merge block
  let merge_block = env.context.insert_basic_block_after(
    else_block.unwrap_or(then_block),
    "merge",
  );
  
  // Create the if statement
  env.builder.build_conditional_branch(condition.into_boolean(), then_block, else_block.unwrap_or(merge_block));

  // Emit the then block
  env.state.push_scope();
  env.builder.position_at_end(then_block);
  then.iter().for_each(|expr| { expr.codegen(env); });
  env.builder.build_unconditional_branch(merge_block);
  env.state.pop_scope();

  // Emit the else block
  if let Some(else_block) = else_block {
    env.state.push_scope();
    env.builder.position_at_end(else_block);
    else_.iter().for_each(|expr| { expr.codegen(env); });
    env.builder.build_unconditional_branch(merge_block);
    env.state.pop_scope();
  }

  // Go to the merge block
  env.builder.position_at_end(merge_block);

  AnyValue::Null
}