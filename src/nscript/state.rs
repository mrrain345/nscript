use std::collections::HashMap;

use inkwell::basic_block::BasicBlock;

use super::values::AnyValue;

#[derive(Debug)]
pub struct State<'ctx> {
  scopes: Vec<HashMap<String, AnyValue<'ctx>>>,
  current_block: Option<BasicBlock<'ctx>>,
}

impl<'ctx> State<'ctx> {
  pub fn new() -> Self {
    State {
      scopes: vec![HashMap::new()],
      current_block: None,
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
  pub fn scope(&self) -> &HashMap<String, AnyValue<'ctx>> {
    self.scopes.last().unwrap()
  }

  /// Returns the topmost scope mutably
  pub fn scope_mut(&mut self) -> &mut HashMap<String, AnyValue<'ctx>> {
    self.scopes.last_mut().unwrap()
  }

  // Current block

  /// Get current block
  pub fn current_block(&self) -> BasicBlock<'ctx> {
    self.current_block.clone().expect("Current block is not set")
  }

  /// Set current block
  pub fn set_current_block(&mut self, block: BasicBlock<'ctx>) {
    self.current_block = Some(block);
  }

  // Get/Add/Set

  /// Returns the value of the element from any scope
  pub fn get(&self, name: &str) -> Option<AnyValue<'ctx>> {
    // Find the first scope that contains the element
    let value = self.scopes.iter().rev()
      .find(|scope| scope.contains_key(name))
      .and_then(|scope| scope.get(name));

    // If the element doesn't exist, return None
    if value.is_none() { return None; }

    let value = value.unwrap();
    Some(value.clone())
  }

  /// Adds a new element to the topmost scope
  pub fn add(&mut self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    // Return if first scope contains the element
    if self.scope().contains_key(&name) { return None; }

    // Add the element to the current scope
    if self.scope_mut().insert(name.clone(), value.clone()).is_some() { return None; }
    Some(value)
  }

  /// Changes the value of the element from any scope
  pub fn set(&mut self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    // Find the first scope that contains the element
    let scope = self.scopes.iter_mut().rev()
      .find(|scope| scope.contains_key(&name));

    // If the element doesn't exist, return none
    if scope.is_none() { return None; }
    let scope = scope.unwrap();

    // Set new value in the same scope
    scope.insert(name, value.clone());
    Some(value)
  }
}