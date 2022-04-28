use std::collections::HashMap;

use inkwell::{values::{FunctionValue, PointerValue}};

use super::any_value::{AnyValue, AnyType};

#[derive(Debug)]
pub struct State<'ctx> {
  pub values: HashMap<String, (AnyValue<'ctx>, StateType)>,
  pub current_function: Option<FunctionValue<'ctx>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StateType {
  Label,
  Variable,
  Function,
}

impl<'ctx> State<'ctx> {
  pub fn new() -> Self {
    State {
      values: HashMap::new(),
      current_function: None,
    }
  }

  pub fn get(&self, name: &str, type_: Option<StateType>) -> Option<(AnyValue<'ctx>, StateType)> {
    let value = self.values.get(name);
    if value.is_none() { return None; }
    let value = value.unwrap();

    if type_.is_none() { return Some(*value); }
    if value.1 == type_.unwrap() { return Some(*value); }
    None
  }

  pub fn add(&mut self, name: String, type_: StateType, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    if self.values.get(&name).is_some() { return None; }
    if self.values.insert(name, (value, type_)).is_some() { return None; }
    Some(value)
  }

  pub fn set(&mut self, name: String, type_: StateType, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    if self.values.get(&name).is_none() { return None; }
    self.values.insert(name, (value, type_));
    Some(value)
  }
  
  // Label
  pub fn get_label(&self, name: &str) -> Option<AnyValue<'ctx>> {
    self.get(name, Some(StateType::Label)).map(|(value, _)| value)
  }

  pub fn add_label(&mut self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    self.add(name, StateType::Label, value)
  }

  pub fn set_label(&mut self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    self.set(name, StateType::Label, value)
  }

  // Variable
  pub fn get_variable(&self, name: &str) -> Option<AnyValue<'ctx>> {
    self.get(name, Some(StateType::Variable)).map(|(value, _)| value)
  }

  pub fn add_variable(&mut self, name: String, ptr: PointerValue<'ctx>, type_: AnyType) -> Option<AnyValue<'ctx>> {
    self.add(name, StateType::Variable, AnyValue::Ptr{ ptr, type_ })
  }

  pub fn set_variable(&mut self, name: String, ptr: PointerValue<'ctx>, type_: AnyType) -> Option<AnyValue<'ctx>> {
    self.set(name, StateType::Variable, AnyValue::Ptr{ ptr, type_ })
  }

  // Function
  pub fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
    self.get(name, Some(StateType::Function)).map(|(value, _)| value.into_function())
  }

  pub fn add_function(&mut self, name: String, value: FunctionValue<'ctx>) -> Option<FunctionValue<'ctx>> {
    self.add(name, StateType::Function, AnyValue::Fn(value)).map(|v| v.into_function())
  }

  pub fn set_function(&mut self, name: String, value: FunctionValue<'ctx>) -> Option<FunctionValue<'ctx>> {
    self.set(name, StateType::Function, AnyValue::Fn(value)).map(|v| v.into_function())
  }
}