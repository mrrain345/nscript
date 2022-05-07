use std::collections::HashMap;

use inkwell::{values::{FunctionValue, PointerValue}};

use super::any_value::{AnyValue, AnyType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StateType {
  Label,
  Variable,
  Function,
}

#[derive(Debug)]
pub struct State<'ctx> {
  pub scopes: Vec<HashMap<String, (AnyValue<'ctx>, StateType)>>,
  pub current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> State<'ctx> {
  pub fn new() -> Self {
    State {
      scopes: vec![HashMap::new()],
      current_function: None,
    }
  }

  // Scopes

  /// Creates a new scope on the top of the stack
  pub fn push_scope(&mut self) {
    self.scopes.push(HashMap::new());
  }

  /// Removes the topmost scope from the stack
  pub fn pop_scope(&mut self) {
    self.scopes.pop();
  }

  /// Returns the topmost scope
  pub fn scope(&self) -> &HashMap<String, (AnyValue<'ctx>, StateType)> {
    self.scopes.last().unwrap()
  }

  /// Returns the topmost scope mutably
  pub fn scope_mut(&mut self) -> &mut HashMap<String, (AnyValue<'ctx>, StateType)> {
    self.scopes.last_mut().unwrap()
  }


  /// Returns the value of the element from any scope
  pub fn get(&self, name: &str, type_: Option<StateType>) -> Option<(AnyValue<'ctx>, StateType)> {
    // Find the first scope that contains the element
    let value = self.scopes.iter().rev()
      .find(|scope| scope.contains_key(name))
      .and_then(|scope| scope.get(name));

    // If the element doesn't exist, return None
    if value.is_none() { return None; }
    let value = value.unwrap();

    // If type is not specified, return the value
    if type_.is_none() { return Some(*value); }
    // If type is specified, check if it is the same
    if value.1 == type_.unwrap() { return Some(*value); }
    None
  }

  /// Adds a new element to the topmost scope
  pub fn add(&mut self, name: String, type_: StateType, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    // Check if any scope contains the element
    let scope = self.scopes.iter().rev()
      .find(|scope| scope.contains_key(&name));
    
    // If the element exists, return none
    if scope.is_some() { return None; }

    // Add the element to the current scope
    if self.scope_mut().insert(name, (value, type_)).is_some() { return None; }
    Some(value)
  }

  /// Changes the value of the element from any scope
  pub fn set(&mut self, name: String, type_: StateType, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    // Find the first scope that contains the element
    let scope = self.scopes.iter_mut().rev()
      .find(|scope| scope.contains_key(&name));

    // If the element doesn't exist, return none
    if scope.is_none() { return None; }
    let scope = scope.unwrap();

    // Set new value in the same scope
    scope.insert(name, (value, type_));
    Some(value)
  }
  
  // Labels

  /// Gets the value of the label from any scope
  pub fn get_label(&self, name: &str) -> Option<AnyValue<'ctx>> {
    self.get(name, Some(StateType::Label)).map(|(value, _)| value)
  }

  /// Adds a new label to the topmost scope
  pub fn add_label(&mut self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    self.add(name, StateType::Label, value)
  }

  /// Changes the value of the label from any scope
  pub fn set_label(&mut self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    self.set(name, StateType::Label, value)
  }

  // Variables

  /// Gets the value of the variable from any scope
  pub fn get_variable(&self, name: &str) -> Option<AnyValue<'ctx>> {
    self.get(name, Some(StateType::Variable)).map(|(value, _)| value)
  }

  /// Adds a new variable to the topmost scope
  pub fn add_variable(&mut self, name: String, ptr: PointerValue<'ctx>, type_: AnyType) -> Option<AnyValue<'ctx>> {
    self.add(name, StateType::Variable, AnyValue::Ptr{ ptr, type_ })
  }

  /// Changes the value of the variable from any scope
  pub fn set_variable(&mut self, name: String, ptr: PointerValue<'ctx>, type_: AnyType) -> Option<AnyValue<'ctx>> {
    self.set(name, StateType::Variable, AnyValue::Ptr{ ptr, type_ })
  }

  // Functions
  
  /// Gets the function from any scope
  pub fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
    self.get(name, Some(StateType::Function)).map(|(value, _)| value.into_function())
  }

  /// Adds a new function to the topmost scope
  pub fn add_function(&mut self, name: String, value: FunctionValue<'ctx>) -> Option<FunctionValue<'ctx>> {
    self.add(name, StateType::Function, AnyValue::Fn(value)).map(|v| v.into_function())
  }

  /// Changes the value of the function from any scope
  pub fn set_function(&mut self, name: String, value: FunctionValue<'ctx>) -> Option<FunctionValue<'ctx>> {
    self.set(name, StateType::Function, AnyValue::Fn(value)).map(|v| v.into_function())
  }
}