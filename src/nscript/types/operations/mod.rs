use self::traits::*;

use super::{BooleanType, NullType, ObjectType, NumberType, RefType};

pub mod traits;

mod any_type;
mod integer_type;
mod number_type;

// Default implementations
impl<'ctx> ArithmeticOps<'ctx> for BooleanType {}
impl<'ctx> LogicalOps<'ctx> for BooleanType {}
impl<'ctx> ComparisonOps<'ctx> for BooleanType {}

impl<'ctx> LogicalOps<'ctx> for NumberType {}
impl<'ctx> ComparisonOps<'ctx> for NumberType {}

impl<'ctx> ArithmeticOps<'ctx> for NullType {}
impl<'ctx> LogicalOps<'ctx> for NullType {}
impl<'ctx> ComparisonOps<'ctx> for NullType {}

impl<'ctx> ArithmeticOps<'ctx> for ObjectType<'ctx> {}
impl<'ctx> LogicalOps<'ctx> for ObjectType<'ctx> {}
impl<'ctx> ComparisonOps<'ctx> for ObjectType<'ctx> {}

impl<'ctx> ArithmeticOps<'ctx> for RefType<'ctx> {}
impl<'ctx> LogicalOps<'ctx> for RefType<'ctx> {}
impl<'ctx> ComparisonOps<'ctx> for RefType<'ctx> {}