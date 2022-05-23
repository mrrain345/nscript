use inkwell::values::FunctionValue;

use super::{AnyType, Environment, AnyValue};

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'ctx> {
  fn_value: FunctionValue<'ctx>,
  name: Option<String>,
  args: Vec<(String, AnyType<'ctx>)>,
  return_type: AnyType<'ctx>,
}

impl<'ctx> Function<'ctx> {
  pub fn new(fn_value: FunctionValue<'ctx>, name: Option<String>, args: Vec<(String, AnyType<'ctx>)>, return_type: AnyType<'ctx>) -> Self {
    Function {
      fn_value,
      name,
      args,
      return_type,
    }
  }

  pub fn name(&self) -> Option<&str> {
    self.name.as_deref()
  }

  pub fn name_or_default(&self) -> &str {
    self.name.as_deref().unwrap_or("<function>")
  }

  pub fn function_value(&self) -> FunctionValue<'ctx> {
    self.fn_value
  }

  pub fn args(&self) -> &[(String, AnyType<'ctx>)] {
    &self.args
  }

  pub fn return_type(&self) -> &AnyType<'ctx> {
    &self.return_type
  }

  pub fn call(&self, env: &mut Environment<'ctx>, args: &[AnyValue<'ctx>]) -> AnyValue<'ctx> {
    // Get the function arguments
    let mut fn_args = vec![];

    // Check if the number of arguments is correct
    if args.len() != self.args.len() {
      panic!("Parser error: {} takes {} arguments, but {} were given", self.name_or_default(), self.args.len(), args.len());
    }

    // Check if the arguments types are correct and convert them
    for (arg, (_, type_)) in args.iter().zip(self.args.iter()) {
      let arg = arg.silent_cast(env, type_).expect(format!("Failed to cast argument `{:?}` to `{:?}`", arg, type_).as_str());
      fn_args.push(arg.into_llvm_basic_value().into());
    }

    // Call the function
    let result = env.builder.build_call(self.function_value(), &fn_args[..], self.name_or_default()).try_as_basic_value();

    // Return the result
    if let Some(_) = result.right() {
      AnyValue::Null
    } else {
      let result = result.left().unwrap();
      AnyValue::from_basic_value(self.return_type(), result)
    }
  }
}