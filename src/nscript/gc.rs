use super::{Object};

#[derive(Debug)]
pub struct GarbageCollector<'ctx> {
  objects: Vec<Object<'ctx>>
}

impl<'ctx> GarbageCollector<'ctx> {
  pub fn new() -> Self {
    GarbageCollector {
      objects: Vec::new()
    }
  }

  pub fn add(&mut self, object: Object<'ctx>) -> &Object<'ctx> {
    self.objects.push(object);
    self.objects.last().unwrap()
  }

  pub fn collect(&mut self) {
    todo!()
  }
}