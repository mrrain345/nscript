use inkwell::{execution_engine::JitFunction, OptimizationLevel};

use crate::parser::Expression;

pub mod types;
pub mod values;

pub use types::Type;

mod fn_print;
mod fn_main;

mod any_type;
mod state;
mod environment;
mod gc;
mod operator;

pub use types::AnyType;
pub use values::AnyValue;
pub use state::{State, StateType};
pub use environment::Environment;
pub use gc::GarbageCollector;
pub use operator::Operator;

use fn_print::fn_print;
use fn_main::fn_main;


pub fn compile<'ctx>(env: &Environment<'ctx>, expressions: &[Expression]) -> JitFunction<'ctx, unsafe extern "C" fn() -> ()> {

  // Create an execution engine
  let execution_engine = env.borrow_mut().module
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