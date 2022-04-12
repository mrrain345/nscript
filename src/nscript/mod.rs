use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction, UnsafeFunctionPointer};
pub struct NScript<'ctx> {
  pub context: &'ctx Context,
  pub module: Module<'ctx>,
  pub builder: Builder<'ctx>,
  pub execution_engine: ExecutionEngine<'ctx>,
}

mod print;
mod add;

pub struct NScriptResult<'ctx> {
  pub print: JitFunction<'ctx, unsafe extern "C" fn(i32) -> ()>,
  pub add: JitFunction<'ctx, unsafe extern "C" fn(i64, i64) -> i64>,
}

impl<'ctx> NScript<'ctx> {
  fn load_function<F: UnsafeFunctionPointer>(&self, name: &str) -> JitFunction<F> {
    unsafe {
      return self.execution_engine.get_function(name).expect(format!("Failed to load `{}` function", name).as_str());
    }
  }

  pub fn compile(&self) -> NScriptResult {
    self.fn_print().expect(           "Failed to compile `print` function");
    self.fn_add().expect(             "Failed to compile `add` function");
    
    return NScriptResult {
      print:                self.load_function("print"),
      add:                  self.load_function("add"),
    };
  }
}