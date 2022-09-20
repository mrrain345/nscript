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
pub use state::State;
pub use environment::Environment;
pub use gc::GarbageCollector;
pub use operator::Operator;

use fn_print::fn_print;
use fn_main::fn_main;

use self::fn_print::print_bool;
use self::fn_print::print_int;
use self::fn_print::print_num;


pub fn compile<'ctx>(env: &Environment<'ctx>, expressions: &[Expression]) -> JitFunction<'ctx, unsafe extern "C" fn() -> ()> {

  // Create an execution engine
  let execution_engine = env.borrow_mut().module
    .create_jit_execution_engine(OptimizationLevel::None)
    .expect("Failed to create execution engine");

  // Compile print functions
  let (fn_print_int, fn_print_num, fn_print_bool) = fn_print(env);
  execution_engine.add_global_mapping(&fn_print_int, print_int as usize);
  execution_engine.add_global_mapping(&fn_print_num, print_num as usize);
  execution_engine.add_global_mapping(&fn_print_bool, print_bool as usize);

  // Compile main function
  fn_main(env, expressions).expect("Failed to compile `main` function");

  // Return main function
  unsafe {
    execution_engine.get_function("main").expect("Failed to load `main` function")
  }
}