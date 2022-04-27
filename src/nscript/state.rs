use std::collections::HashMap;

use inkwell::values::{AnyValueEnum, FunctionValue};

use super::any_value::AnyValue;

#[derive(Debug)]
pub struct State<'ctx> {
  pub values: HashMap<String, (StateType, AnyValueEnum<'ctx>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StateType {
  Label,
  Variable,
  Function,
}

impl<'ctx> State<'ctx> {
  pub fn new() -> Self {
    return State {
      values: HashMap::new(),
    };
  }

  pub fn get(&self, name: &str, type_: Option<StateType>) -> AnyValue<'ctx> {
    let value = self.values.get(name);
    if value.is_none() { return AnyValue(None); }
    let value = value.unwrap();

    if type_.is_none() { return Some(value.1).into(); }
    if value.0 == type_.unwrap() { return Some(value.1).into(); }
    return AnyValue(None);
  }

  pub fn add(&mut self, name: String, type_: StateType, value: AnyValueEnum<'ctx>) -> AnyValue<'ctx> {
    if self.values.get(&name).is_some() { return AnyValue(None); }
    if self.values.insert(name, (type_, value)).is_some() { return AnyValue(None); }
    return Some(value).into();
  }

  pub fn set(&mut self, name: String, type_: StateType, value: AnyValueEnum<'ctx>) {
    self.values.insert(name, (type_, value));
  }
  
  // Label
  pub fn get_label(&self, name: &str) -> AnyValue<'ctx> {
    self.get(name, Some(StateType::Label))
  }

  pub fn add_label(&mut self, name: String, value: AnyValueEnum<'ctx>) -> AnyValue<'ctx> {
    self.add(name, StateType::Label, value)
  }

  pub fn set_label(&mut self, name: String, value: AnyValueEnum<'ctx>) {
    self.set(name, StateType::Label, value);
  }

  // Variable
  pub fn get_variable(&self, name: &str) -> AnyValue<'ctx> {
    self.get(name, Some(StateType::Variable))
  }

  pub fn add_variable(&mut self, name: String, value: AnyValueEnum<'ctx>) -> AnyValue<'ctx> {
    self.add(name, StateType::Variable, value)
  }

  pub fn set_variable(&mut self, name: String, value: AnyValueEnum<'ctx>) {
    self.set(name, StateType::Variable, value);
  }

  // Function
  pub fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
    self.get(name, Some(StateType::Function)).into_option().map(|v| v.into_function_value())
  }

  pub fn add_function(&mut self, name: String, value: FunctionValue<'ctx>) -> Option<FunctionValue<'ctx>> {
    self.add(name, StateType::Function, value.into()).into_option().map(|v| v.into_function_value())
  }

  pub fn set_function(&mut self, name: String, value: FunctionValue<'ctx>) {
    self.set(name, StateType::Function, value.into());
  }
}