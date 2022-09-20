use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};

use inkwell::{context::Context, module::Module, builder::Builder, values::PointerValue, basic_block::BasicBlock};

use super::{state::State, AnyType, GarbageCollector, values::{AnyValue, Function, Class, Ref}};

#[derive(Debug)]
pub struct EnvironmentData<'ctx> {
  pub context: &'ctx Context,
  pub module: Module<'ctx>,
  pub builder: Builder<'ctx>,
  pub state: State<'ctx>,
  pub gc: GarbageCollector<'ctx>,
}

#[derive(Debug)]
pub struct Environment<'ctx> (RwLock<EnvironmentData<'ctx>>);

impl<'ctx> Environment<'ctx> {
  pub fn new(context: &'ctx Context) -> Self {
    let env = EnvironmentData {
      context,
      module: context.create_module("main"),
      builder: context.create_builder(),
      state: State::new(),
      gc: GarbageCollector::new(),
    };

    Environment(RwLock::new(env))
  }

  pub fn borrow_mut(&self) -> RwLockWriteGuard<EnvironmentData<'ctx>> {
    self.0.write().unwrap()
  }

  pub fn borrow(&self) -> RwLockReadGuard<EnvironmentData<'ctx>> {
    self.0.read().unwrap()
  }

  // Current block

  /// Get current block
  pub fn current_block(&self) -> BasicBlock<'ctx> {
    self.borrow().state.current_block()
  }

  /// Set current block
  pub fn set_current_block(&self, block: BasicBlock<'ctx>) {
    self.borrow_mut().state.set_current_block(block);
  }

  /// Gets the value from any scope
  pub fn get(&self, name: &str) -> Option<AnyValue<'ctx>> {
    self.borrow().state.get(name)
  }

  /// Adds a new value to the topmost scope
  pub fn add(&self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    self.borrow_mut().state.add(name, value)
  }

  /// Changes the value from any scope
  pub fn set(&self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    self.borrow_mut().state.set(name, value)
  }

//   // Labels

//   /// Gets the value of the label from any scope
//   pub fn get_label(&self, name: &str) -> Option<AnyValue<'ctx>> {
//     self.borrow().state.get(name)
//   }

//   /// Adds a new label to the topmost scope
//   pub fn add_label(&self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.borrow_mut().state.add(name, value)
//   }

//   // Variables

//   /// Gets the value of the variable from any scope
//   pub fn get_variable(&self, name: &str) -> Option<AnyValue<'ctx>> {
//     self.borrow().state.get(name)
//   }

//   /// Adds a new variable to the topmost scope
//   pub fn add_variable(&self, name: String, ptr: PointerValue<'ctx>, type_: AnyType<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.borrow_mut().state.add(name, Ref::new(type_, ptr).into())
//   }

//   /// Changes the value of the variable from any scope
//   pub fn set_variable(&self, name: String, ptr: PointerValue<'ctx>, type_: AnyType<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.borrow_mut().state.set(name, Ref::new(type_, ptr).into())
//   }

//   // Functions

//   /// Gets the function from any scope
//   pub fn get_function(&self, name: &str) -> Option<Function<'ctx>> {
//     self.borrow().state.get(name)
//       .map(|value| value.into())
//   }

//   /// Adds a new function to the topmost scope
//   pub fn add_function(&self, name: String, function: Function<'ctx>) -> Option<Function<'ctx>> {
//     self.borrow_mut().state.add(name.clone(), AnyValue::Function(function))
//       .map(|value| value.into())
//   }

//   /// Changes the value of the function from any scope
//   pub fn set_function(&self, name: String, function: Function<'ctx>) -> Option<Function<'ctx>> {
//     self.borrow_mut().state.set(name.clone(), AnyValue::Function(function))
//       .map(|value| value.into())
//   }

//   // Classes

//   /// Gets the class from any scope
//   pub fn get_class(&self, name: &str) -> Option<AnyValue<'ctx>> {
//     self.borrow().state.get(name)
//   }

//   /// Adds a new class to the topmost scope
//   pub fn add_class(&self, name: String, class: Class<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.borrow_mut().state.add(name.clone(), AnyValue::Class(class))
//   }

//   /// Changes the value of the class from any scope
//   pub fn set_class(&self, name: String, class: Class<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.borrow_mut().state.set(name.clone(), AnyValue::Class(class))
//   }
// }


// impl<'ctx> EnvironmentData<'ctx> {
  
//   // Current block

//   /// Get current block
//   pub fn current_block(&self) -> BasicBlock<'ctx> {
//     self.state.current_block()
//   }

//   /// Set current block
//   pub fn set_current_block(&mut self, block: BasicBlock<'ctx>) {
//     self.state.set_current_block(block);
//   }

//   // Labels

//   /// Gets the value of the label from any scope
//   pub fn get_label(&self, name: &str) -> Option<AnyValue<'ctx>> {
//     self.state.get(name)
//   }

//   /// Adds a new label to the topmost scope
//   pub fn add_label(&mut self, name: String, value: AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.state.add(name, value)
//   }

//   // Variables

//   /// Gets the value of the variable from any scope
//   pub fn get_variable(&self, name: &str) -> Option<AnyValue<'ctx>> {
//     self.state.get(name)
//   }

//   /// Adds a new variable to the topmost scope
//   pub fn add_variable(&mut self, name: String, ptr: PointerValue<'ctx>, type_: AnyType<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.state.add(name, Ref::new(type_, ptr).into())
//   }

//   /// Changes the value of the variable from any scope
//   pub fn set_variable(&mut self, name: String, ptr: PointerValue<'ctx>, type_: AnyType<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.state.set(name, Ref::new(type_, ptr).into())
//   }

//   // Functions

//   /// Gets the function from any scope
//   pub fn get_function(&self, name: &str) -> Option<Function<'ctx>> {
//     self.state.get(name)
//       .map(|value| value.into())
//   }

//   /// Adds a new function to the topmost scope
//   pub fn add_function(&mut self, name: String, function: Function<'ctx>) -> Option<Function<'ctx>> {
//     self.state.add(name.clone(), AnyValue::Function(function))
//       .map(|value| value.into())
//   }

//   /// Changes the value of the function from any scope
//   pub fn set_function(&mut self, name: String, function: Function<'ctx>) -> Option<Function<'ctx>> {
//     self.state.set(name.clone(), AnyValue::Function(function))
//       .map(|value| value.into())
//   }

//   // Classes

//   /// Gets the class from any scope
//   pub fn get_class(&self, name: &str) -> Option<AnyValue<'ctx>> {
//     self.state.get(name)
//   }

//   /// Adds a new class to the topmost scope
//   pub fn add_class(&mut self, name: String, class: Class<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.state.add(name.clone(), AnyValue::Class(class))
//   }

//   /// Changes the value of the class from any scope
//   pub fn set_class(&mut self, name: String, class: Class<'ctx>) -> Option<AnyValue<'ctx>> {
//     self.state.set(name.clone(), AnyValue::Class(class))
//   }
}