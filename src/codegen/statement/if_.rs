use crate::{nscript::{Environment, AnyValue}, parser::Expression};

pub fn if_<'ctx>(env: &Environment<'ctx>, condition: &Expression, then: &[Expression], else_: &[Expression]) -> AnyValue<'ctx> {
  let condition = condition.codegen(env);

  if !condition.is_boolean() && !condition.is_null() {
    panic!("Parser error: Condition must be a boolean or null");
  }
  
  // Create the then block
  let function_block = env.current_block();
  let then_block = env.borrow_mut().context.insert_basic_block_after(function_block, "then");

  // Create the else block
  let else_block = if else_.len() != 0 {
    Some(env.borrow_mut().context.insert_basic_block_after(then_block, "else"))
  } else {
    None
  };

  // Create the merge block
  let merge_block = env.borrow_mut().context.insert_basic_block_after(
    else_block.unwrap_or(then_block),
    "merge",
  );
  
  {
    let mut env = env.borrow_mut();
    // Create the if statement
    env.builder.build_conditional_branch(condition.into_boolean().unwrap().value, then_block, else_block.unwrap_or(merge_block));

    // Emit the then block
    env.state.push_scope();
    env.builder.position_at_end(then_block);
  }

  for expr in then {
    expr.codegen(env);
  }

  {
    let mut env = env.borrow_mut();
    env.builder.build_unconditional_branch(merge_block);
    env.state.pop_scope();
  }

  // Emit the else block
  if let Some(else_block) = else_block {
    {
      let mut env = env.borrow_mut();
      env.state.push_scope();
      env.builder.position_at_end(else_block);
    }

    for expr in else_ {
      expr.codegen(env);
    }

    {
      let mut env = env.borrow_mut();
      env.builder.build_unconditional_branch(merge_block);
      env.state.pop_scope();
    }
  }

  // Go to the merge block
  env.borrow_mut().builder.position_at_end(merge_block);

  AnyValue::Null
}