use combine::{parser, RangeStream, optional};

use crate::{parser::{expressions::Expression, operations::operation}};
use crate::tokenizer::*;

parser! {
  pub fn return_['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    keyword("return").with(
      optional(operation()), // value
    )
    .map(|value| Expression::Return (
      Box::new(value.unwrap_or(Expression::Null))
    ))
  }
}