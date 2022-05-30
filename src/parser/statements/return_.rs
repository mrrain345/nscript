use combine::{Stream, parser, optional};
use crate::parser::{Expression, tokens::*, expression};

parser! {
  /// Syntax:
  /// ```
  /// return [<expression>]
  pub fn return_[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    ignore_newlines!(
      keyword(Keyword::Return),
      optional(expression()), // value
    )

    .map(|(_, value)| Expression::Return (
      Box::new(value.unwrap_or(Expression::Null))
    ))
  }
}