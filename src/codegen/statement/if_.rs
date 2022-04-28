use crate::{nscript::{environment::Environment, any_value::AnyValue}, parser::expressions::Expression};

pub fn if_<'ctx>(env: &mut Environment<'ctx>, condition: &Expression, then: &[Expression], else_: &[Expression]) -> AnyValue<'ctx> {
  let condition = condition.codegen(env);

  if !condition.is_boolean() && !condition.is_null() {
    panic!("Parser error: Condition of if statement must be a boolean or null");
  }
  
  // Create blocks for the then, else and merge
  let function = env.state.current_function.unwrap();
  let then_block = env.context.append_basic_block(function, "then");

  let else_block = if else_.len() != 0 {
    Some(env.context.append_basic_block(function, "else"))
  } else {
    None
  };

  let merge_block = env.context.append_basic_block(function, "merge");
  
  // Create the if statement
  env.builder.build_conditional_branch(condition.into_boolean(), then_block, else_block.unwrap_or(merge_block));

  // Emit the then block
  env.builder.position_at_end(then_block);
  then.iter().for_each(|expr| { expr.codegen(env); });
  env.builder.build_unconditional_branch(merge_block);

  // Emit the else block
  if let Some(else_block) = else_block {
    env.builder.position_at_end(else_block);
    else_.iter().for_each(|expr| { expr.codegen(env); });
    env.builder.build_unconditional_branch(merge_block);
  }

  // Go to the merge block
  env.builder.position_at_end(merge_block);

  AnyValue::Null
}