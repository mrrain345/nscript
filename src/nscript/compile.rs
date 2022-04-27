use inkwell::OptimizationLevel;
use inkwell::execution_engine::JitFunction;

use crate::parser::expressions::Expression;

use super::environment::Environment;
use super::fn_main::fn_main;
use super::fn_print::fn_print;

pub fn compile<'ctx>(env: &mut Environment<'ctx>, expressions: &[Expression]) -> JitFunction<'ctx, unsafe extern "C" fn() -> ()> {

  // Create an execution engine
  let execution_engine = env.module
    .create_jit_execution_engine(OptimizationLevel::None)
    .expect("Failed to create execution engine");

  // Compile functions
  fn_print(env).expect("Failed to compile `print` function");
  fn_main(env, expressions).expect("Failed to compile `main` function");

  // Return the main function
  unsafe {
    execution_engine.get_function("main")
      .expect("Failed to load `main` function")
  }
}