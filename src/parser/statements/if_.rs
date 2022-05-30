use combine::{Stream, parser::repeat, parser, optional};
use crate::parser::{Expression, tokens::*, expression};

parser! {
  /// Syntax:
  /// ```
  /// if <expression> { <expression>* } [else { <expression>* }]
  pub fn if_[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    ignore_newlines!(
      keyword(Keyword::If),
      expression(), // condition
      punctuator(Punctuator::LeftBrace),
      repeat::sep_end_by(expression(), terminator()), // then
      punctuator(Punctuator::RightBrace),
      optional(
        ignore_newlines!(
          keyword(Keyword::Else),
          punctuator(Punctuator::LeftBrace),
          repeat::sep_end_by(expression(), terminator()), // else
          punctuator(Punctuator::RightBrace),
        ).map(|(_, _, body, _)| body)
      ),
    )

    .map(|(_, condition, _, then, _, else_)| Expression::If {
      condition: Box::new(condition),
      then,
      else_: else_.unwrap_or(vec![]),
    })
  }
}