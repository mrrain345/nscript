use combine::{Stream, parser, optional};
use crate::parser::{Expression, tokens::*, operations::operation};

parser! {
  pub fn return_[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    keyword(Keyword::Return).with(
      optional(operation()), // value
    )
    .map(|value| Expression::Return (
      Box::new(value.unwrap_or(Expression::Null))
    ))
  }
}