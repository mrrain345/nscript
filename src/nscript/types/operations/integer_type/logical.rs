use crate::nscript::{types::{operations::traits::LogicalOps, IntegerType}, Environment, AnyValue, Type};

impl<'ctx> LogicalOps<'ctx> for IntegerType {}