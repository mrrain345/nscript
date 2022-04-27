use combine::{Parser, Stream};
use inkwell::values::{AnyValueEnum, IntValue, FloatValue};

pub struct AnyValue<'ctx> ( pub Option<AnyValueEnum<'ctx>> );

impl<'src, 'ctx: 'src, Input> Parser<Input> for AnyValue<'ctx> 
where
  Input: Stream {
  
  type Output = AnyValue<'ctx>;
  type PartialState = ();
}

impl<'src, 'ctx: 'src> From<Option<AnyValueEnum<'ctx>>> for AnyValue<'ctx> {
  fn from(value: Option<AnyValueEnum<'ctx>>) -> Self {
    AnyValue(value)
  }
}

impl<'src, 'ctx: 'src> From<IntValue<'ctx>> for AnyValue<'ctx> {
  fn from(value: IntValue<'ctx>) -> Self {
    AnyValue(Some(value.into()))
  }
}

impl<'src, 'ctx: 'src> From<FloatValue<'ctx>> for AnyValue<'ctx> {
  fn from(value: FloatValue<'ctx>) -> Self {
    AnyValue(Some(value.into()))
  }
}

impl<'src, 'ctx: 'src> From<()> for AnyValue<'ctx> {
  fn from(_: ()) -> Self {
    AnyValue(None)
  }
}

impl<'src, 'ctx: 'src> From<AnyValue<'ctx>> for Option<AnyValueEnum<'ctx>> {
  fn from(value: AnyValue<'ctx>) -> Self {
    value.0
  }
}

impl<'src, 'ctx: 'src> AnyValue<'ctx> {
  pub fn into_option(self) -> Option<AnyValueEnum<'ctx>> {
    self.0
  }

  pub fn is_some(&self) -> bool {
    self.0.is_some()
  }

  pub fn is_none(&self) -> bool {
    self.0.is_none()
  }

  pub fn unwrap(self) -> AnyValueEnum<'ctx> {
    self.0.unwrap()
  }
}