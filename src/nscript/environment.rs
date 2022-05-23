use std::collections::HashMap;

use inkwell::{context::Context, module::Module, builder::Builder, values::{IntValue, FloatValue, FunctionValue, PointerValue}, basic_block::BasicBlock, types::StructType};

use super::{state::{State, StateValue}, StateType, AnyValue, Property, Type, AnyType, Class, Function};

#[derive(Debug)]
pub struct Environment<'ctx> {
  pub context: &'ctx Context,
  pub module: Module<'ctx>,
  pub builder: Builder<'ctx>,
  pub state: State<'ctx>,
}

impl<'ctx> Environment<'ctx> {
  pub fn new(context: &'ctx Context) -> Self {
    Environment {
      context,
      module: context.create_module("main"),
      builder: context.create_builder(),
      state: State::new(),
    }
  }

  pub fn integer(&self, value: i32) -> IntValue<'ctx> {
    self.context.i32_type().const_int(value as u64, false)
  }

  pub fn number(&self, value: f64) -> FloatValue<'ctx> {
    self.context.f64_type().const_float(value)
  }

  pub fn string(&self, value: &str) -> String {
    todo!()
  }

  pub fn boolean(&self, value: bool) -> IntValue<'ctx> {
    self.context.bool_type().const_int(value as u64, false)
  }

  pub fn null(&self) -> IntValue<'ctx> {
    self.context.i32_type().const_int(0, false)
  }


  // Scopes

  /// Creates a new scope on the top of the stack
  pub fn push_scope(&mut self) {
    self.state.push_scope();
  }

  /// Removes the topmost scope from the stack
  pub fn pop_scope(&mut self) {
    self.state.pop_scope();
  }

  /// Returns the topmost scope
  pub fn scope(&self) -> &HashMap<String, StateValue<'ctx>> {
    self.state.scope()
  }

  /// Returns the topmost scope mutably
  pub fn scope_mut(&mut self) -> &mut HashMap<String, StateValue<'ctx>> {
    self.state.scope_mut()
  }

  // Current block

  /// Get current block
  pub fn current_block(&self) -> BasicBlock<'ctx> {
    self.state.current_block()
  }

  /// Set current block
  pub fn set_current_block(&mut self, block: BasicBlock<'ctx>) {
    self.state.set_current_block(block);
  }

  // Labels

  /// Gets the value of the label from any scope
  pub fn get_label(&self, name: &str) -> Option<AnyValue<'ctx>> {
    self.state.get(name, Some(StateType::Label)).map(|(value, ..)| value)
  }

  /// Adds a new label to the topmost scope
  pub fn add_label(&mut self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    self.state.add(name, value, StateType::Label)
  }

  /// Changes the value of the label from any scope
  pub fn set_label(&mut self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    self.state.set(name, value, StateType::Label)
  }

  // Variables

  /// Gets the value of the variable from any scope
  pub fn get_variable(&self, name: &str) -> Option<AnyValue<'ctx>> {
    self.state.get(name, Some(StateType::Variable)).map(|(value, ..)| value)
  }

  /// Adds a new variable to the topmost scope
  pub fn add_variable(&mut self, name: String, ptr: PointerValue<'ctx>, type_: AnyType<'ctx>) -> Option<AnyValue<'ctx>> {
    self.state.add(name, AnyValue::Ptr{ ptr, type_ }, StateType::Variable)
  }

  /// Changes the value of the variable from any scope
  pub fn set_variable(&mut self, name: String, ptr: PointerValue<'ctx>, type_: AnyType<'ctx>) -> Option<AnyValue<'ctx>> {
    self.state.set(name, AnyValue::Ptr{ ptr, type_ }, StateType::Variable)
  }

  // Functions

  /// Gets the function from any scope
  pub fn get_function(&self, name: &str) -> Option<Box<Function<'ctx>>> {
    self.state.get(name, Some(StateType::Function)).map(|(value, ..)| value)
      .map(|value| value.into_function())
  }

  /// Adds a new function to the topmost scope
  pub fn add_function(&mut self, name: String, function: Function<'ctx>) -> Option<Box<Function<'ctx>>> {
    self.state.add(name.clone(), AnyValue::Fn(Box::new(function)), StateType::Function)
      .map(|value| value.into_function())
  }

  /// Changes the value of the function from any scope
  pub fn set_function(&mut self, name: String, function: Function<'ctx>) -> Option<Box<Function<'ctx>>> {
    self.state.set(name.clone(), AnyValue::Fn(Box::new(function)), StateType::Function)
      .map(|value| value.into_function())
  }

  // Classes

  /// Gets the class from any scope
  pub fn get_class(&self, name: &str) -> Option<AnyValue<'ctx>> {
    self.state.get(name, Some(StateType::Class)).map(|(value, ..)| value)
  }

  /// Adds a new class to the topmost scope
  pub fn add_class(&mut self, name: String, class: Class<'ctx>) -> Option<AnyValue<'ctx>> {
    let class = self.state.add_class(class);
    self.state.add(name.clone(), AnyValue::Class(class), StateType::Class)
  }

  /// Changes the value of the class from any scope
  pub fn set_class(&mut self, name: String, class: Class<'ctx>) -> Option<AnyValue<'ctx>> {
    let class = self.state.add_class(class);
    self.state.set(name.clone(), AnyValue::Class(class), StateType::Class)
  }
}
