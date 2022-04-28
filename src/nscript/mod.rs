use inkwell::{execution_engine::JitFunction, OptimizationLevel};

use crate::parser::expressions::Expression;

use self::{environment::Environment, fn_print::fn_print, fn_main::fn_main};

pub mod fn_print;
pub mod fn_main;

pub mod any_value;
pub mod state;
pub mod environment;


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
    execution_engine.get_function("main").expect("Failed to load `main` function")
  }
}