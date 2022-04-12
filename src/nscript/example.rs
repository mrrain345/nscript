use inkwell::OptimizationLevel;
use inkwell::context::Context;

mod nscript;
use nscript::NScript;

pub fn example() {
  // Create a new context
  let context = Context::create();

  // Create a module
  let module = context.create_module("module");

  // Create an execution engine
  let execution_engine = module
    .create_jit_execution_engine(OptimizationLevel::None)
    .expect("Failed to create execution engine");
  
  // Create a new NScript instance
  let nscript = NScript {
    context: &context,
    module,
    builder: context.create_builder(),
    execution_engine,
  };

  // Compile a module
  let result = nscript.compile();
  let print = result.print;                         // fn print(i32) -> ()
  let add = result.add;                             // fn add(i64, i64) -> i64


  // Test
  unsafe {
    let x = 1;
    let y = 2;

    println!("{} + {} = {}", x, y, add.call(x, y));
    print.call(12345);
  }
}