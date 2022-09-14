use super::{Environment, AnyValue, AnyType};

pub struct Operator;

impl Operator {
  pub fn add<'ctx>(env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyValue<'ctx>> {
    Self::common_type(env, left, right)
      .map(|type_| type_.op_add(env, &left, &right))
      .flatten()
  }



  fn common_type<'ctx>(env: &Environment<'ctx>, left: &AnyValue<'ctx>, right: &AnyValue<'ctx>) -> Option<AnyType<'ctx>> {
    if left.get_type() == right.get_type() { return Some(left.get_type()); }

    let cast = right.silent_cast(env, &left.get_type());
    if cast.is_some() { return Some(cast.unwrap().get_type()); }
    let cast = left.silent_cast(env, &right.get_type());
    if cast.is_some() { return Some(cast.unwrap().get_type()); }

    None
  }
}