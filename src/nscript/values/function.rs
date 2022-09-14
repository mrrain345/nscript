use std::{rc::Rc, ops::Deref};

use inkwell::values::{FunctionValue, PointerValue, BasicValueEnum, BasicMetadataValueEnum};

use crate::nscript::{AnyType, Environment, types::FunctionType};

use super::{value::Value, AnyValue};

#[derive(Debug, PartialEq)]
pub struct FunctionData<'ctx> {
  fn_value: FunctionValue<'ctx>,
  name: Option<String>,
  args: Vec<(String, AnyType<'ctx>)>,
  return_type: AnyType<'ctx>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'ctx> ( Rc<FunctionData<'ctx>> );

impl<'ctx> Deref for Function<'ctx> {
  type Target = FunctionData<'ctx>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'ctx> Function<'ctx> {
  pub fn new(fn_value: FunctionValue<'ctx>, name: Option<String>, args: Vec<(String, AnyType<'ctx>)>, return_type: AnyType<'ctx>) -> Self {
    Function(Rc::new(FunctionData {
      fn_value,
      name,
      args,
      return_type,
    }))
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

  pub fn call(&self, env: &Environment<'ctx>, args: &[AnyValue<'ctx>]) -> AnyValue<'ctx> {
    // Get the function arguments
    let mut fn_args: Vec<BasicMetadataValueEnum> = vec![];

    // Check if the number of arguments is correct
    if args.len() != self.args.len() {
      panic!("Parser error: {} takes {} arguments, but {} were given", self.name_or_default(), self.args.len(), args.len());
    }

    // Check if the arguments types are correct and convert them
    for (arg, (_, type_)) in args.iter().zip(self.args.iter()) {
      let arg = if arg.get_type() != *type_ {
        arg.silent_cast(env, type_).expect(format!("Failed to cast argument `{}` to `{}`", arg, type_).as_str())
      } else {
        arg.clone()
      };

      fn_args.push(arg.llvm_basic_value(env).unwrap().into());
    }

    // Call the function
    let result = env.borrow_mut().builder.build_call(self.function_value(), &fn_args[..], self.name_or_default()).try_as_basic_value();

    // Return the result
    if let Some(_) = result.right() {
      AnyValue::Null
    } else {
      let result = result.left().unwrap();
      self.return_type.create_value(env, result.into())
    }
  }
}

impl<'ctx> Value<'ctx> for Function<'ctx> {
  type Type = FunctionType;
  type LLVMValue = inkwell::values::FunctionValue<'ctx>;

  fn allocate(&self, env: &Environment<'ctx>) -> PointerValue<'ctx> {
    todo!()
  }

  fn store(&self, env: &Environment<'ctx>, ptr: PointerValue<'ctx>) {
    todo!()
  }

  fn get_type(&self) -> AnyType<'ctx> {
    todo!()
  }

  fn llvm_value(&self, env: &Environment<'ctx>) -> Self::LLVMValue {
    self.fn_value
  }

  fn llvm_basic_value(&self, env: &Environment<'ctx>) -> Option<BasicValueEnum<'ctx>> {
    None
  }
}

impl<'ctx> Into<AnyValue<'ctx>> for Function<'ctx> {
  fn into(self) -> AnyValue<'ctx> {
    AnyValue::Function(self)
  }
}

impl<'ctx> From<AnyValue<'ctx>> for Function<'ctx> {
  fn from(value: AnyValue<'ctx>) -> Self {
    match value {
      AnyValue::Function(value) => value,
      _ => panic!("Invalid type"),
    }
  }
}