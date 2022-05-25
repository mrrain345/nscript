use inkwell::{execution_engine::JitFunction, OptimizationLevel};

use crate::parser::Expression;

mod fn_print;
mod fn_main;

mod any_value;
mod any_type;
mod state;
mod environment;
mod type_;
mod class;
mod object;
mod function;
mod gc;

pub use any_value::AnyValue;
pub use any_type::AnyType;
pub use state::{State, StateType};
pub use environment::Environment;
pub use type_::Type;
pub use class::{Class, Property};
pub use object::Object;
pub use function::Function;
pub use gc::GarbageCollector;

use fn_print::fn_print;
use fn_main::fn_main;


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